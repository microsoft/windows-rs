//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       combaseapi.h
//
//  Contents:   Base Component Object Model defintions.
//
//----------------------------------------------------------------------------

#include <apiset.h>
#include <apisetcconv.h>

//TODO version number should be bumped when _APISET_TARGET_VERSION_WIN10_RS2 becomes available

#include <rpc.h>
#include <rpcndr.h>

#if (NTDDI_VERSION >= NTDDI_VISTA && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0600
#endif

#if (NTDDI_VERSION >= NTDDI_WS03 && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0502
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0501
#endif

#if (NTDDI_VERSION >= NTDDI_WIN2K && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0500
#endif

#if !defined(_COMBASEAPI_H_)
#define _COMBASEAPI_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <pshpack8.h>

//TODO change _OLE32_ to _COMBASEAPI_
#ifdef _OLE32_
#define WINOLEAPI        STDAPI
#define WINOLEAPI_(type) STDAPI_(type)
#else

#ifdef _68K_
#ifndef REQUIRESAPPLEPASCAL
#define WINOLEAPI        EXTERN_C DECLSPEC_IMPORT HRESULT PASCAL
#define WINOLEAPI_(type) EXTERN_C DECLSPEC_IMPORT type PASCAL
#else
#define WINOLEAPI        EXTERN_C DECLSPEC_IMPORT PASCAL HRESULT
#define WINOLEAPI_(type) EXTERN_C DECLSPEC_IMPORT PASCAL type
#endif
#else
#define WINOLEAPI        EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define WINOLEAPI_(type) EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif

#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/****** Interface Declaration ***********************************************/

/*
 *      These are macros for declaring interfaces.  They exist so that
 *      a single definition of the interface is simulataneously a proper
 *      declaration of the interface structures (C++ abstract classes)
 *      for both C and C++.
 *
 *      DECLARE_INTERFACE(iface) is used to declare an interface that does
 *      not derive from a base interface.
 *      DECLARE_INTERFACE_(iface, baseiface) is used to declare an interface
 *      that does derive from a base interface.
 *
 *      By default if the source file has a .c extension the C version of
 *      the interface declaratations will be expanded; if it has a .cpp
 *      extension the C++ version will be expanded. if you want to force
 *      the C version expansion even though the source file has a .cpp
 *      extension, then define the macro "CINTERFACE".
 *      eg.     cl -DCINTERFACE file.cpp
 *
 *      Example Interface declaration:
 *
 *          #undef  INTERFACE
 *          #define INTERFACE   IClassFactory
 *
 *          DECLARE_INTERFACE_(IClassFactory, IUnknown)
 *          {
 *              // *** IUnknown methods ***
 *              STDMETHOD(QueryInterface) (THIS_
 *                                        REFIID riid,
 *                                        LPVOID FAR* ppvObj) PURE;
 *              STDMETHOD_(ULONG,AddRef) (THIS) PURE;
 *              STDMETHOD_(ULONG,Release) (THIS) PURE;
 *
 *              // *** IClassFactory methods ***
 *              STDMETHOD(CreateInstance) (THIS_
 *                                        LPUNKNOWN pUnkOuter,
 *                                        REFIID riid,
 *                                        LPVOID FAR* ppvObject) PURE;
 *          };
 *
 *      Example C++ expansion:
 *
 *          struct FAR IClassFactory : public IUnknown
 *          {
 *              virtual HRESULT STDMETHODCALLTYPE QueryInterface(
 *                                                  IID FAR& riid,
 *                                                  LPVOID FAR* ppvObj) = 0;
 *              virtual HRESULT STDMETHODCALLTYPE AddRef(void) = 0;
 *              virtual HRESULT STDMETHODCALLTYPE Release(void) = 0;
 *              virtual HRESULT STDMETHODCALLTYPE CreateInstance(
 *                                              LPUNKNOWN pUnkOuter,
 *                                              IID FAR& riid,
 *                                              LPVOID FAR* ppvObject) = 0;
 *          };
 *
 *          NOTE: Our documentation says '#define interface class' but we use
 *          'struct' instead of 'class' to keep a lot of 'public:' lines
 *          out of the interfaces.  The 'FAR' forces the 'this' pointers to
 *          be far, which is what we need.
 *
 *      Example C expansion:
 *
 *          typedef struct IClassFactory
 *          {
 *              const struct IClassFactoryVtbl FAR* lpVtbl;
 *          } IClassFactory;
 *
 *          typedef struct IClassFactoryVtbl IClassFactoryVtbl;
 *
 *          struct IClassFactoryVtbl
 *          {
 *              HRESULT (STDMETHODCALLTYPE * QueryInterface) (
 *                                                  IClassFactory FAR* This,
 *                                                  IID FAR* riid,
 *                                                  LPVOID FAR* ppvObj) ;
 *              HRESULT (STDMETHODCALLTYPE * AddRef) (IClassFactory FAR* This) ;
 *              HRESULT (STDMETHODCALLTYPE * Release) (IClassFactory FAR* This) ;
 *              HRESULT (STDMETHODCALLTYPE * CreateInstance) (
 *                                                  IClassFactory FAR* This,
 *                                                  LPUNKNOWN pUnkOuter,
 *                                                  IID FAR* riid,
 *                                                  LPVOID FAR* ppvObject);
 *              HRESULT (STDMETHODCALLTYPE * LockServer) (
 *                                                  IClassFactory FAR* This,
 *                                                  BOOL fLock);
 *          };
 */

