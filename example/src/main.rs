extern crate objectbox;

use objectbox::{macros::entity, opt::Opt, store::Store};

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
  hello: String,
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
  let mut model = ob::make_model();
  let mut opt = Opt::from_model(&mut model);
  let mut store = Store::from_options(&mut opt);

  let trait_map = ob::make_factory_map();
  store.trait_map = Some(trait_map);

  // box is a reserved keyword use r#box or simply something else
  let mut box1 = store.get_box::<Entity3>();

  let mut e_before = Entity3 {
    id: 0,
    hello: "Hello world!".to_string(),
  };

  let new_id = match box1.put(&mut e_before) {
    Err(err) => panic!("{err}"),
    Ok(item_id) => item_id
  };

  match box1.get(new_id) {
    Err(err) => panic!("{err}"),
    Ok(found_item) => {
      if let Some(object) = found_item {
        println!("{}", object.hello);
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use std::rc;
    use objectbox::flatbuffers::{FlatBufferBuilder, Table};
    use objectbox::traits::{self, IdExt, FBOBBridge};
    use objectbox::{opt::Opt,store::Store};

    use super::*;

    #[test]
    fn test_write_and_read_fb () {
      let trait_map2 = ob::make_factory_map();
      let f1 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity>>>().unwrap().clone();
      let f2 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity2>>>().unwrap().clone();
      let f3 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity3>>>().unwrap().clone();

      let mut e1 = f1.new_entity();
      let mut e2 = f2.new_entity();
      let mut e3 = f3.new_entity();

      e1.id = 0xFFFFFFFF;
      e2.id = 0xFFFFFFFF;
      e3.id = 0xFFFFFFFF;

      let mut fbb = FlatBufferBuilder::new();

      unsafe {
        e3.to_fb(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();
  
        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e3_copy = f3.make(&mut table);

        assert_eq!(e3_copy.id, e3.id);
      }

      unsafe {
        e2.to_fb(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();
  
        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e2_copy = f2.make(&mut table);

        assert_eq!(e2_copy.id, e2.id);
      }

      unsafe {
        e1.to_fb(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();
  
        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e1_copy = f1.make(&mut table);

        assert_eq!(e1_copy.id, e1.id);
      }
    }

    #[test]
    fn test_box_put_and_count_and_remove_all() {
      let mut model = ob::make_model();
      let mut opt = Opt::from_model(&mut model);
      let mut store = Store::from_options(&mut opt);

      let trait_map = ob::make_factory_map();
      store.trait_map = Some(trait_map);

      let mut box3 = store.get_box::<Entity3>();
      box3.remove_all();
      let mut box2 = store.get_box::<Entity2>();
      box2.remove_all();
      let mut box1 = store.get_box::<Entity>();
      box1.remove_all();
      
      let trait_map2 = ob::make_factory_map();
      let f1 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity>>>().unwrap().clone();
      let f2 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity2>>>().unwrap().clone();
      let f3 = trait_map2.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity3>>>().unwrap().clone();

      let mut e1 = f1.new_entity();
      let mut e2 = f2.new_entity();
      let mut e3 = f3.new_entity();

      if let Err(err) = box1.put(&mut e1) {
        panic!("{err}");
      }
      if let Err(err) = box2.put(&mut e2) {
        panic!("{err}");
      }
      if let Err(err) = box3.put(&mut e3) {
        panic!("{err}");
      }

      assert_eq!(false, e1.get_id() == 0, "Set new ID after put");
      assert_eq!(false, e2.get_id() == 0);
      assert_eq!(false, e3.get_id() == 0);

      assert_eq!(false, box1.is_empty(), "{:#?}", e1);
      assert_eq!(false, box2.is_empty(), "{:#?}", e2);
      assert_eq!(false, box3.is_empty(), "{:#?}", e3);

      assert_eq!(1, box1.count());
      assert_eq!(1, box2.count());
      assert_eq!(1, box3.count());
      assert_eq!(1, box1.count_with_limit(1));
      assert_eq!(1, box2.count_with_limit(1));
      assert_eq!(1, box3.count_with_limit(1));
      assert_eq!(1, box1.count_with_cursor());
      assert_eq!(1, box2.count_with_cursor());
      assert_eq!(1, box3.count_with_cursor());

      box1.remove_all();
      assert!(box1.is_empty());
      assert_eq!(0, box1.count_with_cursor());
      
      box2.remove_all();
      assert!(box2.is_empty());
      assert_eq!(0, box2.count_with_cursor());
      
      box3.remove_all();
      assert!(box3.is_empty());
      assert_eq!(0, box3.count_with_cursor());

      // put then get, then clear
      {
        let mut e1 = f1.new_entity();
        e1.t_u16 = 0xFFF;

        let new_id = match box1.put(&mut e1) {
          Err(err) => panic!("{err}"),
          Ok(id) => id
        };

        match box1.get(new_id) {
          Err(err) => panic!("{err}"),
          Ok(opt) => {
            assert_eq!(0xFFF, opt.unwrap().t_u16);
          }
        }
        box1.remove_all();
      }

      // put_many, get_many, get_all
      {
        let mut ids = match box1.put_many(vec![&mut f1.new_entity(), &mut f1.new_entity()]) {
          Err(err) => panic!("{err}"),
          Ok(ids) => ids
        };

        ids.push(404);

        let objects = match box1.get_many(ids.as_slice()) {
          Err(err) => panic!("{err}"),
          Ok(v) => v
        };

        assert!(objects[0].is_some());
        assert!(objects[1].is_some());
        assert!(objects[2].is_none());

        let all_objects = match box1.get_all() {
          Err(err) => panic!("{err}"),
          Ok(objs) => objs
        };

        assert_eq!(2, all_objects.len());
      }
    }
}