use std::ops::{Deref, DerefMut};

use mongodb::{bson::RawDocumentBuf, sync::Cursor};

use crate::{
    bson::{bson_error_t, bson_new, bson_t},
    client::mongoc_client_t,
};

pub struct mongoc_cursor_t {
    rust_cursor: Cursor<RawDocumentBuf>,
}

impl Deref for mongoc_cursor_t {
    type Target = Cursor<RawDocumentBuf>;

    fn deref(&self) -> &Self::Target {
        &self.rust_cursor
    }
}

impl DerefMut for mongoc_cursor_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_cursor
    }
}

impl mongoc_cursor_t {
    pub(crate) fn new(cursor: Cursor<RawDocumentBuf>) -> Self {
        Self {
            rust_cursor: cursor,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_new_from_command_reply_with_opts(
    _client: *mut mongoc_client_t,
    _reply: *mut bson_t<'static>,
    _opts: *const bson_t<'static>,
) -> *mut mongoc_cursor_t {
    panic!("cant implement")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_next(
    cursor: *mut mongoc_cursor_t,
    bson: *mut *const bson_t,
) -> bool {
    let result: anyhow::Result<_> = (|| {
        let result = (*cursor).next().transpose()?;
        Ok(result)
    })();

    match result {
        Ok(Some(doc)) => {
            *bson = Box::into_raw(Box::new(doc.into()));
            true
        }
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_more(_cursor: *mut mongoc_cursor_t) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_get_id(cursor: *mut mongoc_cursor_t) -> i64 {
    (*cursor).id()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_error_document(
    _cursor: *mut mongoc_cursor_t,
    _error: *mut bson_error_t,
    _reply: *const *mut bson_t<'static>,
) -> bool {
    // TODO: implement this
    false
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_cursor_destroy(cursor: *mut mongoc_cursor_t) {
    drop(Box::from_raw(cursor))
}
