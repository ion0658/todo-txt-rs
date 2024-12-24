pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("exception invalid syntax")]
    Syntax,
    #[error("exception invalid index")]
    InvalidIndex,

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Convert(#[from] core::convert::Infallible),
}
