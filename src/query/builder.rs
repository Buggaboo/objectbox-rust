#![allow(dead_code)]
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    rc::Rc,
};

// TODO also error check before chaining the next call (obx_qb_cond)
// TODO depending on property type, allow only certain calls at compile time?
// TODO compile time determined extension blanket traits?
use crate::{
    c::{self, *},
    error,
    r#box::Box,
    traits::{FactoryBlanket, OBBlanket},
    util::*,
};

use super::condition::Condition;
use crate::query::Query;

impl<T: OBBlanket> Drop for Builder<T> {
    fn drop(&mut self) {
        if !self.has_built_query && !self.obx_query_builder.is_null() {
            if let Err(err) = self.close() {
                eprintln!("Error: async: {err}");
            }
            self.obx_query_builder = std::ptr::null_mut();
        }
    }
}

pub struct Builder<T: OBBlanket> {
    obx_store: *mut OBX_store,
    helper: Rc<dyn FactoryBlanket<T>>,
    property_id: obx_schema_id,
    obx_query_builder: *mut OBX_query_builder,
    case_sensitive: bool,
    has_built_query: bool,
    phantom_data: PhantomData<T>,
}

/// The generic type should cause compile time errors
/// if a condition from another table is passed
impl<T: OBBlanket> Builder<T> {
    pub(crate) fn new(box_store: &Box<T>, condition: &mut Condition<T>) -> error::Result<Self> {
        let entity_id = box_store.helper.get_entity_id(); // call factory
        let obx_store = box_store.get_store();
        new_mut(obx_store)?;
        let obx_query_builder = unsafe { obx_query_builder(obx_store, entity_id) };
        new_mut(obx_query_builder)?;

        let mut builder = Builder {
            obx_store,
            helper: box_store.helper.clone(),
            property_id: 0,
            obx_query_builder,
            has_built_query: false,
            case_sensitive: false,
            phantom_data: PhantomData,
        };

        condition.visit_dfs(&mut |c| builder.get_condition_integer(c));

        c::get_result(builder.error_code(), builder)
    }

    fn get_group_integer(&self, c: &Condition<T>, mut f: impl FnMut(*const i32, usize) -> c::obx_qb_cond) -> c::obx_qb_cond {
        let cs = c.collect_results();
        // do not build the (sub)tree with one result
        if cs.len() == 1 {
            cs[0]
        } else if cs.len() > 1 {
            let (ptr, len) = cs.as_ptr_and_length_tuple::<c::obx_qb_cond>();
            f(ptr, len)
        } else {
            QUERY_NO_OP
        }
    }

