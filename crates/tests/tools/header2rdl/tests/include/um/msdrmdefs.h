#pragma once
/*
//-----------------------------------------------------------------------------
//
// File: msdrmdefs.h
//
// Copyright (C) 2001-2004 Microsoft Corporation.  All Rights Reserved.
//
//-----------------------------------------------------------------------------
*/

#ifndef __MSDRMDEFS_H_
#define __MSDRMDEFS_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if !defined(__midl)
#ifndef _INC_WINDOWS
    #include <windows.h>
    #include <wtypes.h>
#endif // _INC_WINDOWS
#endif // __midl

typedef ULONG DRMHANDLE;
typedef ULONG DRMQUERYHANDLE;
typedef ULONG DRMENVHANDLE;

typedef ULONG DRMHSESSION;
typedef ULONG DRMPUBHANDLE;


#define DRMHANDLE_INVALID       0
#define DRMENVHANDLE_INVALID    0
#define DRMQUERYHANDLE_INVALID  0

#define DRMHSESSION_INVALID     0
#define DRMPUBHANDLE_INVALID    0


#define UDAPICALL __stdcall

#ifdef __cplusplus
#define DRMEXPORT extern "C"
#else
#define DRMEXPORT __declspec(dllexport)
#endif

#ifndef IN
    #define IN
#endif
#ifndef OUT
    #define OUT
#endif

const UINT DRMIDVERSION = 0;
typedef struct _DRMID {

    UINT  uVersion;
    PWSTR wszIDType;
    PWSTR wszID;

    #ifdef __cplusplus

    _DRMID() : uVersion(DRMIDVERSION),
               wszIDType(NULL),
               wszID(NULL)
    {
    }

    _DRMID(_In_opt_ PWSTR wszTypein,
           _In_opt_ PWSTR wszIDin) : uVersion(DRMIDVERSION),
                            wszIDType(wszTypein),
                            wszID(wszIDin)
    {
    }

    #endif

} DRMID;

// DRM flag sets

typedef enum _DRMTIMETYPE { DRMTIMETYPE_SYSTEMUTC,
                            DRMTIMETYPE_SYSTEMLOCAL }
              DRMTIMETYPE;

typedef enum _DRMENCODINGTYPE { DRMENCODINGTYPE_BASE64,
                                DRMENCODINGTYPE_STRING,
                                DRMENCODINGTYPE_LONG,
                                DRMENCODINGTYPE_TIME,
                                DRMENCODINGTYPE_UINT,
                                DRMENCODINGTYPE_RAW }
              DRMENCODINGTYPE;

typedef enum _DRMATTESTTYPE { DRMATTESTTYPE_FULLENVIRONMENT,
                              DRMATTESTTYPE_HASHONLY }
              DRMATTESTTYPE;

typedef enum _DRMSPECTYPE { DRMSPECTYPE_UNKNOWN,
                            DRMSPECTYPE_FILENAME }
              DRMSPECTYPE;

// security provider types
typedef enum _DRMSECURITYPROVIDERTYPE { DRMSECURITYPROVIDERTYPE_SOFTWARESECREP }
              DRMSECURITYPROVIDERTYPE;

typedef enum _DRMGLOBALOPTIONS  { DRMGLOBALOPTIONS_USE_WINHTTP,
                                  DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR }
              DRMGLOBALOPTIONS;

const UINT DRMBOUNDLICENSEPARAMSVERSION = 1;
typedef struct _DRMBOUNDLICENSEPARAMS {    

    UINT        uVersion;
    DRMHANDLE   hEnablingPrincipal;
    DRMHANDLE   hSecureStore;          // reserved for future, must be NULL for now
    PWSTR       wszRightsRequested;
    PWSTR       wszRightsGroup;
    DRMID       idResource;
    UINT        cAuthenticatorCount;   // reserved for future, must be 0 for now
    DRMHANDLE*  rghAuthenticators;     // reserved for future, must be NULL for now
    PWSTR       wszDefaultEnablingPrincipalCredentials;    

    DWORD       dwFlags;

#ifdef __cplusplus
    _DRMBOUNDLICENSEPARAMS() : uVersion(DRMBOUNDLICENSEPARAMSVERSION),
                               hEnablingPrincipal(NULL),
                               hSecureStore(NULL),
                               wszRightsRequested(NULL),
                               wszRightsGroup(NULL),
                               cAuthenticatorCount(0),
                               rghAuthenticators(NULL),
                               wszDefaultEnablingPrincipalCredentials(NULL),
                               idResource(NULL,NULL),
                               dwFlags(0)
    {
    }
#endif // #ifdef __cplusplus

} DRMBOUNDLICENSEPARAMS;

