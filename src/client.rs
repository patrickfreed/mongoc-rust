use std::{ffi::CStr, ops::Deref, os::raw::c_char};

use mongodb::{
    bson::{Document, RawBsonRef, RawDocumentBuf},
    sync::Client,
};

use crate::{
    bson::{bson_error_t, bson_t},
    database::mongoc_database_t,
    session::mongoc_client_session_t,
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
pub unsafe extern "C" fn mongoc_client_database(
    client: *mut mongoc_client_t,
    db_name: *const c_char,
) -> *mut mongoc_database_t {
    let name = CStr::from_ptr(db_name).to_string_lossy();
    let db = mongoc_database_t::new((*client).deref(), name);
    Box::into_raw(Box::new(db))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_start_session(
    client: *mut mongoc_client_t,
    _opts: *mut u8,
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
