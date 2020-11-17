#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error made by data structure being transformed")]
    Message(String),

    #[error("ran out of bytes while deserializing value")]
    ValueTruncated,
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
