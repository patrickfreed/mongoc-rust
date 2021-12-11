use mongodb::{
    bson::{rawdoc, Document, RawDocumentBuf},
    options::{DeleteOptions, InsertManyOptions, InsertOneOptions, ReplaceOptions, UpdateOptions},
    sync::Collection,
};

use crate::{
    bson::{bson_error_t, bson_t},
    write_concern::mongoc_write_concern_t,
};

pub struct mongoc_bulk_operation_t {
    operation: Operation,
    collection: Collection<RawDocumentBuf>,
}

impl mongoc_bulk_operation_t {
    pub(crate) fn new(collection: Collection<RawDocumentBuf>) -> Self {
        Self {
            operation: Operation::None,
            collection,
        }
    }
}

#[derive(Debug)]
enum Operation {
    None,
    UpdateOne {
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>,
    },
    UpdateMany {
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>,
    },
    ReplaceOne {
        filter: Document,
        replacement: RawDocumentBuf,
        options: Option<ReplaceOptions>,
    },
    InsertOne {
        document: RawDocumentBuf,
        options: Option<InsertOneOptions>,
    },
    InsertMany {
        documents: Vec<RawDocumentBuf>,
        options: Option<InsertManyOptions>,
    },
    DeleteOne {
        filter: Document,
        options: Option<DeleteOptions>,
    },
    DeleteMany {
        filter: Document,
        options: Option<DeleteOptions>,
    },
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_insert_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    document: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::InsertOne {
            document: (*document).to_owned(),
            options: None,
        },
        Operation::InsertOne {
            document: existing_document,
            options,
        } => Operation::InsertMany {
            documents: vec![existing_document.clone(), (*document).to_owned()],
            options: None,
        },
        o => panic!("cant add insert one to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_remove_many_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    filter: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::DeleteMany {
            filter: (*filter).to_document().unwrap(),
            options: None,
        },
        o => panic!("cant add insert one to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_remove_one_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    filter: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::DeleteOne {
            filter: (*filter).to_document().unwrap(),
            options: None,
        },
        o => panic!("cant add insert one to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_replace_one_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    filter: *const bson_t<'static>,
    replacement: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::ReplaceOne {
            filter: (*filter).to_document().unwrap(),
            replacement: (*replacement).to_owned(),
            options: None,
        },
        o => panic!("cant add insert one to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_update_many_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    filter: *const bson_t<'static>,
    update: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::UpdateMany {
            filter: (*filter).to_document().unwrap(),
            update: (*update).to_document().unwrap(),
            options: None,
        },
        o => panic!("cant add update many to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_update_one_with_opts(
    bulk: *mut mongoc_bulk_operation_t,
    filter: *const bson_t<'static>,
    update: *const bson_t<'static>,
    _opts: *const bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    let new_op = match &(*bulk).operation {
        Operation::None => Operation::UpdateOne {
            filter: (*filter).to_document().unwrap(),
            update: (*update).to_document().unwrap(),
            options: None,
        },
        o => panic!("cant add update one to {:?}", o),
    };
    (*bulk).operation = new_op;
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_execute(
    bulk: *mut mongoc_bulk_operation_t,
    reply: *mut bson_t<'static>,
    _error: *mut bson_error_t,
) -> u32 {
    println!("RUST DEBUG: executing bulk write");
    let result: anyhow::Result<_> = (|| match &(*bulk).operation {
        Operation::InsertOne { document, options } => {
            let _insert_one = (*bulk).collection.insert_one(document, options.clone())?;
            Ok(rawdoc! {
                "nInserted": 1
            })
        }
        _ => todo!(),
    })();
    match result {
        Ok(r) => {
            *reply = r.into();
            1
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_get_write_concern(
    bulk: *mut mongoc_bulk_operation_t,
) -> *const mongoc_write_concern_t {
    todo!("implement get write concern bulk")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_bulk_operation_destroy(bulk: *mut mongoc_bulk_operation_t) {
    drop(Box::from_raw(bulk))
}
