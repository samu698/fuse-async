#![deny(missing_debug_implementations)]
#![allow(non_camel_case_types)]

use zerocopy::{FromZeros, Immutable, IntoBytes, KnownLayout};

pub const FUSE_KERNEL_VERSION: u32 = 7;
pub const FUSE_KERNEL_MINOR_VERSION: u32 = 45;

pub const FUSE_ROOT_ID: u64 = 1;
pub const FUSE_IOCTL_MAX_IOV: u32 = 256;

// Type aliases for clarity
type seconds = u64;
type nanos = u32;

mod requests;
pub use requests::*;
//
// mod notify
//
mod flags;
pub use flags::*;
//
mod types;
pub use types::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[derive(KnownLayout, Immutable, IntoBytes)]
/// Padding value that must be zero.
///
/// Some FUSE structures have fields that are used a padding or are unused,
/// these fields must be filled with zeros, this type enforces this requirement.
pub struct Padding<T: FromZeros + Copy>(T);

impl<T: FromZeros + Copy> Padding<T> {
    #[inline]
    /// Constructs a padding value filled with zeros
    pub fn new() -> Self {
        Self(T::new_zeroed())
    }

    #[inline]
    /// Constructs a padding value filled with the chosen value
    ///
    /// The user must check that provided value is acceptable, for the given
    /// field.
    pub unsafe fn with_nonzero(value: T) -> Self {
        Self(value)
    }
}

impl<T: FromZeros + Copy> Default for Padding<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
