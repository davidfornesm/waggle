mod de;
mod error;
mod ser;

pub use crate::error::{Error, Result};
pub use crate::ser::{to_bytes, to_writer};
