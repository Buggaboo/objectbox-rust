
use std::ffi::{CStr, c_uint};
use std::path::Path;

use crate::c::*;
use crate::error::Error;
use crate::model::Model;
use crate::util::{ToCChar, ToCVoid};

pub struct Opt {
  pub(crate) error: Option<Error>,
  pub(crate) obx_opt: *mut OBX_store_options,
  pub(crate) ptr_consumed: bool,
}

impl Drop for Opt {
  fn drop(&mut self) {
    unsafe {
      if !self.ptr_consumed {
        obx_opt_free(self.obx_opt);
        self.obx_opt = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        eprintln!("Error: {err}");
      }
    }
  }
}

impl Opt {
  pub fn new() -> Self {
    let obx_opt = unsafe { obx_opt() };
    Opt {
      error: None,
      obx_opt,
      ptr_consumed: false,
    }
  }

  pub fn from_model(model: &mut Model) -> Self {
    let mut itself = Self::new();
    if let Some(err) = &model.error {
      panic!("Error: opt: {err}");
    }
    itself.model(model);
    if itself.error.is_none() {
      model.ptr_consumed = true;
    }
    itself
  }

  pub fn directory(&mut self, dir: &Path) -> &mut Self {
    self.error = call(unsafe { obx_opt_directory(self.obx_opt, dir.to_c_char()) }, "opt::directory".to_string()).err();
    self
  }

  pub fn max_db_size_in_kb(&mut self, size_in_kb: u64) -> &mut Self {
    unsafe {
      obx_opt_max_db_size_in_kb(self.obx_opt, size_in_kb);
    }
    self
  }

  pub fn max_data_size_in_kb(&mut self, size_in_kb: u64) -> &mut Self {
    unsafe {
      obx_opt_max_data_size_in_kb(self.obx_opt, size_in_kb);
    }
    self
  }

  pub fn file_mode(&mut self, file_mode: u32) -> &mut Self {
    unsafe {
      obx_opt_file_mode(self.obx_opt, file_mode as c_uint);
    }
    self
  }

  pub fn max_readers(&mut self, max_readers: u32) -> &mut Self {
    unsafe {
      obx_opt_max_readers(self.obx_opt, max_readers as c_uint);
    }
    self
  }

  pub fn no_reader_thread_locals(&mut self, flag: bool) -> &mut Self {
    unsafe {
      obx_opt_no_reader_thread_locals(self.obx_opt, flag);
    }
    self
  }

  pub(crate) fn model(&mut self, model: &mut Model) {
    self.error = call(unsafe { obx_opt_model(self.obx_opt, model.obx_model) }, "opt::model".to_string()).err();
  }

  pub fn model_bytes(&mut self, bytes: &Vec<u8>, size: usize) -> &mut Self {
    self.error = call(unsafe { obx_opt_model_bytes(self.obx_opt, bytes.to_const_c_void(), size) }, "opt::model_bytes".to_string()).err();
    self
  }

  pub fn model_bytes_direct(&mut self, bytes: &Vec<u8>, size: usize) -> &mut Self {
    self.error = call(unsafe { obx_opt_model_bytes_direct(self.obx_opt, bytes.to_const_c_void(), size) }, "opt::model_bytes_direct".to_string()).err();
    self
  }

  pub fn validate_on_open(&mut self, page_limit: usize, leaf_level: bool) -> &mut Self {
    unsafe {
      obx_opt_validate_on_open(self.obx_opt, page_limit, leaf_level);
    }
    self
  }

  pub fn put_padding_mode(&mut self, mode: OBXPutPaddingMode) -> &mut Self {
    unsafe {
      obx_opt_put_padding_mode(self.obx_opt, mode);
    }
    self
  }

  pub fn read_schema(&mut self, value: bool) -> &mut Self {
    unsafe {
      obx_opt_read_schema(self.obx_opt, value);
    }
    self
  }

  pub fn use_previous_commit(&mut self, value: bool) -> &mut Self {
    unsafe {
      obx_opt_use_previous_commit(self.obx_opt, value);
    }
    self
  }

  pub fn read_only(&mut self, value: bool) -> &mut Self {
    unsafe {
      obx_opt_read_only(self.obx_opt, value);
    }
    self
  }

