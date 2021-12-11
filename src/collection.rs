use std::{borrow::Borrow, ffi::CStr, marker::PhantomData, ops::Deref, os::raw::c_char, ptr};

use mongodb::{
    bson::{doc, Document, RawBsonRef, RawDocument, RawDocumentBuf},
    options::{
        AggregateOptions, CountOptions, EstimatedDocumentCountOptions, FindOptions,
        InsertOneOptions, ListIndexesOptions,
    },
    sync::{Collection, Database},
};

use crate::{
    bson::{bson_error_t, bson_t},
    bulk::mongoc_bulk_operation_t,
    client::{make_agg_pipeline, mongoc_client_t},
    cursor::mongoc_cursor_t,
    database::mongoc_database_t,
    find_and_modify::mongoc_find_and_modify_opts_t,
    mongoc_query_flags_t,
    read_concern::mongoc_read_concern_t,
    read_pref::mongoc_read_prefs_t,
    write_concern::mongoc_write_concern_t,
};

#[allow(non_camel_case_types)]
pub struct mongoc_collection_t {
    rust_collection: Collection<RawDocumentBuf>,
}

impl mongoc_collection_t {
    pub(crate) fn new(database: &Database, name: impl AsRef<str>) -> mongoc_collection_t {
        mongoc_collection_t {
            rust_collection: database.collection(name.as_ref()),
        }
    }
}

impl Deref for mongoc_collection_t {
    type Target = Collection<RawDocumentBuf>;

    fn deref(&self) -> &Self::Target {
        &self.rust_collection
    }
}

