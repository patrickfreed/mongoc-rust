pub mod api;
pub mod apm;
pub mod bson;
pub mod bulk;
mod change_stream;
mod client;
pub mod client_pool;
mod collection;
mod cursor;
mod database;
pub mod error;
pub mod find_and_modify;
pub mod read_concern;
pub mod read_pref;
pub mod session;
pub mod uri;
pub mod write_concern;

use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn mongoc_init() {}

#[no_mangle]
pub extern "C" fn mongoc_handshake_data_append(
    _driver_name: *const c_char,
    _driver_version: *const c_char,
    _platform: *const c_char,
) -> bool {
    // TODO: implement this
    true
}

#[no_mangle]
pub extern "C" fn mongoc_cleanup() {}

#[repr(C)]
pub enum mongoc_query_flags_t {
    MONGOC_QUERY_NONE = 0,
    MONGOC_QUERY_TAILABLE_CURSOR = 1 << 1,
    MONGOC_QUERY_SECONDARY_OK = 1 << 2,
    MONGOC_QUERY_OPLOG_REPLAY = 1 << 3,
    MONGOC_QUERY_NO_CURSOR_TIMEOUT = 1 << 4,
    MONGOC_QUERY_AWAIT_DATA = 1 << 5,
    MONGOC_QUERY_EXHAUST = 1 << 6,
    MONGOC_QUERY_PARTIAL = 1 << 7,
}

pub const MONGOC_ERROR_API_VERSION_2: i32 = 2;
