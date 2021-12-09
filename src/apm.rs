use std::os::raw::c_char;

use crate::bson::bson_t;

pub struct mongoc_apm_command_started_t {}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_context(
    _event: *const mongoc_apm_command_started_t,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_command(
    _event: *const mongoc_apm_command_started_t,
) -> *const bson_t {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_database_name(
    _event: *const mongoc_apm_command_started_t,
) -> *const c_char {
    "placeholder".as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_command_name(
    _event: *const mongoc_apm_command_started_t,
) -> *const c_char {
    "placeholder".as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_request_id(
    _event: *const mongoc_apm_command_started_t,
) -> i64 {
    12
}
