use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    InvalidInput,
    Database,
    Network,
    TradingView,
    Config,
    Internal,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("database error: {0}")]
    Database(#[source] sqlx::Error),

    #[error("network error: {0}")]
    Network(String),

    #[error("tradingview error: {0}")]
    TradingView(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::NotFound(_) => ErrorKind::NotFound,
            Self::InvalidInput(_) => ErrorKind::InvalidInput,
            Self::Database(_) => ErrorKind::Database,
            Self::Network(_) => ErrorKind::Network,
            Self::TradingView(_) => ErrorKind::TradingView,
            Self::Config(_) => ErrorKind::Config,
            Self::Internal(_) => ErrorKind::Internal,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<tvdata_rs::Error> for Error {
    fn from(err: tvdata_rs::Error) -> Self {
        Self::TradingView(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let _status = match self.kind() {
            ErrorKind::NotFound => axum::http::StatusCode::NOT_FOUND,
            ErrorKind::InvalidInput => axum::http::StatusCode::BAD_REQUEST,
            ErrorKind::Database => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Network => axum::http::StatusCode::BAD_GATEWAY,
            ErrorKind::TradingView => axum::http::StatusCode::BAD_GATEWAY,
            ErrorKind::Config => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        axum::Json(serde_json::json!({
            "error": self.to_string(),
            "kind": format!("{:?}", self.kind())
        }))
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind_classification() {
        let not_found = Error::NotFound("monitor".to_string());
        assert_eq!(not_found.kind(), ErrorKind::NotFound);

        let invalid = Error::InvalidInput("bad symbol".to_string());
        assert_eq!(invalid.kind(), ErrorKind::InvalidInput);

        let config = Error::Config("missing field".to_string());
        assert_eq!(config.kind(), ErrorKind::Config);
    }
}
