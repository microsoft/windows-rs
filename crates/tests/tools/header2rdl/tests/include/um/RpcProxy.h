/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rpcproxy.h

Abstract:

    Definitions for rpc proxy  stubs.

Compiler switches:

    -DREGISTER_PROXY_DLL
        Generates DllMain, DllRegisterServer, and DllUnregisterServer functions
        for automatically registering a proxy DLL.

    -DPROXY_CLSID=clsid
        Specifies a class ID to be used by the proxy DLL.

    -DPROXY_CLSID_IS={0x6f11fe5c,0x2fc5,0x101b,{0x9e,0x45,0x00,0x00,0x0b,0x65,0xc7,0xef}}
        Specifies the value of the class ID to be used by the proxy DLL.

    -DENTRY_PREFIX=<prefix>
        String to be prepended on all the DllGetClassObject etc routines
        in dlldata.c.  This includes: DllGetClassObject, DllCanUnloadNow
        and DllMain, DllRegisterServer, and DllUnregisterServer.

    -DNT35_STRICT
        Specifies that the target platform is Windows NT 3.5. This switch disables
        the new functions added after the Windows NT 3.5 release.

--*/

// This version of the rpcndr.h file corresponds to MIDL version 5.0.+
// used with NT5 beta1+ env from build #1700 on.

#ifndef __RPCPROXY_H_VERSION__
#define __RPCPROXY_H_VERSION__      ( 477 )
#endif // __RPCPROXY_H_VERSION__

#include <winapifamily.h>

#if !defined(_KRPCENV_)

#ifndef __RPCPROXY_H__
#define __RPCPROXY_H__
#if _MSC_VER > 1000
#pragma once
#endif

#define __midl_proxy

#ifdef __REQUIRED_RPCPROXY_H_VERSION__
    #if ( __RPCPROXY_H_VERSION__ < __REQUIRED_RPCPROXY_H_VERSION__ )
        #error incorrect <rpcproxy.h> version. Use the header that matches with the MIDL compiler.
    #endif
#endif

// If this is the first file included __RPC_WIN64__ is not defined yet.
#if defined(_M_IA64) || defined(_M_AMD64)
#include <pshpack8.h>
#endif

#include <basetsd.h>

#ifndef INC_OLE2
#define INC_OLE2
#endif

#ifndef DECLSPEC_SELECTANY
#if (_MSC_VER >= 1200)
#define DECLSPEC_SELECTANY __declspec(selectany)
#else
#define DECLSPEC_SELECTANY
#endif
#endif

#ifndef DECLSPEC_NOINLINE
#if (_MSC_VER >= 1200)
#define DECLSPEC_NOINLINE __declspec(noinline)
#else
#define DECLSPEC_NOINLINE
#endif
#endif

#ifndef RPCPROXY_IID_DECLSPEC_SECTION
#if (_MSC_VER >= 1100)
#pragma section("rpcproxy$_iid", read)
#define RPCPROXY_IID_DECLSPEC_SECTION __declspec(allocate("rpcproxy$_iid"))
#else
#define RPCPROXY_IID_DECLSPEC_SECTION
#endif
#endif


#if defined(WIN32) || defined(_M_AMD64)

//We need to define REFIID, REFCLSID, REFGUID, & REFFMTID here so that the
//proxy code won't get the const GUID *const definition.
#ifndef GUID_DEFINED
#include <guiddef.h>
#endif /* GUID_DEFINED */

