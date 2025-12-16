use crate::data::connection_pool::ConnectionPool;
use std::sync::Arc;

pub struct OrderRepository {
    connection_pool:Arc<ConnectionPool>,
}
impl OrderRepository {
    /// This is all far to overcompliciated but I am trying to figure out how lifetimes work
    /// I have now removed it anyway
    pub fn new(given_pool: Arc<ConnectionPool>) -> Self {
        Self {
            connection_pool: given_pool
        }
    }
}