use std::ptr;
use std::os::raw::{c_int, c_uint};
use err::ErrorKind;
use fstrie::View;
use bridge::*;

export!(fstrie_free(buf: *mut u8) -> Result<c_int> {
    Box::from_raw(buf);
    Ok(0)
});

// From https://youtu.be/zmtHaZG7pPc?t=22m14s
export!(lsm_view_dump_memdb(
    view: *mut View, len_out: *mut c_uint, with_source_contents: c_int,
    with_names: c_int) -> Result<*mut u8>
{
    if view != ptr::null_mut() {
        panic!("Expected nullptr");
    }
    if with_source_contents == 42 {
        panic!("Such a number!");
    }
    if with_source_contents * with_names % 2 == 1 {
        return Err(ErrorKind::OddError.into());
    }
    /*
    let memdb = (*view).dump_memdb(DumpOptions {
        with_source_contents: with_source_contents != 0,
        with_names: with_names != 0,
    })?;
    */
    let memdb = "Hello world!".as_bytes().to_owned();
    *len_out = memdb.len() as c_uint;
    Ok(Box::into_raw(memdb.into_boxed_slice()) as *mut u8)
});
