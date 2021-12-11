use std::{ffi::CStr, ops::Deref, os::raw::c_char};

use mongodb::{
    bson::{Document, RawBsonRef, RawDocumentBuf},
    sync::Client,
};

use crate::{
    bson::{bson_error_t, bson_t},
    collection::mongoc_collection_t,
    cursor::mongoc_cursor_t,
    database::{mongoc_database_command_with_opts, mongoc_database_destroy, mongoc_database_t},
    read_concern::mongoc_read_concern_t,
    read_pref::mongoc_read_prefs_t,
    session::{mongoc_client_session_t, mongoc_session_opt_t},
    write_concern::mongoc_write_concern_t,
};

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

impl From<Client> for mongoc_client_t {
    fn from(client: Client) -> Self {
        Self { client }
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
        .run_command((*command).to_document().unwrap(), None)
        .unwrap();

    *reply = RawDocumentBuf::from_document(&result).unwrap().into();
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_read_command_with_opts(
    client: *mut mongoc_client_t,
    db_name: *const c_char,
    command: *const bson_t,
    read_pref: *const mongoc_read_prefs_t,
    opts: *const bson_t,
    reply: *mut bson_t,
    error: *mut bson_error_t,
) -> bool {
    let database = mongoc_client_get_database(client, db_name);
    let result =
        mongoc_database_command_with_opts(database, command, read_pref, opts, reply, error);
    mongoc_database_destroy(database);
    result
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_database(
    client: *mut mongoc_client_t,
    db_name: *const c_char,
) -> *mut mongoc_database_t {
    let name = CStr::from_ptr(db_name).to_string_lossy();
    println!("getting database {}", name);
    let db = mongoc_database_t::new((*client).deref(), name);
    Box::into_raw(Box::new(db))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_collection(
    client: *mut mongoc_client_t,
    db_name: *const c_char,
    coll_name: *const c_char,
) -> *mut mongoc_collection_t {
    let db_name = CStr::from_ptr(db_name).to_string_lossy();
    let coll_name = CStr::from_ptr(coll_name).to_string_lossy();
    let coll = (*client)
        .database(db_name.as_ref())
        .collection(coll_name.as_ref());
    Box::into_raw(Box::new(coll.into()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_start_session(
    client: *mut mongoc_client_t,
    _opts: *const mongoc_session_opt_t,
    _error: *mut bson_error_t,
) -> *mut mongoc_client_session_t {
    match (*client).start_session(None) {
        Ok(s) => Box::into_raw(Box::new(s.into())),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_server_description(
    client: *mut mongoc_client_t,
    server_id: u32,
) -> *const u8 {
    panic!("server id not exposed by driver")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_read_concern(
    client: *mut mongoc_client_t,
) -> *const mongoc_read_concern_t {
    todo!("implement this")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_read_prefs(
    client: *mut mongoc_client_t,
) -> *const mongoc_read_prefs_t {
    todo!("implement this")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_get_write_concern(
    client: *mut mongoc_client_t,
) -> *const mongoc_write_concern_t {
    todo!("implement this")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_watch(
    client: *mut mongoc_client_t,
    pipeline: *const bson_t<'static>,
    opts: *const bson_t<'static>,
) -> *const mongoc_cursor_t {
    panic!("driver doesnt have change streams yet")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_destroy(client: *mut mongoc_client_t) {
    drop(Box::from_raw(client));
}

pub(crate) unsafe fn make_agg_pipeline(pipeline: *const bson_t) -> anyhow::Result<Vec<Document>> {
    let pipeline_doc = match (*pipeline).into_iter().next().transpose()? {
        Some(("0", _)) | None => (*pipeline).deref(),
        Some(("pipeline", RawBsonRef::Document(d))) => d,
        _ => anyhow::bail!("invalid pipeline document: {:#?}", (*pipeline).deref()),
    };

    let pipeline: Vec<Document> = pipeline_doc
        .into_iter()
        .map(|kvp| match kvp?.1 {
            RawBsonRef::Document(d) => Ok(mongodb::bson::to_document(&d)?),
            o => anyhow::bail!("expected document in pipeline, got {:?} instead", o),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(pipeline)
}
