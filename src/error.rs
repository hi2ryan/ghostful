use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhostfulError {
    #[error("connection failed: {0}")]
    Connection(String),
    #[error("element not found: {0}")]
    ElementNotFound(String),
    #[error("timed out waiting for {0}")]
    Timeout(String),
    #[error("navigation failed: {0}")]
    Navigation(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("page crashed")]
    PageCrashed,
    #[error("browser process crashed")]
    BrowserCrashed,
}

pub type Result<T> = std::result::Result<T, GhostfulError>;
