//! This crate provides a [`serde`] Serializer/Deserializer implementation
//! similar to [`bincode`].
//!
//! [`serde`]: https://docs.rs/serde
//! [`bincode`]: https://docs.rs/bincode
//!
//! # Examples
//!
//! ```rust
//! # extern crate serialization;
//! use serialization::{
//!     from_bytes,
//!     to_bytes,
//! };
//!
//! let greeting = "Hello, World!";
//! let encoded_message: Vec<u8> = to_bytes(&greeting).unwrap();
//! let decoded_message: &str = from_bytes(&encoded_message).unwrap();
//! assert_eq!(greeting, decoded_message);
//! ```

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

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
