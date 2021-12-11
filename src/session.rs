use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use mongodb::{
    bson::{RawDocumentBuf, Timestamp},
    options::{SessionOptions, TransactionOptions},
    sync::ClientSession,
    ClusterTime,
};

use crate::{
    bson::{bson_error_t, bson_t},
    read_concern::mongoc_read_concern_t,
    read_pref::mongoc_read_prefs_t,
    write_concern::mongoc_write_concern_t,
};

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
    opts: *const mongoc_transaction_opt_t,
    _error: *mut bson_error_t,
) -> bool {
    (*session)
        .rust_session
        .start_transaction((*opts).clone())
        .is_ok()
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

pub struct mongoc_session_opt_t {
    rust_opts: SessionOptions,
}

impl mongoc_session_opt_t {
    fn new() -> Self {
        Self {
            rust_opts: SessionOptions::builder().build(),
        }
    }
}

impl Deref for mongoc_session_opt_t {
    type Target = SessionOptions;

    fn deref(&self) -> &Self::Target {
        &self.rust_opts
    }
}

impl DerefMut for mongoc_session_opt_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_opts
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_session_opts_new() -> *mut mongoc_session_opt_t {
    Box::into_raw(Box::new(mongoc_session_opt_t::new()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_session_opts_set_causal_consistency(
    opts: *mut mongoc_session_opt_t,
    cc: bool,
) {
    (*opts).causal_consistency = Some(cc);
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_session_opts_set_snapshot(
    opts: *mut mongoc_session_opt_t,
    snapshot: bool,
) {
    (*opts).snapshot = Some(snapshot);
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_session_opts_set_default_transaction_opts(
    opts: *mut mongoc_session_opt_t,
    txn_opts: *const mongoc_transaction_opt_t,
) {
    (*opts).default_transaction_options = Some((*txn_opts).clone());
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_session_opts_destroy(opts: *mut mongoc_session_opt_t) {
    drop(Box::from_raw(opts))
}

pub struct mongoc_transaction_opt_t {
    rust_opts: TransactionOptions,
}

impl mongoc_transaction_opt_t {
    fn new() -> Self {
        Self {
            rust_opts: TransactionOptions::builder().build(),
        }
    }
}

impl Deref for mongoc_transaction_opt_t {
    type Target = TransactionOptions;

    fn deref(&self) -> &Self::Target {
        &self.rust_opts
    }
}

impl DerefMut for mongoc_transaction_opt_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_opts
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_new() -> *mut mongoc_transaction_opt_t {
    Box::into_raw(Box::new(mongoc_transaction_opt_t::new()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_set_max_commit_time_ms(
    opts: *mut mongoc_transaction_opt_t,
    commit_time: i64,
) {
    (*opts).max_commit_time = Some(Duration::from_millis(commit_time.try_into().unwrap()));
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_set_read_concern(
    _opts: *mut mongoc_transaction_opt_t,
    _rc: *const mongoc_read_concern_t,
) {
    todo!("txn opts read concern not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_set_write_concern(
    _opts: *mut mongoc_transaction_opt_t,
    _rc: *const mongoc_write_concern_t,
) {
    todo!("txn opts write concern not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_set_read_prefs(
    _opts: *mut mongoc_transaction_opt_t,
    _rc: *const mongoc_read_prefs_t,
) {
    todo!("txn opts read prefs not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_transaction_opts_destroy(opts: *mut mongoc_transaction_opt_t) {
    drop(Box::from_raw(opts))
}
