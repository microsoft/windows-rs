/*++

Copyright (c) 1996-1999 Microsoft Corporation

Module Name:

    scesvc.h

Abstract:

    Wrapper APIs for services

Revision History:

--*/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DEFINE_GUID(cNodetypeSceTemplateServices, 0x24a7f717, 0x1f0c, 0x11d1, 0xaf, 0xfb, 0x0, 0xc0, 0x4f, 0xb9, 0x84, 0xf9);
DEFINE_GUID(cNodetypeSceAnalysisServices, 0x678050c7, 0x1ff8, 0x11d1, 0xaf, 0xfb, 0x0, 0xc0, 0x4f, 0xb9, 0x84, 0xf9);
DEFINE_GUID(cNodetypeSceEventLog, 0x2ce06698, 0x4bf3, 0x11d1, 0x8c, 0x30, 0x0, 0xc0, 0x4f, 0xb9, 0x84, 0xf9);
DEFINE_GUID(IID_ISceSvcAttachmentPersistInfo, 0x6d90e0d0, 0x200d, 0x11d1, 0xaf, 0xfb, 0x0, 0xc0, 0x4f, 0xb9, 0x84, 0xf9);
DEFINE_GUID(IID_ISceSvcAttachmentData, 0x17c35fde, 0x200d, 0x11d1, 0xaf, 0xfb, 0x0, 0xc0, 0x4f, 0xb9, 0x84, 0xf9);

#ifndef _scesvc_
#define _scesvc_

