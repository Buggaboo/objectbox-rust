extern crate objectbox;

use objectbox::macros::{entity, index};

mod objectbox_gen;
use objectbox_gen as ob;

#[entity]
pub struct Entity {
  #[id]
  id: u32,
  #[index]
  index: u64,
  t_bool : bool,
  t_u8 : u8,
  t_i8 : i8,
  t_i16: i16,
  t_u16: u16,
  #[unique]
  // t_char: char, // TODO support WiP
  t_i32: i32,
  t_u32: u32,
  t_u64: u64,
  t_i64: i64,
  t_f32: f32,
  t_f64: f64,
  t_string: String,
}

fn main() {
    println!("Hello, world!");
}
