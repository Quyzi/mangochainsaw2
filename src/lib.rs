pub mod backends;
pub mod error;
pub mod label;
pub mod object;
pub mod storage;

pub(crate) mod internal {
    pub use crate::error::Error;
    pub use crate::error::Result;
}