#if defined(__cplusplus)
extern "C"
{
#endif

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// forward declarations

// By default, this header cannot be compiled as C++ without defining CINTERFACE (which uses a
// C-style view of COM interfaces for C++ in MIDL-generated headers), because type definitions here
// depend on *Vtbl types not present in this configuration. Define RPCPROXY_ENABLE_CPP_NO_CINTERFACE
// to enable this when needed.
#if defined(RPCPROXY_ENABLE_CPP_NO_CINTERFACE) && defined(__cplusplus) && !defined(CINTERFACE)
typedef struct IRpcStubBufferVtbl IRpcStubBufferVtbl;
typedef struct ICallFactoryVtbl ICallFactoryVtbl;
typedef struct IReleaseMarshalBuffersVtbl IReleaseMarshalBuffersVtbl;
typedef struct IPSFactoryBufferVtbl IPSFactoryBufferVtbl;
#endif

typedef struct tagCInterfaceStubVtbl CInterfaceStubVtbl;
typedef struct tagCInterfaceProxyVtbl CInterfaceProxyVtbl;

typedef struct tagCInterfaceStubVtbl *  PCInterfaceStubVtblList;
typedef struct tagCInterfaceProxyVtbl *  PCInterfaceProxyVtblList;
typedef const char *                    PCInterfaceName;
typedef int __stdcall IIDLookupRtn( const IID * pIID, int * pIndex );
typedef IIDLookupRtn * PIIDLookup;

// Uses a default lookup mechanism
#define NdrDefaultIIDLookup ((PIIDLookup)-1)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4610) // struct can never be instantiated - user defined constructor required
#pragma warning(disable:4510) // default constructor could not be generated
#pragma warning(disable:4512) // assignment operator could not be generated
#endif

// pointers to arrays of CInterfaceProxyVtbl's and CInterfaceStubVtbls
typedef struct tagProxyFileInfo
{
    const PCInterfaceProxyVtblList *pProxyVtblList;
    const PCInterfaceStubVtblList  *pStubVtblList;
    const PCInterfaceName *         pNamesArray;
    const IID **                    pDelegatedIIDs;
    const PIIDLookup                pIIDLookupRtn;
    unsigned short                  TableSize;
    unsigned short                  TableVersion;
    const IID **                    pAsyncIIDLookup;
    LONG_PTR                        Filler2;
    LONG_PTR                        Filler3;
    LONG_PTR                        Filler4;
}ProxyFileInfo;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

// extended info with list of interface names
typedef ProxyFileInfo ExtendedProxyFileInfo;

#include <rpc.h>
#include <rpcndr.h>
#include <string.h>
#include <memory.h>

typedef struct tagCInterfaceProxyHeader
{
    //
    // New fields should be added here, at the beginning of the structure.
    //
#ifdef USE_STUBLESS_PROXY
    const void *    pStublessProxyInfo;
#endif
    const IID *     piid;
} CInterfaceProxyHeader;

// Macro used for ANSI compatible stubs.

#define CINTERFACE_PROXY_VTABLE( n )  \
struct \
{                                     \
    CInterfaceProxyHeader header;     \
    void *Vtbl[ n ];                  \
}

#define IInspectableInterfaceProxyTag ((void *)-1)
#define IUnknownInterfaceProxyTag ((void *)-2)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning( disable:4200 )
typedef struct tagCInterfaceProxyVtbl
{
    CInterfaceProxyHeader header;
#if defined( _MSC_VER )
    void *Vtbl[];
#else
    void *Vtbl[1];
#endif
} CInterfaceProxyVtbl;
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default:4200 )
#endif

typedef
void
(__RPC_STUB * PRPC_STUB_FUNCTION) (
    IRpcStubBuffer          *   This,
    IRpcChannelBuffer       *   _pRpcChannelBuffer,
    PRPC_MESSAGE                _pRpcMessage,
    DWORD *pdwStubPhase);

typedef struct tagCInterfaceStubHeader
{
    //New fields should be added here, at the beginning of the structure.
    const IID               *   piid;
    const MIDL_SERVER_INFO  *   pServerInfo;
    ULONG               DispatchTableCount;
    const PRPC_STUB_FUNCTION *  pDispatchTable;
} CInterfaceStubHeader;

#define IInspectableNdrStubCall2CommonStubListTag ((const PRPC_STUB_FUNCTION*)-1)
#define IInspectableNdrStubCall3CommonStubListTag ((const PRPC_STUB_FUNCTION*)-2)

