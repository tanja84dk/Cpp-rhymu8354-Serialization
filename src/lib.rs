#![warn(clippy::pedantic)]
// TODO: Remove this once ready to publish.
#![allow(clippy::missing_errors_doc)]
// TODO: Uncomment this once ready to publish.
//#![warn(missing_docs)]

mod error;
mod ser;

pub use error::{
    Error,
    Result,
};
pub use ser::Serializer;
