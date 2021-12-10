use std::ops::{Deref, DerefMut};

use mongodb::options::{ServerApi, ServerApiVersion};

pub struct mongoc_server_api_t {
    rust_server_api: ServerApi,
}

impl From<ServerApi> for mongoc_server_api_t {
    fn from(s: ServerApi) -> Self {
        Self { rust_server_api: s }
    }
}

impl Deref for mongoc_server_api_t {
    type Target = ServerApi;

    fn deref(&self) -> &Self::Target {
        &self.rust_server_api
    }
}

impl DerefMut for mongoc_server_api_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rust_server_api
    }
}

#[repr(C)]
pub enum mongoc_server_api_version_t {
    MONGOC_SERVER_API_V1,
}

impl From<mongoc_server_api_version_t> for ServerApiVersion {
    fn from(mv: mongoc_server_api_version_t) -> Self {
        match mv {
            mongoc_server_api_version_t::MONGOC_SERVER_API_V1 => ServerApiVersion::V1,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_server_api_new(
    version: mongoc_server_api_version_t,
) -> *mut mongoc_server_api_t {
    let sa = ServerApi::builder().version(version).build();
    Box::into_raw(Box::new(sa.into()))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_server_api_deprecation_errors(
    api: *mut mongoc_server_api_t,
    de: bool,
) {
    (*api).deprecation_errors = Some(de)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_server_api_strict(api: *mut mongoc_server_api_t, strict: bool) {
    (*api).strict = Some(strict)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_server_api_destroy(api: *mut mongoc_server_api_t) {
    drop(Box::from_raw(api))
}
