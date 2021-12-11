use std::{
    ffi::CStr,
    ops::{Deref, DerefMut},
    os::raw::c_char,
};

use mongodb::options::{ReadConcern, ReadConcernLevel};

pub struct mongoc_read_concern_t {
    rust_read_concern: Option<ReadConcern>,
}

impl From<ReadConcern> for mongoc_read_concern_t {
    fn from(rc: ReadConcern) -> Self {
        Self {
            rust_read_concern: rc.into(),
        }
    }
}

// impl Deref for mongoc_read_concern_t {
//     type Target = ReadConcern;

//     fn deref(&self) -> &Self::Target {
//         &self.rust_read_concern
//     }
// }

// impl DerefMut for mongoc_read_concern_t {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.rust_read_concern
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_concern_new() -> *mut mongoc_read_concern_t {
    Box::into_raw(Box::new(mongoc_read_concern_t {
        rust_read_concern: None,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_concern_get_level(
    rc: *const mongoc_read_concern_t,
) -> *const c_char {
    let level_bytes = match (*rc).rust_read_concern {
        Some(ref rc) => Some(match rc.level {
            ReadConcernLevel::Local => "local\0".as_bytes(),
            ReadConcernLevel::Available => "available\0".as_bytes(),
            ReadConcernLevel::Linearizable => "linearizeable\0".as_bytes(),
            ReadConcernLevel::Majority => "majority\0".as_bytes(),
            ReadConcernLevel::Snapshot => "snapshot\0".as_bytes(),
            ref c => panic!("cant return custom rc level {:?}", c),
        }),
        None => None,
    };

    match level_bytes {
        Some(b) => CStr::from_bytes_with_nul(b).unwrap().as_ptr() as *const c_char,
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_concern_set_level(
    rc: *mut mongoc_read_concern_t,
    level: *const c_char,
) -> bool {
    let level_str = CStr::from_ptr(level).to_string_lossy();
    let level = match level_str.as_ref() {
        "available" => ReadConcernLevel::Available,
        "local" => ReadConcernLevel::Local,
        "linearizeable" => ReadConcernLevel::Linearizable,
        "majority" => ReadConcernLevel::Majority,
        "snapshot" => ReadConcernLevel::Snapshot,
        other => ReadConcernLevel::Custom(other.to_string()),
    };

    (*rc).rust_read_concern = Some(level.into());
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_concern_destroy(rc: *mut mongoc_read_concern_t) {
    drop(Box::from_raw(rc))
}