#ifdef __cplusplus
extern "C" {
#endif

#ifndef _SCE_SHARED_HEADER
#define _SCE_SHARED_HEADER

#define SCESTATUS_SUCCESS              0L
#define SCESTATUS_INVALID_PARAMETER    1L
#define SCESTATUS_RECORD_NOT_FOUND     2L
#define SCESTATUS_INVALID_DATA         3L
#define SCESTATUS_OBJECT_EXIST         4L
#define SCESTATUS_BUFFER_TOO_SMALL     5L
#define SCESTATUS_PROFILE_NOT_FOUND    6L
#define SCESTATUS_BAD_FORMAT           7L
#define SCESTATUS_NOT_ENOUGH_RESOURCE  8L
#define SCESTATUS_ACCESS_DENIED        9L
#define SCESTATUS_CANT_DELETE          10L
#define SCESTATUS_PREFIX_OVERFLOW      11L
#define SCESTATUS_OTHER_ERROR          12L
#define SCESTATUS_ALREADY_RUNNING      13L
#define SCESTATUS_SERVICE_NOT_SUPPORT  14L
#define SCESTATUS_MOD_NOT_FOUND        15L
#define SCESTATUS_EXCEPTION_IN_SERVER  16L
#define SCESTATUS_NO_TEMPLATE_GIVEN    17L
#define SCESTATUS_NO_MAPPING           18L
#define SCESTATUS_TRUST_FAIL           19L

typedef _Return_type_success_(return == SCESTATUS_SUCCESS) DWORD    SCESTATUS;

typedef struct _SCESVC_CONFIGURATION_LINE_ {

    LPTSTR  Key;
    LPTSTR  Value;
    DWORD   ValueLen; // number of bytes

} SCESVC_CONFIGURATION_LINE, *PSCESVC_CONFIGURATION_LINE;

typedef struct _SCESVC_CONFIGURATION_INFO_ {

    DWORD   Count;
    PSCESVC_CONFIGURATION_LINE Lines;

} SCESVC_CONFIGURATION_INFO, *PSCESVC_CONFIGURATION_INFO;

typedef PVOID SCE_HANDLE;
typedef ULONG SCE_ENUMERATION_CONTEXT, *PSCE_ENUMERATION_CONTEXT;

typedef enum _SCESVC_INFO_TYPE {

    SceSvcConfigurationInfo,
    SceSvcMergedPolicyInfo,
    SceSvcAnalysisInfo,
    SceSvcInternalUse                   // !!!do not use this type!!!

} SCESVC_INFO_TYPE;

#define SCE_ROOT_PATH TEXT("Software\\Microsoft\\Windows NT\\CurrentVersion\\SeCEdit")

#define SCE_ROOT_SERVICE_PATH   \
            SCE_ROOT_PATH TEXT("\\SvcEngs")

#endif

typedef PVOID SCESVC_HANDLE;

typedef struct _SCESVC_ANALYSIS_LINE_ {

    LPTSTR  Key;
    PBYTE   Value;
    DWORD   ValueLen; // number of bytes

} SCESVC_ANALYSIS_LINE, *PSCESVC_ANALYSIS_LINE;

typedef struct _SCESVC_ANALYSIS_INFO_ {

    DWORD   Count;
    PSCESVC_ANALYSIS_LINE Lines;

} SCESVC_ANALYSIS_INFO, *PSCESVC_ANALYSIS_INFO;


#define SCESVC_ENUMERATION_MAX  100L

typedef
SCESTATUS
(CALLBACK *PFSCE_QUERY_INFO)(
    IN SCE_HANDLE           sceHandle,
    IN SCESVC_INFO_TYPE     sceType,
    IN LPTSTR               lpPrefix OPTIONAL,
    IN BOOL                 bExact,
    OUT PVOID               *ppvInfo,
    OUT PSCE_ENUMERATION_CONTEXT psceEnumHandle
);


typedef
SCESTATUS
(CALLBACK *PFSCE_SET_INFO)(
    IN SCE_HANDLE           sceHandle,
    IN SCESVC_INFO_TYPE     sceType,
    IN LPTSTR               lpPrefix OPTIONAL,
    IN BOOL                 bExact,
    IN PVOID                pvInfo
);

typedef
SCESTATUS
(CALLBACK *PFSCE_FREE_INFO)(
    IN PVOID                pvServiceInfo
);

#define SCE_LOG_LEVEL_ALWAYS      0
#define SCE_LOG_LEVEL_ERROR       1
#define SCE_LOG_LEVEL_DETAIL      2
#define SCE_LOG_LEVEL_DEBUG       3

typedef
SCESTATUS
(CALLBACK *PFSCE_LOG_INFO)(
    IN INT ErrLevel,
    IN DWORD  Win32rc,
    IN LPTSTR pErrFmt,
    ...
);

typedef struct _SCESVC_CALLBACK_INFO_ {

    SCE_HANDLE              sceHandle;
    PFSCE_QUERY_INFO        pfQueryInfo;
    PFSCE_SET_INFO          pfSetInfo;
    PFSCE_FREE_INFO         pfFreeInfo;
    PFSCE_LOG_INFO          pfLogInfo;

} SCESVC_CALLBACK_INFO, *PSCESVC_CALLBACK_INFO;


typedef
SCESTATUS
(*PF_ConfigAnalyzeService)(
    IN PSCESVC_CALLBACK_INFO pSceCbInfo
);

typedef
SCESTATUS
(*PF_UpdateService)(
    IN PSCESVC_CALLBACK_INFO      pSceCbInfo,
    IN PSCESVC_CONFIGURATION_INFO ServiceInfo
);

#ifdef __cplusplus
}
#endif

#endif

//#if _WIN32_WINNT==0x0500
//
// headers for extension snap-ins
// only define this for NT5
//

#ifndef _UUIDS_SCE_ATTACHMENT_
#define _UUIDS_SCE_ATTACHMENT_

#include "rpc.h"
#include "rpcndr.h"

//#ifndef __RPCNDR_H_VERSION__
//#error this stub requires an updated version of <rpcndr.h>
//#endif // __RPCNDR_H_VERSION__

#if __RPCNDR_H_VERSION__ < 440
#define __RPCNDR_H_VERSION__ 440

#ifndef MIDL_INTERFACE
#define MIDL_INTERFACE(x) interface
#endif

