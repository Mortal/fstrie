from ._lowlevel import lib as _lib, ffi as _ffi
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


_special_errors = {
    1: InternalError,
    2: UnicodeDecodeError,
    3: RootDoesNotExistError,
    4: IOError,
}


# From https://youtu.be/zmtHaZG7pPc?t=22m29s
def _rustcall(func, *args):
    err = _ffi.new('struct fstrie_error *')
    rv = func(*(args + (err,)))
    if not err[0].failed:
        return rv
    try:
        exc_class = _special_errors.get(err[0].code, FstrieError)
        exc = exc_class(_ffi.string(err[0].message).decode('utf-8', 'replace'))
    finally:
        _lib.fstrie_free(err[0].message)
    raise exc


class Database:
    def __init__(self, root):
        self._root = root

    def __enter__(self):
        self._handle = _rustcall(_lib.fstrie_load, self._root.encode('utf-8'))
        return self

    def __exit__(self, exc_type, exc_value, exc_tb):
        _rustcall(_lib.fstrie_unload, self._handle)
        del self._handle

    def __getitem__(self, key):
        result_ptr = _rustcall(_lib.fstrie_lookup, self._handle, key.encode('utf-8'))
        try:
            result = []
            i = 0
            while result_ptr[i] != _ffi.NULL:
                result.append(_ffi.string(result_ptr[i]).decode('utf-8', 'replace'))
                i += 1
            return result
        finally:
            _lib.fstrie_free_list(result_ptr)
