use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpeedTestError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO Error: {0}")]
    Io(#[from] ::std::io::Error),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("AddrParseError: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("RoXmlTreeError: {0}")]
    RoXmlTreeError(#[from] roxmltree::Error),
    #[error("ConfigParseError")]
    ConfigParseError,
    #[error("ServerParseError")]
    ServerParseError,
    #[error("LatencyTestInvalidPath")]
    LatencyTestInvalidPath,
    #[error("LatencyTestClosestError")]
    LatencyTestClosestError,
    #[error("URL Parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("System Time Error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("ParseShareUrlError")]
    ParseShareUrlError,
    #[error("ThreadPool Error: {0}")]
    ThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),
}
