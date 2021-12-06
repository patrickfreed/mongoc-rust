use std::ops::Deref;

use mongodb::{
    bson::RawDocumentBuf,
    options::InsertOneOptions,
    sync::{Collection, Database},
};

use crate::{bson::bson_t, client::mongoc_client_t, database::mongoc_database_t};

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
    _error: *const u8,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        let options: Option<InsertOneOptions> = if !options.is_null() {
            Some(mongodb::bson::from_slice((*options).as_bytes())?)
        } else {
            None
        };
        let result = (*collection).insert_one(&(*document).doc, options)?;
        Ok(mongodb::bson::to_raw_document_buf(&result)?)
    })();

    match result {
        Ok(r) => {
            *reply = bson_t { doc: r };
            true
        }
        Err(e) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_collection_destroy(collection: *mut mongoc_collection_t) {
    drop(Box::from_raw(collection));
}