    fn get_condition_integer(&mut self, c: &mut Condition<T>) -> c::obx_qb_cond {
        // invariant to 0 parameter condition functions
        self.property_id = c.get_property_id();

        use super::enums::ConditionOp;
        unsafe {
            let result = match &c.op {
                ConditionOp::IsNull => self.is_null(),
                ConditionOp::NotNull => self.not_null(),
                ConditionOp::OrderFlags(flags) => {
                    self.order(*flags);
                    QUERY_NO_OP
                }
                ConditionOp::CaseSensitive(b) => {
                    self.case_sensitive = *b;
                    QUERY_NO_OP
                }
                ConditionOp::Contains(s) => self.contains_string(s.as_c_char_ptr()),
                ConditionOp::ContainsElement(s) => self.contains_element_string(s.as_c_char_ptr()),
                ConditionOp::StartsWith(s) => self.starts_with_string(s.as_c_char_ptr()),
                ConditionOp::EndsWith(s) => self.ends_with_string(s.as_c_char_ptr()),
                ConditionOp::AnyEquals(s) => self.any_equals_string(s.as_c_char_ptr()),
                ConditionOp::Eq_i64(i) => self.equals_int(*i),
                ConditionOp::Ne_i64(i) => self.not_equals_int(*i),
                ConditionOp::Lt_i64(i) => self.less_than_int(*i),
                ConditionOp::Gt_i64(i) => self.greater_than_int(*i),
                ConditionOp::Le_i64(i) => self.less_or_equal_int(*i),
                ConditionOp::Ge_i64(i) => self.greater_or_equal_int(*i),
                ConditionOp::Lt_f64(f) => self.less_than_double(*f),
                ConditionOp::Gt_f64(f) => self.greater_than_double(*f),
                ConditionOp::Le_f64(f) => self.less_or_equal_double(*f),
                ConditionOp::Ge_f64(f) => self.greater_or_equal_double(*f),
                ConditionOp::Eq_string(s) => self.equals_string(s.as_c_char_ptr()),
                ConditionOp::Ne_string(s) => self.not_equals_string(s.as_c_char_ptr()),
                ConditionOp::Lt_string(s) => self.less_than_string(s.as_c_char_ptr()),
                ConditionOp::Gt_string(s) => self.greater_than_string(s.as_c_char_ptr()),
                ConditionOp::Le_string(s) => self.less_or_equal_string(s.as_c_char_ptr()),
                ConditionOp::Ge_string(s) => self.greater_or_equal_string(s.as_c_char_ptr()),
                ConditionOp::All => {
                    self.get_group_integer(c,  |ptr, len| self.all(ptr, len))
                }
                ConditionOp::Any => {
                    self.get_group_integer(c,  |ptr, len| self.any(ptr, len))
                }
                ConditionOp::ContainsKeyValue(k, v) => {
                    self.contains_key_value_string(k.as_c_char_ptr(), v.as_c_char_ptr())
                }
                ConditionOp::Eq_vecu8(vec_u8) => {
                    let (ptr, len) = vec_u8.as_ptr_and_length_tuple::<u8>();
                    self.equals_bytes(ptr as ConstVoidPtr, len)
                }
                ConditionOp::Lt_vecu8(vec_u8) => {
                    let (ptr, len) = vec_u8.as_ptr_and_length_tuple::<u8>();
                    self.less_than_bytes(ptr as ConstVoidPtr, len)
                }
                ConditionOp::Gt_vecu8(vec_u8) => {
                    let (ptr, len) = vec_u8.as_ptr_and_length_tuple::<u8>();
                    self.greater_than_bytes(ptr as ConstVoidPtr, len)
                }
                ConditionOp::Le_vecu8(vec_u8) => {
                    let (ptr, len) = vec_u8.as_ptr_and_length_tuple::<u8>();
                    self.less_or_equal_bytes(ptr as ConstVoidPtr, len)
                }
                ConditionOp::Ge_vecu8(vec_u8) => {
                    let (ptr, len) = vec_u8.as_ptr_and_length_tuple::<u8>();
                    self.greater_or_equal_bytes(ptr as ConstVoidPtr, len)
                }
                ConditionOp::Between_i64(start, end) => self.between_2ints(*start, *end),
                ConditionOp::Between_f64(start, end) => self.between_2doubles(*start, *end),
                ConditionOp::In_i32(vec_i32) => {
                    let (ptr, len) = vec_i32.as_ptr_and_length_tuple::<i32>();
                    self.in_int32s(ptr, len)
                }
                ConditionOp::NotIn_i32(vec_i32) => {
                    let (ptr, len) = vec_i32.as_ptr_and_length_tuple::<i32>();
                    self.not_in_int32s(ptr, len)
                }
                ConditionOp::In_i64(vec_i64) => {
                    let (ptr, len) = vec_i64.as_ptr_and_length_tuple::<i64>();
                    self.in_int64s(ptr, len)
                }
                ConditionOp::NotIn_i64(vec_i64) => {
                    let (ptr, len) = vec_i64.as_ptr_and_length_tuple::<i64>();
                    self.not_in_int64s(ptr, len)
                }
                ConditionOp::In_String(strs) => {
                    let mut new_strings = Vec::<String>::new();
                    for s in strs {
                        let mut new_string = String::new();
                        new_string.push_str(s.as_str());
                        new_string.push('\0');
                        new_strings.push(new_string);
                        if let Err(_) = CString::new(s.as_bytes()) {
                            // TODO bring back self.error? or leave string validation to user?
                            // TODO validate strings outside function
                            // error::Error::new_local(&format!(
                            //     "Bad string conversion (in_strings: {})",
                            //     err.to_string()
                            // ))?;
                            return QUERY_NO_OP;
                        }
                    }
                    let vec: Vec<_> = new_strings
                        .iter()
                        // TODO rewrite the unwrap
                        .map(|s| CString::new(s.as_bytes()).unwrap())
                        .collect();
                    let cstrs: Vec<&CStr> = vec.iter().map(|c| c.as_c_str()).collect();
                    let (ptr, len) = cstrs.as_ptr_and_length_tuple();
                    self.in_strings(ptr, len)
                }
                ConditionOp::NoOp => QUERY_NO_OP,
            };
            result
        }
    }

