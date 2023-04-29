use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    username: Username,
    password: Password,
}

impl Credential {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: Username::new(username.into()),
            password: Password::new(password.into()),
        }
    }

    pub fn username(&self) -> Username {
        self.username.to_owned()
    }

    pub fn password(&self) -> Password {
        self.password.to_owned()
    }
}
