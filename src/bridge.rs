use std::{fmt, mem, panic, result};
use std::os::raw::{c_uint, c_int};

// From https://youtu.be/zmtHaZG7pPc?t=21m29s
fn silent_panic_handler(_pi: &panic::PanicInfo) {
    /* noop */
}

#[no_mangle]
pub unsafe extern "C" fn mylib_init() {
    panic::set_hook(Box::new(silent_panic_handler));
}
// End from

#[repr(C)]
pub struct CError {
    message: *mut u8,
    failed: c_uint,
    code: c_uint,
}

#[derive(Debug)]
enum ErrorKind {
    InternalError,
}

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
}

type Result<T> = result::Result<T, Error>;

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error { kind: self }
    }
}

impl Error {
    fn get_error_code(&self) -> c_uint {
        match self.kind {
            ErrorKind::InternalError => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Defer to Debug
        write!(f, "{:?}", self)
    }
}

// From https://youtu.be/zmtHaZG7pPc?t=21m39s
unsafe fn set_err(err: Error, err_out: *mut CError) {
    if err_out.is_null() {
        return;
    }
    let s = format!("{}\x00", err);
    (*err_out).message = Box::into_raw(s.into_boxed_str()) as *mut u8;
    (*err_out).code = err.get_error_code();
    (*err_out).failed = 1;
}
// End from

// From https://youtu.be/zmtHaZG7pPc?t=21m54s
unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(
    f: F, err_out: *mut CError) -> T
{
    if let Ok(rv) = panic::catch_unwind(f) {
        rv.map_err(|err| set_err(err, err_out)).unwrap_or(mem::zeroed())
    } else {
        set_err(ErrorKind::InternalError.into(), err_out);
        mem::zeroed()
    }
}

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