#endif

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifdef __cplusplus
extern "C" {
#endif

#define struuidNodetypeSceTemplateServices      "{24a7f717-1f0c-11d1-affb-00c04fb984f9}"

#define lstruuidNodetypeSceTemplateServices      L"{24a7f717-1f0c-11d1-affb-00c04fb984f9}"


#define struuidNodetypeSceAnalysisServices      "{678050c7-1ff8-11d1-affb-00c04fb984f9}"

#define lstruuidNodetypeSceAnalysisServices      L"{678050c7-1ff8-11d1-affb-00c04fb984f9}"



#define struuidNodetypeSceEventLog      "{2ce06698-4bf3-11d1-8c30-00c04fb984f9}"
#define lstruuidNodetypeSceEventLog      L"{2ce06698-4bf3-11d1-8c30-00c04fb984f9}"

typedef PSCESVC_CONFIGURATION_INFO __RPC_FAR *LPSCESVC_CONFIGURATION_INFO;
typedef PSCESVC_ANALYSIS_INFO __RPC_FAR *LPSCESVC_ANALYSIS_INFO;

#define CCF_SCESVC_ATTACHMENT        L"CCF_SCESVC_ATTACHMENT"
#define CCF_SCESVC_ATTACHMENT_DATA   L"CCF_SCESVC_ATTACHMENT_DATA"

//
// implemented by service attachment
//
typedef interface ISceSvcAttachmentPersistInfo ISceSvcAttachmentPersistInfo;
typedef ISceSvcAttachmentPersistInfo __RPC_FAR *LPSCESVCATTACHMENTPERSISTINFO;

//extern const IID IID_ISceSvcAttachmentPersistInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("6d90e0d0-200d-11d1-affb-00c04fb984f9")

    ISceSvcAttachmentPersistInfo : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Save(
            /* [in] */  LPTSTR lpTemplateName,
            /* [out] */ SCESVC_HANDLE *scesvcHandle,
            /* [out] */ PVOID *ppvData,
            /* [out] */ PBOOL pbOverwriteAll ) = 0;

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsDirty(
            /* [in] */  LPTSTR lpTemplateName ) = 0;

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FreeBuffer(
            /* [in] */ PVOID pvData) = 0;

    };

#else

    // c style interface

    typedef struct ISceSvcAttachmentPersistInfoVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )(
            ISceSvcAttachmentPersistInfo __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);

        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )(
            ISceSvcAttachmentPersistInfo __RPC_FAR * This);

        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )(
            ISceSvcAttachmentPersistInfo __RPC_FAR * This);

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Save )(
            /* [in] */ ISceSvcAttachmentPersistInfo __RPC_FAR * This,
            /* [in] */ LPTSTR lpTemplateName,
            /* [in] */ SCESVC_HANDLE scesvcHandle,
            /* [out] */ PVOID *ppvData,
            /* [in out] */ PBOOL pbOverwriteAll );

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *FreeBuffer )(
            /* [in] */ ISceSvcAttachmentPersistInfo __RPC_FAR * This,
            /* [in] */ PVOID pvData);

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *IsDirty )(
            /* [in] */ ISceSvcAttachmentPersistInfo __RPC_FAR * This,
            /* [in] */ LPTSTR lpTemplateName );

        END_INTERFACE
    } ISceSvcAttachmentPersistInfoVtbl;

    interface ISceSvcAttachmentPersistInfo
    {
        CONST_VTBL struct ISceSvcAttachmentPersistInfoVtbl __RPC_FAR *lpVtbl;
    };



#ifdef COBJMACROS


#define IData_QueryInterface(This,riid,ppvObject)   \
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define ISceSvcAttachmentPersistInfo_AddRef(This)   \
    (This)->lpVtbl -> AddRef(This)

#define ISceSvcAttachmentPersistInfo_Release(This)  \
    (This)->lpVtbl -> Release(This)


#define ISceSvcAttachmentPersistInfo_Save(This, lpTemplateName, scesvcHandle, ppvData, pbOverwriteAll) \
    (This)->lpVtbl -> Save(lpTemplateName, scesvcHandle, ppvData, pbOverwriteAll)

#define ISceSvcAttachmentPersistInfo_FreeBuffer(This,pvData)    \
    (This)->lpVtbl -> FreeBuffer(pvData)

#define ISceSvcAttachmentPersistInfo_CloseHandle(This, lpTemplateName)  \
    (This)->lpVtbl -> IsDirty(lpTemplateName)

#endif /* COBJMACROS */

#endif

