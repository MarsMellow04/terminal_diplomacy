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

// Adding stuff for game manager 
use crate::game::game_service::{self, GameService};

pub struct ConnectionsManager {
    pool: Arc<ConnectionPool>, // This in teh future should just be replaced with the proper srvices
    game_service: Arc<GameService>,
    order_service: Arc<OrderService>
}

impl ConnectionsManager {
    pub fn new(pool: Arc<ConnectionPool>, game_service: Arc<GameService>, order_service: Arc<OrderService>) -> Self {
        Self { pool, game_service, order_service }
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

   
    pub async fn handle_registration(&self, username: String, password: String) -> Result<(), sea_orm::DbErr> {
        let conn = self.pool.get_connection();
        let hashed_password = hash_password(&password);
        let user_model = ActiveUserModel {
            user_id: NotSet,
            username: Set(username),
            password_hash: Set(hashed_password),
            created_at: NotSet,
        };

        user_model.insert(conn).await?;
        Ok(())
    }

    pub async fn handle_join(&self, game_str: &str ) {
        let game_id = match Uuid::parse_str(game_str) {
            Ok(id) => id,
            Err(e) => {
                eprint!("This is annoying {e}");
                return;
            }
        };

        // Time being just making a fake user id:
        let user_id = Uuid::new_v4();
        match self.game_service.join_game( &game_id,user_id ).await {
            Ok(()) => {println!("Game joined succesffully ");}
            Err(e) => {
                println!("lol not dealing with this {e}");
                return;
            }
        }
    }

    pub async fn handle_create(&self) {
        match self.game_service.create_game().await {
            game_id => {
                println!("Game created: {}", game_id)
            }
        }
    }

    pub async fn handle_order_submission(&self, order_str: &str, game_str: &str) {
        // Time being just making a fake user id:
        let user_id = Uuid::new_v4();
        let Ok(game_id) = Uuid::parse_str(game_str) else {
            eprintln!("game_id parsed is not a uuid");
            return;
        };

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

