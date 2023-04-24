use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedAt(OffsetDateTime);

impl From<CreatedAt> for OffsetDateTime {
    fn from(value: CreatedAt) -> Self {
        value.0
    }
}

impl AsRef<OffsetDateTime> for CreatedAt {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl TryFrom<String> for CreatedAt {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let unix_timestamp = value.parse::<i64>()?;
        Ok(CreatedAt::new(OffsetDateTime::from_unix_timestamp(
            unix_timestamp,
        )?))
    }
}

impl CreatedAt {
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}
