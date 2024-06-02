//! Contains various lightweight utility functions, macros, and wrappers not found in the rust standard library.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude {
    pub use crate::iter::IterCloneExt;
    pub use crate::iter::IterExt;
    pub use crate::slice::SliceExt;
    pub use crate::str::StrExt;
}

pub mod iter;
pub mod slice;
pub mod span;
pub mod str;

mod cstr;
mod transmute;
mod wrappers;

pub use crate::span::s::Span;
pub use crate::transmute::*;
pub use crate::wrappers::*;
