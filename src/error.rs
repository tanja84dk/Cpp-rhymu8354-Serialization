#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error made by data structure being transformed")]
    Message(String),

    #[error("ran out of bytes while deserializing value")]
    ValueTruncated,

    #[error("cannot serialize container of unknown length")]
    LengthRequired,
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
