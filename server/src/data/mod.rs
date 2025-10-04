use postgres::{Client, NoTls, Error};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Database, DatabaseConnection, Set};
mod user;

use user::{ActiveModel as UserModel, Entity as User};
use common::hash::hash_password;
use sea_orm::EntityTrait;

pub async fn start_database() -> Result<DatabaseConnection, Error> {
    // Start database locally with 
    // docker run --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_USER=postgres -e POSTGRES_DB=terminal_diplomacy -p 5432:5432 -d postgres
    let connection_string = "postgresql://postgres:mysecretpassword@localhost/terminal_diplomacy";

    let connection = Database::connect(connection_string).await.unwrap();
    let all_users = User::find().all(&connection).await.unwrap();
    for user in all_users {
        println!("This is the user {:?}", user);
    }


    Ok(connection)

}

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