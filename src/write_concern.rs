use std::{
    ffi::CStr,
    ops::{Deref, DerefMut},
    os::raw::c_char,
    time::Duration,
};

use mongodb::options::{Acknowledgment, WriteConcern};

pub const MONGOC_WRITE_CONCERN_W_DEFAULT: i32 = -2;
pub const MONGOC_WRITE_CONCERN_W_MAJORITY: i32 = -3;
pub const MONGOC_WRITE_CONCERN_W_TAG: i32 = -4;

pub struct mongoc_write_concern_t {
    rust_write_concern: WriteConcern,
}

impl Deref for mongoc_write_concern_t {
    type Target = WriteConcern;

    fn deref(&self) -> &Self::Target {
        &self.rust_write_concern
    }
}

impl DerefMut for mongoc_write_concern_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_write_concern
    }
}

impl From<WriteConcern> for mongoc_write_concern_t {
    fn from(wc: WriteConcern) -> Self {
        Self {
            rust_write_concern: wc,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_new() -> *mut mongoc_write_concern_t {
    Box::into_raw(Box::new(mongoc_write_concern_t {
        rust_write_concern: Default::default(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_journal_is_set(
    wc: *const mongoc_write_concern_t,
) -> bool {
    (*wc).journal.is_some()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_get_journal(
    wc: *const mongoc_write_concern_t,
) -> bool {
    (*wc).journal.unwrap_or(true)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_set_journal(
    wc: *mut mongoc_write_concern_t,
    journal: bool,
) -> bool {
    (*wc).journal = Some(journal);
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_get_w(wc: *const mongoc_write_concern_t) -> i32 {
    match &(*wc).w {
        Some(Acknowledgment::Majority) => MONGOC_WRITE_CONCERN_W_MAJORITY,
        Some(Acknowledgment::Nodes(n)) => *n as i32,
        Some(Acknowledgment::Custom(_)) => MONGOC_WRITE_CONCERN_W_TAG,
        None => MONGOC_WRITE_CONCERN_W_DEFAULT,
        _ => unreachable!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_set_w(
    wc: *mut mongoc_write_concern_t,
    w: i32,
) -> bool {
    if w >= 0 {
        (*wc).w = Acknowledgment::Nodes(w as u32).into();
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_get_wtag(
    wc: *const mongoc_write_concern_t,
) -> *const c_char {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_set_wtag(
    wc: *mut mongoc_write_concern_t,
    tag: *const c_char,
) -> bool {
    let tag = CStr::from_ptr(tag).to_string_lossy();
    (*wc).w = Acknowledgment::Custom(tag.to_string()).into();
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_get_wtimeout_int64(
    wc: *const mongoc_write_concern_t,
) -> i64 {
    (*wc).w_timeout.map(|d| d.as_millis() as i64).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_set_wtimeout_int64(
    wc: *mut mongoc_write_concern_t,
    wtimeout: i64,
) -> bool {
    (*wc).w_timeout = Some(Duration::from_secs(wtimeout as u64));
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_write_concern_destroy(wc: *mut mongoc_write_concern_t) {
    drop(Box::from_raw(wc))
}
