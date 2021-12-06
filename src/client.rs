use std::{ffi::CStr, ops::Deref, os::raw::c_char};

use mongodb::{bson::RawDocumentBuf, sync::Client};
use tokio::runtime::Runtime;

use crate::{bson::bson_t, database::mongoc_database_t};

#[allow(non_camel_case_types)]
pub struct mongoc_client_t {
    client: Client,
}

impl Deref for mongoc_client_t {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_new(uri_str: *const c_char) -> *mut mongoc_client_t {
    if uri_str.is_null() {
        panic!("null uri string")
    }

    let uri = CStr::from_ptr(uri_str);
    let client = Client::with_uri_str(uri.to_string_lossy().as_ref()).unwrap();
    Box::into_raw(Box::new(mongoc_client_t { client }))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_command_simple(
    client: *const mongoc_client_t,
    db_name: *const c_char,
    command: *const bson_t,
    _read_pref: *const u8,
    reply: *mut bson_t,
    _error: *const u8,
) {
    let result = (*client)
        .client
        .database(CStr::from_ptr(db_name).to_str().unwrap())
        .run_command((*command).doc.to_document().unwrap(), None)
        .unwrap();

    *reply = bson_t {
        doc: RawDocumentBuf::from_document(&result).unwrap(),
    };
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_database(
    client: *mut mongoc_client_t,
    db_name: *const c_char,
) -> *mut mongoc_database_t {
    let name = CStr::from_ptr(db_name).to_string_lossy();
    let db = mongoc_database_t::new((*client).deref(), name);
    Box::into_raw(Box::new(db))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_destroy(client: *mut mongoc_client_t) {
    drop(Box::from_raw(client));
}
