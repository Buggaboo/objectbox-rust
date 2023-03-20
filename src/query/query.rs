use crate::c;
use crate::c::*;
use crate::cursor::Cursor;
use crate::error;
use crate::traits::FactoryBlanket;
use crate::traits::OBBlanket;
use crate::util::test_fn_ptr_on_char_ptr;
use core::slice;
use std::marker::PhantomData;
use std::ptr;
use std::rc::Rc;

// TODO pass generic type from box, via fn
impl<T: OBBlanket> Drop for Query<T> {
    fn drop(&mut self) {
        if !self.obx_query.is_null() {
            // always close regardless, no flags to set, no potential double frees
            if let Err(err) = self.close() {
                eprintln!("Error: async: {err}");
            }
            self.obx_query = ptr::null_mut();
        }
    }
}

pub struct Query<T: OBBlanket> {
    obx_query: *mut OBX_query,
    obx_store: *mut OBX_store,
    pub(crate) helper: Rc<dyn FactoryBlanket<T>>,
    phantom_data: PhantomData<T>,
}

impl<T: OBBlanket> Query<T> {
    pub(crate) fn new(
        obx_store: *mut OBX_store,
        helper: Rc<dyn FactoryBlanket<T>>,
        builder: *mut OBX_query_builder,
    ) -> error::Result<Self> {
        unsafe {
            let obx_query = obx_query(builder);
            let _ = c::new_mut(obx_query)?;
            Ok(Query {
                obx_query,
                obx_store,
                helper: helper.clone(),
                phantom_data: PhantomData,
            })
        }
    }

    fn close(&self) -> error::Result<()> {
        let code = unsafe { obx_query_close(self.obx_query) };
        c::call(code)
    }

    // No Clone trait here, because that implies Copy,
    // which prevents using Drop
    pub fn clone(&self) -> error::Result<Self> {
        unsafe {
            let clone = obx_query_clone(self.obx_query);
            let _ = c::new_mut(clone)?;

            // if they are the same, a double free will occur
            // otherwise, the same drop semantics will apply
            assert_ne!(self.obx_query, clone);

            Ok(Query {
                obx_query: clone,
                obx_store: self.obx_store,
                helper: self.helper.clone(),
                phantom_data: PhantomData,
            })
        }
    }

    /// Paging related
    pub fn offset(&self, offset: usize) -> error::Result<&Self> {
        unsafe {
            let result = obx_query_offset(self.obx_query, offset);
            c::call(result).map(|_| self)
        }
    }

    /// Paging related
    pub fn offset_limit(&self, offset: usize, limit: usize) -> error::Result<&Self> {
        unsafe {
            let result = obx_query_offset_limit(self.obx_query, offset, limit);
            c::call(result).map(|_| self)
        }
    }

    /// Paging related
    pub fn limit(&self, limit: usize) -> error::Result<&Self> {
        unsafe {
            let result = obx_query_limit(self.obx_query, limit);
            c::call(result).map(|_| self)
        }
    }

    // elect the cursor version
    // pub(crate) unsafe fn find(&self) -> *mut OBX_bytes_array {
    //     obx_query_find(self.obx_query)
    // }

    // WONTFIX: don't implement unless anyone asks for it
    // pub(crate) unsafe fn find_first(
    //     &mut self,
    //     data: MutConstVoidPtr,
    //     size: *mut usize,
    // ) -> obx_err {
    //     obx_query_find_first(self.obx_query, data, size)
    // }

    // WONTFIX: don't implement unless anyone asks for it
    // pub(crate) unsafe fn find_unique(
    //     &mut self,
    //     data: MutConstVoidPtr,
    //     size: *mut usize,
    // ) -> obx_err {
    //     obx_query_find_unique(self.obx_query, data, size)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn visit(
    //     &mut self,
    //     visitor: obx_data_visitor,
    //     user_data: *mut ::std::os::raw::c_void,
    // ) -> obx_err {
    //     obx_query_visit(self.obx_query, visitor, user_data)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn find_ids(&self) -> *mut OBX_id_array {
    //     obx_query_find_ids(self.obx_query)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn count(&self, out_count: *mut u64) -> obx_err {
    //     obx_query_count(self.obx_query, out_count)
    // }

    // elect the cursor version
    // pub(crate) unsafe fn remove(&self, out_count: *mut u64) -> obx_err {
    //     obx_query_remove(self.obx_query, out_count)
    // }

