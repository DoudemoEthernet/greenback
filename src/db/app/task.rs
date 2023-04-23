use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;
use worker::{Database};

use crate::db::{
    entity::{
        account::AccountId,
        task::{Task, TaskId},
    },
    error::DatabaseError,
    repository::task::TaskRepository,
};

pub struct D1TaskDatabase {
    pub db: DatabaseWrapper,
}

impl D1TaskDatabase {
    pub fn new(database: Database) -> Self {
        Self {
            db: DatabaseWrapper(Arc::new(database)),
        }
    }
}

#[derive(Clone)]
pub struct DatabaseWrapper(Arc<Database>);

#[derive(Debug, Deserialize)]
struct InternalData {
    id: String,
    account_id: i64,
    title: String,
    description: String,
    difficulty: i64,
    created_at: i64,
}

impl InternalData {
    fn to_task(&self) -> Task {
        Task::new(
            Uuid::from_str(&self.id).unwrap(),
            self.account_id,
            self.title.to_owned(),
            self.description.to_owned(),
            self.difficulty as i8,
            OffsetDateTime::from_unix_timestamp(self.created_at).unwrap(),
        )
    }
}

#[async_trait(?Send)]
impl TaskRepository for D1TaskDatabase {
    async fn create(&self, create: &Task) -> Result<(), DatabaseError> {
        let query = "INSERT INTO tasks (id, account_id, title, description, difficulty, created_at) VALUES (?, ?, ?, ?, ?, ?);";
        let queue = self
            .db
            .0
            .prepare(query)
            .bind(&[
                create.id().as_ref().to_string().into(),
                create.account().as_ref().to_string().into(),
                create.title().as_ref().into(),
                create.description().as_ref().into(),
                create.difficulty().as_ref().to_string().into(),
                create
                    .created_at()
                    .as_ref()
                    .unix_timestamp()
                    .to_string()
                    .into(),
            ])
            .map_err(DatabaseError::TransactionError)?;

        let _ = queue
            .first::<Task>(None)
            .await
            .map_err(DatabaseError::TransactionError)?;

        Ok(())
    }

    async fn update(&self, update: &Task) -> Result<(), DatabaseError> {
        let query = "UPDATE tasks SET title = ?, description = ?, difficulty = ? where id = ?";
        let queue = self
            .db
            .0
            .prepare(query)
            .bind(&[
                update.title().as_ref().into(),
                update.description().as_ref().into(),
                update.difficulty().as_ref().to_string().into(),
                update.id().as_ref().to_string().into(),
            ])
            .map_err(DatabaseError::TransactionError)?;
        queue
            .first::<Task>(None)
            .await
            .map_err(DatabaseError::TransactionError)?;

        Ok(())
    }

    async fn get_from_account(&self, account_id: &AccountId) -> Result<Vec<Task>, DatabaseError> {
        let query = "SELECT * FROM tasks WHERE account_id = ? order by created_at desc;";
        let queue = self
            .db
            .0
            .prepare(query)
            .bind(&[account_id.as_ref().to_string().into()])
            .map_err(DatabaseError::TransactionError)?;
        queue
            .all()
            .await
            .and_then(|r| {
                r.results::<InternalData>()
                    .map(|i| i.iter().map(|data| data.to_task()).collect::<Vec<Task>>())
            })
            .map_err(DatabaseError::TransactionError)
    }

    async fn get_from_id(&self, task_id: &TaskId) -> Result<Task, DatabaseError> {
        let query = "SELECT * FROM tasks WHERE id = ?;";
        let queue = self
            .db
            .0
            .prepare(query)
            .bind(&[task_id.as_ref().to_string().into()])
            .map_err(DatabaseError::TransactionError)?;
        let result = queue
            .first::<InternalData>(None)
            .await
            .map_err(DatabaseError::TransactionError)?;

        match result {
            Some(internal) => Ok(internal.to_task()),
            None => Err(DatabaseError::NotFound("task".to_string())),
        }
    }

    async fn delete(&self, task_id: &TaskId) -> Result<(), DatabaseError> {
        let query = "DELETE FROM tasks WHERE id = ?;";
        let queue = self
            .db
            .0
            .prepare(query)
            .bind(&[task_id.as_ref().to_string().into()])
            .map_err(DatabaseError::TransactionError)?;
        queue.run().await.map_err(DatabaseError::TransactionError)?;
        Ok(())
    }
}
