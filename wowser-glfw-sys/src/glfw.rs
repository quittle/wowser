//! Contains all rust declarations of generated bindings.

#![allow(deref_nullptr)] // See https://github.com/rust-lang/rust-bindgen/issues/1651
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::from_over_into)]
#![allow(rustdoc::broken_intra_doc_links)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
