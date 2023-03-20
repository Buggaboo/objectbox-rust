#![allow(dead_code)]
use std::ptr;
use std::rc::Rc;

use crate::c::{self, *};
use crate::error;

use crate::cursor::Cursor;
use crate::query::builder::Builder;
use crate::query::condition::Condition;
use crate::query::Query;
use crate::traits::{FactoryBlanket, OBBlanket};
use crate::util::{MutConstVoidPtr, NOT_FOUND_404};
use flatbuffers::FlatBufferBuilder;

// This Box type will confuse a lot of rust users of std::boxed::Box
pub struct Box<'a, T: OBBlanket> {
    pub(crate) helper: Rc<dyn FactoryBlanket<T>>,
    pub(crate) obx_box: *mut OBX_box,
    builder: FlatBufferBuilder<'a>,
    // pub(crate) async_: std::boxed::Box<Async>, // TODO
}

impl<T: OBBlanket> Box<'_, T> {
    pub(crate) fn new(store: *mut OBX_store, helper: Rc<dyn FactoryBlanket<T>>) -> Self {
        unsafe {
            let obx_box = c::obx_box(store, helper.get_entity_id());

            Box {
                helper,
                obx_box,
                builder: FlatBufferBuilder::new(),
            }
        }
    }

    // This should only be exposed between threads, channels, etc.
    pub(crate) fn get_store(&self) -> *mut OBX_store {
        unsafe { obx_box_store(self.obx_box) }
    }

    pub fn contains(&mut self, id: obx_id) -> error::Result<bool> {
        let mut contains = false;
        c::get_result(
            unsafe { obx_box_contains(self.obx_box, id, &mut contains) },
            contains,
        )
    }

    pub fn contains_many(&mut self, ids: &Vec<obx_id>) -> error::Result<Vec<bool>> {
        let mut r = Vec::<bool>::new();
        for id in ids {
            match self.contains(*id) {
                Ok(v) => r.push(v),
                Err(err) => err.as_result()?,
            }
        }
        Ok(r)
    }

    /*
      // TODO extension trait for Vec<u32?/OBX_id> -> OBX_id_array, see util.rs
      // TODO alternative: run contains one by one
      pub fn contains_many_id_array(&mut self, ids: *const OBX_id_array) -> bool {
          let mut contains = false;
          c::call(unsafe { obx_box_contains_many(self.obx_box, ids, &mut contains) });
          contains
      }

      // TODO extension trait for mut_const_c_void -> slice -> Vec<u8> to be processed by flatbuffers
      pub fn get_data_ptr(
          &mut self,
          id: obx_id,
      ) -> (*mut *const ::std::os::raw::c_void, usize) {
          let data = std::ptr::null_mut(); // this is wrong, and will explode
          let mut size = 0;
          c::call(unsafe { obx_box_get(self.obx_box, id, data, &mut size) });
          (data, size)
      }

      // TODO extension trait for Vec<u32?/OBX_id> -> &[Entity], see util.rs
      fn get_many_bytes_array(&self, ids: *const OBX_id_array) -> *mut OBX_bytes_array {
          unsafe { obx_box_get_many(self.obx_box, ids) }
      }

      // TODO convert OBX_bytes_array into &[Entity]
      fn get_all_bytes_array(&self) -> *mut OBX_bytes_array {
          unsafe { obx_box_get_all(self.obx_box) }
      }

      pub fn id_for_put(&self, id_or_zero: obx_id) -> obx_id {
          unsafe { obx_box_id_for_put(self.obx_box, id_or_zero) }
      }

      pub fn ids_for_put(&mut self, count: u64) -> obx_id {
          let mut first_id = 0;
          c::call(unsafe { obx_box_ids_for_put(self.obx_box, count, &mut first_id) });
          first_id
      }

      pub fn put_vec_u8(
          &mut self,
          id: obx_id,
          data: &Vec<u8>,
      ) {
        c::call(unsafe { obx_box_put(self.obx_box, id, data.to_const_c_void(), data.len()) });
      }

      pub fn insert_vec_u8(
          &mut self,
          id: obx_id,
          data: &Vec<u8>,
      ) {
        c::call(unsafe { obx_box_insert(self.obx_box, id, data.to_const_c_void(), data.len()) });
      }

      pub fn update_vec_u8(
          &mut self,
          id: obx_id,
          data: &Vec<u8>,
      ) {
        c::call(unsafe { obx_box_update(self.obx_box, id, data.to_const_c_void(), data.len()) });
      }

      pub fn put5_vec_u8(
          &mut self,
          id: obx_id,
          data: &Vec<u8>,
          mode: OBXPutMode,
      ) {
        c::call(unsafe { obx_box_put5(self.obx_box, id, data.to_const_c_void(), data.len(), mode) });
      }

      pub fn put_object(
          &self,
          data: &mut Vec<u8>,
      ) -> obx_id {
          unsafe { obx_box_put_object(self.obx_box, data.to_mut_c_void(), data.len()) }
      }

      pub fn put_object4(
          &self,
          data: &mut Vec<u8>,
          mode: OBXPutMode,
      ) -> obx_id {
          unsafe { obx_box_put_object4(self.obx_box, data.to_mut_c_void(), data.len(), mode) }
      }

      pub fn put_many_bytes_array(
          &mut self,
          objects: *const OBX_bytes_array,
          ids: *const obx_id,
          mode: OBXPutMode,
      ) {
        c::call(unsafe { obx_box_put_many(self.obx_box, objects, ids, mode) });
      }

      pub fn put_many5_bytes_array(&mut self, objects: *const OBX_bytes_array, ids: *const obx_id, mode: OBXPutMode, fail_on_id_failure: bool) {
        c::call(unsafe { obx_box_put_many5(self.obx_box, objects, ids, mode, fail_on_id_failure) });
      }

    // TODO size 16, align 8
    fn remove_many_id_array(&mut self, ids: *const OBX_id_array) -> u64 {
        let out_count: u64 = 0;
        c::call(
            unsafe { obx_box_remove_many(self.obx_box, ids, out_count as *mut u64) },
            Some("box::remove_many_id_array"),
        )
        out_count
    }
    */

    pub fn remove_with_id(&mut self, id: obx_id) -> error::Result<bool> {
        unsafe {
            let code = obx_box_remove(self.obx_box, id);
            c::get_result(code, code == 0)
        }
    }

    pub fn remove_many(&mut self, ids: &Vec<c::obx_id>) -> error::Result<Vec<bool>> {
        let mut r = Vec::<bool>::new();
        for id in ids {
            match self.remove_with_id(*id) {
                Ok(v) => r.push(v),
                Err(err) => err.as_result()?,
            }
        }
        Ok(r)
    }

    // TODO check if this is ACID (or go with cursor instead)
    pub fn remove_all(&mut self) -> error::Result<u64> {
        unsafe {
            let out_count: *mut u64 = &mut 0;
            c::get_result(
                obx_box_remove_all(self.obx_box, out_count as *mut u64),
                *out_count,
            )
        }
    }

    pub fn is_empty(&mut self) -> error::Result<bool> {
        unsafe {
            let out_is_empty: *mut bool = &mut false; // coerce
            c::get_result(obx_box_is_empty(self.obx_box, out_is_empty), *out_is_empty)
        }
    }

    pub fn count(&mut self) -> error::Result<u64> {
        self.count_with_limit(0)
    }

    pub fn count_with_limit(&mut self, limit: u64) -> error::Result<u64> {
        unsafe {
            let out_count: *mut u64 = &mut 0;
            c::get_result(obx_box_count(self.obx_box, limit, out_count), *out_count)
        }
    }
    /*
      pub fn get_backlink_ids(&self, property_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
          unsafe { obx_box_get_backlink_ids(self.obx_box, property_id, id) }
      }

      pub fn rel_put(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
          unsafe { obx_box_rel_put(self.obx_box, relation_id, source_id, target_id) }
      }

      pub fn rel_remove(&self, relation_id: obx_schema_id, source_id: obx_id, target_id: obx_id) -> obx_err {
          unsafe { obx_box_rel_remove(self.obx_box, relation_id, source_id, target_id) }
      }

      pub fn rel_get_ids(&self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
          unsafe { obx_box_rel_get_ids(self.obx_box, relation_id, id) }
      }

      // TODO convert user_data to Vec<u8>
      pub fn visit_all(&mut self, visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
        unsafe {
            obx_box_visit_all(self.obx_box, visitor, user_data)
        }
      }

        // TODO fix sooner than later
        fn visit_many(&mut self, ids: &[c::obx_id], visitor: obx_data_visitor, user_data: *mut ::std::os::raw::c_void) -> obx_err {
            unsafe {
                obx_box_visit_many(self.obx_box, ids.as_ptr(), visitor, user_data)
            }
        }

      pub fn rel_get_backlink_ids(&mut self, relation_id: obx_schema_id, id: obx_id) -> *mut OBX_id_array {
        unsafe {
            obx_box_rel_get_backlink_ids(self.obx_box, relation_id, id)
        }
      }

      pub fn ts_min_max(&mut self, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
        unsafe {
            obx_box_ts_min_max(self.obx_box, out_min_id, out_min_value, out_max_id, out_max_value)
        }
      }

      pub fn ts_min_max_range(&mut self, range_begin: i64, range_end: i64, out_min_id: *mut obx_id, out_min_value: *mut i64, out_max_id: *mut obx_id, out_max_value: *mut i64) -> obx_err {
        unsafe {
            obx_box_ts_min_max_range(self.obx_box, range_begin, range_end, out_min_id, out_min_value, out_max_id, out_max_value)
        }
      }
    */

    /// A box has a longer lifetime than a cursor,
    /// and the only thing keeping this method here
    /// is the FB Builder.
    /// To prevent reinitializing builders for
    /// every cursor operation, we keep this method here,
    /// it's better to recycle.
    pub(crate) fn put_entity_in_ob(
        &mut self,
        cursor: &mut Cursor<T>,
        object: &mut T,
    ) -> error::Result<c::obx_id> {
        let old_id = object.get_id();
        let is_object_new = old_id == 0;
        let new_id = cursor.id_for_put(old_id);
        object.set_id(new_id);

        object.flatten(&mut self.builder);
        let data = Vec::from(self.builder.finished_data());

        if is_object_new {
            cursor.put_new(new_id, &data)?;
        } else {
            cursor.put(new_id, &data)?;
        }

        Ok(new_id)
    }

    pub fn put(&mut self, object: &mut T) -> error::Result<c::obx_id> {
        let mut cursor = Cursor::new(true, self.get_store(), self.helper.clone())?;

        let new_id = self.put_entity_in_ob(&mut cursor, object);
        cursor.get_tx().success()?;

        new_id
    }

    pub fn put_many(&mut self, objects: Vec<&mut T>) -> error::Result<Vec<c::obx_id>> {
        let mut cursor = Cursor::new(true, self.get_store(), self.helper.clone())?;

        let mut vec_out = Vec::<c::obx_id>::new();

        for o in objects {
            vec_out.push(self.put_entity_in_ob(&mut cursor, o)?);
        }

        cursor.get_tx().success()?;
        Ok(vec_out)
    }

    /// For testing purposes
    pub fn count_with_cursor(&self) -> error::Result<u64> {
        let mut cursor = Cursor::new(false, self.get_store(), self.helper.clone())?;
        cursor.count()
    }

    pub fn get(&self, id: c::obx_id) -> error::Result<Option<T>> {
        let mut cursor = Cursor::new(false, self.get_store(), self.helper.clone())?;
        cursor.get_entity(id)
    }

    pub fn get_many(&self, ids: &[c::obx_id]) -> error::Result<Vec<Option<T>>> {
        let mut cursor = Cursor::new(false, self.get_store(), self.helper.clone())?;

        let mut r = Vec::<Option<T>>::new();

        for id in ids {
            r.push(cursor.get_entity(*id)?);
        }
        Ok(r)
    }

    /// Returns all stored objects in this Box
    pub fn get_all(&self) -> error::Result<Vec<T>> {
        let mut cursor = Cursor::new(false, self.get_store(), self.helper.clone())?;

        let data_ptr_ptr: *mut *mut u8 = &mut ptr::null_mut();

        let size_ptr: *mut usize = &mut 0;

        let mut vec: Vec<T> = Vec::new();

        let mut code = cursor.first(data_ptr_ptr as MutConstVoidPtr, size_ptr)?;

        // c::OBX_NOT_FOUND was a C #define that became a u32
        // which is incompatible with obx_err === i32
        while code != NOT_FOUND_404 {
            unsafe {
                vec.push(cursor.from_raw_parts_to_object(data_ptr_ptr, size_ptr));
            }
            code = cursor.next(data_ptr_ptr as MutConstVoidPtr, size_ptr)?;
        }

        Ok(vec)
    }

    // TODO
    // pub fn query_all(conditions: &Vec<Condition<T>>) -> Builder<T> {}

    // TODO
    // pub fn query_any(conditions: &Vec<Condition<T>>) -> Builder<T> {}

    /// Fetch the intermediate query builder, then if necessary call Builder::build()
    pub fn query_builder(&self, root: &mut Condition<T>) -> error::Result<Builder<T>> {
        Builder::<T>::new(&self, root)
    }

    /// Immediately build the query
    pub fn query(&self, root: &mut Condition<T>) -> error::Result<Query<T>> {
        self.query_builder(root)?.build()
    }
}
