use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exp(i64);

impl Exp {
    pub fn new(exp: i64) -> Self {
        Self(exp)
    }
}

impl From<Exp> for i64 {
    fn from(value: Exp) -> Self {
        value.0
    }
}