    /// Why does Self::build have to be called separately?
    pub fn build(&mut self) -> error::Result<Query<T>> {
        let query = Query::new(self.obx_store, self.helper.clone(), self.obx_query_builder)?;
        // iff a query is built properly, then do not drop, else drop
        self.has_built_query = true;
        Ok(query)
    }

    /// private, in case of double frees
    fn close(&mut self) -> error::Result<()> {
        let code = unsafe { obx_qb_close(self.obx_query_builder) };
        c::call(code)
    }

    // pub(crate) fn type_id(&self) -> obx_schema_id {
    //     unsafe { obx_qb_type_id(self.obx_query_builder) }
    // }

    // TODO call this before finalizing build
    fn error_code(&self) -> obx_err {
        unsafe { obx_qb_error_code(self.obx_query_builder) }
    }

    // TODO call this before finalizing build
    fn error_message(&self) -> PtrConstChar {
        unsafe { obx_qb_error_message(self.obx_query_builder) }
    }

    // TODO implement Option<*> properties, or this will always return false
    unsafe fn is_null(&mut self) -> obx_qb_cond {
        obx_qb_null(self.obx_query_builder, self.property_id)
    }

    // TODO implement Option<*> properties, or this will always return true
    unsafe fn not_null(&mut self) -> obx_qb_cond {
        obx_qb_not_null(self.obx_query_builder, self.property_id)
    }