    /// For testing and debugging
    /// A function pointer is passed here, to prevent dealing with lifetime issues.
    pub fn describe(&self, fn_ptr: fn(String) -> bool) -> bool {
        unsafe {
            let out_ptr = obx_query_describe(self.obx_query);
            test_fn_ptr_on_char_ptr(out_ptr, fn_ptr)
        }
    }

    /// For testing and debugging
    /// A function pointer is passed here, to prevent dealing with lifetime issues.
    pub fn describe_params(&self, fn_ptr: fn(String) -> bool) -> bool {
        unsafe {
            let out_ptr = obx_query_describe_params(self.obx_query);
            test_fn_ptr_on_char_ptr(out_ptr, fn_ptr)
        }
    }

    /*
        // TODO pass a closure fn to this in the pub fn impl
        // TODO translate to iter trait?
        unsafe fn cursor_visit(
            &mut self,
            cursor: &mut OBX_cursor,
            visitor: obx_data_visitor, // typedef bool obx_data_visitor(const void *data, size_t size, void *user_data)
            user_data: *mut ::std::os::raw::c_void,
        ) -> obx_err {
            obx_query_cursor_visit(self.obx_query, cursor, visitor, user_data)
        }

        unsafe fn cursor_find(&self, cursor: &mut OBX_cursor) -> *mut OBX_bytes_array {
            obx_query_cursor_find(self.obx_query, cursor)
        }
    */

    // Reuse what you have, until someone has the time
    // and shares a PR, and improves on this
    // by calling obx_query_cursor_find
    pub fn find(&self) -> error::Result<Vec<T>> {
        let mut vec = Vec::new();
        let mut cursor = Cursor::new(false, self.obx_store, self.helper.clone())?;
        let ids = self.find_ids()?;

        for id in ids {
            vec.push(
                cursor
                    .get_entity(id)?
                    .map_or(self.helper.new_object(), |e| e),
            );
        }
        Ok(vec)
    }

    unsafe fn cursor_find_ids(&self, cursor: &mut OBX_cursor) -> *mut OBX_id_array {
        obx_query_cursor_find_ids(self.obx_query, cursor)
    }

    // TODO write test
    pub fn find_ids(&self) -> error::Result<Vec<c::obx_id>> {
        let mut vec = Vec::new();
        unsafe {
            let cursor = Cursor::new(false, self.obx_store, self.helper.clone())?;
            let c_id_array = self.cursor_find_ids(&mut *cursor.obx_cursor);
            // TODO error check tx, cursor, with get_result_from_ptr
            let c = &*c_id_array;
            let len = c.count;
            let ptr = c.ids;
            let sl = slice::from_raw_parts(ptr, len);
            vec.extend(sl);
            get_result_from_ptr(ptr, vec)
        }
    }

    fn cursor_count(&self, cursor: &mut OBX_cursor, out_count: *mut u64) -> error::Result<u64> {
        unsafe {
            let code = obx_query_cursor_count(self.obx_query, cursor, out_count);
            c::call(code).map(|_| *out_count)
        }
    }

    // TODO write test
    pub fn count(&self) -> error::Result<u64> {
        unsafe {
            let cursor = Cursor::new(false, self.obx_store, self.helper.clone())?;
            let count: *mut u64 = &mut 0;
            self.cursor_count(&mut *cursor.obx_cursor, count)
        }
    }

    unsafe fn cursor_remove(
        &self,
        cursor: &mut OBX_cursor,
        out_count: *mut u64,
    ) -> error::Result<obx_err> {
        let code = obx_query_cursor_remove(self.obx_query, cursor, out_count);
        c::call(code).map(|_| code)
    }

    pub fn remove(&self) -> error::Result<u64> {
        unsafe {
            let mut cursor = Cursor::new(true, self.obx_store, self.helper.clone())?;
            let count: *mut u64 = &mut 0;
            let err_code = self.cursor_remove(&mut *cursor.obx_cursor, count)?;
            if err_code == 0 {
                cursor.get_tx().success()?;
            }
            Ok(*count)
        }
    }
    // end cursor

