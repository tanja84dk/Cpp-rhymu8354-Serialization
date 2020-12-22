use std::str::Utf8Error;

/// This is the enumeration of all the different kinds of errors which this
/// crate generates.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The serde convention is to provide a "catch-all" error variant
    /// to hold any kind of error message which the data structure may
    /// emit during serialization.  This variant serves that need.
    #[error("error made by data structure being transformed")]
    Message(String),

    /// This is returned if the input buffer is fully consumed before
    /// the data is fully deserialized.
    #[error("ran out of bytes while deserializing value")]
    ValueTruncated,

    /// This serializer does not support deserializing unbounded sequences.
    #[error("cannot serialize container of unknown length")]
    LengthRequired,

    /// This serializer does not support deserializing values of unknown types.
    #[error("cannot deserialize without knowing type")]
    TypeUnknown,

    /// An overflow (or underflow) occurred while deserializing
    /// an integer.
    #[error("deserialized integer does not fit in its type")]
    IntegerOverflow,

    /// An invalid byte sequence was encountered while deserializing UTF-8
    /// encoded text.
    #[error("invalid UTF-8 sequence deserializing text")]
    InvalidUtf8(#[source] Option<Utf8Error>),

    /// This serializer does not include identifiers in its format and so
    /// cannot determine them during deserialization.
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

/// This type alias provides a convenient wrapper for the Rust standard
/// `Result` type, when the `Error` type argument is our own [`Error`] type.
///
/// [`Error`]: struct.Error.html
pub type Result<T> = std::result::Result<T, Error>;
