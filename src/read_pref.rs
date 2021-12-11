use std::{ops::Deref, time::Duration};

use mongodb::options::ReadPreference;

use crate::bson::bson_t;

pub const MONGOC_SMALLEST_MAX_STALENESS_SECONDS: i64 = 90;
pub const MONGOC_NO_MAX_STALENESS: i64 = -1;

pub struct mongoc_read_prefs_t {
    rust_read_pref: ReadPreference,
}

impl Deref for mongoc_read_prefs_t {
    type Target = ReadPreference;

    fn deref(&self) -> &Self::Target {
        &self.rust_read_pref
    }
}

impl From<ReadPreference> for mongoc_read_prefs_t {
    fn from(rp: ReadPreference) -> Self {
        Self { rust_read_pref: rp }
    }
}

#[repr(C)]
pub enum mongoc_read_mode_t {
    MONGOC_READ_PRIMARY = (1 << 0),
    MONGOC_READ_SECONDARY = (1 << 1),
    MONGOC_READ_PRIMARY_PREFERRED = (1 << 2) | (1 << 0),
    MONGOC_READ_SECONDARY_PREFERRED = (1 << 2) | (1 << 1),
    MONGOC_READ_NEAREST = (1 << 3) | (1 << 1),
}

impl mongoc_read_prefs_t {
    fn new() -> Self {
        mongoc_read_prefs_t {
            rust_read_pref: ReadPreference::Primary,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_new(
    mode: mongoc_read_mode_t,
) -> *mut mongoc_read_prefs_t {
    let rp = match mode {
        mongoc_read_mode_t::MONGOC_READ_PRIMARY => ReadPreference::Primary,
        mongoc_read_mode_t::MONGOC_READ_PRIMARY_PREFERRED => ReadPreference::PrimaryPreferred {
            options: Default::default(),
        },
        mongoc_read_mode_t::MONGOC_READ_SECONDARY => ReadPreference::Secondary {
            options: Default::default(),
        },
        mongoc_read_mode_t::MONGOC_READ_SECONDARY_PREFERRED => ReadPreference::SecondaryPreferred {
            options: Default::default(),
        },
        mongoc_read_mode_t::MONGOC_READ_NEAREST => ReadPreference::Nearest {
            options: Default::default(),
        },
    };
    Box::into_raw(Box::new(rp.into()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_get_mode(
    rp: *const mongoc_read_prefs_t,
) -> mongoc_read_mode_t {
    match &(*rp).rust_read_pref {
        ReadPreference::Primary => mongoc_read_mode_t::MONGOC_READ_PRIMARY,
        ReadPreference::PrimaryPreferred { .. } => {
            mongoc_read_mode_t::MONGOC_READ_PRIMARY_PREFERRED
        }
        ReadPreference::Secondary { .. } => mongoc_read_mode_t::MONGOC_READ_SECONDARY,
        ReadPreference::SecondaryPreferred { .. } => {
            mongoc_read_mode_t::MONGOC_READ_SECONDARY_PREFERRED
        }
        ReadPreference::Nearest { .. } => mongoc_read_mode_t::MONGOC_READ_NEAREST,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_get_tags(
    rp: *const mongoc_read_prefs_t,
) -> *const bson_t<'static> {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_set_tags(
    rp: *const mongoc_read_prefs_t,
    tags: *const bson_t<'static>,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_get_max_staleness_seconds(
    rp: *const mongoc_read_prefs_t,
) -> i64 {
    (*rp)
        .max_staleness()
        .map(|d| d.as_secs() as i64)
        .unwrap_or(MONGOC_NO_MAX_STALENESS)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_set_max_staleness_seconds(
    rp: *mut mongoc_read_prefs_t,
    seconds: i64,
) -> bool {
    let opts = match (*rp).rust_read_pref {
        ReadPreference::Primary => return false,
        ReadPreference::Secondary { ref mut options } => options,
        ReadPreference::SecondaryPreferred { ref mut options } => options,
        ReadPreference::PrimaryPreferred { ref mut options } => options,
        ReadPreference::Nearest { ref mut options } => options,
    };

    opts.max_staleness = Duration::from_secs(seconds as u64).into();
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_read_prefs_destroy(rp: *mut mongoc_read_prefs_t) {
    drop(Box::from_raw(rp))
}