#if defined(__cplusplus) && !defined(CINTERFACE)
//#define interface               struct FAR

#ifdef COM_STDMETHOD_CAN_THROW
#define COM_DECLSPEC_NOTHROW
#else
#define COM_DECLSPEC_NOTHROW DECLSPEC_NOTHROW
#endif

#define __STRUCT__ struct
#define interface __STRUCT__
#define STDMETHOD(method)        virtual COM_DECLSPEC_NOTHROW HRESULT STDMETHODCALLTYPE method
#define STDMETHOD_CHPE_PATCHABLE(method)        virtual COM_DECLSPEC_NOTHROW DECLSPEC_CHPE_PATCHABLE HRESULT STDMETHODCALLTYPE method
#define STDMETHOD_(type,method)  virtual COM_DECLSPEC_NOTHROW type STDMETHODCALLTYPE method
#define STDMETHODV(method)       virtual COM_DECLSPEC_NOTHROW HRESULT STDMETHODVCALLTYPE method
#define STDMETHODV_(type,method) virtual COM_DECLSPEC_NOTHROW type STDMETHODVCALLTYPE method
#define PURE                    = 0
#define THIS_
#define THIS                    void
#define DECLARE_INTERFACE(iface)                        interface DECLSPEC_NOVTABLE iface
#define DECLARE_INTERFACE_(iface, baseiface)            interface DECLSPEC_NOVTABLE iface : public baseiface
#define DECLARE_INTERFACE_IID(iface, iid)               interface DECLSPEC_UUID(iid) DECLSPEC_NOVTABLE iface
#define DECLARE_INTERFACE_IID_(iface, baseiface, iid)   interface DECLSPEC_UUID(iid) DECLSPEC_NOVTABLE iface : public baseiface

#define IFACEMETHOD(method)         __override STDMETHOD(method)
#define IFACEMETHOD_(type,method)   __override STDMETHOD_(type,method)
#define IFACEMETHODV(method)        __override STDMETHODV(method)
#define IFACEMETHODV_(type,method)  __override STDMETHODV_(type,method)

#if !defined(BEGIN_INTERFACE)
#if defined(_MPPC_) && ((defined(_MSC_VER) || defined(__SC__) || defined(__MWERKS__)) && !defined(NO_NULL_VTABLE_ENTRY))
   #define BEGIN_INTERFACE virtual void a() {}
   #define END_INTERFACE
#else
   #define BEGIN_INTERFACE
   #define END_INTERFACE
#endif
#endif

#ifndef SORTPP_PASS

// This forward declaration has been left where this type was previously defined, to preserve ordering.
extern "C++"
{
    template<typename T> _Post_equal_to_(pp) _Post_satisfies_(return == pp) void** IID_PPV_ARGS_Helper(T** pp);
}

#endif  // !SORTPP_PASS

#define IID_PPV_ARGS(ppType) __uuidof(**(ppType)), IID_PPV_ARGS_Helper(ppType)

#else

#define interface               struct

#define STDMETHOD(method)       HRESULT (STDMETHODCALLTYPE * method)
#define STDMETHOD_(type,method) type (STDMETHODCALLTYPE * method)
#define STDMETHODV(method)       HRESULT (STDMETHODVCALLTYPE * method)
#define STDMETHODV_(type,method) type (STDMETHODVCALLTYPE * method)

#define IFACEMETHOD(method)         __override STDMETHOD(method)
#define IFACEMETHOD_(type,method)   __override STDMETHOD_(type,method)
#define IFACEMETHODV(method)        __override STDMETHODV(method)
#define IFACEMETHODV_(type,method)  __override STDMETHODV_(type,method)

