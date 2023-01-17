mod c;

pub mod error;
pub mod model;
pub mod version;

pub mod store;
pub mod r#box; // escape keyword
pub mod traits;

pub extern crate objectbox_generator as generator;
pub extern crate objectbox_macros as macros;
pub extern crate flatbuffers as flatbuffers;
