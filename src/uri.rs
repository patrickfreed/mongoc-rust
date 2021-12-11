use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    str::FromStr,
};

use mongodb::options::{AuthMechanism, ClientOptions, Credential};

use crate::{
    bson::{bson_error_t, bson_t},
    read_concern::mongoc_read_concern_t,
    read_pref::mongoc_read_prefs_t,
    write_concern::mongoc_write_concern_t,
};

#[derive(Clone)]
pub struct mongoc_uri_t {
    uri: String,
    options: ClientOptions,
}

impl mongoc_uri_t {
    fn new(s: String) -> Self {
        let options = ClientOptions::parse(s.as_str()).unwrap();
        Self { uri: s, options }
    }

    pub fn as_str(&self) -> &str {
        self.uri.as_str()
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_new(uri: *const c_char) -> *mut mongoc_uri_t {
    let s = CStr::from_ptr(uri).to_string_lossy().into_owned();
    Box::into_raw(Box::new(mongoc_uri_t::new(s)))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_copy(uri: *const mongoc_uri_t) -> *mut mongoc_uri_t {
    let copy = (*uri).clone();
    Box::into_raw(Box::new(copy))
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_string(uri: *const mongoc_uri_t) -> *const c_char {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_new_with_error(
    uri: *const c_char,
    _error: *mut bson_error_t,
) -> *mut mongoc_uri_t {
    // TODO: handle errors
    mongoc_uri_new(uri)
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_service(uri: *mut mongoc_uri_t) -> *const c_char {
    if (*uri).as_str().contains("mongodb+srv") {
        CStr::from_bytes_with_nul(b"mongodb+srv://\0")
            .unwrap()
            .as_ptr() as *const c_char
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_read_concern(
    uri: *mut mongoc_uri_t,
) -> *const mongoc_read_concern_t {
    todo!("get read concern")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_read_concern(
    uri: *mut mongoc_uri_t,
    rc: *const mongoc_read_concern_t,
) -> bool {
    todo!("set read concern")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_write_concern(
    uri: *mut mongoc_uri_t,
) -> *const mongoc_write_concern_t {
    todo!("get read concern")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_write_concern(
    uri: *mut mongoc_uri_t,
    rc: *const mongoc_write_concern_t,
) -> bool {
    todo!("set write concern")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_read_prefs_t(
    uri: *mut mongoc_uri_t,
) -> *const mongoc_read_prefs_t {
    todo!("get read prefs")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_read_prefs_t(
    uri: *mut mongoc_uri_t,
    rc: *const mongoc_read_prefs_t,
) -> bool {
    todo!("set read prefs")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_option_as_bool(
    uri: *mut mongoc_uri_t,
    option: *const c_char,
    val: bool,
) -> bool {
    todo!("TODO: uri stuff")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_option_as_utf8(
    uri: *mut mongoc_uri_t,
    option: *const c_char,
    val: *const c_char,
) -> bool {
    todo!("TODO: uri stuff")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_option_as_utf8(
    uri: *mut mongoc_uri_t,
    option: *const c_char,
    fallback: *const c_char,
) -> *const c_char {
    todo!("get utf8")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_option_as_int32(
    uri: *mut mongoc_uri_t,
    option: *const c_char,
    val: i32,
) -> bool {
    todo!("TODO: uri stuff")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_has_option(
    uri: *mut mongoc_uri_t,
    option: *const c_char,
) -> bool {
    todo!("TODO: uri stuff")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_username(
    uri: *mut mongoc_uri_t,
    username: *const c_char,
) -> bool {
    let credential = (*uri)
        .options
        .credential
        .get_or_insert_with(Credential::default);
    let s = CStr::from_ptr(username).to_string_lossy();
    credential.username = Some(s.into_owned());
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_username(uri: *mut mongoc_uri_t) -> *const c_char {
    panic!("cant implement")
    // if let Some(username) = (*uri).options.credential.and_then(|c| c.username.and_then(String::as_str)) {
    //     CStr::from_
    // }
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_password(
    uri: *mut mongoc_uri_t,
    password: *const c_char,
) -> bool {
    let credential = (*uri)
        .options
        .credential
        .get_or_insert_with(Credential::default);
    let s = CStr::from_ptr(password).to_string_lossy();
    credential.password = Some(s.into_owned());
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_password(uri: *mut mongoc_uri_t) -> *const c_char {
    panic!("cant implement")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_auth_source(
    uri: *mut mongoc_uri_t,
    source: *const c_char,
) -> bool {
    let credential = (*uri)
        .options
        .credential
        .get_or_insert_with(Credential::default);
    let s = CStr::from_ptr(source).to_string_lossy();
    credential.source = Some(s.into_owned());
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_auth_source(uri: *mut mongoc_uri_t) -> *const c_char {
    panic!("cant implement")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_auth_mechanism(
    uri: *mut mongoc_uri_t,
    mechanism: *const c_char,
) -> bool {
    let credential = (*uri)
        .options
        .credential
        .get_or_insert_with(Credential::default);
    let s = CStr::from_ptr(mechanism).to_string_lossy();
    credential.mechanism = Some(AuthMechanism::from_str(&s).unwrap());
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_auth_mechanism(uri: *mut mongoc_uri_t) -> *const c_char {
    panic!("cant implement")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_mechanism_properties(
    uri: *mut mongoc_uri_t,
    mechanism_properties: *const bson_t<'static>,
) -> bool {
    let credential = (*uri)
        .options
        .credential
        .get_or_insert_with(Credential::default);
    let document = (*mechanism_properties).to_document().unwrap();
    credential.mechanism_properties = Some(document);
    true
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_database(uri: *mut mongoc_uri_t) -> *const c_char {
    panic!("cant implement")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_options(uri: *mut mongoc_uri_t) -> *const bson_t<'static> {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_set_compressors(
    uri: *mut mongoc_uri_t,
    compressor: *const c_char,
) -> bool {
    todo!("set compressors")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_compressors(
    uri: *mut mongoc_uri_t,
) -> *const bson_t<'static> {
    todo!("get compressors")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_get_replica_set(uri: *mut mongoc_uri_t) -> *const c_char {
    todo!("get replica set")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_uri_destroy(uri: *mut mongoc_uri_t) {
    drop(Box::from_raw(uri))
}

pub const MONGOC_URI_APPNAME: &'static str = "appname";
pub const MONGOC_URI_AUTHMECHANISM: &'static str = "authmechanism";
pub const MONGOC_URI_AUTHMECHANISMPROPERTIES: &'static str = "authmechanismproperties";
pub const MONGOC_URI_AUTHSOURCE: &'static str = "authsource";
pub const MONGOC_URI_CANONICALIZEHOSTNAME: &'static str = "canonicalizehostname";
pub const MONGOC_URI_CONNECTTIMEOUTMS: &'static str = "connecttimeoutms";
pub const MONGOC_URI_COMPRESSORS: &'static str = "compressors";
pub const MONGOC_URI_DIRECTCONNECTION: &'static str = "directconnection";
pub const MONGOC_URI_GSSAPISERVICENAME: &'static str = "gssapiservicename";
pub const MONGOC_URI_HEARTBEATFREQUENCYMS: &'static str = "heartbeatfrequencyms";
pub const MONGOC_URI_JOURNAL: &'static str = "journal";
pub const MONGOC_URI_LOADBALANCED: &'static str = "loadbalanced";
pub const MONGOC_URI_LOCALTHRESHOLDMS: &'static str = "localthresholdms";
pub const MONGOC_URI_MAXIDLETIMEMS: &'static str = "maxidletimems";
pub const MONGOC_URI_MAXPOOLSIZE: &'static str = "maxpoolsize";
pub const MONGOC_URI_MAXSTALENESSSECONDS: &'static str = "maxstalenessseconds";
pub const MONGOC_URI_MINPOOLSIZE: &'static str = "minpoolsize";
pub const MONGOC_URI_READCONCERNLEVEL: &'static str = "readconcernlevel";
pub const MONGOC_URI_READPREFERENCE: &'static str = "readpreference";
pub const MONGOC_URI_READPREFERENCETAGS: &'static str = "readpreferencetags";
pub const MONGOC_URI_REPLICASET: &'static str = "replicaset";
pub const MONGOC_URI_RETRYREADS: &'static str = "retryreads";
pub const MONGOC_URI_RETRYWRITES: &'static str = "retrywrites";
pub const MONGOC_URI_SAFE: &'static str = "safe";
pub const MONGOC_URI_SERVERSELECTIONTIMEOUTMS: &'static str = "serverselectiontimeoutms";
pub const MONGOC_URI_SERVERSELECTIONTRYONCE: &'static str = "serverselectiontryonce";
pub const MONGOC_URI_SLAVEOK: &'static str = "slaveok";
pub const MONGOC_URI_SOCKETCHECKINTERVALMS: &'static str = "socketcheckintervalms";
pub const MONGOC_URI_SOCKETTIMEOUTMS: &'static str = "sockettimeoutms";
pub const MONGOC_URI_TLS: &'static str = "tls";
pub const MONGOC_URI_TLSCERTIFICATEKEYFILE: &'static str = "tlscertificatekeyfile";
pub const MONGOC_URI_TLSCERTIFICATEKEYFILEPASSWORD: &'static str = "tlscertificatekeyfilepassword";
pub const MONGOC_URI_TLSCAFILE: &'static str = "tlscafile";
pub const MONGOC_URI_TLSALLOWINVALIDCERTIFICATES: &'static str = "tlsallowinvalidcertificates";
pub const MONGOC_URI_TLSALLOWINVALIDHOSTNAMES: &'static str = "tlsallowinvalidhostnames";
pub const MONGOC_URI_TLSINSECURE: &'static str = "tlsinsecure";
pub const MONGOC_URI_TLSDISABLECERTIFICATEREVOCATIONCHECK: &'static str =
    "tlsdisablecertificaterevocationcheck";
pub const MONGOC_URI_TLSDISABLEOCSPENDPOINTCHECK: &'static str = "tlsdisableocspendpointcheck";
pub const MONGOC_URI_W: &'static str = "w";
pub const MONGOC_URI_WAITQUEUEMULTIPLE: &'static str = "waitqueuemultiple";
pub const MONGOC_URI_WAITQUEUETIMEOUTMS: &'static str = "waitqueuetimeoutms";
pub const MONGOC_URI_WTIMEOUTMS: &'static str = "wtimeoutms";
pub const MONGOC_URI_ZLIBCOMPRESSIONLEVEL: &'static str = "zlibcompressionlevel";