#if !defined(BEGIN_INTERFACE)
#if defined(_MPPC_)
    #define BEGIN_INTERFACE       void    *b;
    #define END_INTERFACE
#else
    #define BEGIN_INTERFACE
    #define END_INTERFACE
#endif
#endif

#define PURE
#define THIS_                   INTERFACE FAR* This,
#define THIS                    INTERFACE FAR* This
#ifdef CONST_VTABLE
#undef CONST_VTBL
#define CONST_VTBL const
#define DECLARE_INTERFACE(iface)    typedef interface iface { \
                                    const struct iface##Vtbl FAR* lpVtbl; \
                                } iface; \
                                typedef const struct iface##Vtbl iface##Vtbl; \
                                const struct iface##Vtbl
#else
#undef CONST_VTBL
#define CONST_VTBL
#define DECLARE_INTERFACE(iface)    typedef interface iface { \
                                    struct iface##Vtbl FAR* lpVtbl; \
                                } iface; \
                                typedef struct iface##Vtbl iface##Vtbl; \
                                struct iface##Vtbl
#endif
#define DECLARE_INTERFACE_(iface, baseiface)    DECLARE_INTERFACE(iface)
#define DECLARE_INTERFACE_IID(iface, iid)               DECLARE_INTERFACE(iface)
#define DECLARE_INTERFACE_IID_(iface, baseiface, iid)   DECLARE_INTERFACE_(iface, baseiface)
#endif

/****** Additional basic types **********************************************/

#ifndef FARSTRUCT
#ifdef __cplusplus
#define FARSTRUCT   FAR
#else
#define FARSTRUCT
#endif  // __cplusplus
#endif  // FARSTRUCT

#ifndef HUGEP
#if defined(_WIN32) || defined(_MPPC_)
#define HUGEP
#else
#define HUGEP __huge
#endif // WIN32
#endif // HUGEP

#include <stdlib.h>

#define LISet32(li, v) ((li).HighPart = ((LONG) (v)) < 0 ? -1 : 0, (li).LowPart = (v))

#define ULISet32(li, v) ((li).HighPart = 0, (li).LowPart = (v))

#define CLSCTX_INPROC           (CLSCTX_INPROC_SERVER|CLSCTX_INPROC_HANDLER)

// With DCOM, CLSCTX_REMOTE_SERVER should be included
// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)
#define CLSCTX_ALL              (CLSCTX_INPROC_SERVER| \
                                 CLSCTX_INPROC_HANDLER| \
                                 CLSCTX_LOCAL_SERVER| \
                                 CLSCTX_REMOTE_SERVER)

#define CLSCTX_SERVER           (CLSCTX_INPROC_SERVER|CLSCTX_LOCAL_SERVER|CLSCTX_REMOTE_SERVER)
#else
#define CLSCTX_ALL              (CLSCTX_INPROC_SERVER| \
                                 CLSCTX_INPROC_HANDLER| \
                                 CLSCTX_LOCAL_SERVER )

#define CLSCTX_SERVER           (CLSCTX_INPROC_SERVER|CLSCTX_LOCAL_SERVER)
#endif

// class registration flags; passed to CoRegisterClassObject
typedef enum tagREGCLS
{
    REGCLS_SINGLEUSE = 0,       // class object only generates one instance
    REGCLS_MULTIPLEUSE = 1,     // same class object genereates multiple inst.
                                // and local automatically goes into inproc tbl.
    REGCLS_MULTI_SEPARATE = 2,  // multiple use, but separate control over each
                                // context.
    REGCLS_SUSPENDED      = 4,  // register is as suspended, will be activated
                                // when app calls CoResumeClassObjects
    REGCLS_SURROGATE      = 8,  // must be used when a surrogate process
                                // is registering a class object that will be
                                // loaded in the surrogate
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
    REGCLS_AGILE = 0x10,        // Class object aggregates the free-threaded marshaler
                                // and will be made visible to all inproc apartments.
                                // Can be used together with other flags - for example,
                                // REGCLS_AGILE | REGCLS_MULTIPLEUSE to register a
                                // class object that can be used multiple times from
                                // different apartments. Without other flags, behavior
                                // will retain REGCLS_SINGLEUSE semantics in that only
                                // one instance can be generated.
#endif
} REGCLS;
DEFINE_ENUM_FLAG_OPERATORS(REGCLS);

