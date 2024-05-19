mod collection;
mod common;
mod instructions;
mod marketplace;
mod mmm;
mod token;
mod wallet;

pub use collection::*;
pub use common::*;
pub use instructions::*;
pub use marketplace::*;
pub use mmm::*;
pub use token::*;
pub use wallet::*;

mod impls;
use derive_builder::UninitializedFieldError;

use crate::error::MagicedenError;

impl From<UninitializedFieldError> for MagicedenError {
    fn from(value: UninitializedFieldError) -> Self {
        MagicedenError::InvalidArgument(value.to_string())
    }
}
