use std::ptr;
use std::os::raw::c_int;
use std::ffi::CStr;
use fstrie::Database;
use bridge::*;

export!(fstrie_free(buf: *mut u8) -> Result<c_int> {
    Box::from_raw(buf);
    Ok(0)
});

export!(fstrie_free_list(buf: *mut *mut u8) -> Result<c_int> {
    let mut i = 0;
    while *buf.offset(i) != ptr::null_mut() {
        Box::from_raw(*buf.offset(i));
        i += 1;
    }
    Box::from_raw(buf);
    Ok(0)
});

export!(fstrie_load(root: *const i8) -> Result<*mut Database> {
    Ok(Box::into_raw(Box::new(Database::new(CStr::from_ptr(root).to_str()?)?)) as *mut Database)
});

export!(fstrie_unload(db: *mut Database) -> Result<c_int> {
    Box::from_raw(db);
    Ok(0)
});

export!(fstrie_lookup(db: *mut Database, key: *const i8) -> Result<*mut *mut i8> {
    let r_vec = (*db).lookup(CStr::from_ptr(key).to_str()?)?;
    let mut r_ptrs = Vec::new();
    for mut r in r_vec {
        r.push(b'\x00');
        r_ptrs.push(
            Box::into_raw(r.into_boxed_slice()) as *mut i8);
    }
    r_ptrs.push(ptr::null_mut());
    Ok(Box::into_raw(r_ptrs.into_boxed_slice()) as *mut *mut i8)
});
