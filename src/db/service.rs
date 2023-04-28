use crate::{
    db::{
        entity::task::{Task, TaskId},
        error::DatabaseError,
        repository::task::TaskRepository,
    },
    util::task::{PatchTask, PostTask},
};

use super::{
    entity::account::{Credential, Username},
    repository::account::CredentialRepository,
};

#[derive(Debug, Clone)]
pub struct Service<TRepository: TaskRepository, ARepository: CredentialRepository> {
    task_repository: TRepository,
    credential_repository: ARepository,
}

impl<TRepository, ARepository> Service<TRepository, ARepository>
where
    TRepository: TaskRepository,
    ARepository: CredentialRepository,
{
    pub fn new(t_repository: TRepository, c_repostitory: ARepository) -> Self {
        Self {
            task_repository: t_repository,
            credential_repository: c_repostitory,
        }
    }

    pub async fn create_task(&self, create: PostTask) -> Result<(), DatabaseError> {
        self.task_repository
            .create(&create.create_task(Username::new("dummy-user".to_string())))
            .await
    }

    pub async fn update_task(&self, update: PatchTask) -> Result<(), DatabaseError> {
        let original = self.task_repository.get_from_id(&update.id()).await?;
        let task = update.create_task(
            original.username().to_owned(),
            original.created_at().to_owned(),
        );
        self.task_repository.update(&task).await
    }

    pub async fn delete_task(&self, task_id: TaskId) -> Result<(), DatabaseError> {
        self.task_repository.delete(&task_id).await
    }

    pub async fn get_all_tasks(&self, username: Username) -> Result<Vec<Task>, DatabaseError> {
        self.task_repository.get_from_account(&username).await
    }

    pub async fn create_credential(&self, create: &Credential) -> Result<(), DatabaseError> {
        self.credential_repository.create(create).await
    }

    pub async fn get_credential(&self, username: Username) -> Result<Credential, DatabaseError> {
        self.credential_repository.get(&username).await
    }

    pub async fn delete_credenatial(&self, username: Username) -> Result<(), DatabaseError> {
        self.credential_repository.delete(&username).await
    }
}
