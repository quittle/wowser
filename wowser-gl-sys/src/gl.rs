//! Contains all rust declarations of generated bindings.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::upper_case_acronyms)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
