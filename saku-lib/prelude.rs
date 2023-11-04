// Re-export the crate Error.
pub use crate::error::Error;

pub use crate::make_err;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

pub use log::{debug, error, info, trace, warn};

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);
