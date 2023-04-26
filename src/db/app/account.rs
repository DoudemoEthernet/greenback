use async_trait::async_trait;
use serde::Deserialize;
use worker::{worker_sys::D1Result, D1Result, Database};

use crate::db::{
    entity::account::{AccountId, Credential, Username},
    error::DatabaseError,
    repository::account::CredentialRepository,
};

use super::DatabaseWrapper;

struct D1AccountDatabase {
    pub db: DatabaseWrapper,
}

impl D1AccountDatabase {
    pub fn new(database: Database) -> Self {
        Self {
            db: DatabaseWrapper::new(database),
        }
    }
}

#[derive(Debug, Deserialize)]
struct InternalData {
    username: String,
    password: String,
}

impl InternalData {
    fn to_credential(self) -> Credential {
        Credential::new(self.username, self.password)
    }
}

#[async_trait(?Send)]
impl CredentialRepository for D1AccountDatabase {
    async fn create(&self, create: &Credential) -> Result<(), DatabaseError> {
        let query = "INSERT INTO credentials (username, password) VALUES (?, ?);";
        self.db
            .0
            .prepare(query)
            .bind(&[
                create.username().as_ref().into(),
                create.password().as_ref().into(),
            ])
            .map_err(DatabaseError::TransactionError)?
            .run()
            .await
            .map_err(DatabaseError::TransactionError)?;
        Ok(())
    }

    async fn get(&self, username: &Username) -> Result<Credential, DatabaseError> {}
    async fn delete(&self, username: &Username) -> Result<(), DatabaseError> {}
}
