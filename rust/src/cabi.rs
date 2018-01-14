use std::{ptr, mem};
use std::os::raw::{c_int, c_char};
use std::ffi::{CStr, CString};
use fstrie::Database;
use bridge::*;
use err::Result;

#[no_mangle]
pub unsafe extern "C" fn fstrie_init() {
    set_panic_hook();
}

export!(fstrie_free(buf: *mut c_char) -> Result<c_int> {
    CString::from_raw(buf);
    Ok(0)
});

export!(fstrie_free_list(buf: *mut *mut c_char) -> Result<c_int> {
    let mut i = 0;
    while *buf.offset(i) != ptr::null_mut() {
        CString::from_raw(*buf.offset(i));
        i += 1;
    }
    let i = i as usize;
    Vec::from_raw_parts(buf, i+1, i+1);
    Ok(0)
});

export!(fstrie_load(root: *const c_char) -> Result<*mut Database> {
    Ok(Box::into_raw(Box::new(Database::new(CStr::from_ptr(root).to_str()?)?)))
});

export!(fstrie_unload(db: *mut Database) -> Result<c_int> {
    Box::from_raw(db);
    Ok(0)
});

export!(fstrie_lookup(db: *mut Database, key: *const c_char) -> Result<*mut *mut c_char> {
    let r_vec = (*db).lookup(CStr::from_ptr(key).to_str()?)?;
    let mut r_ptrs = Vec::new();
    for r in r_vec {
        r_ptrs.push(CString::new(r)?.into_raw());
    }
    r_ptrs.push(ptr::null_mut());
    r_ptrs.shrink_to_fit();
    let r_ptrs_raw = r_ptrs.as_mut_ptr();
    mem::forget(r_ptrs);
    Ok(r_ptrs_raw)
});
