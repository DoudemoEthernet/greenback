use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::util::string::{TryFromStringError, TryFromStringErrorKind};

use super::{account::AccountId, time::CreatedAt};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<TaskId> for Uuid {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

impl AsRef<Uuid> for TaskId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl TryFrom<&str> for TaskId {
    type Error = TryFromStringError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tried = Uuid::try_parse(value).map_err(|e| TryFromStringError {
            raw: value.to_string(),
            kind: TryFromStringErrorKind::Uuid(e),
        })?;
        Ok(Self(tried))
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new(title: String) -> Self {
        Self(title)
    }
}

impl From<Title> for String {
    fn from(value: Title) -> Self {
        value.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description(String);

impl From<Description> for String {
    fn from(value: Description) -> Self {
        value.0
    }
}

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Description {
    pub fn new(desc: String) -> Self {
        Self(desc)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diffuculty(i8);

impl From<Diffuculty> for i8 {
    fn from(value: Diffuculty) -> Self {
        value.0
    }
}

impl AsRef<i8> for Diffuculty {
    fn as_ref(&self) -> &i8 {
        &self.0
    }
}

impl Diffuculty {
    pub fn new(difficulty: i8) -> Self {
        Self(difficulty)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    account: AccountId,
    title: Title,
    description: Description,
    difficulty: Diffuculty,
    created_at: CreatedAt,
}

impl Task {
    pub fn new(
        id: impl Into<Uuid>,
        account: impl Into<i64>,
        title: impl Into<String>,
        description: impl Into<String>,
        difficulty: impl Into<i8>,
        created_at: impl Into<OffsetDateTime>,
    ) -> Self {
        Self {
            id: TaskId::new(id.into()),
            account: AccountId::new(account.into()),
            title: Title::new(title.into()),
            description: Description::new(description.into()),
            difficulty: Diffuculty::new(difficulty.into()),
            created_at: CreatedAt::new(created_at.into()),
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn account(&self) -> &AccountId {
        &self.account
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn difficulty(&self) -> &Diffuculty {
        &self.difficulty
    }

    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }
}
