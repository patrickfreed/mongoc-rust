use std::{
    ffi::{CStr, CString},
    ops::Deref,
    os::raw::c_char,
};

use anyhow::Result;
use mongodb::bson::{
    oid::ObjectId, DateTime, DbPointer, Decimal128, Document, RawArrayBuf, RawBinaryRef, RawBson,
    RawBsonRef, RawDocument, RawDocumentBuf, RawJavaScriptCodeWithScope, RawRegexRef, Timestamp,
};

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

    unsafe fn append(&mut self, key: *const c_char, val: impl Into<RawBson>) -> bool {
        let key = CStr::from_ptr(key).to_string_lossy();
        self.doc.append(key, val);
        true
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
pub unsafe extern "C" fn bson_append_int32(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: i32,
) -> bool {
    (*bson).append(key, val)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_int64(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: i64,
) -> bool {
    (*bson).append(key, val)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_utf8(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: *const c_char,
    _length: isize,
) -> bool {
    let s = CStr::from_ptr(val).to_string_lossy();
    (*bson).append(key, s.to_string())
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_document(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: *const bson_t,
) -> bool {
    (*bson).append(key, (*val).deref().to_raw_document_buf())
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_array(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: *const bson_t,
) -> bool {
    (*bson).append(key, RawArrayBuf::from_raw_document_buf((*val).doc.clone()))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_binary(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    subtype: bson_subtype_t,
    val: *const u8,
    length: u32,
) -> bool {
    let slice = std::slice::from_raw_parts(val, length as usize);
    let binary = RawBinaryRef {
        bytes: slice,
        subtype: (subtype as u8).into(),
    };
    (*bson).append(key, RawBsonRef::Binary(binary))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_bool(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: bool,
) -> bool {
    (*bson).append(key, val)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_code(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: *const c_char,
) -> bool {
    let s = CStr::from_ptr(val).to_string_lossy();
    (*bson).append(key, RawBson::JavaScriptCode(s.to_string()))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_code_with_scope(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    code: *const c_char,
    scope: *const bson_t,
) -> bool {
    let s = CStr::from_ptr(code).to_string_lossy();
    let scope = (*scope).doc.clone();
    (*bson).append(
        key,
        RawJavaScriptCodeWithScope {
            code: s.to_string(),
            scope,
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_date_time(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    value: i64,
) -> bool {
    let dt = DateTime::from_millis(value);
    (*bson).append(key, dt)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_dbpointer(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    collection: *const c_char,
    oid: *const bson_oid_t,
) -> bool {
    (*bson).append(
        key,
        DbPointer {
            id: ObjectId::from_bytes((*oid).bytes),
            namespace: CStr::from_ptr(collection).to_string_lossy().to_string(),
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_decimal128(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: *const bson_decimal128_t,
) -> bool {
    let low = (*val).low.to_le_bytes();
    let high = (*val).high.to_le_bytes();

    let bytes = [
        low[0], low[1], low[2], low[3], low[4], low[5], low[6], low[7], high[0], high[1], high[2],
        high[3], high[4], high[5], high[6], high[7],
    ];
    (*bson).append(key, Decimal128::from_bytes(bytes))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_double(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    val: f64,
) -> bool {
    (*bson).append(key, val)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_maxkey(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
) -> bool {
    (*bson).append(key, RawBson::MaxKey)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_minkey(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
) -> bool {
    (*bson).append(key, RawBson::MinKey)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_now_utc(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
) -> bool {
    (*bson).append(key, DateTime::now())
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_null(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
) -> bool {
    (*bson).append(key, RawBson::Null)
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_oid(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    oid: *const bson_oid_t,
) -> bool {
    (*bson).append(key, ObjectId::from_bytes((*oid).bytes))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_regex(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    regex: *const c_char,
    options: *const c_char,
) -> bool {
    let regex = CStr::from_ptr(regex).to_string_lossy();
    let options = CStr::from_ptr(options).to_string_lossy();
    (*bson).append(
        key,
        RawBsonRef::RegularExpression(RawRegexRef {
            pattern: regex.as_ref(),
            options: options.as_ref(),
        }),
    )
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_symbol(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    value: *const c_char,
    _length: isize,
) -> bool {
    let s = CStr::from_ptr(value).to_string_lossy();
    (*bson).append(key, RawBsonRef::Symbol(s.as_ref()))
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_timestamp(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
    time: u32,
    increment: u32,
) -> bool {
    (*bson).append(key, Timestamp { time, increment })
}

#[no_mangle]
pub unsafe extern "C" fn bson_append_undefined(
    bson: *mut bson_t,
    key: *const c_char,
    _key_length: isize,
) -> bool {
    (*bson).append(key, RawBson::Undefined)
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

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum bson_subtype_t {
    BSON_SUBTYPE_BINARY = 0x00,
    BSON_SUBTYPE_FUNCTION = 0x01,
    BSON_SUBTYPE_BINARY_DEPRECATED = 0x02,
    BSON_SUBTYPE_UUID_DEPRECATED = 0x03,
    BSON_SUBTYPE_UUID = 0x04,
    BSON_SUBTYPE_MD5 = 0x05,
    BSON_SUBTYPE_COLUMN = 0x07,
    BSON_SUBTYPE_USER = 0x80,
}

#[repr(C)]
pub struct bson_oid_t {
    bytes: [u8; 12],
}

#[repr(C)]
pub struct bson_decimal128_t {
    low: u64,
    high: u64,
}
