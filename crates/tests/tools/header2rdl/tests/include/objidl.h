#pragma once
/* Shim for <objidl.h> — provides ISequentialStream and IStream. */
#include <unknwn.h>

/* -------------------------------------------------------------------------
 * ISequentialStream
 * ---------------------------------------------------------------------- */
#ifndef __ISequentialStream_INTERFACE_DEFINED__
#define __ISequentialStream_INTERFACE_DEFINED__
MIDL_INTERFACE("0c733a30-2a1c-11ce-ade5-00aa0044773d")
ISequentialStream : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Read(
        void *pv, ULONG cb, ULONG *pcbRead) = 0;
    virtual HRESULT STDMETHODCALLTYPE Write(
        const void *pv, ULONG cb, ULONG *pcbWritten) = 0;
};
#endif /* __ISequentialStream_INTERFACE_DEFINED__ */

/* -------------------------------------------------------------------------
 * STATSTG
 * ---------------------------------------------------------------------- */
typedef struct tagSTATSTG {
    LPWSTR  pwcsName;
    DWORD   type;
    UINT64  cbSize;
    struct { DWORD dwLowDateTime; DWORD dwHighDateTime; } mtime;
    struct { DWORD dwLowDateTime; DWORD dwHighDateTime; } ctime;
    struct { DWORD dwLowDateTime; DWORD dwHighDateTime; } atime;
    DWORD   grfMode;
    DWORD   grfLocksSupported;
    CLSID   clsid;
    DWORD   grfStateBits;
    DWORD   reserved;
} STATSTG;

/* -------------------------------------------------------------------------
 * IStream
 * ---------------------------------------------------------------------- */
#ifndef __IStream_INTERFACE_DEFINED__
#define __IStream_INTERFACE_DEFINED__
MIDL_INTERFACE("0000000c-0000-0000-C000-000000000046")
IStream : public ISequentialStream {
public:
    virtual HRESULT STDMETHODCALLTYPE Seek(
        long long dlibMove, DWORD dwOrigin, UINT64 *plibNewPosition) = 0;
    virtual HRESULT STDMETHODCALLTYPE SetSize(UINT64 libNewSize) = 0;
    virtual HRESULT STDMETHODCALLTYPE CopyTo(
        IStream *pstm, UINT64 cb,
        UINT64 *pcbRead, UINT64 *pcbWritten) = 0;
    virtual HRESULT STDMETHODCALLTYPE Commit(DWORD grfCommitFlags) = 0;
    virtual HRESULT STDMETHODCALLTYPE Revert() = 0;
    virtual HRESULT STDMETHODCALLTYPE LockRegion(
        UINT64 libOffset, UINT64 cb, DWORD dwLockType) = 0;
    virtual HRESULT STDMETHODCALLTYPE UnlockRegion(
        UINT64 libOffset, UINT64 cb, DWORD dwLockType) = 0;
    virtual HRESULT STDMETHODCALLTYPE Stat(
        STATSTG *pstatstg, DWORD grfStatFlag) = 0;
    virtual HRESULT STDMETHODCALLTYPE Clone(IStream **ppstm) = 0;
};
#endif /* __IStream_INTERFACE_DEFINED__ */
