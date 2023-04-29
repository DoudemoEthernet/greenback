use std::sync::Arc;

use worker::Database;

pub mod account;
pub mod task;

#[derive(Clone)]
pub struct DatabaseWrapper(Arc<Database>);

impl DatabaseWrapper {
    pub fn new(database: Database) -> Self {
        DatabaseWrapper(Arc::new(database))
    }
}