// flags for the dwFlags parameter of the binding parameters structure
const DWORD DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS = 0x1;



const UINT DRMLICENSEACQDATAVERSION = 0;
typedef struct _DRM_LICENSE_ACQ_DATA
{    
    UINT    uVersion;
    PWSTR   wszURL;
    PWSTR   wszLocalFilename;
    BYTE*   pbPostData;
    DWORD   dwPostDataSize;
    PWSTR   wszFriendlyName;

#ifdef __cplusplus
    _DRM_LICENSE_ACQ_DATA() : uVersion(DRMLICENSEACQDATAVERSION),
                              wszURL(NULL),
                              wszLocalFilename(NULL),
                              pbPostData(NULL),
                              dwPostDataSize(NULL),
                              wszFriendlyName(NULL)
    {
    }
#endif // #ifdef __cplusplus
} DRM_LICENSE_ACQ_DATA;

const UINT DRMACTSERVINFOVERSION = 0;
typedef struct _DRM_ACTSERV_INFO
{    
    UINT    uVersion;
    PWSTR   wszPubKey;
    PWSTR   wszURL;

#ifdef __cplusplus
    _DRM_ACTSERV_INFO() : uVersion(DRMACTSERVINFOVERSION),
                          wszPubKey(NULL),
                          wszURL(NULL)
    {
    }
#endif // #ifdef __cplusplus
} DRM_ACTSERV_INFO;

const UINT DRMCLIENTSTRUCTVERSION = 1;
typedef struct _DRM_CLIENT_VERSION_INFO {    

    UINT        uStructVersion;
    DWORD       dwVersion[4];
    WCHAR       wszHierarchy[256];
    WCHAR       wszProductId[256];
    WCHAR       wszProductDescription[256];

#ifdef __cplusplus
    _DRM_CLIENT_VERSION_INFO() : uStructVersion(DRMCLIENTSTRUCTVERSION)
    {
        dwVersion[0] = 0;
        dwVersion[1] = 0;
        dwVersion[2] = 0;
        dwVersion[3] = 0;
        wszHierarchy[0] = 0;
        wszProductId[0] = 0;
        wszProductDescription[0] = 0;
    }
#endif // #ifdef __cplusplus

} DRM_CLIENT_VERSION_INFO;

typedef enum _DRM_STATUS_MSG
{
    DRM_MSG_ACTIVATE_MACHINE,
    DRM_MSG_ACTIVATE_GROUPIDENTITY,
    DRM_MSG_ACQUIRE_LICENSE,
    DRM_MSG_ACQUIRE_ADVISORY,
    DRM_MSG_SIGN_ISSUANCE_LICENSE,
    DRM_MSG_ACQUIRE_CLIENTLICENSOR,
    DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE
}DRM_STATUS_MSG;

typedef enum _DRM_USAGEPOLICY_TYPE
{
    DRM_USAGEPOLICY_TYPE_BYNAME,
    DRM_USAGEPOLICY_TYPE_BYPUBLICKEY,
    DRM_USAGEPOLICY_TYPE_BYDIGEST,
    DRM_USAGEPOLICY_TYPE_OSEXCLUSION
}DRM_USAGEPOLICY_TYPE;


// DRM_ACQUIRE_LICENSE_FLAGS

#define DRM_AL_NONSILENT        0x01    // Acquire non-silently
#define DRM_AL_NOPERSIST        0x02    // Don't persist the license
#define DRM_AL_CANCEL           0x04    // Cancel previous request
#define DRM_AL_FETCHNOADVISORY  0x08    // Don't acquire advisories
#define DRM_AL_NOUI             0x10    // Don't display any Authentication UI


// DRM_ACTIVATION_FLAGS

