use async_trait::async_trait;

use crate::db::{
    entity::account::{Credential, Username},
    error::DatabaseError,
};

#[async_trait(?Send)]
pub trait CredentialRepository {
    async fn create(&self, create: &Credential) -> Result<(), DatabaseError>;
    async fn get_from_name(&self, name: &Username) -> Result<Credential, DatabaseError>;
    async fn delete(&self, id: &Username) -> Result<(), DatabaseError>;
}
