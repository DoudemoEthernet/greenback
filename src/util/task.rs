use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::db::entity::{
    account::Username,
    task::{Description, Diffuculty, Task, TaskId, Title},
    time::CreatedAt,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTask {
    title: Title,
    description: Description,
    difficulty: Diffuculty,
}

impl PostTask {
    pub fn create_task(&self, username: Username) -> Task {
        Task::new(
            TaskId::default(),
            username,
            self.title.to_owned(),
            self.description.to_owned(),
            self.difficulty,
            OffsetDateTime::now_utc(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTask {
    id: TaskId,
    title: Title,
    description: Description,
    difficulty: Diffuculty,
}

impl PatchTask {
    pub fn create_task(&self, username: Username, created_at: CreatedAt) -> Task {
        Task::new(
            self.id,
            username,
            self.title.to_owned(),
            self.description.to_owned(),
            self.difficulty,
            created_at,
        )
    }

    pub fn id(&self) -> TaskId {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTask {
    id: TaskId,
    title: Title,
    description: Description,
    difficulty: Diffuculty,
}

impl From<Task> for ResponseTask {
    fn from(value: Task) -> Self {
        ResponseTask {
            id: value.id().to_owned(),
            title: value.title().to_owned(),
            description: value.description().to_owned(),
            difficulty: value.difficulty().to_owned(),
        }
    }
}
