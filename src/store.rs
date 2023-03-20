#![allow(dead_code)]
use std::ffi::CString;
use std::path::Path;
use std::rc::Rc;

use anymap::AnyMap;

use crate::c::{self, *};
use crate::error::{self, Error};

use crate::opt::Opt;
use crate::traits::{FactoryBlanket, OBBlanket};
use crate::util::ToCChar;

// Caveat: copy and drop are mutually exclusive

pub struct Store {
    pub trait_map: AnyMap, // passed as a ref to a Box
    // TODO confirm: model and opt are cleaned up already and zero'ed, or else we'll have a double-free
    pub(crate) obx_store: *mut OBX_store, // TODO confirm: model and opt are cleaned up already
}

impl Drop for Store {
    fn drop(&mut self) {
        if !self.obx_store.is_null() {
            match self.prepare_then_close() {
                Err(err) => eprintln!("Error: store: {err}"),
                _ => (),
            }
            self.obx_store = std::ptr::null_mut();
        }
    }
}

// TODO Bonus: start admin http in debug from store?

impl Store {
    /// Assumes ownership of map, and Opt,
    pub fn new(mut opt: Opt, map: AnyMap) -> error::Result<Self> {
        let obx_store = c::new_mut(unsafe { obx_store_open(opt.obx_opt) })?;
        // This prevents a double free
        opt.ptr_consumed = !obx_store.is_null();
        let r = Store {
            trait_map: map,
            obx_store,
        };
        Ok(r)
    }

    pub fn get_box<T: 'static + OBBlanket>(&self) -> error::Result<crate::r#box::Box<T>> {
        let helper = if let Some(h) = self.trait_map.get::<Rc<dyn FactoryBlanket<T>>>() {
            h
        } else {
            Error::new_local("Error: unable to get entity helper").as_result()?
        };
        Ok(crate::r#box::Box::<T>::new(self.obx_store, helper.clone()))
    }

    pub fn is_open(path: &Path) -> bool {
        unsafe { obx_store_is_open(path.as_c_char_ptr()) }
    }

    // TODO support later
    /*
    pub fn from_path_attach(path: &Path) -> Self {
        Store {
            obx_store: unsafe { obx_store_attach(path.to_c_char()) },
            error: None,
            trait_map: None,
        }
    }

    pub fn from_store_id_attach(store_id: u64) -> Self {
        Store {
            obx_store: unsafe { obx_store_attach_id(store_id) },
            error: None,
            trait_map: None,
        }
    }

    pub fn attach_or_open(
        opt: *mut OBX_store_options,
        check_matching_options: bool,
        out_attached: *mut bool,
    ) -> Self {
        Store {
            obx_store: unsafe {
                obx_store_attach_or_open(opt, check_matching_options, out_attached)
            },
            error: None,
            trait_map: None,
        }
    }
    */

    // TODO Determine if this is safe
    pub fn id(&self) -> u64 {
        unsafe { obx_store_id(self.obx_store) }
    }

    // TODO impl without Copy/Clone trait, because Drop, then use over channels
    // pub fn clone(&self) -> Self {
    //   Store {
    //     obx_store: unsafe { obx_store_clone(self.obx_store) },
    //     error: None,
    //   }
    // }

    pub fn from_core_wrap(core_store: &mut Vec<u8>, map: AnyMap) -> error::Result<Self> {
        // TODO test
        let ptr = unsafe { obx_store_wrap(core_store.as_ptr() as *mut std::ffi::c_void) };
        c::new_mut(ptr).map(|s| Store {
            obx_store: s,
            trait_map: map,
        })
    }

    fn set_entity_id(&self, entity_name: &str) -> error::Result<obx_schema_id> {
        unsafe {
            if let Ok(cstr) = CString::new(entity_name) {
                Ok(obx_store_entity_id(self.obx_store, cstr.as_ptr()))
            } else {
                Error::new_local("Error: unable to parse the entity id").as_result()?
            }
        }
    }

    fn entity_property_id(
        &self,
        entity_id: obx_schema_id,
        property_name: &str,
    ) -> error::Result<obx_schema_id> {
        unsafe {
            if let Ok(cstr) = CString::new(property_name) {
                Ok(obx_store_entity_property_id(
                    self.obx_store,
                    entity_id,
                    cstr.as_ptr(),
                ))
            } else {
                Error::new_local("Error: unable to parse the property id").as_result()?
            }
        }
    }

    pub fn await_async_completion(&self) -> bool {
        unsafe { obx_store_await_async_completion(self.obx_store) }
    }

    pub fn await_async_submitted(&self) -> bool {
        unsafe { obx_store_await_async_submitted(self.obx_store) }
    }

    pub fn debug_flags(&self, flags: OBXDebugFlags) -> error::Result<&Self> {
        c::call(unsafe { obx_store_debug_flags(self.obx_store, flags) }).map(|_| self)
    }

    pub fn opened_with_previous_commit(&self) -> bool {
        unsafe { obx_store_opened_with_previous_commit(self.obx_store) }
    }

    fn prepare_to_close(&self) -> error::Result<&Self> {
        c::call(unsafe { obx_store_prepare_to_close(self.obx_store) }).map(|_| self)
    }

    fn close(&self) -> error::Result<&Self> {
        c::call(unsafe { obx_store_close(self.obx_store) }).map(|_| self)
    }

    fn prepare_then_close(&self) -> error::Result<&Self> {
        self.prepare_to_close()?.close()
    }
}
