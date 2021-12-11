use std::{ffi::CStr, ops::Deref, os::raw::c_char};

use mongodb::{
    bson::RawDocumentBuf,
    options::{AggregateOptions, CreateCollectionOptions},
    sync::{Client, Database},
};

use crate::{
    bson::{bson_error_t, bson_t},
    client::{make_agg_pipeline, mongoc_client_t},
    collection::mongoc_collection_t,
    cursor::mongoc_cursor_t,
    read_concern::mongoc_read_concern_t,
    read_pref::mongoc_read_prefs_t,
    write_concern::mongoc_write_concern_t,
};

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
pub unsafe extern "C" fn mongoc_database_set_read_concern(
    database: *const mongoc_database_t,
    read_concern: *const mongoc_read_concern_t,
) -> bool {
    panic!("cant do this in rust")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_get_read_concern(
    database: *const mongoc_database_t,
) -> *const mongoc_read_concern_t {
    panic!("cant do this in rust")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_set_write_concern(
    database: *const mongoc_database_t,
    read_concern: *const mongoc_write_concern_t,
) -> bool {
    panic!("cant do this in rust")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_get_write_concern(
    database: *const mongoc_database_t,
) -> *const mongoc_write_concern_t {
    panic!("cant do this in rust")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_set_read_prefs(
    database: *const mongoc_database_t,
    read_concern: *const mongoc_read_prefs_t,
) -> bool {
    panic!("cant do this in rust")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_get_read_prefs(
    database: *const mongoc_database_t,
) -> *const mongoc_read_prefs_t {
    panic!("cant do this in rust")
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
            *reply = r.into();
            true
        }
        Err(e) => {
            // TODO: set error here
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_command_with_opts(
    database: *const mongoc_database_t,
    command: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
    opts: *const bson_t,
    reply: *mut bson_t,
    _error: *mut bson_error_t,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        println!("building command");
        let mut command = (*command).to_document()?;
        command.extend((*opts).to_document()?);
        println!("got command: {}", command);
        let reply = (*database).database.run_command(command, None)?;
        println!("got reply: {}", reply);
        Ok(RawDocumentBuf::from_document(&reply)?)
    })();

    match result {
        Ok(r) => {
            println!("assigning reply");
            *reply = r.into();
            println!("done");
            true
        }
        Err(_e) => {
            // TODO: set error here
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_read_command_with_opts(
    database: *const mongoc_database_t,
    command: *const bson_t,
    read_pref: *const mongoc_read_prefs_t,
    opts: *const bson_t,
    reply: *mut bson_t,
    error: *mut bson_error_t,
) -> bool {
    mongoc_database_command_with_opts(database, command, read_pref, opts, reply, error)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_write_command_with_opts(
    database: *const mongoc_database_t,
    command: *const bson_t,
    opts: *const bson_t,
    reply: *mut bson_t,
    error: *mut bson_error_t,
) -> bool {
    mongoc_database_command_with_opts(database, command, std::ptr::null(), opts, reply, error)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_aggregate(
    database: *const mongoc_database_t,
    pipeline: *const bson_t,
    options: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
) -> *const mongoc_cursor_t {
    let result: anyhow::Result<_> = (|| {
        let opts: AggregateOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

        let pipeline = make_agg_pipeline(pipeline)?;

        let result = (*database).aggregate(pipeline, opts)?;
        Ok(Box::into_raw(Box::new(mongoc_cursor_t::new(
            result.with_type(),
        ))))
    })();

    match result {
        Ok(r) => r,
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_watch(
    database: *mut mongoc_database_t,
    pipeline: *const bson_t<'static>,
    opts: *const bson_t<'static>,
) -> *const mongoc_cursor_t {
    panic!("driver doesnt have change streams yet")
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
pub unsafe extern "C" fn mongoc_database_create_collection(
    database: *const mongoc_database_t,
    coll_name: *const c_char,
    opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> *mut mongoc_collection_t {
    let name = CStr::from_ptr(coll_name).to_string_lossy();
    let result: anyhow::Result<_> = (|| {
        let opts: CreateCollectionOptions = if !opts.is_null() {
            mongodb::bson::from_slice((*opts).as_bytes())?
        } else {
            Default::default()
        };

        let result = (*database).create_collection(name, opts)?;
        Ok(mongoc_database_get_collection(database, coll_name))
    })();

    match result {
        Ok(c) => c,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_database_destroy(database: *mut mongoc_database_t) {
    drop(Box::from_raw(database));
}