  pub fn debug_flags(&mut self, flags: u32) -> &mut Self {
    unsafe {
      obx_opt_debug_flags(self.obx_opt, flags);
    }
    self
  }

  pub fn add_debug_flags(&mut self, flags: u32) -> &mut Self {
    unsafe {
      obx_opt_add_debug_flags(self.obx_opt, flags);
    }
    self
  }

  pub fn async_max_queue_length(&mut self, value: usize) -> &mut Self {
    unsafe {
      obx_opt_async_max_queue_length(self.obx_opt, value);
    }
    self
  }

  pub fn async_throttle_at_queue_length(&mut self, value: usize) -> &mut Self {
    unsafe {
      obx_opt_async_throttle_at_queue_length(self.obx_opt, value);
    }
    self
  }

  pub fn async_throttle_micros(&mut self, value: u32) -> &mut Self {
    unsafe {
      obx_opt_async_throttle_micros(self.obx_opt, value);
    }
    self
  }

  pub fn async_max_in_tx_duration(&mut self, micros: u32) -> &mut Self {
    unsafe {
      obx_opt_async_max_in_tx_duration(self.obx_opt, micros);
    }
    self
  }

  pub fn async_max_in_tx_operations(&mut self, value: u32) -> &mut Self {
    unsafe {
      obx_opt_async_max_in_tx_operations(self.obx_opt, value);
    }
    self
  }

  pub fn async_pre_txn_delay(&mut self, delay_micros: u32) -> &mut Self {
    unsafe {
      obx_opt_async_pre_txn_delay(self.obx_opt, delay_micros);
    }
    self
  }

  pub fn async_pre_txn_delay4(&mut self, delay_micros: u32, delay2_micros: u32, min_queue_length_for_delay2: usize) -> &mut Self {
    unsafe {
      obx_opt_async_pre_txn_delay4(self.obx_opt, delay_micros, delay2_micros, min_queue_length_for_delay2);
    }
    self
  }

  pub fn async_post_txn_delay(&mut self, delay_micros: u32) -> &mut Self {
    unsafe {
      obx_opt_async_post_txn_delay(self.obx_opt, delay_micros);
    }
    self
  }

  pub fn async_post_txn_delay5(&mut self, delay_micros: u32, delay2_micros: u32, min_queue_length_for_delay2: usize, subtract_processing_time: bool) -> &mut Self {
    unsafe {
      obx_opt_async_post_txn_delay5(self.obx_opt, delay_micros, delay2_micros, min_queue_length_for_delay2, subtract_processing_time);
    }
    self
  }

  pub fn async_minor_refill_threshold(&mut self, queue_length: usize) -> &mut Self {
    unsafe {
      obx_opt_async_minor_refill_threshold(self.obx_opt, queue_length);
    }
    self
  }

  pub fn async_minor_refill_max_count(&mut self, value: u32) -> &mut Self {
    unsafe {
      obx_opt_async_minor_refill_max_count(self.obx_opt, value);
    }
    self
  }

  pub fn async_max_tx_pool_size(&mut self, value: usize) -> &mut Self {
    unsafe {
      obx_opt_async_max_tx_pool_size(self.obx_opt, value);
    }
    self
  }

  pub fn async_object_bytes_max_cache_size(&mut self, value: u64) -> &mut Self {
    unsafe {
      obx_opt_async_object_bytes_max_cache_size(self.obx_opt, value);
    }
    self
  }

  pub fn async_object_bytes_max_size_to_cache(&mut self, value: u64) -> &mut Self {
    unsafe {
      obx_opt_async_object_bytes_max_size_to_cache(self.obx_opt, value);
    }
    self
  }

  pub fn get_directory(&self) -> &str {
    unsafe { 
      let c_str = obx_opt_get_directory(self.obx_opt);
      if let Ok(r) = CStr::from_ptr(c_str).to_str() {
        r
      }else {
        panic!("Error: can't get directory");
      }
    }
  }

  pub fn get_max_db_size_in_kb(&self) -> u64 {
    unsafe { obx_opt_get_max_db_size_in_kb(self.obx_opt) }
  }

  pub fn get_max_data_size_in_kb(&self) -> u64 {
    unsafe { obx_opt_get_max_data_size_in_kb(self.obx_opt) }
  }

  pub fn get_debug_flags(&self) -> u32 {
    unsafe { obx_opt_get_debug_flags(self.obx_opt) }
  }
}
