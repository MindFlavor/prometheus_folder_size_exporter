use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ExporterError {
    #[error("Hyper error: {}", e)]
    Hyper { e: hyper::Error },

    #[error("http error: {}", e)]
    Http { e: http::Error },

    #[error("UTF-8 error: {}", e)]
    UTF8 { e: std::string::FromUtf8Error },

    #[error("JSON format error: {}", e)]
    Json { e: serde_json::error::Error },

    #[error("IO Error: {}", e)]
    IO { e: std::io::Error },
}

impl From<std::io::Error> for ExporterError {
    fn from(e: std::io::Error) -> Self {
        ExporterError::IO { e }
    }
}

impl From<hyper::Error> for ExporterError {
    fn from(e: hyper::Error) -> Self {
        ExporterError::Hyper { e }
    }
}

impl From<http::Error> for ExporterError {
    fn from(e: http::Error) -> Self {
        ExporterError::Http { e }
    }
}

impl From<std::string::FromUtf8Error> for ExporterError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        ExporterError::UTF8 { e }
    }
}

impl From<serde_json::error::Error> for ExporterError {
    fn from(e: serde_json::error::Error) -> Self {
        ExporterError::Json { e }
    }
}
