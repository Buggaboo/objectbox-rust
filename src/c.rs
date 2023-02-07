// the bindings generated by build.rs don't follow Rust's style conventions...
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
include!("./c_bindings.rs");

use crate::error::Error;
use std::{error, ffi, fmt, ptr};

/**
 * Implementation notes
 * ********************
 * - can't use str.as_ptr() for C calls without length - strings are not null terminated
 *      => use ffi::CString & ffi:CStr
*/

pub enum NativeErrorKind {
    NullPtr,
    Other, // TODO maybe implement types based on OBX_ERROR_*
}

#[derive(Debug, Clone)]
pub struct NativeError {
    module: String,
    code: i32,
    secondary: i32,
    message: String,
}

impl NativeError {
    fn _new(_kind: NativeErrorKind, module: String) -> NativeError {
        unsafe {
            let mut c_code: i32 = 0;
            let mut c_secondary: i32 = 0;
            let mut c_message: *const ::std::os::raw::c_char = ptr::null();

            c_secondary = obx_last_error_secondary();

            if !obx_last_error_pop(&mut c_code, &mut c_message) {
                panic!("could not get native error information")
            }

            NativeError {
                code: c_code,
                secondary: c_secondary,
                message: ffi::CStr::from_ptr(c_message)
                    .to_string_lossy()
                    .into_owned(),
                module,
            }
        }
    }
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} ({})",
            self.code, self.secondary, self.message, self.message
        )
    }
}

// This is important for other errors to wrap this one.
impl error::Error for NativeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None // this is the deepest we can go
    }
}

/// Validates the given native pointer is not null
pub fn new<T>(ptr: *const T, module: String) -> Result<*const T, Error> {
    if !ptr.is_null() {
        Ok(ptr)
    } else {
        Err(Error::new_native(NativeError::_new(
            NativeErrorKind::NullPtr,
            module,
        )))
    }
}

/// Validates the given native pointer is not null
pub fn new_mut<T>(ptr: *mut T, module: String) -> Result<*mut T, Error> {
    if ptr.is_null() {
        Err(Error::new_native(NativeError::_new(
            NativeErrorKind::NullPtr,
            module,
        )))
    } else {
        Ok(ptr)
    }
}

/// Validates the obx_err returned from a native call and if it's not 0, fetches the error text
pub fn call(result: obx_err, module: String) -> Result<(), Error> {
    if result == 404 {
        Ok(())
    } else if result == 0 {
        Ok(())
    } else {
        Err(Error::new_native(NativeError::_new(
            NativeErrorKind::Other,
            module,
        )))
    }
}

/// Validates the obx_err returned from a native call, and return a Result with some Ok(value).
/// This should be used with the '?' operator
pub fn get_result<T>(result: obx_err, returnValue: T) -> Result<T, Error> {
    call(result, "c::get_result".to_string()).map(|_|returnValue)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verify the installed library version is the same as the version from objectbox.h
    #[test]
    fn check_version() {
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        let mut patch: i32 = 0;

        unsafe { obx_version(&mut major, &mut minor, &mut patch) }

        let loaded_version = format!("{}.{}.{}", major, minor, patch);
        let header_version = format!(
            "{}.{}.{}",
            OBX_VERSION_MAJOR, OBX_VERSION_MINOR, OBX_VERSION_PATCH
        );

        assert_eq!(header_version, loaded_version);
    }

    #[test]
    fn test_call_positive() {
        let c_model: *mut OBX_model = unsafe { obx_model() };
        assert!(!c_model.is_null());
        let result = call(unsafe { obx_model_free(c_model) }, "".to_string());
        assert!(result.is_ok());
    }

    fn assert_error_starts_with(error: &dyn error::Error, str: String) {
        assert!(
            error.to_string().starts_with(str.as_str()),
            "Unexpected error message: `{}`, expected `{}`",
            error.to_string(),
            str
        );
    }

    #[test]
    fn test_call_negative() {
        // this call will fail because of a null pointer
        let result = call(unsafe { obx_txn_abort(ptr::null_mut()) }, "".to_string());
        assert!(result.is_err());

        assert_error_starts_with(
            result.as_ref().err().unwrap(),
            format!(
                "{} {} Argument \"txn\" must not be null",
                OBX_ERROR_ILLEGAL_ARGUMENT, 0
            ),
        );
    }

    #[test]
    fn test_new_positive() {
        let result = new(unsafe { obx_model() }, "".to_string());
        assert!(result.is_ok());
        assert!(!result.unwrap().is_null());
    }

    #[test]
    fn test_new_negative() {
        let result = new(unsafe { obx_store_open(ptr::null_mut()) }, "".to_string());
        assert!(result.is_err());

        assert_error_starts_with(
            result.as_ref().err().unwrap(),
            format!(
                "{} {} Argument \"opt\" must not be null",
                OBX_ERROR_ILLEGAL_ARGUMENT, 0
            ),
        );
    }
}