/* here is where we pull in the MIDL generated headers for the interfaces */
typedef interface    IRpcStubBuffer     IRpcStubBuffer;
typedef interface    IRpcChannelBuffer  IRpcChannelBuffer;

// COM initialization flags; passed to CoInitialize.
typedef enum tagCOINITBASE
{
// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)
  // These constants are only valid on Windows NT 4.0
  COINITBASE_MULTITHREADED      = 0x0,      // OLE calls objects on any thread.
#endif // DCOM
} COINITBASE;

#include <wtypesbase.h>
#include <unknwnbase.h>

#if defined(__cplusplus) && !defined(CINTERFACE)

//  IID_PPV_ARGS(ppType)
//      ppType is the variable of type IType that will be filled
//
//      RESULTS in:  IID_IType, ppvType
//      will create a compiler error if wrong level of indirection is used.
//
extern "C++"
{
    template<typename T> _Post_equal_to_(pp) _Post_satisfies_(return == pp) void** IID_PPV_ARGS_Helper(T** pp)
    {
#pragma prefast(suppress: 6269, "Tool issue with unused static_cast")
        (void)static_cast<IUnknown*>(*pp);    // make sure everyone derives from IUnknown
        return reinterpret_cast<void**>(pp);
    }
}

#endif // defined(__cplusplus) && !defined(CINTERFACE)

#include <objidlbase.h>

#include <guiddef.h>

#ifndef INITGUID
// TODO change to cguidbase.h
#include <cguid.h>
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

