use rusqlite;

#[derive(Debug)]
pub enum FeatureFlagError {
    RusqliteError(rusqlite::Error),
}

impl From<rusqlite::Error> for FeatureFlagError {
    fn from(error: rusqlite::Error) -> Self {
        FeatureFlagError::RusqliteError(error)
    }
}