    // TODO implement later
    // start aliasing
    /*
    pub(crate) unsafe fn param_string(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: PtrConstChar,
    ) -> obx_err {
        obx_query_param_string(self.obx_query, entity_id, property_id, value)
    }

    // For strings
    pub(crate) unsafe fn param_2strings(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: PtrConstChar,
        value2: PtrConstChar,
    ) -> obx_err {
        obx_query_param_2strings(self.obx_query, entity_id, property_id, value, value2)
    }

    pub(crate) unsafe fn param_strings(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const PtrConstChar, // ptr ptr === array of CString
        count: usize,
    ) -> obx_err {
        obx_query_param_strings(self.obx_query, entity_id, property_id, values, count)
    }

    // For ints
    pub(crate) unsafe fn param_int(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: i64,
    ) -> obx_err {
        obx_query_param_int(self.obx_query, entity_id, property_id, value)
    }

    pub(crate) unsafe fn param_2ints(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value_a: i64,
        value_b: i64,
    ) -> obx_err {
        obx_query_param_2ints(
            self.obx_query,
            entity_id,
            property_id,
            value_a,
            value_b,
        )
    }

    pub(crate) unsafe fn param_int64s(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const i64,
        count: usize,
    ) -> obx_err {
        obx_query_param_int64s(self.obx_query, entity_id, property_id, values, count)
    }

    pub(crate) unsafe fn param_int32s(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        values: *const i32,
        count: usize,
    ) -> obx_err {
        obx_query_param_int32s(self.obx_query, entity_id, property_id, values, count)
    }

    // For doubles
    pub(crate) unsafe fn param_double(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: f64,
    ) -> obx_err {
        obx_query_param_double(self.obx_query, entity_id, property_id, value)
    }

    pub(crate) unsafe fn param_2doubles(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value_a: f64,
        value_b: f64,
    ) -> obx_err {
        obx_query_param_2doubles(
            self.obx_query,
            entity_id,
            property_id,
            value_a,
            value_b,
        )
    }

    // For bytes
    pub(crate) unsafe fn param_bytes(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
        value: ConstVoidPtr,
        size: usize,
    ) -> obx_err {
        obx_query_param_bytes(self.obx_query, entity_id, property_id, value, size)
    }

    pub(crate) unsafe fn param_get_type_size(
        &mut self,
        entity_id: obx_schema_id,
        property_id: obx_schema_id,
    ) -> usize {
        obx_query_param_get_type_size(self.obx_query, entity_id, property_id)
    }

    // For aliases
    pub(crate) unsafe fn param_alias_string(
        &mut self,
        alias: PtrConstChar,
        value: PtrConstChar,
    ) -> obx_err {
        obx_query_param_alias_string(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_strings(
        &mut self,
        alias: PtrConstChar,
        values: *const PtrConstChar,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_strings(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_int(
        &mut self,
        alias: PtrConstChar,
        value: i64,
    ) -> obx_err {
        obx_query_param_alias_int(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_2ints(
        &mut self,
        alias: PtrConstChar,
        value_a: i64,
        value_b: i64,
    ) -> obx_err {
        obx_query_param_alias_2ints(self.obx_query, alias, value_a, value_b)
    }

    pub(crate) unsafe fn param_alias_int64s(
        &mut self,
        alias: PtrConstChar,
        values: *const i64,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_int64s(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_int32s(
        &mut self,
        alias: PtrConstChar,
        values: *const i32,
        count: usize,
    ) -> obx_err {
        obx_query_param_alias_int32s(self.obx_query, alias, values, count)
    }

    pub(crate) unsafe fn param_alias_double(
        &mut self,
        alias: PtrConstChar,
        value: f64,
    ) -> obx_err {
        obx_query_param_alias_double(self.obx_query, alias, value)
    }

    pub(crate) unsafe fn param_alias_2doubles(
        &mut self,
        alias: PtrConstChar,
        value_a: f64,
        value_b: f64,
    ) -> obx_err {
        obx_query_param_alias_2doubles(self.obx_query, alias, value_a, value_b)
    }

    pub(crate) unsafe fn param_alias_bytes(
        &mut self,
        alias: PtrConstChar,
        value: ConstVoidPtr,
        size: usize,
    ) -> obx_err {
        obx_query_param_alias_bytes(self.obx_query, alias, value, size)
    }

    pub(crate) unsafe fn param_alias_get_type_size(
        &mut self,
        alias: PtrConstChar,
    ) -> usize {
        obx_query_param_alias_get_type_size(self.obx_query, alias)
    }
    */
}