#define DRM_ACTIVATE_MACHINE                0x01    // Activate machine
#define DRM_ACTIVATE_GROUPIDENTITY          0x02    // Activate Group Identity
#define DRM_ACTIVATE_TEMPORARY              0x04    // Temporary certificate
#define DRM_ACTIVATE_CANCEL                 0x08    // Cancel previous request
#define DRM_ACTIVATE_SILENT                 0x10    // Silent Activation
#define DRM_ACTIVATE_SHARED_GROUPIDENTITY   0x20    // Shared Group Identity certificate
#define DRM_ACTIVATE_DELAYED                0x40    // Delayed activation


//  DRM_ENUMERATE_LICENSE_FLAGS

#define DRM_EL_MACHINE                      0x0001
#define DRM_EL_GROUPIDENTITY                0x0002
#define DRM_EL_GROUPIDENTITY_NAME           0x0004
#define DRM_EL_GROUPIDENTITY_LID            0x0008
#define DRM_EL_SPECIFIED_GROUPIDENTITY      0x0010
#define DRM_EL_EUL                          0x0020
#define DRM_EL_EUL_LID                      0x0040
#define DRM_EL_CLIENTLICENSOR               0x0080
#define DRM_EL_CLIENTLICENSOR_LID           0x0100
#define DRM_EL_SPECIFIED_CLIENTLICENSOR     0x0200
#define DRM_EL_REVOCATIONLIST               0x0400
#define DRM_EL_REVOCATIONLIST_LID           0x0800
#define DRM_EL_EXPIRED                      0x1000
#define DRM_EL_ISSUERNAME                   0x2000
#define DRM_EL_ISSUANCELICENSE_TEMPLATE     0x4000
#define DRM_EL_ISSUANCELICENSE_TEMPLATE_LID 0x8000


//DRM_ADD_LICENSE_FLAGS
#define DRM_ADD_LICENSE_NOPERSIST           0x00
#define DRM_ADD_LICENSE_PERSIST             0x01


//DRM_SERVICE_TYPE
#define DRM_SERVICE_TYPE_ACTIVATION         0x01
#define DRM_SERVICE_TYPE_CERTIFICATION      0x02
#define DRM_SERVICE_TYPE_PUBLISHING         0x04
#define DRM_SERVICE_TYPE_CLIENTLICENSOR     0x08
#define DRM_SERVICE_TYPE_SILENT             0x10

//DRM_SERVICE_LOCATION
#define DRM_SERVICE_LOCATION_INTERNET       0x01
#define DRM_SERVICE_LOCATION_ENTERPRISE     0x02

//GROUPID Provider Type
#define DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH L"WindowsAuthProvider"
#define DRM_DEFAULTGROUPIDTYPE_PASSPORT    L"PassportAuthProvider"

//ISSUANCE LICENSE SIGN
#define DRM_SIGN_ONLINE                0x01
#define DRM_SIGN_OFFLINE               0x02
#define DRM_SIGN_CANCEL                0x04
#define DRM_SERVER_ISSUANCELICENSE     0x08
#define DRM_AUTO_GENERATE_KEY          0x10
#define DRM_OWNER_LICENSE_NOPERSIST    0x20
#define DRM_REUSE_KEY                  0x40

// LOCKBOX TYPE
#define DRM_LOCKBOXTYPE_NONE        0x00
#define DRM_LOCKBOXTYPE_WHITEBOX    0x01
#define DRM_LOCKBOXTYPE_BLACKBOX    0x02
#define DRM_LOCKBOXTYPE_DEFAULT     DRM_LOCKBOXTYPE_BLACKBOX

// Template Distribution

#define DRM_AILT_NONSILENT          0x01
#define DRM_AILT_OBTAIN_ALL         0x02
#define DRM_AILT_CANCEL             0x04


typedef enum _DRM_DISTRIBUTION_POINT_INFO
{
    DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION,
    DRM_DISTRIBUTION_POINT_PUBLISHING,
    DRM_DISTRIBUTION_POINT_REFERRAL_INFO
}DRM_DISTRIBUTION_POINT_INFO;



//    DRM Callback
const UINT DRMCALLBACKVERSION = 1;
typedef HRESULT (__stdcall *DRMCALLBACK)(DRM_STATUS_MSG,HRESULT,void*,void*);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // #ifndef __MSDRMDEFS_H_
