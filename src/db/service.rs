use crate::{db::{repository::task::TaskRepository, error::DatabaseError, entity::{account::AccountId, task::{TaskId, Task}}}, util::task::{PostTask, PatchTask}};

#[derive(Debug, Clone)]
pub struct Service<TRepository: TaskRepository> {
    task_repository: TRepository
}

impl<TRepository> Service<TRepository>
where
    TRepository: TaskRepository,
{
    pub fn new(t_repository: TRepository) -> Self {
        Self {
            task_repository: t_repository
        }
    }

    pub async fn create_task(&self, create: PostTask) -> Result<(), DatabaseError> {
        self.task_repository.create(&create.create_task(AccountId::new(0))).await
    }


    pub async fn update_task(&self, update: PatchTask) -> Result<(), DatabaseError> {
        let original = self.task_repository.get_from_id(&update.id()).await?;
        let task = update.create_task(original.account().to_owned(), original.created_at().to_owned());
        self.task_repository.update(&task).await
    }

    pub async fn delete_task(&self, task_id: TaskId) -> Result<(), DatabaseError> {
        self.task_repository.delete(&task_id).await
    }

    pub async fn get_all_tasks(&self, account_id: AccountId) -> Result<Vec<Task>, DatabaseError> {
        self.task_repository.get_from_account(&account_id).await
    }
}
