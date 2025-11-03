use sea_orm::{Database, DatabaseConnection,};
use std::sync::Arc;


use crate::data::user::{Entity as User};
use sea_orm::EntityTrait;

// ConnectionPool 
#[derive(Clone)]
pub struct ConnectionPool {
    connection: Arc<DatabaseConnection>,
}

impl ConnectionPool {
    pub async fn connect(connection_string: &str) -> Self {
        let connection = Arc::new(Database::connect(connection_string).await.expect("Connection has failed"));
        //This part is just debug!
        let all_users = User::find().all(&*connection).await.unwrap();
        for user in all_users {
            println!("This is the user {:?}", user);
        }
        Self { connection }
    } 

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

}