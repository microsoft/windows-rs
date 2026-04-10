#pragma once
/*
 * Minimal Windows type shim for cross-platform libclang parsing on Linux.
 *
 * This file provides the subset of Windows SDK types and macros needed to
 * parse COM/MIDL-style headers (e.g. WebView2.h) with clang targeting
 * x86_64-pc-windows-msvc on a Linux host.  It is added via -isystem so
 * the types defined here are treated as system-header definitions and are
 * NOT emitted in the RDL output.
 */

/* -------------------------------------------------------------------------
 * Basic integer types
 * ---------------------------------------------------------------------- */
typedef long                HRESULT;
typedef int                 BOOL;
typedef int                 INT;
typedef unsigned int        UINT;
typedef unsigned int        UINT32;
typedef unsigned long long  UINT64;
typedef unsigned long       ULONG;
typedef long                LONG;
typedef unsigned char       BYTE;
typedef unsigned short      WORD;
typedef unsigned long       DWORD;
typedef float               FLOAT;
typedef double              DOUBLE;
typedef void               *LPVOID;

/* -------------------------------------------------------------------------
 * Wide-character types
 * ---------------------------------------------------------------------- */
typedef wchar_t             WCHAR;
typedef WCHAR              *LPWSTR;
typedef const WCHAR        *LPCWSTR;
typedef const WCHAR        *PCWSTR;
typedef WCHAR              *BSTR;

/* -------------------------------------------------------------------------
 * Pointer-sized handle types
 * ---------------------------------------------------------------------- */
typedef void               *HANDLE;
typedef HANDLE              HWND;
typedef HANDLE              HMODULE;
typedef HANDLE              HINSTANCE;
typedef HANDLE              HICON;
typedef HANDLE              HCURSOR;
typedef HANDLE              HBRUSH;
typedef HANDLE              HMENU;
typedef HANDLE              HDC;
typedef HANDLE              HBITMAP;

/* -------------------------------------------------------------------------
 * Geometry
 * ---------------------------------------------------------------------- */
typedef struct tagPOINT { LONG x; LONG y; } POINT;
typedef struct tagSIZE  { LONG cx; LONG cy; } SIZE;
typedef struct tagRECT  { LONG left; LONG top; LONG right; LONG bottom; } RECT;
typedef RECT *LPRECT;
typedef const RECT *LPCRECT;

/* -------------------------------------------------------------------------
 * GUID / IID / CLSID
 * ---------------------------------------------------------------------- */
typedef struct _GUID {
    unsigned long  Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char  Data4[8];
} GUID;
typedef GUID  IID;
typedef GUID  CLSID;
typedef const IID &REFIID;
typedef const CLSID &REFCLSID;

/* -------------------------------------------------------------------------
 * Calling-convention / linkage macros
 * ---------------------------------------------------------------------- */
#define WINAPI          __stdcall
#define APIENTRY        __stdcall
#define CALLBACK        __stdcall
#define STDAPICALLTYPE  __cdecl
#define STDMETHODCALLTYPE __stdcall
#define __RPC_FAR
#define FAR

/* -------------------------------------------------------------------------
 * HRESULT constants
 * ---------------------------------------------------------------------- */
#define S_OK        ((HRESULT)0L)
#define S_FALSE     ((HRESULT)1L)
#define E_NOINTERFACE ((HRESULT)0x80004002L)
#define E_POINTER   ((HRESULT)0x80004003L)
#define E_FAIL      ((HRESULT)0x80004005L)
#define E_OUTOFMEMORY ((HRESULT)0x8007000EL)

/* -------------------------------------------------------------------------
 * COM interface macros
 * ---------------------------------------------------------------------- */
#ifndef interface
#define interface struct
#endif

#define MIDL_INTERFACE(x) struct __declspec(uuid(x))

#define STDMETHOD(name)        virtual HRESULT STDMETHODCALLTYPE name
#define STDMETHOD_(type, name) virtual type    STDMETHODCALLTYPE name
#define PURE = 0

#define STDAPI        extern "C" HRESULT STDAPICALLTYPE

/* -------------------------------------------------------------------------
 * IUnknown — the root COM interface
 * ---------------------------------------------------------------------- */
#ifndef __IUnknown_INTERFACE_DEFINED__
#define __IUnknown_INTERFACE_DEFINED__
MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void **ppvObject) = 0;
    virtual ULONG   STDMETHODCALLTYPE AddRef() = 0;
    virtual ULONG   STDMETHODCALLTYPE Release() = 0;
};
#endif /* __IUnknown_INTERFACE_DEFINED__ */
