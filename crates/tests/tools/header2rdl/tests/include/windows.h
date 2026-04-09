/* Minimal Windows SDK shim for COM interface parsing.
 *
 * This file is a self-contained substitute for <windows.h> that provides just
 * enough definitions to parse MIDL-generated COM interface headers without
 * requiring the real Windows SDK or MinGW headers.  It is used by the
 * test_header2rdl convert test via the `--include include` sidecar option.
 */
#pragma once

#ifdef __cplusplus

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown {
    virtual long __stdcall QueryInterface(const void* riid, void** ppv) = 0;
    virtual unsigned long __stdcall AddRef() = 0;
    virtual unsigned long __stdcall Release() = 0;
};

#endif /* __cplusplus */

#define MIDL_INTERFACE(x)  struct __declspec(uuid(x))
#define STDMETHODCALLTYPE  __stdcall
#define STDMETHOD(m)       virtual long  STDMETHODCALLTYPE m
#define STDMETHOD_(t, m)   virtual t     STDMETHODCALLTYPE m
#define PURE               = 0