impl From<Collection<RawDocumentBuf>> for mongoc_collection_t {
    fn from(c: Collection<RawDocumentBuf>) -> Self {
        Self { rust_collection: c }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_set_read_concern(
    collection: *const mongoc_collection_t,
    rc: *const mongoc_read_concern_t,
) -> bool {
    panic!("cant set read concern after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_get_read_concern(
    collection: *const mongoc_collection_t,
) -> *const mongoc_read_concern_t {
    todo!("get read concern after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_set_write_concern(
    collection: *const mongoc_collection_t,
    rc: *const mongoc_write_concern_t,
) -> bool {
    panic!("cant set write concern after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_get_write_concern(
    collection: *const mongoc_collection_t,
) -> *const mongoc_write_concern_t {
    panic!("cant get write concern after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_set_read_prefs(
    collection: *const mongoc_collection_t,
    rc: *const mongoc_read_prefs_t,
) -> bool {
    panic!("cant set read prefs after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_get_read_prefs(
    collection: *const mongoc_collection_t,
) -> *const mongoc_read_prefs_t {
    panic!("cant get read prefs after creation")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_insert_one(
    collection: *const mongoc_collection_t,
    document: *const bson_t,
    options: *const bson_t,
    reply: *mut bson_t,
    _error: *mut bson_error_t,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        let options: Option<InsertOneOptions> = if !options.is_null() {
            Some(mongodb::bson::from_slice((*options).as_bytes())?)
        } else {
            None
        };
        let result = (*collection)
            .clone_with_type::<&RawDocument>()
            .insert_one((*document).deref(), options)?;
        Ok(mongodb::bson::to_raw_document_buf(&result)?)
    })();

    match result {
        Ok(r) => {
            *reply = r.into();
            true
        }
        Err(e) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_find_with_opts(
    collection: *const mongoc_collection_t,
    filter: *const bson_t,
    options: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
) -> *mut mongoc_cursor_t {
    let result: anyhow::Result<_> = (|| {
        let opts: Option<FindOptions> = if !options.is_null() {
            Some(mongodb::bson::from_slice((*options).as_bytes())?)
        } else {
            None
        };
        let result = (*collection).find((*filter).to_document()?, opts)?;
        Ok(Box::into_raw(Box::new(mongoc_cursor_t::new(result))))
    })();

    match result {
        Ok(r) => r,
        Err(_e) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_aggregate(
    collection: *const mongoc_collection_t,
    _flags: mongoc_query_flags_t,
    pipeline: *const bson_t,
    options: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
) -> *mut mongoc_cursor_t {
    let result: anyhow::Result<_> = (|| {
        let opts: AggregateOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

        let pipeline = make_agg_pipeline(pipeline)?;

        let result = (*collection).aggregate(pipeline, opts)?;
        Ok(Box::into_raw(Box::new(mongoc_cursor_t::new(
            result.with_type(),
        ))))
    })();

    match result {
        Ok(r) => r,
        Err(_e) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_watch(
    database: *mut mongoc_database_t,
    pipeline: *const bson_t<'static>,
    opts: *const bson_t<'static>,
) -> *const mongoc_cursor_t {
    panic!("driver doesnt have change streams yet")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_count_documents(
    collection: *const mongoc_collection_t,
    filter: *const bson_t,
    options: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
    _reply: *mut bson_t,
    _error: *mut bson_error_t,
) -> i64 {
    let result: anyhow::Result<_> = (|| {
        let opts: CountOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

        let filter = (*filter).to_document()?;

        let result = (*collection).count_documents(filter, opts)?;
        Ok(result)
    })();

    match result {
        Ok(r) => r as i64,
        Err(_e) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_estimated_document_count(
    collection: *const mongoc_collection_t,
    options: *const bson_t,
    _read_pref: *const mongoc_read_prefs_t,
    _reply: *mut bson_t,
    _error: *mut bson_error_t,
) -> i64 {
    let result: anyhow::Result<_> = (|| {
        let opts: EstimatedDocumentCountOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

        let result = (*collection).estimated_document_count(opts)?;
        Ok(result)
    })();

    match result {
        Ok(r) => r as i64,
        Err(_e) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_find_indexes_with_opts(
    collection: *const mongoc_collection_t,
    options: *const bson_t,
) -> *mut mongoc_cursor_t {
    let result: anyhow::Result<_> = (|| {
        let opts: ListIndexesOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

        let result = (*collection).list_indexes(opts)?;
        Ok(result)
    })();

    match result {
        Ok(r) => Box::into_raw(Box::new(mongoc_cursor_t::new(r.with_type()))),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_rename_with_opts(
    collection: *mut mongoc_collection_t,
    new_db: *const c_char,
    new_name: *const c_char,
    drop_target_before_rename: bool,
    options: *const bson_t,
    _error: *const bson_error_t,
) -> bool {
    let new_db = CStr::from_ptr(new_db).to_string_lossy();
    let new_name = CStr::from_ptr(new_name).to_string_lossy();

    let result: anyhow::Result<_> = (|| {
        let opts = if !options.is_null() {
            (*options).to_document()?
        } else {
            Document::new()
        };

        let mut cmd = doc! {
            "renameCollection": format!("{}", (*collection).namespace()),
            "to": format!("{}.{}", new_db, new_name),
            "dropTarget": drop_target_before_rename
        };
        cmd.extend(opts);

        Ok((*collection).database().run_command(cmd, None)?)
    })();

    result.is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_read_command_with_opts(
    collection: *mut mongoc_collection_t,
    command: *const bson_t<'static>,
    _rp: *const mongoc_read_prefs_t,
    opts: *const bson_t<'static>,
    reply: *mut bson_t<'static>,
    _error: *const bson_error_t,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        let opts = if !opts.is_null() {
            (*opts).to_document()?
        } else {
            Document::new()
        };

        let mut cmd = (*command).to_document()?;
        cmd.extend(opts);

        let doc_reply = (*collection).database().run_command(cmd, None)?;
        let raw_doc_reply = RawDocumentBuf::from_document(&doc_reply)?;
        Ok(raw_doc_reply)
    })();

    match result {
        Ok(r) => {
            *reply = r.into();
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_write_command_with_opts(
    collection: *mut mongoc_collection_t,
    command: *const bson_t<'static>,
    opts: *const bson_t<'static>,
    reply: *mut bson_t<'static>,
    error: *const bson_error_t,
) -> bool {
    mongoc_collection_read_command_with_opts(
        collection,
        command,
        std::ptr::null(),
        opts,
        reply,
        error,
    )
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_create_bulk_operation_with_opts(
    collection: *mut mongoc_collection_t,
    opts: *const bson_t<'static>,
) -> *mut mongoc_bulk_operation_t {
    Box::into_raw(Box::new(mongoc_bulk_operation_t::new(
        (*collection).clone(),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_find_and_modify_with_opts(
    collection: *mut mongoc_collection_t,
    query: *const bson_t<'static>,
    opts: *const mongoc_find_and_modify_opts_t,
    reply: *mut bson_t<'static>,
    error: *mut bson_error_t,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_destroy(collection: *mut mongoc_collection_t) {
    drop(Box::from_raw(collection));
}
