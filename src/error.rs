use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("HTTP Error: {}\n Fount at {}", source, backtrace))]
    Http {
        source: reqwest::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("JSON Error in {}: {}\nFound at {}", source.path(), source.inner(), backtrace))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        backtrace: Backtrace,
    },
    Url {
        source: url::ParseError,
        backtrace: Backtrace,
    },
}
