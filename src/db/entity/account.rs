use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountId(i64);

impl From<AccountId> for i64 {
    fn from(value: AccountId) -> Self {
        value.0
    }
}

impl AsRef<i64> for AccountId {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AccountId {
    pub fn new(id: impl Into<i64>) -> Self {
        Self(id.into())
    }
}

impl Default for AccountId {
    fn default() -> Self {
        use rand::Rng;
        let gen = rand::thread_rng().gen_range(1000_0000_0000_0000..=9999_9999_9999_9999);
        Self(gen)
    }
}
