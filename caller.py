# From https://youtu.be/zmtHaZG7pPc?t=22m29s
def rustcall(func, *args):
    err = _ffi.new('lsm_error_t *')
    rv = func(*(args + (err,)))
    if not err[0].failed:
        return rv
    try:
        cls = special_errors.get(err[0].code, SourceMapError)
        exc = cls(_ffi.string(err[0].message).decode('utf-8', 'replace'))
    finally:
        _lib.lsm_buffer_free(err[0].message)
    raise exc
