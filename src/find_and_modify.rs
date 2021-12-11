use crate::bson::bson_t;

pub struct mongoc_find_and_modify_opts_t {}

#[repr(C)]
pub enum mongoc_find_and_modify_flags_t {
    MONGOC_FIND_AND_MODIFY_NONE = 0,
    MONGOC_FIND_AND_MODIFY_REMOVE = 1 << 0,
    MONGOC_FIND_AND_MODIFY_UPSERT = 1 << 1,
    MONGOC_FIND_AND_MODIFY_RETURN_NEW = 1 << 2,
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_new() -> *mut mongoc_find_and_modify_opts_t {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_set_bypass_document_validation(
    opts: *mut mongoc_find_and_modify_opts_t,
    bypass: bool,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_set_fields(
    opts: *mut mongoc_find_and_modify_opts_t,
    fields: *const bson_t<'static>,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_set_flags(
    opts: *mut mongoc_find_and_modify_opts_t,
    flags: mongoc_find_and_modify_flags_t,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_set_sort(
    opts: *mut mongoc_find_and_modify_opts_t,
    sort: *const bson_t<'static>,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_set_update(
    opts: *mut mongoc_find_and_modify_opts_t,
    update: *const bson_t<'static>,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_append(
    opts: *mut mongoc_find_and_modify_opts_t,
    extra: *const bson_t<'static>,
) -> bool {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mongoc_find_and_modify_opts_destroy(
    opts: *mut mongoc_find_and_modify_opts_t,
) {
    todo!()
}
