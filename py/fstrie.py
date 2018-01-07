import os
from cffi import FFI


_ffi = FFI()
_ffi.cdef("""
/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
typedef void fstrie_db_t;

struct fstrie_error {
    char *message;
    int failed;
    int code;
};

void fstrie_init();

fstrie_db_t *fstrie_load(const char *root, struct fstrie_error *);
void fstrie_unload(fstrie_db_t *, struct fstrie_error *);
char **fstrie_lookup(fstrie_db_t *, const char *key, struct fstrie_error *);

void fstrie_free(char *);
void fstrie_free_list(char **);
""")
_lib = _ffi.dlopen(os.path.join(
    os.path.dirname(__file__), '../target/debug/libfstrie.so'))
_lib.fstrie_init()


class FstrieError(Exception):
    pass


class InternalError(FstrieError):
    pass


class UnicodeDecodeError(FstrieError):
    pass


class RootDoesNotExistError(FstrieError):
    pass


class IOError(FstrieError):
    pass


special_errors = {
    1: InternalError,
    2: UnicodeDecodeError,
    3: RootDoesNotExistError,
    4: IOError,
}


# From https://youtu.be/zmtHaZG7pPc?t=22m29s
def rustcall(func, *args):
    err = _ffi.new('struct fstrie_error *')
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


class Database:
    def __init__(self, root):
        self._root = root

    def __enter__(self):
        self._handle = rustcall(_lib.fstrie_load, self._root.encode('utf-8'))
        return self

    def __exit__(self, exc_type, exc_value, exc_tb):
        rustcall(_lib.fstrie_unload, self._handle)
        del self._handle

    def __getitem__(self, key):
        result_ptr = rustcall(_lib.fstrie_lookup, self._handle, key.encode('utf-8'))
        try:
            result = []
            i = 0
            while result_ptr[i] != _ffi.NULL:
                result.append(_ffi.string(result_ptr[i]).decode('utf-8', 'replace'))
                i += 1
            return result
        finally:
            _lib.fstrie_free_list(result_ptr)
