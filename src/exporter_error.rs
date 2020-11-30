#[derive(Debug, Fail)]
pub(crate) enum ExporterError {
    #[fail(display = "Hyper error: {}", e)]
    Hyper { e: hyper::error::Error },

    #[fail(display = "http error: {}", e)]
    Http { e: http::Error },

    #[fail(display = "UTF-8 error: {}", e)]
    UTF8 { e: std::string::FromUtf8Error },

    #[fail(display = "JSON format error: {}", e)]
    JSON { e: serde_json::error::Error },

    #[fail(display = "IO Error: {}", e)]
    IO { e: std::io::Error },
}

impl From<std::io::Error> for ExporterError {
    fn from(e: std::io::Error) -> Self {
        ExporterError::IO { e }
    }
}

impl From<hyper::error::Error> for ExporterError {
    fn from(e: hyper::error::Error) -> Self {
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
        ExporterError::JSON { e }
    }
}
