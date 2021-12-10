use crate::bson::{bson_error_t, bson_t};

pub struct mongoc_change_stream_t;

#[no_mangle]
pub unsafe extern "C" fn mongoc_change_stream_next(
    _change_stream: *mut mongoc_change_stream_t,
    _bson: *mut *const bson_t,
) -> bool {
    todo!("change streams not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_change_stream_get_resume_token(
    _change_stream: *mut mongoc_change_stream_t,
) -> *const bson_t<'static> {
    todo!("change streams not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_change_stream_error_document(
    _change_stream: *mut mongoc_change_stream_t,
    _error: *const bson_error_t,
    _reply: *const *mut bson_t<'static>,
) -> bool {
    todo!("change streams not implemented")
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_change_stream_destroy(_change_stream: *mut mongoc_change_stream_t) {
    todo!("change streams not implemented")
}
