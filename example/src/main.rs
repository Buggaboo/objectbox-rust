extern crate objectbox;

use objectbox::macros::{entity, index};
use objectbox::{opt::Opt,store::Store};

// uncomment the next two lines
// when the mod hasn't been generated yet
mod objectbox_gen;
use objectbox_gen as ob;

// hard assumption: your Entity must be on the crate's
// ground-level, so the generated code can access it
// via crate::Entity
#[derive(Debug)]
#[entity]
pub struct Entity3 {
  #[id]
  id: u64,
}

#[derive(Debug)]
#[entity]
pub struct Entity2 {
  #[id]
  id: u64,
  #[index]
  index_u64: u64,
}

#[derive(Debug)]
#[entity]
pub struct Entity {
  #[id]
  id: u64,
  #[index]
  index_u32: u32,
  t_bool : bool,
  t_u8 : u8,
  t_i8 : i8,
  t_i16: i16,
  t_u16: u16,
  #[unique]
  unique_i32: i32,
  t_i32: i32,
  t_u32: u32,
  t_u64: u64,
  t_i64: i64,
  t_f32: f32,
  t_f64: f64,
  t_string: String,
  t_char: char,
  t_vec_string: Vec<String>,
  t_vec_bytes: Vec<u8>,
  // transient: Option<bool> // not yet supported
}

fn main() {
  // let mut model = objectbox_gen::make_model();
  // let mut opt = Opt::from_model(&mut model);
  // let mut store = Store::from_options(&mut opt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
      let mut model = objectbox_gen::make_model();
      let mut opt = Opt::from_model(&mut model);
      let mut store = Store::from_options(&mut opt);
      store.trait_map = Some(ob::make_factory_map());
      // let mut box1 = store.get_box::<Entity>();
      // let mut box2 = store.get_box::<Entity2>();

      let mut box3 = store.get_box::<Entity3>();
      box3.remove_all();
      let e3 = &mut Entity3 {
        id: 0,
      };
      box3.put(e3);
      assert_eq!(false, box3.is_empty(), "{:#?}", e3);

      // assert_eq!(1, box3.count());

      box3.remove_all();
      assert!(box3.is_empty());
    }
}