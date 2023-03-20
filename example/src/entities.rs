extern crate objectbox;

use objectbox::macros::entity;

#[derive(Debug)]
#[entity]
pub struct Entity3 {
    #[id]
    pub id: u64,
    pub hello: String,
}

#[derive(Debug)]
#[entity]
pub struct Entity2 {
    #[id]
    pub id: u64,
    #[index]
    pub index_u64: u64,
}

#[derive(Debug)]
#[entity]
pub struct Entity {
    #[id]
    pub id: u64,
    #[index]
    pub index_u32: u32,
    pub t_bool: bool,
    pub t_u8: u8,
    pub t_i8: i8,
    pub t_i16: i16,
    pub t_u16: u16,
    #[unique]
    pub unique_i32: i32,
    pub t_i32: i32,
    pub t_u32: u32,
    pub t_u64: u64,
    pub t_i64: i64,
    pub t_f32: f32,
    pub t_f64: f64,
    pub t_string: String,
    pub t_char: char,
    pub t_vec_string: Vec<String>,
    pub t_vec_bytes: Vec<u8>,
    // transient: Option<bool> // not yet supported
}
