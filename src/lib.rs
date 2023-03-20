#![allow(dead_code)]
// pub extern crate predicates;
pub extern crate anymap as map;
pub extern crate objectbox_generator as generator;
pub extern crate objectbox_macros as macros;
pub extern crate lean_buffer;

extern crate flatbuffers as flatbuffers;

pub mod r#box;
pub mod c;
pub mod error;
pub mod model;
pub mod opt;
pub mod store;
pub mod util;
pub mod version;

pub mod query;
pub mod traits;

mod r#async;
mod cursor;
mod txn;

// TODO do the prelude thing, in the generated objectbox_gen.rs
// use objectbox::prelude::*;
