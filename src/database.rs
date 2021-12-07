use std::{ffi::CStr, ops::Deref, os::raw::c_char};

use mongodb::{
    bson::RawDocumentBuf,
    sync::{Client, Database},
};

use crate::{bson::bson_t, client::mongoc_client_t, collection::mongoc_collection_t};

#[allow(non_camel_case_types)]
pub struct mongoc_database_t {
    database: Database,
}

impl mongoc_database_t {
    pub(crate) unsafe fn new(client: &Client, name: impl AsRef<str>) -> mongoc_database_t {
        mongoc_database_t {
            database: (*client).database(name.as_ref()),
        }
    }
}

impl Deref for mongoc_database_t {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.database
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_command_simple(
    database: *const mongoc_database_t,
    command: *const bson_t,
    _read_pref: *const u8,
    reply: *mut bson_t,
    _error: *const u8,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        let reply = (*database)
            .database
            .run_command((*command).to_document()?, None)?;
        Ok(RawDocumentBuf::from_document(&reply)?)
    })();

    match result {
        Ok(r) => {
            *reply = bson_t { doc: r };
            true
        }
        Err(e) => {
            // TODO: set error here
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_get_collection(
    database: *const mongoc_database_t,
    coll_name: *const c_char,
) -> *mut mongoc_collection_t {
    let name = CStr::from_ptr(coll_name).to_string_lossy();
    let coll = mongoc_collection_t::new((*database).deref(), name);
    Box::into_raw(Box::new(coll))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_destroy(database: *mut mongoc_database_t) {
    drop(Box::from_raw(database));
}