    // eq_String
    unsafe fn equals_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        obx_qb_equals_string(
            self.obx_query_builder,
            self.property_id,
            value,
            self.case_sensitive,
        )
    }

    // ne_String
    unsafe fn not_equals_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_not_equals_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // contains_String
    unsafe fn contains_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // contains_element_String
    unsafe fn contains_element_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_element_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // contains_key_value_String
    unsafe fn contains_key_value_string(
        &mut self,
        key: PtrConstChar,
        value: PtrConstChar,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_contains_key_value_string(
                self.obx_query_builder,
                self.property_id,
                key,
                value,
                self.case_sensitive,
            )
        }
    }

    // starts_with_String
    unsafe fn starts_with_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_starts_with_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // ends_with_String
    unsafe fn ends_with_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_ends_with_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // gt_String
    unsafe fn greater_than_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_than_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // ge_String
    unsafe fn greater_or_equal_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_greater_or_equal_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // lt String
    unsafe fn less_than_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_less_than_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // le_String
    unsafe fn less_or_equal_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_less_or_equal_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // member_of_Strings / in_Strings
    unsafe fn in_strings(&mut self, values: *const PtrConstChar, count: usize) -> obx_qb_cond {
        unsafe {
            obx_qb_in_strings(
                self.obx_query_builder,
                self.property_id,
                values,
                count,
                self.case_sensitive,
            )
        }
    }

    // any_equals_String
    unsafe fn any_equals_string(&mut self, value: PtrConstChar) -> obx_qb_cond {
        unsafe {
            obx_qb_any_equals_string(
                self.obx_query_builder,
                self.property_id,
                value,
                self.case_sensitive,
            )
        }
    }

    // Eq (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn equals_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_equals_int(self.obx_query_builder, self.property_id, value)
    }

    // Ne (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn not_equals_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_not_equals_int(self.obx_query_builder, self.property_id, value)
    }

    // Gt (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn greater_than_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_greater_than_int(self.obx_query_builder, self.property_id, value)
    }

    // Ge (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn greater_or_equal_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_greater_or_equal_int(self.obx_query_builder, self.property_id, value)
    }

    // Lt (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn less_than_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_less_than_int(self.obx_query_builder, self.property_id, value)
    }

    // Le (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn less_or_equal_int(&mut self, value: i64) -> obx_qb_cond {
        obx_qb_less_or_equal_int(self.obx_query_builder, self.property_id, value)
    }

    // between (u8, i8, u16, i16, u32, i32, u64, i64)
    unsafe fn between_2ints(&mut self, value_a: i64, value_b: i64) -> obx_qb_cond {
        obx_qb_between_2ints(self.obx_query_builder, self.property_id, value_a, value_b)
    }

    // in / member of (i64, u64?)
    unsafe fn in_int64s(&mut self, values: *const i64, count: usize) -> obx_qb_cond {
        obx_qb_in_int64s(self.obx_query_builder, self.property_id, values, count)
    }

    // not in / not member of (i64, u64?)
    unsafe fn not_in_int64s(&mut self, values: *const i64, count: usize) -> obx_qb_cond {
        obx_qb_not_in_int64s(self.obx_query_builder, self.property_id, values, count)
    }

    // in / member of (i32, u32?)
    unsafe fn in_int32s(&mut self, values: *const i32, count: usize) -> obx_qb_cond {
        obx_qb_in_int32s(self.obx_query_builder, self.property_id, values, count)
    }

    // not in / not member of (i32, u32?)
    unsafe fn not_in_int32s(&self, values: *const i32, count: usize) -> obx_qb_cond {
        obx_qb_not_in_int32s(self.obx_query_builder, self.property_id, values, count)
    }

    // gt f64
    unsafe fn greater_than_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_greater_than_double(self.obx_query_builder, self.property_id, value)
    }

    // ge f64
    unsafe fn greater_or_equal_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_greater_or_equal_double(self.obx_query_builder, self.property_id, value)
    }

    // lt f64
    unsafe fn less_than_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_less_than_double(self.obx_query_builder, self.property_id, value)
    }

    // le f64
    unsafe fn less_or_equal_double(&self, value: f64) -> obx_qb_cond {
        obx_qb_less_or_equal_double(self.obx_query_builder, self.property_id, value)
    }

    // between f64
    unsafe fn between_2doubles(&self, value_a: f64, value_b: f64) -> obx_qb_cond {
        obx_qb_between_2doubles(self.obx_query_builder, self.property_id, value_a, value_b)
    }

    // eq Vec<u8>
    unsafe fn equals_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_equals_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // gt Vec<u8>
    unsafe fn greater_than_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_than_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // ge Vec<u8>
    unsafe fn greater_or_equal_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_greater_or_equal_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // lt Vec<u8>
    unsafe fn less_than_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_than_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // le Vec<u8>
    unsafe fn less_or_equal_bytes(
        &self,
        value: *const ::std::os::raw::c_void,
        size: usize,
    ) -> obx_qb_cond {
        obx_qb_less_or_equal_bytes(self.obx_query_builder, self.property_id, value, size)
    }

    // TODO create all!() macro, substitute varargs
    unsafe fn all(&self, conditions: *const obx_qb_cond, count: usize) -> obx_qb_cond {
        obx_qb_all(self.obx_query_builder, conditions, count)
    }

    // TODO create any!() macro, substitute varargs
    unsafe fn any(&self, conditions: *const obx_qb_cond, count: usize) -> obx_qb_cond {
        obx_qb_any(self.obx_query_builder, conditions, count)
    }

    unsafe fn order(&self, flags: OBXOrderFlags) -> obx_err {
        obx_qb_order(self.obx_query_builder, self.property_id, flags)
    }

    // TODO support later
    /*
    unsafe fn param_alias(&self, alias: PtrConstChar) -> obx_err {
        obx_qb_param_alias(self.obx_query_builder, alias)
    }
    unsafe fn relation_count_property(
        &self,
        relation_entity_id: obx_schema_id,
        relation_property_id: obx_schema_id,
        relation_count: i32,
    ) -> obx_qb_cond {
        unsafe {
            obx_qb_relation_count_property(
                self.obx_query_builder,
                relation_entity_id,
                relation_property_id,
                relation_count,
            )
        }
    }
    unsafe fn link_property(
        &self,
    ) -> *mut OBX_query_builder {
        obx_qb_link_property(self.obx_query_builder, self.property_id)
    }

    unsafe fn backlink_property(
        &self,
        source_entity_id: obx_schema_id,
        source_property_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        unsafe {
            obx_qb_backlink_property(self.obx_query_builder, source_entity_id, source_property_id)
        }
    }

    unsafe fn link_standalone(
        &self,
        relation_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        obx_qb_link_standalone(self.obx_query_builder, relation_id)
    }

    unsafe fn backlink_standalone(
        &self,
        relation_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        obx_qb_backlink_standalone(self.obx_query_builder, relation_id)
    }

    unsafe fn link_time(
        &self,
        linked_entity_id: obx_schema_id,
        begin_property_id: obx_schema_id,
        end_property_id: obx_schema_id,
    ) -> *mut OBX_query_builder {
        unsafe {
            obx_qb_link_time(
                self.obx_query_builder,
                linked_entity_id,
                begin_self.property_id,
                end_self.property_id,
            )
        }
    }
    */
}
