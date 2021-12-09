use std::ops::{Deref, DerefMut};

use mongodb::{
    bson::{RawDocumentBuf, Timestamp},
    sync::ClientSession,
    ClusterTime,
};

use crate::bson::{bson_error_t, bson_t};

#[allow(non_camel_case_types)]
pub struct mongoc_client_session_t {
    rust_session: ClientSession,
    lsid: bson_t<'static>,
    cluster_time: bson_t<'static>,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum mongoc_transaction_state_t {
    MONGOC_TRANSACTION_NONE = 0x00,
    MONGOC_TRANSACTION_STARTING = 0x01,
    MONGOC_TRANSACTION_IN_PROGRESS = 0x02,
    MONGOC_TRANSACTION_COMMITTED = 0x03,
    MONGOC_TRANSACTION_ABORTED = 0x04,
}

impl From<ClientSession> for mongoc_client_session_t {
    fn from(s: ClientSession) -> Self {
        let id = RawDocumentBuf::from_document(s.id()).unwrap();
        Self {
            rust_session: s,
            lsid: id.into(),
            cluster_time: RawDocumentBuf::new().into(),
        }
    }
}

impl Deref for mongoc_client_session_t {
    type Target = ClientSession;

    fn deref(&self) -> &Self::Target {
        &self.rust_session
    }
}

impl DerefMut for mongoc_client_session_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_session
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_lsid(
    session: *const mongoc_client_session_t,
) -> *const bson_t<'static> {
    &(*session).lsid
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_append(
    _session: *const mongoc_client_session_t,
    _opts: *mut bson_t,
    _error: *mut bson_error_t,
) -> bool {
    todo!("lookup session by id")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_operation_time(
    session: *mut mongoc_client_session_t,
    timestamp: *mut u32,
    increment: *mut u32,
) {
    if let Some(ts) = (*session).rust_session.operation_time() {
        *timestamp = ts.time;
        *increment = ts.increment;
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_advance_operation_time(
    session: *mut mongoc_client_session_t,
    time: u32,
    increment: u32,
) {
    (*session).advance_operation_time(Timestamp { time, increment })
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_cluster_time(
    session: *mut mongoc_client_session_t,
) -> *const bson_t<'static> {
    match (*session).cluster_time() {
        Some(ct) => {
            let doc = mongodb::bson::to_raw_document_buf(ct).unwrap();
            (*session).cluster_time = doc.into();
            &(*session).cluster_time
        }
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_advance_cluster_time(
    session: *mut mongoc_client_session_t,
    cluster_time: *const bson_t<'static>,
) {
    let ct: ClusterTime = mongodb::bson::from_slice((*cluster_time).as_bytes()).unwrap();
    (*session).advance_cluster_time(&ct)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_start_transaction(
    session: *mut mongoc_client_session_t,
    _opts: *mut u8,
    _error: *mut bson_error_t,
) -> bool {
    (*session).rust_session.start_transaction(None).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_abort_transaction(
    session: *mut mongoc_client_session_t,
    _error: *mut bson_error_t,
) -> bool {
    (*session).rust_session.abort_transaction().is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_commit_transaction(
    session: *mut mongoc_client_session_t,
    _reply: *mut bson_t<'static>,
    _error: *mut bson_error_t,
) -> bool {
    (*session).rust_session.commit_transaction().is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_server_id(
    _session: *mut mongoc_client_session_t,
) -> u32 {
    panic!("cant get server id from rust session")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_transaction_state(
    _session: *mut mongoc_client_session_t,
) -> mongoc_transaction_state_t {
    panic!("rust doesn't expose transaction state")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_get_dirty(
    _session: *mut mongoc_client_session_t,
) -> bool {
    panic!("rust doesn't expose dirty state")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_session_destroy(session: *mut mongoc_client_session_t) {
    drop(Box::from_raw(session))
}
