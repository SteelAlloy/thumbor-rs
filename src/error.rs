#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("URL cannot be a base")]
    UrlCannotBeABase,
}
