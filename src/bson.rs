use std::{ffi::{CStr, CString}, ops::Deref, os::raw::c_char};

use anyhow::Result;
use mongodb::bson::{Document, RawDocument, RawDocumentBuf};

#[allow(non_camel_case_types)]
pub struct bson_t {
    pub(crate) doc: RawDocumentBuf,
}

impl Deref for bson_t {
    type Target = RawDocument;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl bson_t {
    pub(crate) fn to_document(&self) -> Result<Document> {
        Ok(self.doc.to_document()?)
    }
}

impl From<RawDocumentBuf> for bson_t {
    fn from(doc: RawDocumentBuf) -> Self {
        Self { doc }
    }
}

#[no_mangle]
pub unsafe extern "C" fn bson_new() -> *mut bson_t {
    Box::into_raw(Box::new(bson_t {
        doc: RawDocumentBuf::new(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_int32(bson: *mut bson_t, key: *const c_char, val: i32) {
    (*bson)
        .doc
        .append(CStr::from_ptr(key).to_str().unwrap(), val)
}

#[no_mangle]
pub unsafe extern "C" fn bson_to_string(bson: *const bson_t) -> *mut c_char {
    CString::new(format!("{}", (*bson).doc.to_document().unwrap()))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn bson_destroy(bson: *mut bson_t) {
    drop(Box::from_raw(bson));
}
