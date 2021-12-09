use std::{borrow::Borrow, marker::PhantomData, ops::Deref, ptr};

use mongodb::{
    bson::{Document, RawBsonRef, RawDocument, RawDocumentBuf},
    options::{AggregateOptions, CountOptions, FindOptions, InsertOneOptions},
    sync::{Collection, Database},
};

use crate::{
    bson::{bson_error_t, bson_t},
    client::mongoc_client_t,
    cursor::mongoc_cursor_t,
    database::mongoc_database_t,
    mongoc_query_flags_t,
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
    _read_pref: *const u8,
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
    _flags: u32,
    pipeline: *const bson_t,
    options: *const bson_t,
    _read_pref: *const u8,
) -> *mut mongoc_cursor_t {
    let result: anyhow::Result<_> = (|| {
        let opts: AggregateOptions = if !options.is_null() {
            mongodb::bson::from_slice((*options).as_bytes())?
        } else {
            Default::default()
        };

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
pub unsafe extern "C" fn mongoc_collection_count_documents(
    collection: *const mongoc_collection_t,
    filter: *const bson_t,
    options: *const bson_t,
    _read_pref: *const u8,
    _reply: *mut bson_t,
    _error: *const u8,
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
pub unsafe extern "C" fn mongoc_collection_destroy(collection: *mut mongoc_collection_t) {
    drop(Box::from_raw(collection));
}
