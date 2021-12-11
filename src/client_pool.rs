use std::ops::Deref;

use mongodb::sync::Client;

use crate::{
    api::mongoc_server_api_t,
    bson::bson_error_t,
    client::{mongoc_client_destroy, mongoc_client_t},
    uri::mongoc_uri_t,
};

pub struct mongoc_client_pool_t {
    rust_client: Client,
}

impl From<Client> for mongoc_client_pool_t {
    fn from(client: Client) -> Self {
        Self {
            rust_client: client,
        }
    }
}

impl Deref for mongoc_client_pool_t {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.rust_client
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_new(
    uri: *const mongoc_uri_t,
) -> *mut mongoc_client_pool_t {
    let client = Client::with_uri_str((*uri).as_str()).unwrap();
    Box::into_raw(Box::new(client.into()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_set_error_api(
    _pool: *mut mongoc_client_pool_t,
    _api: i32,
) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_set_min_heartbeat_frequency_msec(
    _pool: *mut mongoc_client_pool_t,
    _ms: u64,
) -> bool {
    // TODO: implement
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_set_server_api(
    _pool: *mut mongoc_client_pool_t,
    _api: *const mongoc_server_api_t,
    _error: *mut bson_error_t,
) -> bool {
    // TODO: implement
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_pop(
    pool: *mut mongoc_client_pool_t,
) -> *mut mongoc_client_t {
    Box::into_raw(Box::new((*pool).clone().into()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_try_pop(
    pool: *mut mongoc_client_pool_t,
) -> *mut mongoc_client_t {
    mongoc_client_pool_pop(pool)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_push(
    _pool: *mut mongoc_client_pool_t,
    client: *mut mongoc_client_t,
) {
    mongoc_client_destroy(client)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_client_pool_destroy(pool: *mut mongoc_client_pool_t) {
    drop(Box::from_raw(pool))
}
