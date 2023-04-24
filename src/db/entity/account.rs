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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Username {
    pub fn new(username: String) -> Self {
        Self(username)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Password(String);

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credential {
    account_id: AccountId,
    username: Username,
    password: Password
}

impl Credential {
    pub fn new(
        id: impl Into<i64>,
        username: impl Into<String>,
        password: impl Into<String>
    ) -> Self {
        Self { 
            account_id: AccountId::new(id.into()), 
            username: Username::new(username.into()), 
            password: Password::new(password.into())
        }
    }
}
