use std::os::raw::c_char;

use crate::bson::{bson_oid_t, bson_t};

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
) -> *const bson_t<'static> {
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

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_operation_id(
    _event: *const mongoc_apm_command_started_t,
) -> i64 {
    12
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_host(
    _event: *const mongoc_apm_command_started_t,
) -> *const mongoc_host_list_t {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_started_get_service_id(
    _event: *const mongoc_apm_command_started_t,
) -> *const bson_oid_t {
    std::ptr::null()
}

#[repr(C)]
pub struct mongoc_host_list_t {
    next: *const u8,
}

pub struct mongoc_apm_command_succeeded_t {}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_succeeded_get_context(
    _event: *const mongoc_apm_command_succeeded_t,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_succeeded_get_duration(
    _event: *const mongoc_apm_command_succeeded_t,
) -> i64 {
    12
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_apm_command_succeeded_get_reply(
    _event: *const mongoc_apm_command_succeeded_t,
) -> *const bson_t<'static> {
    std::ptr::null()
}