/****** STD Object API Prototypes *****************************************/

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetMalloc(
    _In_ DWORD dwMemContext,
    _Outptr_ LPMALLOC  FAR * ppMalloc
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
CreateStreamOnHGlobal(
    HGLOBAL hGlobal,
    _In_ BOOL fDeleteOnRelease,
    _Outptr_ LPSTREAM  FAR * ppstm
    );

_Check_return_ WINOLEAPI
GetHGlobalFromStream(
    _In_ LPSTREAM pstm,
    _Out_ HGLOBAL  FAR * phglobal
    );

/* init/uninit */

WINOLEAPI_(void)
CoUninitialize(
    void
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI_(DWORD)
CoGetCurrentProcess(
    void
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
CoInitializeEx(
    _In_opt_ LPVOID pvReserved,
    _In_ DWORD dwCoInit
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI
CoGetCallerTID(
    _Out_ LPDWORD lpdwTID
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI
CoGetCurrentLogicalThreadId(
    _Out_ GUID* pguid
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // DCOM

#if (_WIN32_WINNT >= 0x0501)

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
CoGetContextToken(
    _Out_ ULONG_PTR* pToken
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetDefaultContext(
    _In_ APTTYPE aptType,
    _In_ REFIID riid,
    _Outptr_ void** ppv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// definition for Win7 new APIs

#if (NTDDI_VERSION >= NTDDI_WIN7)

_Check_return_ WINOLEAPI
CoGetApartmentType(
    _Out_ APTTYPE* pAptType,
    _Out_ APTTYPEQUALIFIER* pAptQualifier
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

// definition for Win8 new APIs

#if (NTDDI_VERSION >= NTDDI_WIN8)

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagServerInformation
{
    DWORD  dwServerPid;
    DWORD  dwServerTid;
    UINT64 ui64ServerAddress;
} ServerInformation, *PServerInformation;

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoDecodeProxy(
    _In_ DWORD dwClientPid,
    _In_ UINT64 ui64ProxyAddress,
    _Out_ PServerInformation pServerInformation
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DECLARE_HANDLE(CO_MTA_USAGE_COOKIE);

_Check_return_ WINOLEAPI
CoIncrementMTAUsage(
    _Out_ CO_MTA_USAGE_COOKIE* pCookie
    );

               WINOLEAPI
CoDecrementMTAUsage(
    _In_ CO_MTA_USAGE_COOKIE Cookie
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINOLEAPI
CoAllowUnmarshalerCLSID(
    _In_ REFCLSID clsid
    );

// Predefine _EXE_INITIALIZE_MTA to have the runtime initialize an MTA for your process
// prior to initializing globals (i.e. dynamic initializers)
#if !defined RC_INVOKED // _EXE_INITIALIZE_MTA et al. are too long for rc
    #if defined _M_IX86
        #define _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX "_"
    #elif defined _M_X64 || defined _M_ARM || defined _M_ARM64
        #define _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX ""
    #endif

    #if defined _EXE_INITIALIZE_MTA && !defined _M_CEE
        #pragma comment(lib, "exe_initialize_mta")
        #pragma comment(linker, "/include:" _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX "__PLEASE_LINK_WITH_exe_initialize_mta.lib")
    #endif
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#endif

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetObjectContext(
    _In_ REFIID riid,
    _Outptr_ LPVOID  FAR * ppv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/* register/revoke/get class objects */

_Check_return_ WINOLEAPI
CoGetClassObject(
    _In_ REFCLSID rclsid,
    _In_ DWORD dwClsContext,
    _In_opt_ LPVOID pvReserved,
    _In_ REFIID riid,
    _Outptr_ LPVOID  FAR * ppv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
CoRegisterClassObject(
    _In_ REFCLSID rclsid,
    _In_ LPUNKNOWN pUnk,
    _In_ DWORD dwClsContext,
    _In_ DWORD flags,
    _Out_ LPDWORD lpdwRegister
    );

WINOLEAPI
CoRevokeClassObject(
    _In_ DWORD dwRegister
    );

_Check_return_ WINOLEAPI
CoResumeClassObjects(
    void
    );

_Check_return_ WINOLEAPI
CoSuspendClassObjects(
    void
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI_(ULONG)
CoAddRefServerProcess(
    void
    );

WINOLEAPI_(ULONG)
CoReleaseServerProcess(
    void
    );

_Check_return_ WINOLEAPI
CoGetPSClsid(
    _In_ REFIID riid,
    _Out_ CLSID* pClsid
    );

_Check_return_ WINOLEAPI
CoRegisterPSClsid(
    _In_ REFIID riid,
    _In_ REFCLSID rclsid
    );

// Registering surrogate processes
_Check_return_ WINOLEAPI
CoRegisterSurrogate(
    _In_ LPSURROGATE pSurrogate
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* marshaling interface pointers */

_Check_return_ WINOLEAPI
CoGetMarshalSizeMax(
    _Out_ ULONG* pulSize,
    _In_ REFIID riid,
    _In_ LPUNKNOWN pUnk,
    _In_ DWORD dwDestContext,
    _In_opt_ LPVOID pvDestContext,
    _In_ DWORD mshlflags
    );

_Check_return_ WINOLEAPI
CoMarshalInterface(
    _In_ LPSTREAM pStm,
    _In_ REFIID riid,
    _In_ LPUNKNOWN pUnk,
    _In_ DWORD dwDestContext,
    _In_opt_ LPVOID pvDestContext,
    _In_ DWORD mshlflags
    );

_Check_return_ WINOLEAPI
CoUnmarshalInterface(
    _In_ LPSTREAM pStm,
    _In_ REFIID riid,
    _COM_Outptr_ LPVOID  FAR * ppv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI
CoMarshalHresult(
    _In_ LPSTREAM pstm,
    _In_ HRESULT hresult
    );

WINOLEAPI
CoUnmarshalHresult(
    _In_ LPSTREAM pstm,
    _Out_ HRESULT  FAR * phresult
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoReleaseMarshalData(
    _In_ LPSTREAM pStm
    );

_Check_return_ WINOLEAPI
CoDisconnectObject(
    _In_ LPUNKNOWN pUnk,
    _In_ DWORD dwReserved
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoLockObjectExternal(
    _In_ LPUNKNOWN pUnk,
    _In_ BOOL fLock,
    _In_ BOOL fLastUnlockReleases
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetStandardMarshal(
    _In_ REFIID riid,
    _In_opt_ LPUNKNOWN pUnk,
    _In_ DWORD dwDestContext,
    _In_opt_ LPVOID pvDestContext,
    _In_ DWORD mshlflags,
    _Outptr_ LPMARSHAL  FAR * ppMarshal
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetStdMarshalEx(
    _In_ LPUNKNOWN pUnkOuter,
    _In_ DWORD smexflags,
    _Outptr_ LPUNKNOWN  FAR * ppUnkInner
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* flags for CoGetStdMarshalEx */
typedef enum tagSTDMSHLFLAGS
{
    SMEXF_SERVER     = 0x01,       // server side aggregated std marshaler
    SMEXF_HANDLER    = 0x02        // client side (handler) agg std marshaler
} STDMSHLFLAGS;

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI_(BOOL)
CoIsHandlerConnected(
    _In_ LPUNKNOWN pUnk
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// Apartment model inter-thread interface passing helpers
_Check_return_ WINOLEAPI
CoMarshalInterThreadInterfaceInStream(
    _In_ REFIID riid,
    _In_ LPUNKNOWN pUnk,
    _Outptr_ LPSTREAM* ppStm
    );

_Check_return_ WINOLEAPI
CoGetInterfaceAndReleaseStream(
    _In_ LPSTREAM pStm,
    _In_ REFIID iid,
    _COM_Outptr_ LPVOID  FAR * ppv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
CoCreateFreeThreadedMarshaler(
    _In_opt_ LPUNKNOWN punkOuter,
    _Outptr_ LPUNKNOWN* ppunkMarshal
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI_(void)
CoFreeUnusedLibraries(
    void
    );

#if (_WIN32_WINNT >= 0x0501)
WINOLEAPI_(void)
CoFreeUnusedLibrariesEx(
    _In_ DWORD dwUnloadDelay,
    _In_ DWORD dwReserved
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#if (_WIN32_WINNT >= 0x0600)

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoDisconnectContext(
    DWORD dwTimeout
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/* Call Security. */

_Check_return_ WINOLEAPI
CoInitializeSecurity(
    _In_opt_ PSECURITY_DESCRIPTOR pSecDesc,
    _In_ LONG cAuthSvc,
    _In_reads_opt_(cAuthSvc) SOLE_AUTHENTICATION_SERVICE* asAuthSvc,
    _In_opt_ void* pReserved1,
    _In_ DWORD dwAuthnLevel,
    _In_ DWORD dwImpLevel,
    _In_opt_ void* pAuthList,
    _In_ DWORD dwCapabilities,
    _In_opt_ void* pReserved3
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetCallContext(
    _In_ REFIID riid,
    _Outptr_ void** ppInterface
    );

_Check_return_ WINOLEAPI
CoQueryProxyBlanket(
    _In_ IUnknown* pProxy,
    _Out_opt_ DWORD* pwAuthnSvc,
    _Out_opt_ DWORD* pAuthzSvc,
    _Outptr_opt_ LPOLESTR* pServerPrincName,
    _Out_opt_ DWORD* pAuthnLevel,
    _Out_opt_ DWORD* pImpLevel,
    _Out_opt_ RPC_AUTH_IDENTITY_HANDLE* pAuthInfo,
    _Out_opt_ DWORD* pCapabilites
    );

_Check_return_ WINOLEAPI
CoSetProxyBlanket(
    _In_ IUnknown* pProxy,
    _In_ DWORD dwAuthnSvc,
    _In_ DWORD dwAuthzSvc,
    _In_opt_ OLECHAR* pServerPrincName,
    _In_ DWORD dwAuthnLevel,
    _In_ DWORD dwImpLevel,
    _In_opt_ RPC_AUTH_IDENTITY_HANDLE pAuthInfo,
    _In_ DWORD dwCapabilities
    );

_Check_return_ WINOLEAPI
CoCopyProxy(
    _In_ IUnknown* pProxy,
    _Outptr_ IUnknown** ppCopy
    );

_Check_return_ WINOLEAPI
CoQueryClientBlanket(
    _Out_opt_ DWORD* pAuthnSvc,
    _Out_opt_ DWORD* pAuthzSvc,
    _Outptr_opt_ LPOLESTR* pServerPrincName,
    _Out_opt_ DWORD* pAuthnLevel,
    _Out_opt_ DWORD* pImpLevel,
    _Outptr_opt_result_buffer_(_Inexpressible_("depends on pAuthnSvc")) RPC_AUTHZ_HANDLE* pPrivs,
    _Inout_opt_ DWORD* pCapabilities
    );

_Check_return_ WINOLEAPI
CoImpersonateClient(
    void
    );

_Check_return_ WINOLEAPI
CoRevertToSelf(
    void
    );

_Check_return_ WINOLEAPI
CoQueryAuthenticationServices(
    _Out_ DWORD* pcAuthSvc,
    _Outptr_result_buffer_(*pcAuthSvc) SOLE_AUTHENTICATION_SERVICE** asAuthSvc
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoSwitchCallContext(
    _In_opt_ IUnknown* pNewObject,
    _Outptr_ IUnknown** ppOldObject
    );

#define COM_RIGHTS_EXECUTE 1
#define COM_RIGHTS_EXECUTE_LOCAL 2
#define COM_RIGHTS_EXECUTE_REMOTE 4
#define COM_RIGHTS_ACTIVATE_LOCAL 8
#define COM_RIGHTS_ACTIVATE_REMOTE 16
#define COM_RIGHTS_RESERVED1 32
#define COM_RIGHTS_RESERVED2 64

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // DCOM

#pragma region App or OneCore Family or Games Family
#if (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES))

/* helper for creating instances */

_Check_return_ WINOLEAPI
CoCreateInstance(
    _In_ REFCLSID rclsid,
    _In_opt_ LPUNKNOWN pUnkOuter,
    _In_ DWORD dwClsContext,
    _In_ REFIID riid,
    _COM_Outptr_ _At_(*ppv, _Post_readable_size_(_Inexpressible_(varies))) LPVOID  FAR * ppv
    );

#endif // (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES))
#pragma endregion

#pragma region App or OneCore Family
#if (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM))

// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)

_Check_return_ WINOLEAPI
CoCreateInstanceEx(
    _In_ REFCLSID Clsid,
    _In_opt_ IUnknown* punkOuter,
    _In_ DWORD dwClsCtx,
    _In_opt_ COSERVERINFO* pServerInfo,
    _In_ DWORD dwCount,
    _Inout_updates_(dwCount) MULTI_QI* pResults
    );

#endif // DCOM

#if (_WIN32_WINNT >= 0x0602)

_Check_return_ WINOLEAPI
CoCreateInstanceFromApp(
    _In_ REFCLSID Clsid,
    _In_opt_ IUnknown* punkOuter,
    _In_ DWORD dwClsCtx,
    _In_opt_ PVOID reserved,
    _In_ DWORD dwCount,
    _Inout_updates_(dwCount) MULTI_QI* pResults
    );

#endif // (_WIN32_WINNT >= 0x0602)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI
CoRegisterActivationFilter(
    _In_ IActivationFilter* pActivationFilter
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

/* Call related APIs */
// DCOM
#if (_WIN32_WINNT >= 0x0500) || defined(_WIN32_DCOM)

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetCancelObject(
    _In_ DWORD dwThreadId,
    _In_ REFIID iid,
    _Outptr_ void** ppUnk
    );

_Check_return_ WINOLEAPI
CoSetCancelObject(
    _In_opt_ IUnknown* pUnk
    );

_Check_return_ WINOLEAPI
CoCancelCall(
    _In_ DWORD dwThreadId,
    _In_ ULONG ulTimeout
    );

_Check_return_ WINOLEAPI
CoTestCancel(
    void
    );

_Check_return_ WINOLEAPI
CoEnableCallCancellation(
    _In_opt_ LPVOID pReserved
    );

_Check_return_ WINOLEAPI
CoDisableCallCancellation(
    _In_opt_ LPVOID pReserved
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/* other helpers */

_Check_return_ WINOLEAPI
StringFromCLSID(
    _In_ REFCLSID rclsid,
    _Outptr_ LPOLESTR  FAR * lplpsz
    );

_Check_return_ WINOLEAPI
CLSIDFromString(
    _In_ LPCOLESTR lpsz,
    _Out_ LPCLSID pclsid
    );

_Check_return_ WINOLEAPI
StringFromIID(
    _In_ REFIID rclsid,
    _Outptr_ LPOLESTR  FAR * lplpsz
    );

_Check_return_ WINOLEAPI
IIDFromString(
    _In_ LPCOLESTR lpsz,
    _Out_ LPIID lpiid
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI
ProgIDFromCLSID(
    _In_ REFCLSID clsid,
    _Outptr_ LPOLESTR  FAR * lplpszProgID
    );

_Check_return_ WINOLEAPI
CLSIDFromProgID(
    _In_ LPCOLESTR lpszProgID,
    _Out_ LPCLSID lpclsid
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_Check_return_ WINOLEAPI_(int)
StringFromGUID2(
    _In_ REFGUID rguid,
    _Out_writes_to_(cchMax, return) LPOLESTR lpsz,
    _In_ int cchMax
    );

_Check_return_ WINOLEAPI
CoCreateGuid(
    _Out_ GUID  FAR * pguid
    );

/* Prop variant support */

typedef struct tagPROPVARIANT PROPVARIANT;

_Check_return_
WINOLEAPI
PropVariantCopy(
    _Out_ PROPVARIANT* pvarDest,
    _In_ const PROPVARIANT* pvarSrc
    );

WINOLEAPI
PropVariantClear(
    _Inout_ PROPVARIANT* pvar
    );

WINOLEAPI
FreePropVariantArray(
    _In_ ULONG cVariants,
    _Inout_updates_(cVariants) PROPVARIANT* rgvars
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // DCOM

// DCOM
#if (_WIN32_WINNT >= 0x0400) || defined(_WIN32_DCOM)
/* Synchronization API */

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoWaitForMultipleHandles(
    _In_ DWORD dwFlags,
    _In_ DWORD dwTimeout,
    _In_ ULONG cHandles,
    _In_reads_(cHandles) LPHANDLE pHandles,
    _Out_ LPDWORD lpdwindex
    );

/* Flags for Synchronization API and Classes */

typedef enum tagCOWAIT_FLAGS
{
  COWAIT_DEFAULT = 0,
  COWAIT_WAITALL = 1,
  COWAIT_ALERTABLE = 2,
  COWAIT_INPUTAVAILABLE = 4,
  COWAIT_DISPATCH_CALLS = 8,
  COWAIT_DISPATCH_WINDOW_MESSAGES = 0x10,
}COWAIT_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(COWAIT_FLAGS);

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef enum CWMO_FLAGS
{
  CWMO_DEFAULT = 0,
  CWMO_DISPATCH_CALLS = 1,
  CWMO_DISPATCH_WINDOW_MESSAGES = 2,
} CWMO_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(CWMO_FLAGS);

WINOLEAPI
CoWaitForMultipleObjects(
    _In_ DWORD dwFlags,
    _In_ DWORD dwTimeout,
    _In_ ULONG cHandles,
    _In_reads_(cHandles) const HANDLE* pHandles,
    _Out_ LPDWORD lpdwindex
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#define CWMO_MAX_HANDLES 56

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // DCOM

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoGetTreatAsClass(
    _In_ REFCLSID clsidOld,
    _Out_ LPCLSID pClsidNew
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

/* for flushing OLESCM remote binding handles */

#if (_WIN32_WINNT >= 0x0501)

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_ WINOLEAPI
CoInvalidateRemoteMachineBindings(
    _In_ LPOLESTR pszMachineName
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

enum AgileReferenceOptions
{
    AGILEREFERENCE_DEFAULT        = 0,
    AGILEREFERENCE_DELAYEDMARSHAL = 1,
};

_Check_return_ WINOLEAPI
RoGetAgileReference(
    _In_ enum AgileReferenceOptions options,
    _In_ REFIID riid,
    _In_ IUnknown* pUnk,
    _COM_Outptr_ IAgileReference** ppAgileReference
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/* the server dlls must define their DllGetClassObject and DllCanUnloadNow
 * to match these; the typedefs are located here to ensure all are changed at
 * the same time.
 */

typedef HRESULT (STDAPICALLTYPE * LPFNGETCLASSOBJECT) (REFCLSID, REFIID, LPVOID *);
typedef HRESULT (STDAPICALLTYPE * LPFNCANUNLOADNOW)(void);

_Check_return_
STDAPI  DllGetClassObject(_In_ REFCLSID rclsid, _In_ REFIID riid, _Outptr_ LPVOID FAR* ppv);

__control_entrypoint(DllExport)
STDAPI  DllCanUnloadNow(void);

/****** Default Memory Allocation ******************************************/
WINOLEAPI_(_Ret_opt_ _Post_writable_byte_size_(cb)  __drv_allocatesMem(Mem) _Check_return_ LPVOID)
CoTaskMemAlloc(
    _In_ SIZE_T cb
    );

WINOLEAPI_(_Ret_opt_ _Post_writable_byte_size_(cb)  _When_(cb > 0, __drv_allocatesMem(Mem) _Check_return_) LPVOID)
CoTaskMemRealloc(
    _Pre_maybenull_ __drv_freesMem(Mem) _Post_invalid_ LPVOID pv,
    _In_ SIZE_T cb
    );

WINOLEAPI_(void)
CoTaskMemFree(
    _Frees_ptr_opt_ LPVOID pv
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAPI
CoFileTimeNow(
    _Out_ FILETIME  FAR * lpFileTime
    );

_Check_return_ WINOLEAPI
CLSIDFromProgIDEx(
    _In_ LPCOLESTR lpszProgID,
    _Out_ LPCLSID lpclsid
    );

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

#if !defined(_CO_DEVICE_CATALOG_)
#define _CO_DEVICE_CATALOG_
#endif

DECLARE_HANDLE(CO_DEVICE_CATALOG_COOKIE);

_Check_return_
WINOLEAPI
CoRegisterDeviceCatalog(
    _In_ PCWSTR deviceInstanceId,
    _Out_ CO_DEVICE_CATALOG_COOKIE* cookie
    );

_Check_return_
WINOLEAPI
CoRevokeDeviceCatalog(
    _In_ CO_DEVICE_CATALOG_COOKIE cookie
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifndef RC_INVOKED
#include <poppack.h>
#endif // RC_INVOKED

#endif     // __COMBASEAPI_H__