//
// implemented by SecEdit
//
typedef interface ISceSvcAttachmentData ISceSvcAttachmentData;
typedef ISceSvcAttachmentData __RPC_FAR *LPSCESVCATTACHMENTDATA;

//extern const IID IID_ISceSvcAttachmentData;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("17c35fde-200d-11d1-affb-00c04fb984f9")

    ISceSvcAttachmentData  : public IUnknown
    {
    public:

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetData(
            /* [in] */ SCESVC_HANDLE scesvcHandle,
            /* [in] */ SCESVC_INFO_TYPE sceType,
            /* [out] */ PVOID *ppvData,
            /* [in out] */ PSCE_ENUMERATION_CONTEXT psceEnumHandle ) = 0;

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize(
            /* [in] */ LPCTSTR lpServiceName,
            /* [in] */ LPCTSTR lpTemplateName,
            /* [in] */ LPSCESVCATTACHMENTPERSISTINFO lpSceSvcPersistInfo,
            /* [out] */ SCESVC_HANDLE *pscesvcHandle) = 0;

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FreeBuffer(
            /* [in] */ PVOID pvData) = 0;

        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CloseHandle(
            /* [in] */ SCESVC_HANDLE scesvcHandle) = 0;

    };
#else
    // c style interface

    typedef struct ISceSvcAttachmentDataVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )(
            ISceSvcAttachmentData __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);

        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )(
            ISceSvcAttachmentData __RPC_FAR * This);

        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )(
            ISceSvcAttachmentData __RPC_FAR * This);

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Initialize )(
            /* [in] */ ISceSvcAttachmentData __RPC_FAR * This,
            /* [in] */ LPCTSTR lpServiceName,
            /* [in] */ LPCTSTR lpTemplateName,
            /* [in] */ LPSCESVCATTACHMENTPERSISTINFO lpSceSvcPersistInfo,
            /* [out] */ SCESVC_HANDLE *pscesvcHandle);

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetData )(
            /* [in] */ ISceSvcAttachmentData __RPC_FAR * This,
            /* [in] */ SCESVC_HANDLE scesvcHandle,
            /* [in] */ SCESVC_INFO_TYPE sceType,
            /* [out] */ PVOID *ppvData,
            /* [in out] */ PSCE_ENUMERATION_CONTEXT psceEnumHandle );

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *FreeBuffer )(
            /* [in] */ ISceSvcAttachmentData __RPC_FAR * This,
            /* [in] */ PVOID pvData);

        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseHandle )(
            /* [in] */ ISceSvcAttachmentData __RPC_FAR * This,
            /* [in] */ SCESVC_HANDLE scesvcHandle);

        END_INTERFACE
    } ISceSvcAttachmentDataVtbl;

    interface ISceSvcAttachmentData
    {
        CONST_VTBL struct ISceSvcAttachmentDataVtbl __RPC_FAR *lpVtbl;
    };



#ifdef COBJMACROS


#define IData_QueryInterface(This,riid,ppvObject)   \
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define ISceSvcAttachmentData_AddRef(This)  \
    (This)->lpVtbl -> AddRef(This)

#define ISceSvcAttachmentData_Release(This) \
    (This)->lpVtbl -> Release(This)


#define ISceSvcAttachmentData_Initialize(This, lpServiceName, lpTemplateName, lpSceSvcPersistInfo, pscesvcHandle)    \
    (This)->lpVtbl -> Initialize(lpServiceName, lpTemplateName, lpSceSvcPersistInfo, pscesvcHandle)

#define ISceSvcAttachmentData_GetData(This, scesvcHandle, sceType, ppvData, psceEnumHandle) \
    (This)->lpVtbl -> GetData(scesvcHandle, sceType, ppvData, psceEnumHandle)

#define ISceSvcAttachmentData_FreeBuffer(This,pvData)   \
    (This)->lpVtbl -> FreeBuffer(pvData)

#define ISceSvcAttachmentData_CloseHandle(This, scesvcHandle)  \
    (This)->lpVtbl -> CloseHandle(scesvcHandle)

#endif /* COBJMACROS */


#endif


#ifdef __cplusplus
}
#endif

#endif // _UUIDS_COMPUTER_MANAGEMENT_

//#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

