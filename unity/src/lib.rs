//! TODO

#![deny(
    unused_results,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion
)]
#![forbid(rust_2018_idioms)]
#![allow(clippy::inherent_to_string, clippy::type_complexity, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod runtime;

pub mod common;
pub mod il2cpp;
pub mod libs;
pub mod mono;

pub mod utils;
