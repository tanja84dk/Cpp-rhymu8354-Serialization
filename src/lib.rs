#![warn(clippy::pedantic)]
// TODO: Remove this once ready to publish.
#![allow(clippy::missing_errors_doc)]
// TODO: Uncomment this once ready to publish.
//#![warn(missing_docs)]

mod de;
mod error;
mod ser;

pub use de::{
    from_bytes,
    Deserializer,
};
pub use error::{
    Error,
    Result,
};
pub use ser::{
    to_bytes,
    Serializer,
};
