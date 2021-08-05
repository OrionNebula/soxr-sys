#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Flags should be u64, but in compat mode they're incorrectly u32
#[cfg(feature = "compatibility")]
type Flags = u32;

#[cfg(not(feature = "compatibility"))]
type Flags = u64;

// These two constants are defined in terms of other constants, so they don't get picked up by bindgen
pub const SOXR_HQ: Flags = SOXR_20_BITQ;
pub const SOXR_VHQ: Flags = SOXR_28_BITQ;

#[allow(dead_code)]
mod compat {
    // In compat mode soxr_datatype_t constants are at the top level
    pub use crate::_soxr_datatype_t::{
        SOXR_FLOAT32,
        SOXR_FLOAT64,
        SOXR_INT32,
        SOXR_INT16,
        SOXR_SPLIT,
        SOXR_FLOAT32_I,
        SOXR_FLOAT64_I,
        SOXR_INT32_I,
        SOXR_INT16_I,
        SOXR_FLOAT32_S,
        SOXR_FLOAT64_S,
        SOXR_INT32_S,
        SOXR_INT16_S,
    };

    pub type soxr_datatype_t = crate::_soxr_datatype_t::Type;

    // In compat mode, size_t is defined as usize. Kinda superfluous, but nice to have.
    pub type size_t = usize;
}

#[cfg(feature = "compatibility")]
pub use compat::*;

#[cfg(not(feature = "compatibility"))]
pub use crate::_soxr_datatype_t as soxr_datatype_t;
