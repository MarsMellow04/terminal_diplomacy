use postgres::{Client, NoTls, Error};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Database, DatabaseConnection, Set};
pub mod user;

use user::{ActiveModel as UserModel, Entity as User};
use common::hash::hash_password;
use sea_orm::EntityTrait;

pub mod connection_pool;

pub async fn add_user(username: String, password: String, connection: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let hashed_password = hash_password(&password);
    let user_model = UserModel {
        id: NotSet,
        username: Set(username),
        password_hash: Set(hashed_password),
        created_at: NotSet,
    };

    user_model.insert(connection).await?;
    Ok(())

}