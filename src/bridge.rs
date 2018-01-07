use std::{panic, mem, ptr};
use std::os::raw::{c_int, c_uint};
use err::{Error, ErrorKind, Result};

// From https://youtu.be/zmtHaZG7pPc?t=22m09s
macro_rules! export (
    ($n:ident($($an:ident: $aty:ty),*) -> Result<$rv:ty> $body:block) => (
        #[no_mangle]
        pub unsafe extern "C" fn $n($($an: $aty,)* err: *mut CError) -> $rv
        {
            landingpad(|| $body, err)
        }
    );
);

pub struct View {}

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

export!(lsm_buffer_free(buf: *mut u8) -> Result<c_int> {
    Box::from_raw(buf);
    Ok(0)
});

#[repr(C)]
pub struct CError {
    message: *mut u8,
    failed: c_uint,
    code: c_uint,
}

static mut PANIC_INFO: Option<String> = None;

// From https://youtu.be/zmtHaZG7pPc?t=21m39s
unsafe fn set_err(err: Error, err_out: *mut CError) {
    if err_out.is_null() {
        return;
    }
    let s = match err.kind {
        ErrorKind::InternalError => get_panic(),
        _ => format!("{}\x00", err),
    };
    (*err_out).message = Box::into_raw(s.into_boxed_str()) as *mut u8;
    (*err_out).code = err.get_error_code();
    (*err_out).failed = 1;
}
// End from

// From https://youtu.be/zmtHaZG7pPc?t=21m29s
fn silent_panic_handler(pi: &panic::PanicInfo) {
    let pl = pi.payload();
    let payload = if let Some(s) = pl.downcast_ref::<&str>() { s }
    else { "?" };
    let position = if let Some(p) = pi.location() {
        format!("At {}:{}: ", p.file(), p.line())
    }
    else { "".to_owned() };
    unsafe {
        PANIC_INFO = Some(format!("{}{}", position, payload));
    }
}

#[no_mangle]
pub unsafe extern "C" fn mylib_init() {
    panic::set_hook(Box::new(silent_panic_handler));
}
// End from

// From https://youtu.be/zmtHaZG7pPc?t=21m54s
pub unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(
    f: F, err_out: *mut CError) -> T
{
    if let Ok(rv) = panic::catch_unwind(f) {
        rv.map_err(|err| set_err(err, err_out)).unwrap_or(mem::zeroed())
    } else {
        set_err(ErrorKind::InternalError.into(), err_out);
        mem::zeroed()
    }
}

unsafe fn get_panic() -> String {
    match &PANIC_INFO {
        &Some(ref s) => format!("{}\x00", s),
        &None => "no panic info\x00".to_owned(),
    }
}
