use async_trait::async_trait;

use crate::db::{
    entity::{
        account::Username,
        task::{Task, TaskId},
    },
    error::DatabaseError,
};

#[async_trait(?Send)]
pub trait TaskRepository {
    async fn create(&self, create: &Task) -> Result<(), DatabaseError>;
    async fn update(&self, update: &Task) -> Result<(), DatabaseError>;
    async fn get_from_account(&self, username: &Username) -> Result<Vec<Task>, DatabaseError>;
    async fn get_from_id(&self, task_id: &TaskId) -> Result<Task, DatabaseError>;
    async fn delete(&self, task_id: &TaskId) -> Result<(), DatabaseError>;
}
