//! Contains various lightweight utility functions, macros, and wrappers not found in the rust standard library.

#![no_std]
pub mod prelude {
    pub use crate::slice::SliceExt;
}

pub mod slice;

mod cstr;
mod transmute;
mod wrappers;
pub use crate::cstr::*;
pub use crate::transmute::*;
pub use crate::wrappers::*;
