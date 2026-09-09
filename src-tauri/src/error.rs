use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObError {
    #[error("UNAVAILABLE — {0}")]
    Unavailable(String),
    #[error("NOT CONNECTED — {0}")]
    NotConnected(String),
    #[error("REQUIRES CONFIGURATION — {0}")]
    RequiresConfiguration(String),
    #[error("PERMISSION REQUIRED — {0}")]
    PermissionRequired(String),
    #[error("SECURITY BLOCKED — {0}")]
    SecurityBlocked(String),
    #[error("VALIDATION FAILED — {0}")]
    Validation(String),
    #[error("OPERATION CANCELLED")]
    Cancelled,
    #[error("DATABASE ERROR — {0}")]
    Database(#[from] rusqlite::Error),
    #[error("NETWORK ERROR — {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O ERROR — {0}")]
    Io(#[from] std::io::Error),
    #[error("SERIALIZATION ERROR — {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("INTERNAL ERROR — {0}")]
    Internal(String),
}

pub type ObResult<T> = Result<T, ObError>;
impl From<ObError> for String { fn from(value: ObError) -> Self { value.to_string() } }
