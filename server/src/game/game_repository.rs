use std::sync::Arc;

use sea_orm::{ActiveModelTrait, DbErr};
use sea_orm::ActiveValue::{Set, NotSet};
use sea_orm::DatabaseConnection;
use sea_orm::error;
use uuid::Uuid;

//  The Game Model
use crate::data::connection_pool::ConnectionPool;
use crate::data::game::ActiveModel;
use crate::data::game::GamePhase;

pub struct GameRepository {
    connection_pool:Arc<ConnectionPool>,
}
impl GameRepository {
    /// This is all far to overcompliciated but I am trying to figure out how lifetimes work
    /// I have now removed it anyway
    pub fn new(given_pool: Arc<ConnectionPool>) -> Self {
        Self {
            connection_pool: given_pool
        }
    }

    pub async fn insert_game(&self, game_id: Uuid) -> Result<(), DbErr>{
        let game_year: i32 = 1901;
        let conn: &DatabaseConnection = self.connection_pool.get_connection();
        let game_model: ActiveModel = ActiveModel {
            game_id: NotSet,
            name: Set(game_id.to_string()),
            year: Set(game_year),
            game_phase: Set(GamePhase::SpringMovement),
            created_at: NotSet
        };
        game_model.insert(conn).await?;
        Ok(())
    }
}