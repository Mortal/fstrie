import os
from cffi import FFI


_ffi = FFI()
_ffi.cdef("""
/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
typedef void lsm_view_t;
typedef struct lsm_error_s {
    char *message;
    int failed;
    int code;
} lsm_error_t;

void fstrie_init();

char *lsm_view_dump_memdb(const lsm_view_t *view,
                          unsigned int *len_out,
                          int with_source_contents,
                          int with_names,
                          lsm_error_t *err);

void fstrie_free(char *);
""")
_lib = _ffi.dlopen(os.path.join(
    os.path.dirname(__file__), '../target/debug/libfstrie.so'))
_lib.fstrie_init()


class FstrieError(Exception):
    pass


class InternalError(FstrieError):
    pass


class OddError(FstrieError):
    pass


special_errors = {
    1: InternalError,
    2: OddError,
}


# From https://youtu.be/zmtHaZG7pPc?t=22m29s
def rustcall(func, *args):
    err = _ffi.new('lsm_error_t *')
    rv = func(*(args + (err,)))
    if not err[0].failed:
        return rv
    try:
        exc_class = special_errors.get(err[0].code, FstrieError)
        exc = exc_class(_ffi.string(err[0].message).decode('utf-8', 'replace'))
    finally:
        _lib.fstrie_free(err[0].message)
    raise exc


def lsm_view_dump_memdb(a, b):
    len_out = _ffi.new('unsigned int *')
    res_ptr = rustcall(_lib.lsm_view_dump_memdb, _ffi.NULL, len_out, a, b)
    try:
        return _ffi.unpack(res_ptr, len_out[0])
    finally:
        _lib.fstrie_free(res_ptr)
