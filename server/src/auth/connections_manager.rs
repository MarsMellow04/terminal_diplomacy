use crate::data::connection_pool::ConnectionPool;
use crate::order::order_service::OrderService;
use std::sync::Arc;

use crate::data::user::{Entity as User, Column as UserColumn, Model as UserModel, ActiveModel as ActiveUserModel};
use crate::data::game::{self, ActiveModel as ActiveGameModel, Column as GameColumn, Entity as Game, Model as GameModel};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use sea_orm::DbErr;
use uuid::{Uuid};


// for adding
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Database, DatabaseConnection, Set};
use common::hash::hash_password;
use common::hash::verify_password;
use crate::auth::session::SessionStore;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

// Adding stuff for game manager 
use crate::game::game_service::{self, GameService};

pub type SharedSessionStore = Arc<RwLock<dyn SessionStore>>;

pub struct ConnectionsManager {
    pool: Arc<ConnectionPool>, // This in teh future should just be replaced with the proper srvices
    session_store: SharedSessionStore,
    game_service: Arc<GameService>,
    order_service: Arc<OrderService>
}

impl ConnectionsManager {
    pub fn new(pool: Arc<ConnectionPool>, session_store: SharedSessionStore, game_service: Arc<GameService>, order_service: Arc<OrderService>) -> Self {
        Self { pool, session_store, game_service, order_service }
    }

    pub async fn handle_login(&self, username: String, password: String) -> Result<Option<UserModel>, sea_orm::DbErr> {
        let conn = self.pool.get_connection();
        if let Some(user) = User::find()
            .filter(UserColumn::Username.eq(username))
            .one(conn)
            .await?
            {
                if verify_password(&password, &user.password_hash) {
                    return Ok(Some(user));
                }
                
            }
        Ok( None )
    }

   
    pub async fn handle_registration(&self, username: String, password: String) -> Result<Uuid, sea_orm::DbErr> {
        let conn = self.pool.get_connection();
        let hashed_password = hash_password(&password);
        let user = Uuid::new_v4();
        let user_model = ActiveUserModel {
            user_id: NotSet,
            username: Set(username),
            password_hash: Set(hashed_password),
            created_at: NotSet,
        };

        user_model.insert(conn).await?;

        // Create the session for the user 
        let mut session_store = self.session_store.write().await;
        let res = session_store.create(user);
        Ok(res)
    }

    pub async fn handle_join(&self, game_str: &str, session_id: Uuid ) {
        let game_id = match Uuid::parse_str(game_str) {
            Ok(id) => id,
            Err(e) => {
                eprint!("This is annoying {e}");
                return;
            }
        };

        // The time is now! From the message it sends over the session:
        let mut session_store = self.session_store.write().await;
        let user_session = session_store.get_mut(&session_id).expect("Something has gone wrong and the session is not found");
        
        match self.game_service.join_game( &game_id,user_session.user ).await {
            Ok(()) => {println!("Game joined succesffully ");}
            Err(e) => {
                println!("lol not dealing with this {e}");
                return;
            }
        }

        user_session.current_game = Some(game_id);
    }

    pub async fn handle_create(&self, session_id: Uuid) {
        let game_id = self.game_service.create_game().await;
        // UAtomatically add the user to the game 
        let mut session_store = self.session_store.write().await;
        let user_session = session_store.get_mut(&session_id).expect("Something has gone wrong and the session is not found");
        user_session.current_game = Some(game_id);
    }

    pub async fn handle_order_submission(&self, order_str: &str, session_id: Uuid) {
        // Time being just making a fake user id:
        let session_store = self.session_store.read().await;
        let user_session = session_store.get(&session_id).expect("Something has gone wrong and the session is not found");
        let user_id = user_session.user;
        let game_id = user_session.current_game.expect("To do this there needs to be. agme");

        // Parse the order_str 
        let Ok(orders) = serde_json::from_str(order_str) else {
            eprintln!("orders failed to be parse");
            return;
        };
        
        match self.order_service.send_order(user_id, orders, &game_id).await {
            Ok(()) => {println!("Game joined succesffully ");}
            Err(e) => {
                println!("lol not dealing with this {e}");
                return;
            }
        };
    }
} 