#if !defined(RPCPROXY_ENABLE_CPP_NO_CINTERFACE) || !defined(__cplusplus) || defined(CINTERFACE)
typedef struct tagCInterfaceStubVtbl
{
    CInterfaceStubHeader        header;
    IRpcStubBufferVtbl          Vtbl;
} CInterfaceStubVtbl;

#define RPCPROXY_GET_STUB_HEADER(StubVtblListEntry) (&(StubVtblListEntry)->header)
#else

// No definition of CInterfaceStubVtbl is provided in this configuration, but
// RPCPROXY_GET_STUB_HEADER can be used to get the stub header from an entry in
// a stub vtable list.
#define RPCPROXY_GET_STUB_HEADER(StubVtblListEntry) (reinterpret_cast<const CInterfaceStubHeader*>(StubVtblListEntry))

#endif

typedef struct tagCInterfaceStubVtblTag
{
    CInterfaceStubHeader        header;
    void *                      tag;
} CInterfaceStubVtblTag;

typedef struct tagCStdStubBuffer
{
    const struct IRpcStubBufferVtbl *   lpVtbl; //Points to Vtbl field in CInterfaceStubVtbl.
    LONG                                RefCount;
    struct IUnknown *                   pvServerObject;

    const struct ICallFactoryVtbl *     pCallFactoryVtbl;
    const IID *                         pAsyncIID;
    struct IPSFactoryBuffer *           pPSFactory;
    const struct IReleaseMarshalBuffersVtbl *     pRMBVtbl;
} CStdStubBuffer;

typedef struct tagCStdPSFactoryBuffer
{
    const IPSFactoryBufferVtbl  *   lpVtbl;
    LONG                            RefCount;
    const ProxyFileInfo **          pProxyFileList;
    LONG                            Filler1;  //Reserved for future use.
} CStdPSFactoryBuffer;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
void
RPC_ENTRY
NdrProxyInitialize(
    void *              This,
    PRPC_MESSAGE        pRpcMsg,
    PMIDL_STUB_MESSAGE  pStubMsg,
    PMIDL_STUB_DESC     pStubDescriptor,
    unsigned int        ProcNum );

RPCRTAPI
void
RPC_ENTRY
NdrProxyGetBuffer(
    void *              This,
    PMIDL_STUB_MESSAGE pStubMsg);

RPCRTAPI
void
RPC_ENTRY
NdrProxySendReceive(
    void                *This,
    MIDL_STUB_MESSAGE   *pStubMsg);

RPCRTAPI
void
RPC_ENTRY
NdrProxyFreeBuffer(
    void                *This,
    MIDL_STUB_MESSAGE   *pStubMsg);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
HRESULT
RPC_ENTRY
NdrProxyErrorHandler(
    DWORD dwExceptionCode);
    
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
void
RPC_ENTRY
NdrStubInitialize(
    PRPC_MESSAGE        pRpcMsg,
    PMIDL_STUB_MESSAGE  pStubMsg,
    PMIDL_STUB_DESC     pStubDescriptor,
    IRpcChannelBuffer * pRpcChannelBuffer);
    
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
void
RPC_ENTRY
NdrStubInitializePartial(
    PRPC_MESSAGE        pRpcMsg,
    PMIDL_STUB_MESSAGE  pStubMsg,
    PMIDL_STUB_DESC     pStubDescriptor,
    IRpcChannelBuffer * pRpcChannelBuffer,
    ULONG       RequestedBufferSize);

void __RPC_STUB NdrStubForwardingFunction(
    IN  IRpcStubBuffer *    This,
    IN  IRpcChannelBuffer * pChannel,
    IN  PRPC_MESSAGE        pmsg,
    OUT DWORD             * pdwStubPhase);
    
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
void
RPC_ENTRY
NdrStubGetBuffer(
    IRpcStubBuffer *    This,
    IRpcChannelBuffer * pRpcChannelBuffer,
    PMIDL_STUB_MESSAGE  pStubMsg);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

