#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error made by data structure being transformed")]
    Message(String),

    #[error("ran out of bytes while deserializing value")]
    ValueTruncated,

    #[error("cannot serialize container of unknown length")]
    LengthRequired,

    #[error("cannot deserialize without knowing type")]
    TypeUnknown,

    #[error("deserialized integer does not fit in its type")]
    IntegerOverflow,

    #[error("invalid UTF-8 sequence deserializing text")]
    InvalidUtf8,

    #[error("cannot determine identifiers in this format")]
    IdentifierUnknown,
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
