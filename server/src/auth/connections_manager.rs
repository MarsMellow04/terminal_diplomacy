use crate::data::connection_pool::ConnectionPool;
use std::sync::Arc;

use crate::data::user::{Entity as User, Column as UserColumn, Model as UserModel, ActiveModel};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use sea_orm::DbErr;


// for adding
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Database, DatabaseConnection, Set};
use common::hash::hash_password;
use common::hash::verify_password;

pub struct ConnectionsManager {
    pool: Arc<ConnectionPool>,
}

impl ConnectionsManager {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }

    pub async fn handle_login(&self, username: String, password: String) -> Result<Option<UserModel>, sea_orm::DbErr> {
        let conn = self.pool.get_connection();
        println!("This is the hashed password: {}", hash_password(&password));
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
        let user_model = ActiveModel {
            id: NotSet,
            username: Set(username),
            password_hash: Set(hashed_password),
            created_at: NotSet,
        };

        user_model.insert(conn).await?;
        Ok(())

}

} 