RPCRTAPI
HRESULT
RPC_ENTRY
NdrStubErrorHandler(
    DWORD               dwExceptionCode);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer_QueryInterface(
    IRpcStubBuffer  *   This,
    REFIID              riid,
    void            **  ppvObject);

ULONG STDMETHODCALLTYPE
CStdStubBuffer_AddRef(
    IRpcStubBuffer *    This);

ULONG STDMETHODCALLTYPE
CStdStubBuffer_Release(
    IRpcStubBuffer *    This);

ULONG STDMETHODCALLTYPE
NdrCStdStubBuffer_Release(
    IRpcStubBuffer *    This,
    IPSFactoryBuffer *  pPSF);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer_Connect(
    IRpcStubBuffer *    This,
    IUnknown *          pUnkServer);

void STDMETHODCALLTYPE
CStdStubBuffer_Disconnect(
    IRpcStubBuffer *    This);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer_Invoke(
    IRpcStubBuffer  *   This,
    RPCOLEMESSAGE   *   pRpcMsg,
    IRpcChannelBuffer * pRpcChannelBuffer);

IRpcStubBuffer * STDMETHODCALLTYPE
CStdStubBuffer_IsIIDSupported(
    IRpcStubBuffer  *   This,
    REFIID riid);

ULONG STDMETHODCALLTYPE
CStdStubBuffer_CountRefs(
    IRpcStubBuffer  *   This);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer_DebugServerQueryInterface(
    IRpcStubBuffer  *   This,
    void **ppv);

void STDMETHODCALLTYPE
CStdStubBuffer_DebugServerRelease(
    IRpcStubBuffer  *   This,
    void *pv);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer2_QueryInterface(
    IRpcStubBuffer  *   This,
    REFIID              riid,
    void            **  ppvObject);

ULONG STDMETHODCALLTYPE
CStdStubBuffer2_Release(
    IRpcStubBuffer *This);

HRESULT STDMETHODCALLTYPE
CStdStubBuffer2_Connect(
    IRpcStubBuffer *pthis,
    IUnknown *pUnkServer);

void STDMETHODCALLTYPE
CStdStubBuffer2_Disconnect(
    IRpcStubBuffer *pthis);

ULONG STDMETHODCALLTYPE
CStdStubBuffer2_CountRefs(
    IRpcStubBuffer *pthis);

ULONG STDMETHODCALLTYPE
NdrCStdStubBuffer2_Release(
    IRpcStubBuffer *This,
    IPSFactoryBuffer * pPSF);

HRESULT STDMETHODCALLTYPE
CStdAsyncStubBuffer_QueryInterface(
    IRpcStubBuffer *pthis,
    REFIID riid,
    void **ppvObject);

ULONG STDMETHODCALLTYPE
CStdAsyncStubBuffer_AddRef(
    IRpcStubBuffer *pthis);

ULONG STDMETHODCALLTYPE
CStdAsyncStubBuffer_Release(
    IRpcStubBuffer *pthis);

HRESULT STDMETHODCALLTYPE
CStdAsyncStubBuffer_Connect(
    IRpcStubBuffer *pthis,
    IUnknown *pUnkServer);

void STDMETHODCALLTYPE
CStdAsyncStubBuffer_Disconnect(
    IRpcStubBuffer *pthis);

HRESULT STDMETHODCALLTYPE
CStdAsyncStubBuffer_Invoke(
    IRpcStubBuffer *pthis,
    RPCOLEMESSAGE *_prpcmsg,
    IRpcChannelBuffer *_pRpcChannelBuffer);

ULONG STDMETHODCALLTYPE
CStdAsyncStubBuffer2_Release(
    IRpcStubBuffer *pthis);

HRESULT STDMETHODCALLTYPE
CStdAsyncStubBuffer2_Connect(
    IRpcStubBuffer *pthis,
    IUnknown *pUnkServer);

void STDMETHODCALLTYPE
CStdAsyncStubBuffer2_Disconnect(
    IRpcStubBuffer *pthis);

DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient3(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient4(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient5(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient6(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient7(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient8(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient9(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient10(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient11(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient12(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient13(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient14(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient15(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient16(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient17(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient18(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient19(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient20(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient21(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient22(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient23(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient24(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient25(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient26(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient27(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient28(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient29(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient30(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient31(void);
DECLSPEC_CHPE_GUEST void STDMETHODCALLTYPE ObjectStublessClient32(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction3(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction4(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction5(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction6(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction7(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction8(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction9(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction10(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction11(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction12(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction13(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction14(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction15(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction16(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction17(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction18(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction19(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction20(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction21(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction22(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction23(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction24(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction25(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction26(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction27(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction28(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction29(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction30(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction31(void);

DECLSPEC_CHPE_GUEST
void STDMETHODCALLTYPE
NdrProxyForwardingFunction32(void);

#define CStdStubBuffer_METHODS \
    CStdStubBuffer_QueryInterface, \
    CStdStubBuffer_AddRef, \
    CStdStubBuffer_Release, \
    CStdStubBuffer_Connect, \
    CStdStubBuffer_Disconnect, \
    CStdStubBuffer_Invoke, \
    CStdStubBuffer_IsIIDSupported, \
    CStdStubBuffer_CountRefs, \
    CStdStubBuffer_DebugServerQueryInterface, \
    CStdStubBuffer_DebugServerRelease

#define CStdStubBuffer_DELEGATING_METHODS 0,0,CStdStubBuffer2_Release,0,0,0,0,0,0,0
#define CStdAsyncStubBuffer_METHODS 0,0,0,0,0,0,0,0,0,0
#define CStdAsyncStubBuffer_DELEGATING_METHODS 0,0,0,0,0,0,0,0,0,0

#define CStdStubBuffer_METHODS_OPT              \
    CStdStubBuffer_QueryInterface,              \
    CStdStubBuffer_AddRef,                      \
    CStdStubBuffer_Release,                     \
    CStdStubBuffer_Connect,                     \
    CStdStubBuffer_Disconnect,                  \
    CStdStubBuffer_Invoke,                      \
    CStdStubBuffer_IsIIDSupported,              \
    CStdStubBuffer_CountRefs,                   \
    CStdStubBuffer_DebugServerQueryInterface,   \
    CStdStubBuffer_DebugServerRelease

#define CStdStubBuffer_DELEGATING_METHODS_OPT   \
    CStdStubBuffer2_QueryInterface,             \
    CStdStubBuffer_AddRef,                      \
    CStdStubBuffer2_Release,                    \
    CStdStubBuffer2_Connect,                    \
    CStdStubBuffer2_Disconnect,                 \
    CStdStubBuffer_Invoke,                      \
    CStdStubBuffer_IsIIDSupported,              \
    CStdStubBuffer2_CountRefs,                  \
    CStdStubBuffer_DebugServerQueryInterface,   \
    CStdStubBuffer_DebugServerRelease

#define CStdAsyncStubBuffer_METHODS_OPT             \
    CStdAsyncStubBuffer_QueryInterface,             \
    CStdAsyncStubBuffer_AddRef,                     \
    CStdAsyncStubBuffer_Release,                    \
    CStdAsyncStubBuffer_Connect,                    \
    CStdAsyncStubBuffer_Disconnect,                 \
    CStdAsyncStubBuffer_Invoke,                     \
    CStdStubBuffer_IsIIDSupported,                  \
    CStdStubBuffer_CountRefs,                       \
    CStdStubBuffer_DebugServerQueryInterface,       \
    CStdStubBuffer_DebugServerRelease

#define CStdAsyncStubBuffer_DELEGATING_METHODS_OPT  \
    CStdAsyncStubBuffer_QueryInterface,             \
    CStdAsyncStubBuffer_AddRef,                     \
    CStdAsyncStubBuffer2_Release,                   \
    CStdAsyncStubBuffer2_Connect,                   \
    CStdAsyncStubBuffer2_Disconnect,                \
    CStdAsyncStubBuffer_Invoke,                     \
    CStdStubBuffer_IsIIDSupported,                  \
    CStdStubBuffer2_CountRefs,                      \
    CStdStubBuffer_DebugServerQueryInterface,       \
    CStdStubBuffer_DebugServerRelease

#define CStdStubBuffer_METHODS_TAG ((void *)-1)
#define CStdStubBuffer_DELEGATING_METHODS_TAG ((void *)-2)
#define CStdAsyncStubBuffer_METHODS_TAG ((void *)-3)
#define CStdAsyncStubBuffer_DELEGATING_METHODS_TAG ((void *)-4)

//+-------------------------------------------------------------------------
//
//  Macro definitions for the proxy file
//
//--------------------------------------------------------------------------

#define IID_GENERIC_CHECK_IID(name,pIID,index) memcmp( pIID, name##_ProxyVtblList[ index ]->header.piid, 16 )

#define IID_BS_LOOKUP_SETUP     int result, low=-1;

#define IID_BS_LOOKUP_INITIAL_TEST(name, sz, split)   \
    result = name##_CHECK_IID( split );               \
    if ( result > 0 )                                 \
        { low = sz - split; }                         \
    else if ( !result )                               \
        { low = split; goto found_label; }

#define IID_BS_LOOKUP_NEXT_TEST(name, split )      \
        result = name##_CHECK_IID( low + split );  \
        if ( result >= 0 )                         \
        { low = low + split; if ( !result ) goto found_label; }

#define IID_BS_LOOKUP_RETURN_RESULT(name, sz, index )  \
    low = low + 1;                                     \
    if (low >= sz)                                     \
        goto not_found_label;                          \
    result = name##_CHECK_IID( low );                  \
    if (result)                                        \
        goto not_found_label;                          \
    found_label: (index) = low; return 1;              \
    not_found_label: return 0;

//+-------------------------------------------------------------------------
//
//  Macro and routine definitions for the dlldata file
//
//--------------------------------------------------------------------------

/****************************************************************************
 * Proxy Dll APIs
 ****************************************************************************/

RPCRTAPI
HRESULT
RPC_ENTRY
NdrDllGetClassObject (
    IN  REFCLSID                rclsid,
    IN  REFIID                  riid,
    OUT void **                 ppv,
    IN const ProxyFileInfo **   pProxyFileList,
    IN const CLSID *            pclsid,
    IN CStdPSFactoryBuffer *    pPSFactoryBuffer);

RPCRTAPI
HRESULT
RPC_ENTRY
NdrDllCanUnloadNow(
    IN CStdPSFactoryBuffer *    pPSFactoryBuffer);

// if the user specified a routine prefix, pick it up...
//     if not, add nothing

#ifndef ENTRY_PREFIX

#ifndef DllMain
#define DISABLE_THREAD_LIBRARY_CALLS(x) DisableThreadLibraryCalls(x)
#endif

#define ENTRY_PREFIX
#define DLLREGISTERSERVER_ENTRY DllRegisterServer
#define DLLUNREGISTERSERVER_ENTRY DllUnregisterServer
#define DLLMAIN_ENTRY DllMain

#define DLLGETCLASSOBJECT_ENTRY DllGetClassObject
#define DLLCANUNLOADNOW_ENTRY DllCanUnloadNow
#define DLLRPCDUMMYCALL_ENTRY DllRpcDummyCall

#else   // ENTRY_PREFIX

#define __rpc_macro_expand2(a, b) a##b
#define __rpc_macro_expand(a, b) __rpc_macro_expand2(a,b)
#define DLLREGISTERSERVER_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllRegisterServer)
#define DLLUNREGISTERSERVER_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllUnregisterServer)
#define DLLMAIN_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllMain)

#define DLLGETCLASSOBJECT_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllGetClassObject)
#define DLLCANUNLOADNOW_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllCanUnloadNow)
#define DLLRPCDUMMYCALL_ENTRY __rpc_macro_expand(ENTRY_PREFIX, DllRpcDummyCall)

#endif  // ENTRY_PREFIX

#ifndef DISABLE_THREAD_LIBRARY_CALLS
#define DISABLE_THREAD_LIBRARY_CALLS(x)
#endif

/*************************************************************************
The following new functions were added after the Windows NT 3.5 release.
*************************************************************************/

RPCRTAPI
HRESULT
RPC_ENTRY
NdrDllRegisterProxy(
    IN HMODULE                  hDll,
    IN const ProxyFileInfo **   pProxyFileList,
    IN const CLSID *            pclsid);

RPCRTAPI
HRESULT
RPC_ENTRY
NdrDllUnregisterProxy(
    IN HMODULE                  hDll,
    IN const ProxyFileInfo **   pProxyFileList,
    IN const CLSID *            pclsid);

#define REGISTER_PROXY_DLL_ROUTINES(pProxyFileList, pClsID) \
    \
    HINSTANCE hProxyDll = 0; \
    \
    /*DllMain saves the DLL module handle for later use by DllRegisterServer */ \
    BOOL WINAPI DLLMAIN_ENTRY( \
        HINSTANCE  hinstDLL, \
        DWORD  fdwReason, \
        LPVOID  lpvReserved) \
    { \
        UNREFERENCED_PARAMETER(lpvReserved); \
        if(fdwReason == DLL_PROCESS_ATTACH) \
            { \
            hProxyDll = hinstDLL; \
            DISABLE_THREAD_LIBRARY_CALLS(hinstDLL); \
            } \
        return TRUE; \
    } \
    \
    /* DllRegisterServer registers the interfaces contained in the proxy DLL. */ \
    HRESULT STDAPICALLTYPE DLLREGISTERSERVER_ENTRY() \
    { \
        return NdrDllRegisterProxy(hProxyDll, (const ProxyFileInfo **)pProxyFileList, pClsID); \
    }  \
    \
    /* DllUnregisterServer unregisters the interfaces contained in the proxy DLL. */ \
    HRESULT STDAPICALLTYPE DLLUNREGISTERSERVER_ENTRY() \
    { \
        return NdrDllUnregisterProxy(hProxyDll, (const ProxyFileInfo **)pProxyFileList, pClsID); \
    }

//Delegation support.
#define STUB_FORWARDING_FUNCTION        NdrStubForwardingFunction

/*************************************************************************
End of new functions.
*************************************************************************/

// PROXY_CLSID has precedence over PROXY_CLSID_IS

#ifdef PROXY_CLSID

#define CLSID_PSFACTORYBUFFER extern CLSID PROXY_CLSID;

#else // PROXY_CLSID

#ifdef PROXY_CLSID_IS
#define CLSID_PSFACTORYBUFFER const CLSID CLSID_PSFactoryBuffer = PROXY_CLSID_IS;
#define PROXY_CLSID     CLSID_PSFactoryBuffer
#else // PROXY_CLSID_IS
#define CLSID_PSFACTORYBUFFER
#endif //PROXY_CLSID_IS

#endif //PROXY_CLSID

// if the user specified an override for the class id, it is
// PROXY_CLSID at this point

#ifndef PROXY_CLSID
#define GET_DLL_CLSID   \
    ( aProxyFileList[0]->pStubVtblList[0] != 0 ? \
    aProxyFileList[0]->pStubVtblList[0]->header.piid : 0)
#else  //PROXY_CLSID
#define GET_DLL_CLSID   &PROXY_CLSID
#endif //PROXY_CLSID



#define EXTERN_PROXY_FILE(name) \
    EXTERN_C const ProxyFileInfo name##_ProxyFileInfo;

#define PROXYFILE_LIST_START    \
    extern const ProxyFileInfo* const aProxyFileList[]    = {

#define REFERENCE_PROXY_FILE(name)  \
    & name##_ProxyFileInfo

#define PROXYFILE_LIST_END      \
    0 };

// return pointers to the class information

#define DLLDATA_GETPROXYDLLINFO(pPFList,pClsid) \
    void RPC_ENTRY GetProxyDllInfo( const ProxyFileInfo*** pInfo, const CLSID ** pId )  \
        {   \
        *pInfo  = (const ProxyFileInfo**)pPFList;  \
        *pId    = pClsid;   \
        };

// ole entry points:
#define DLLGETCLASSOBJECTROUTINE(pPFlist, pClsid,pFactory)    \
_Check_return_ HRESULT STDAPICALLTYPE DLLGETCLASSOBJECT_ENTRY ( \
     _In_ REFCLSID rclsid, \
     _In_ REFIID riid, \
     _Outptr_ void ** ppv ) \
        { \
        return  \
            NdrDllGetClassObject(rclsid,riid,ppv,(const ProxyFileInfo**)pPFlist,pClsid,pFactory ); \
        }

#define DLLCANUNLOADNOW(pFactory)   \
 HRESULT STDAPICALLTYPE DLLCANUNLOADNOW_ENTRY()    \
    {   \
    return NdrDllCanUnloadNow( pFactory );    \
    }


#define DLLRPCDUMMYCALL    \
    void __cdecl DLLRPCDUMMYCALL_ENTRY(void)    \
        {   \
        /* Don't define __purecall method anymore */ \
        }

#define CSTDSTUBBUFFERRELEASE(pFactory)   \
DECLSPEC_XFGVIRT(IUnknown, Release) \
ULONG STDMETHODCALLTYPE CStdStubBuffer_Release(IRpcStubBuffer *This) \
    {   \
    return NdrCStdStubBuffer_Release(This,(IPSFactoryBuffer *)pFactory);   \
    }   \

#ifdef PROXY_DELEGATION
#define CSTDSTUBBUFFER2RELEASE(pFactory)   \
DECLSPEC_XFGVIRT(IUnknown, Release) \
ULONG STDMETHODCALLTYPE CStdStubBuffer2_Release(IRpcStubBuffer *This) \
    {   \
    return NdrCStdStubBuffer2_Release(This,(IPSFactoryBuffer *)pFactory);   \
    }
#else
#define CSTDSTUBBUFFER2RELEASE(pFactory)
#endif //PROXY_DELEGATION


#ifdef REGISTER_PROXY_DLL
#define DLLREGISTRY_ROUTINES(pProxyFileList,pClsID ) REGISTER_PROXY_DLL_ROUTINES(pProxyFileList,pClsID )
#else
#define DLLREGISTRY_ROUTINES(pProxyFileList,pClsID )
#endif //REGISTER_PROXY_DLL


// the dll entry points that must be defined
#define DLLDATA_ROUTINES(pProxyFileList,pClsID )    \
    \
    CLSID_PSFACTORYBUFFER \
    \
    CStdPSFactoryBuffer       gPFactory = {0,0,0,0};  \
    \
    DLLDATA_GETPROXYDLLINFO(pProxyFileList,pClsID) \
    \
    DLLGETCLASSOBJECTROUTINE(pProxyFileList,pClsID,&gPFactory)    \
    \
    DLLCANUNLOADNOW(&gPFactory)   \
    \
    CSTDSTUBBUFFERRELEASE(&gPFactory)   \
    \
    CSTDSTUBBUFFER2RELEASE(&gPFactory) \
    \
    DLLRPCDUMMYCALL    \
    \
    DLLREGISTRY_ROUTINES(pProxyFileList, pClsID) \
    \

    // more code goes here...


#define DLLDATA_STANDARD_ROUTINES   \
    DLLDATA_ROUTINES( (const ProxyFileInfo**) pProxyFileList, &CLSID_PSFactoryBuffer )  \

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if defined(__cplusplus)
} // extern "C"
#endif

#endif // WIN32 or _WIN64_

#if defined(_M_IA64) || defined(_M_AMD64)
#include <poppack.h>
#endif

#endif // __RPCPROXY_H__

#endif // _KRPCENV_
