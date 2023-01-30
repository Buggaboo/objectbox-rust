#[allow(dead_code)]

use std::ptr;

use crate::c;
use crate::c::*;
use crate::error::Error;
use crate::store::Store;

pub(crate) struct Tx {
  error: Option<Error>,
  pub(crate) obx_txn: *mut OBX_txn,
}

impl Drop for Tx {
  fn drop(&mut self) {
    unsafe {
      if !self.obx_txn.is_null() {
        self.error = c::call(c::obx_txn_close(self.obx_txn)).err();
        self.obx_txn = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        println!("Error: {}", err);
      }
    }
  }
}

impl Tx {
  // TODO implement new and new_mut as closures
  // fn _new_op(store: &mut Store, op: Fn(*mut OBX_store) -> *mut OBX_txn) -> Self {
  //   if store.obx_store.is_null() {
  //     panic!("Error: uninitialized store");
  //   }
  //   match c::new_mut(unsafe { op(store.obx_store) }) {
  //     Ok(obx_txn) => Tx { obx_txn, error: None },
  //     Err(e) => Tx {
  //       error: Some(e),
  //       obx_txn: ptr::null_mut(),
  //       obx_store: ptr::null_mut(),
  //     },
  //   }
  // }
  
  fn new(store: &Store) -> Self {
    if store.obx_store.is_null() {
      panic!("Error: uninitialized store");
    }
    match c::new_mut(unsafe { obx_txn_read(store.obx_store) }) {
      Ok(obx_txn) => Tx { obx_txn, error: None },
      Err(e) => Tx {
        error: Some(e),
        obx_txn: ptr::null_mut(),
      },
    }
  }
  
  fn new_mut(store: &mut Store) -> Self {
    if store.obx_store.is_null() {
      panic!("Error: uninitialized store");
    }
    match c::new_mut(unsafe { obx_txn_write(store.obx_store) }) {
      Ok(obx_txn) => Tx { obx_txn, error: None },
      Err(e) => Tx {
        error: Some(e),
        obx_txn: ptr::null_mut(),
      },
    }
  }

  fn success(&mut self) {
    self.error = c::call(unsafe {obx_txn_success(self.obx_txn) }).err();
  }

  fn abort(&mut self) {
    self.error = c::call(unsafe { obx_txn_abort(self.obx_txn) }).err();
  }

  fn data_size(&mut self) -> (u64, u64) {
      let mut committed_size = 0;
      let mut size_change = 0;
      self.error = c::call(unsafe {
          obx_txn_data_size(self.obx_txn, &mut committed_size, &mut size_change)
      }).err();
      (committed_size, size_change)
  }
}



