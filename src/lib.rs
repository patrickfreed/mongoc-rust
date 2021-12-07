pub mod bson;
mod client;
mod collection;
mod cursor;
mod database;

use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    pub struct mongoc_query_flags_t: u32 {
        const MONGOC_QUERY_NONE = 0;
        const MONGOC_QUERY_TAILABLE_CURSOR = 1 << 1;
        const MONGOC_QUERY_SECONDARY_OK = 1 << 2;
        const MONGOC_QUERY_OPLOG_REPLAY = 1 << 3;
        const MONGOC_QUERY_NO_CURSOR_TIMEOUT = 1 << 4;
        const MONGOC_QUERY_AWAIT_DATA = 1 << 5;
        const MONGOC_QUERY_EXHAUST = 1 << 6;
        const MONGOC_QUERY_PARTIAL = 1 << 7;
    }
}
