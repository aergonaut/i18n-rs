//! Implementation of Rust i18n backend

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

extern crate yaml_rust;

pub mod backend;
pub mod errors;
