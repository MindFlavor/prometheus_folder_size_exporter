#[derive(Debug, Fail)]
pub(crate) enum ExporterError {
    #[fail(display = "Generic error")]
    GenericError {},

    #[fail(display = "Unsupported path {}. Only /metrics is supported", path)]
    UnsupportedPathError { path: String },

    #[fail(display = "Unsupported verb {}. Only GET is supported", verb)]
    UnsupportedMethodError { verb: String },

    #[fail(display = "Hyper error: {}", e)]
    HyperError { e: hyper::error::Error },

    #[fail(display = "http error: {}", e)]
    HttpError { e: http::Error },

    #[fail(display = "UTF-8 error: {}", e)]
    UTF8Error { e: std::string::FromUtf8Error },

    #[fail(display = "JSON format error: {}", e)]
    JSONError { e: serde_json::error::Error },

    #[fail(display = "IO Error: {}", e)]
    IOError { e: std::io::Error },
}

impl From<std::io::Error> for ExporterError {
    fn from(e: std::io::Error) -> Self {
        ExporterError::IOError { e }
    }
}

impl From<hyper::error::Error> for ExporterError {
    fn from(e: hyper::error::Error) -> Self {
        ExporterError::HyperError { e }
    }
}

impl From<http::Error> for ExporterError {
    fn from(e: http::Error) -> Self {
        ExporterError::HttpError { e }
    }
}

impl From<std::string::FromUtf8Error> for ExporterError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        ExporterError::UTF8Error { e }
    }
}

impl From<serde_json::error::Error> for ExporterError {
    fn from(e: serde_json::error::Error) -> Self {
        ExporterError::JSONError { e }
    }
}
