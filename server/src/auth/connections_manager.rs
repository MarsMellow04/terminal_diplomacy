use crate::data::connection_pool::ConnectionPool;
use std::sync::Arc;

use crate::data::user::{Entity as User, Column as UserColumn, Model as UserModel, ActiveModel as ActiveUserModel};
use crate::data::game::{Entity as Game, Column as GameColumn, Model as GameModel, ActiveModel as ActiveGameModel};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use sea_orm::DbErr;


// for adding
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Database, DatabaseConnection, Set};
use common::hash::hash_password;
use common::hash::verify_password;

// Adding stuff for game manager 
use crate::game::game_service::{self, GameService};

pub struct ConnectionsManager {
    pool: Arc<ConnectionPool>, // This in teh future should just be replaced with the proper srvices
    game_service: Arc<GameService>
}

impl ConnectionsManager {
    pub fn new(pool: Arc<ConnectionPool>, game_service: Arc<GameService>) -> Self {
        Self { pool, game_service }
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
    pub async fn handle_join(&self, game_id: i32 ) -> Result<Option<GameModel>, sea_orm::DbErr>{
        let conn = self.pool.get_connection();
        // neeed to figure out how I do this, I 
        if let Some(game) = Game::find()
            .filter(GameColumn::GameId.eq(game_id))
            .one(conn)
            .await?
            {
                return Ok( Some(game));
            }   
        Ok( None )
    }

    pub async fn handle_create(&self) {
        match self.game_service.create_game().await {
            game_id => {
                println!("Game created: {}", game_id)
            }
        }
    }
} 

