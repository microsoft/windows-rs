

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __mswmdm_h__
#define __mswmdm_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IWMDMMetaData_FWD_DEFINED__
#define __IWMDMMetaData_FWD_DEFINED__
typedef interface IWMDMMetaData IWMDMMetaData;

#endif 	/* __IWMDMMetaData_FWD_DEFINED__ */


#ifndef __IWMDeviceManager_FWD_DEFINED__
#define __IWMDeviceManager_FWD_DEFINED__
typedef interface IWMDeviceManager IWMDeviceManager;

#endif 	/* __IWMDeviceManager_FWD_DEFINED__ */


#ifndef __IWMDeviceManager2_FWD_DEFINED__
#define __IWMDeviceManager2_FWD_DEFINED__
typedef interface IWMDeviceManager2 IWMDeviceManager2;

#endif 	/* __IWMDeviceManager2_FWD_DEFINED__ */


#ifndef __IWMDeviceManager3_FWD_DEFINED__
#define __IWMDeviceManager3_FWD_DEFINED__
typedef interface IWMDeviceManager3 IWMDeviceManager3;

#endif 	/* __IWMDeviceManager3_FWD_DEFINED__ */


#ifndef __IWMDMStorageGlobals_FWD_DEFINED__
#define __IWMDMStorageGlobals_FWD_DEFINED__
typedef interface IWMDMStorageGlobals IWMDMStorageGlobals;

#endif 	/* __IWMDMStorageGlobals_FWD_DEFINED__ */


#ifndef __IWMDMStorage_FWD_DEFINED__
#define __IWMDMStorage_FWD_DEFINED__
typedef interface IWMDMStorage IWMDMStorage;

#endif 	/* __IWMDMStorage_FWD_DEFINED__ */


#ifndef __IWMDMStorage2_FWD_DEFINED__
#define __IWMDMStorage2_FWD_DEFINED__
typedef interface IWMDMStorage2 IWMDMStorage2;

#endif 	/* __IWMDMStorage2_FWD_DEFINED__ */


#ifndef __IWMDMStorage3_FWD_DEFINED__
#define __IWMDMStorage3_FWD_DEFINED__
typedef interface IWMDMStorage3 IWMDMStorage3;

#endif 	/* __IWMDMStorage3_FWD_DEFINED__ */


#ifndef __IWMDMStorage4_FWD_DEFINED__
#define __IWMDMStorage4_FWD_DEFINED__
typedef interface IWMDMStorage4 IWMDMStorage4;

#endif 	/* __IWMDMStorage4_FWD_DEFINED__ */


#ifndef __IWMDMOperation_FWD_DEFINED__
#define __IWMDMOperation_FWD_DEFINED__
typedef interface IWMDMOperation IWMDMOperation;

#endif 	/* __IWMDMOperation_FWD_DEFINED__ */


#ifndef __IWMDMOperation2_FWD_DEFINED__
#define __IWMDMOperation2_FWD_DEFINED__
typedef interface IWMDMOperation2 IWMDMOperation2;

#endif 	/* __IWMDMOperation2_FWD_DEFINED__ */


#ifndef __IWMDMOperation3_FWD_DEFINED__
#define __IWMDMOperation3_FWD_DEFINED__
typedef interface IWMDMOperation3 IWMDMOperation3;

#endif 	/* __IWMDMOperation3_FWD_DEFINED__ */


#ifndef __IWMDMProgress_FWD_DEFINED__
#define __IWMDMProgress_FWD_DEFINED__
typedef interface IWMDMProgress IWMDMProgress;

#endif 	/* __IWMDMProgress_FWD_DEFINED__ */


#ifndef __IWMDMProgress2_FWD_DEFINED__
#define __IWMDMProgress2_FWD_DEFINED__
typedef interface IWMDMProgress2 IWMDMProgress2;

#endif 	/* __IWMDMProgress2_FWD_DEFINED__ */


#ifndef __IWMDMProgress3_FWD_DEFINED__
#define __IWMDMProgress3_FWD_DEFINED__
typedef interface IWMDMProgress3 IWMDMProgress3;

#endif 	/* __IWMDMProgress3_FWD_DEFINED__ */


#ifndef __IWMDMDevice_FWD_DEFINED__
#define __IWMDMDevice_FWD_DEFINED__
typedef interface IWMDMDevice IWMDMDevice;

#endif 	/* __IWMDMDevice_FWD_DEFINED__ */


#ifndef __IWMDMDevice2_FWD_DEFINED__
#define __IWMDMDevice2_FWD_DEFINED__
typedef interface IWMDMDevice2 IWMDMDevice2;

#endif 	/* __IWMDMDevice2_FWD_DEFINED__ */


#ifndef __IWMDMDevice3_FWD_DEFINED__
#define __IWMDMDevice3_FWD_DEFINED__
typedef interface IWMDMDevice3 IWMDMDevice3;

#endif 	/* __IWMDMDevice3_FWD_DEFINED__ */


#ifndef __IWMDMDeviceSession_FWD_DEFINED__
#define __IWMDMDeviceSession_FWD_DEFINED__
typedef interface IWMDMDeviceSession IWMDMDeviceSession;

#endif 	/* __IWMDMDeviceSession_FWD_DEFINED__ */


#ifndef __IWMDMEnumDevice_FWD_DEFINED__
#define __IWMDMEnumDevice_FWD_DEFINED__
typedef interface IWMDMEnumDevice IWMDMEnumDevice;

#endif 	/* __IWMDMEnumDevice_FWD_DEFINED__ */


#ifndef __IWMDMDeviceControl_FWD_DEFINED__
#define __IWMDMDeviceControl_FWD_DEFINED__
typedef interface IWMDMDeviceControl IWMDMDeviceControl;

#endif 	/* __IWMDMDeviceControl_FWD_DEFINED__ */


#ifndef __IWMDMEnumStorage_FWD_DEFINED__
#define __IWMDMEnumStorage_FWD_DEFINED__
typedef interface IWMDMEnumStorage IWMDMEnumStorage;

#endif 	/* __IWMDMEnumStorage_FWD_DEFINED__ */


#ifndef __IWMDMStorageControl_FWD_DEFINED__
#define __IWMDMStorageControl_FWD_DEFINED__
typedef interface IWMDMStorageControl IWMDMStorageControl;

#endif 	/* __IWMDMStorageControl_FWD_DEFINED__ */


#ifndef __IWMDMStorageControl2_FWD_DEFINED__
#define __IWMDMStorageControl2_FWD_DEFINED__
typedef interface IWMDMStorageControl2 IWMDMStorageControl2;

#endif 	/* __IWMDMStorageControl2_FWD_DEFINED__ */


#ifndef __IWMDMStorageControl3_FWD_DEFINED__
#define __IWMDMStorageControl3_FWD_DEFINED__
typedef interface IWMDMStorageControl3 IWMDMStorageControl3;

#endif 	/* __IWMDMStorageControl3_FWD_DEFINED__ */


#ifndef __IWMDMObjectInfo_FWD_DEFINED__
#define __IWMDMObjectInfo_FWD_DEFINED__
typedef interface IWMDMObjectInfo IWMDMObjectInfo;

#endif 	/* __IWMDMObjectInfo_FWD_DEFINED__ */


#ifndef __IWMDMRevoked_FWD_DEFINED__
#define __IWMDMRevoked_FWD_DEFINED__
typedef interface IWMDMRevoked IWMDMRevoked;

#endif 	/* __IWMDMRevoked_FWD_DEFINED__ */


#ifndef __IWMDMNotification_FWD_DEFINED__
#define __IWMDMNotification_FWD_DEFINED__
typedef interface IWMDMNotification IWMDMNotification;

#endif 	/* __IWMDMNotification_FWD_DEFINED__ */


#ifndef __IMDServiceProvider_FWD_DEFINED__
#define __IMDServiceProvider_FWD_DEFINED__
typedef interface IMDServiceProvider IMDServiceProvider;

#endif 	/* __IMDServiceProvider_FWD_DEFINED__ */


#ifndef __IMDServiceProvider2_FWD_DEFINED__
#define __IMDServiceProvider2_FWD_DEFINED__
typedef interface IMDServiceProvider2 IMDServiceProvider2;

#endif 	/* __IMDServiceProvider2_FWD_DEFINED__ */


#ifndef __IMDServiceProvider3_FWD_DEFINED__
#define __IMDServiceProvider3_FWD_DEFINED__
typedef interface IMDServiceProvider3 IMDServiceProvider3;

#endif 	/* __IMDServiceProvider3_FWD_DEFINED__ */


#ifndef __IMDSPEnumDevice_FWD_DEFINED__
#define __IMDSPEnumDevice_FWD_DEFINED__
typedef interface IMDSPEnumDevice IMDSPEnumDevice;

#endif 	/* __IMDSPEnumDevice_FWD_DEFINED__ */


#ifndef __IMDSPDevice_FWD_DEFINED__
#define __IMDSPDevice_FWD_DEFINED__
typedef interface IMDSPDevice IMDSPDevice;

#endif 	/* __IMDSPDevice_FWD_DEFINED__ */


#ifndef __IMDSPDevice2_FWD_DEFINED__
#define __IMDSPDevice2_FWD_DEFINED__
typedef interface IMDSPDevice2 IMDSPDevice2;

#endif 	/* __IMDSPDevice2_FWD_DEFINED__ */


#ifndef __IMDSPDevice3_FWD_DEFINED__
#define __IMDSPDevice3_FWD_DEFINED__
typedef interface IMDSPDevice3 IMDSPDevice3;

#endif 	/* __IMDSPDevice3_FWD_DEFINED__ */


#ifndef __IMDSPDeviceControl_FWD_DEFINED__
#define __IMDSPDeviceControl_FWD_DEFINED__
typedef interface IMDSPDeviceControl IMDSPDeviceControl;

#endif 	/* __IMDSPDeviceControl_FWD_DEFINED__ */


#ifndef __IMDSPEnumStorage_FWD_DEFINED__
#define __IMDSPEnumStorage_FWD_DEFINED__
typedef interface IMDSPEnumStorage IMDSPEnumStorage;

#endif 	/* __IMDSPEnumStorage_FWD_DEFINED__ */


#ifndef __IMDSPStorage_FWD_DEFINED__
#define __IMDSPStorage_FWD_DEFINED__
typedef interface IMDSPStorage IMDSPStorage;

#endif 	/* __IMDSPStorage_FWD_DEFINED__ */


#ifndef __IMDSPStorage2_FWD_DEFINED__
#define __IMDSPStorage2_FWD_DEFINED__
typedef interface IMDSPStorage2 IMDSPStorage2;

#endif 	/* __IMDSPStorage2_FWD_DEFINED__ */


#ifndef __IMDSPStorage3_FWD_DEFINED__
#define __IMDSPStorage3_FWD_DEFINED__
typedef interface IMDSPStorage3 IMDSPStorage3;

#endif 	/* __IMDSPStorage3_FWD_DEFINED__ */


#ifndef __IMDSPStorage4_FWD_DEFINED__
#define __IMDSPStorage4_FWD_DEFINED__
typedef interface IMDSPStorage4 IMDSPStorage4;

#endif 	/* __IMDSPStorage4_FWD_DEFINED__ */


#ifndef __IMDSPStorageGlobals_FWD_DEFINED__
#define __IMDSPStorageGlobals_FWD_DEFINED__
typedef interface IMDSPStorageGlobals IMDSPStorageGlobals;

#endif 	/* __IMDSPStorageGlobals_FWD_DEFINED__ */


#ifndef __IMDSPObjectInfo_FWD_DEFINED__
#define __IMDSPObjectInfo_FWD_DEFINED__
typedef interface IMDSPObjectInfo IMDSPObjectInfo;

#endif 	/* __IMDSPObjectInfo_FWD_DEFINED__ */


#ifndef __IMDSPObject_FWD_DEFINED__
#define __IMDSPObject_FWD_DEFINED__
typedef interface IMDSPObject IMDSPObject;

#endif 	/* __IMDSPObject_FWD_DEFINED__ */


#ifndef __IMDSPObject2_FWD_DEFINED__
#define __IMDSPObject2_FWD_DEFINED__
typedef interface IMDSPObject2 IMDSPObject2;

#endif 	/* __IMDSPObject2_FWD_DEFINED__ */


#ifndef __IMDSPDirectTransfer_FWD_DEFINED__
#define __IMDSPDirectTransfer_FWD_DEFINED__
typedef interface IMDSPDirectTransfer IMDSPDirectTransfer;

#endif 	/* __IMDSPDirectTransfer_FWD_DEFINED__ */


#ifndef __IMDSPRevoked_FWD_DEFINED__
#define __IMDSPRevoked_FWD_DEFINED__
typedef interface IMDSPRevoked IMDSPRevoked;

#endif 	/* __IMDSPRevoked_FWD_DEFINED__ */


#ifndef __ISCPSecureAuthenticate_FWD_DEFINED__
#define __ISCPSecureAuthenticate_FWD_DEFINED__
typedef interface ISCPSecureAuthenticate ISCPSecureAuthenticate;

#endif 	/* __ISCPSecureAuthenticate_FWD_DEFINED__ */


#ifndef __ISCPSecureAuthenticate2_FWD_DEFINED__
#define __ISCPSecureAuthenticate2_FWD_DEFINED__
typedef interface ISCPSecureAuthenticate2 ISCPSecureAuthenticate2;

#endif 	/* __ISCPSecureAuthenticate2_FWD_DEFINED__ */


#ifndef __ISCPSecureQuery_FWD_DEFINED__
#define __ISCPSecureQuery_FWD_DEFINED__
typedef interface ISCPSecureQuery ISCPSecureQuery;

#endif 	/* __ISCPSecureQuery_FWD_DEFINED__ */


#ifndef __ISCPSecureQuery2_FWD_DEFINED__
#define __ISCPSecureQuery2_FWD_DEFINED__
typedef interface ISCPSecureQuery2 ISCPSecureQuery2;

#endif 	/* __ISCPSecureQuery2_FWD_DEFINED__ */


#ifndef __ISCPSecureExchange_FWD_DEFINED__
#define __ISCPSecureExchange_FWD_DEFINED__
typedef interface ISCPSecureExchange ISCPSecureExchange;

#endif 	/* __ISCPSecureExchange_FWD_DEFINED__ */


#ifndef __ISCPSecureExchange2_FWD_DEFINED__
#define __ISCPSecureExchange2_FWD_DEFINED__
typedef interface ISCPSecureExchange2 ISCPSecureExchange2;

#endif 	/* __ISCPSecureExchange2_FWD_DEFINED__ */


#ifndef __ISCPSecureExchange3_FWD_DEFINED__
#define __ISCPSecureExchange3_FWD_DEFINED__
typedef interface ISCPSecureExchange3 ISCPSecureExchange3;

#endif 	/* __ISCPSecureExchange3_FWD_DEFINED__ */


#ifndef __ISCPSession_FWD_DEFINED__
#define __ISCPSession_FWD_DEFINED__
typedef interface ISCPSession ISCPSession;

#endif 	/* __ISCPSession_FWD_DEFINED__ */


#ifndef __ISCPSecureQuery3_FWD_DEFINED__
#define __ISCPSecureQuery3_FWD_DEFINED__
typedef interface ISCPSecureQuery3 ISCPSecureQuery3;

#endif 	/* __ISCPSecureQuery3_FWD_DEFINED__ */


#ifndef __IComponentAuthenticate_FWD_DEFINED__
#define __IComponentAuthenticate_FWD_DEFINED__
typedef interface IComponentAuthenticate IComponentAuthenticate;

#endif 	/* __IComponentAuthenticate_FWD_DEFINED__ */


#ifndef __MediaDevMgrClassFactory_FWD_DEFINED__
#define __MediaDevMgrClassFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class MediaDevMgrClassFactory MediaDevMgrClassFactory;
#else
typedef struct MediaDevMgrClassFactory MediaDevMgrClassFactory;
#endif /* __cplusplus */

#endif 	/* __MediaDevMgrClassFactory_FWD_DEFINED__ */


#ifndef __MediaDevMgr_FWD_DEFINED__
#define __MediaDevMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class MediaDevMgr MediaDevMgr;
#else
typedef struct MediaDevMgr MediaDevMgr;
#endif /* __cplusplus */

#endif 	/* __MediaDevMgr_FWD_DEFINED__ */


#ifndef __WMDMDevice_FWD_DEFINED__
#define __WMDMDevice_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMDMDevice WMDMDevice;
#else
typedef struct WMDMDevice WMDMDevice;
#endif /* __cplusplus */

#endif 	/* __WMDMDevice_FWD_DEFINED__ */


#ifndef __WMDMStorage_FWD_DEFINED__
#define __WMDMStorage_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMDMStorage WMDMStorage;
#else
typedef struct WMDMStorage WMDMStorage;
#endif /* __cplusplus */

#endif 	/* __WMDMStorage_FWD_DEFINED__ */


#ifndef __WMDMStorageGlobal_FWD_DEFINED__
#define __WMDMStorageGlobal_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMDMStorageGlobal WMDMStorageGlobal;
#else
typedef struct WMDMStorageGlobal WMDMStorageGlobal;
#endif /* __cplusplus */

#endif 	/* __WMDMStorageGlobal_FWD_DEFINED__ */


#ifndef __WMDMDeviceEnum_FWD_DEFINED__
#define __WMDMDeviceEnum_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMDMDeviceEnum WMDMDeviceEnum;
#else
typedef struct WMDMDeviceEnum WMDMDeviceEnum;
#endif /* __cplusplus */

#endif 	/* __WMDMDeviceEnum_FWD_DEFINED__ */


#ifndef __WMDMStorageEnum_FWD_DEFINED__
#define __WMDMStorageEnum_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMDMStorageEnum WMDMStorageEnum;
#else
typedef struct WMDMStorageEnum WMDMStorageEnum;
#endif /* __cplusplus */

#endif 	/* __WMDMStorageEnum_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mswmdm_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _DEFINE_WMDM_DEVICE_PROTOCOL_MTP
#define _DEFINE_WMDM_DEVICE_PROTOCOL_MTP
// {979E54E5-0AFC-4604-8D93-DC798A4BCF45}
DEFINE_GUID(WMDM_DEVICE_PROTOCOL_MTP,
0x979e54e5, 0xafc, 0x4604, 0x8d, 0x93, 0xdc, 0x79, 0x8a, 0x4b, 0xcf, 0x45);
#endif
#ifndef _DEFINE_WMDM_DEVICE_PROTOCOL_RAPI
#define _DEFINE_WMDM_DEVICE_PROTOCOL_RAPI
// {2A11ED91-8C8F-41e4-82D1-8386E003561C}
DEFINE_GUID(WMDM_DEVICE_PROTOCOL_RAPI,
0x2a11ed91, 0x8c8f, 0x41e4, 0x82, 0xd1, 0x83, 0x86, 0xe0, 0x3, 0x56, 0x1c);
#endif
#ifndef _DEFINE_WMDM_DEVICE_PROTOCOL_MSC
#define _DEFINE_WMDM_DEVICE_PROTOCOL_MSC
// {A4D2C26C-A881-44bb-BD5D-1F703C71F7A9}
DEFINE_GUID(WMDM_DEVICE_PROTOCOL_MSC,
0xa4d2c26c, 0xa881, 0x44bb, 0xbd, 0x5d, 0x1f, 0x70, 0x3c, 0x71, 0xf7, 0xa9);
#endif
#ifndef _DEFINE_WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT
#define _DEFINE_WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT
// {7DE8686D-78EE-43ea-A496-C625AC91CC5D}
DEFINE_GUID(WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT,
0x7de8686d, 0x78ee, 0x43ea, 0xa4, 0x96, 0xc6, 0x25, 0xac, 0x91, 0xcc, 0x5d);
#endif
typedef 
enum tagWMDM_TAG_DATATYPE
    {
        WMDM_TYPE_DWORD	= 0,
        WMDM_TYPE_STRING	= 1,
        WMDM_TYPE_BINARY	= 2,
        WMDM_TYPE_BOOL	= 3,
        WMDM_TYPE_QWORD	= 4,
        WMDM_TYPE_WORD	= 5,
        WMDM_TYPE_GUID	= 6,
        WMDM_TYPE_DATE	= 7
    } 	WMDM_TAG_DATATYPE;

typedef 
enum tagWMDM_SESSION_TYPE
    {
        WMDM_SESSION_NONE	= 0,
        WMDM_SESSION_TRANSFER_TO_DEVICE	= 0x1,
        WMDM_SESSION_TRANSFER_FROM_DEVICE	= 0x10,
        WMDM_SESSION_DELETE	= 0x100,
        WMDM_SESSION_CUSTOM	= 0x1000
    } 	WMDM_SESSION_TYPE;

typedef struct _tWAVEFORMATEX
    {
    WORD wFormatTag;
    WORD nChannels;
    DWORD nSamplesPerSec;
    DWORD nAvgBytesPerSec;
    WORD nBlockAlign;
    WORD wBitsPerSample;
    WORD cbSize;
    } 	_WAVEFORMATEX;

typedef struct _tagBITMAPINFOHEADER
    {
    DWORD biSize;
    LONG biWidth;
    LONG biHeight;
    WORD biPlanes;
    WORD biBitCount;
    DWORD biCompression;
    DWORD biSizeImage;
    LONG biXPelsPerMeter;
    LONG biYPelsPerMeter;
    DWORD biClrUsed;
    DWORD biClrImportant;
    } 	_BITMAPINFOHEADER;

typedef struct _tagVIDEOINFOHEADER
    {
    RECT rcSource;
    RECT rcTarget;
    DWORD dwBitRate;
    DWORD dwBitErrorRate;
    LONGLONG AvgTimePerFrame;
    _BITMAPINFOHEADER bmiHeader;
    } 	_VIDEOINFOHEADER;

typedef struct _tagWMFILECAPABILITIES
    {
    LPWSTR pwszMimeType;
    DWORD dwReserved;
    } 	WMFILECAPABILITIES;

typedef struct __OPAQUECOMMAND
    {
    GUID guidCommand;
    DWORD dwDataLen;
    /* [size_is] */ BYTE *pData;
    BYTE abMAC[ 20 ];
    } 	OPAQUECOMMAND;

#define	WMDMID_LENGTH	( 128 )

typedef struct __WMDMID
    {
    UINT cbSize;
    DWORD dwVendorID;
    BYTE pID[ 128 ];
    UINT SerialNumberLength;
    } 	WMDMID;

typedef struct __WMDMID *PWMDMID;

typedef struct _WMDMDATETIME
    {
    WORD wYear;
    WORD wMonth;
    WORD wDay;
    WORD wHour;
    WORD wMinute;
    WORD wSecond;
    } 	WMDMDATETIME;

typedef struct _WMDMDATETIME *PWMDMDATETIME;

typedef struct __WMDMRIGHTS
    {
    UINT cbSize;
    DWORD dwContentType;
    DWORD fuFlags;
    DWORD fuRights;
    DWORD dwAppSec;
    DWORD dwPlaybackCount;
    WMDMDATETIME ExpirationDate;
    } 	WMDMRIGHTS;

typedef struct __WMDMRIGHTS *PWMDMRIGHTS;

typedef struct __WMDMMetadataView
    {
    /* [string] */ WCHAR *pwszViewName;
    UINT nDepth;
    /* [size_is][string] */ WCHAR **ppwszTags;
    } 	WMDMMetadataView;

typedef 
enum tagWMDM_STORAGE_ENUM_MODE
    {
        ENUM_MODE_RAW	= 0,
        ENUM_MODE_USE_DEVICE_PREF	= ( ENUM_MODE_RAW + 1 ) ,
        ENUM_MODE_METADATA_VIEWS	= ( ENUM_MODE_USE_DEVICE_PREF + 1 ) 
    } 	WMDM_STORAGE_ENUM_MODE;

typedef /* [v1_enum] */ 
enum tagWMDM_FORMATCODE
    {
        WMDM_FORMATCODE_NOTUSED	= 0,
        WMDM_FORMATCODE_ALLIMAGES	= 0xffffffff,
        WMDM_FORMATCODE_UNDEFINED	= 0x3000,
        WMDM_FORMATCODE_ASSOCIATION	= 0x3001,
        WMDM_FORMATCODE_SCRIPT	= 0x3002,
        WMDM_FORMATCODE_EXECUTABLE	= 0x3003,
        WMDM_FORMATCODE_TEXT	= 0x3004,
        WMDM_FORMATCODE_HTML	= 0x3005,
        WMDM_FORMATCODE_DPOF	= 0x3006,
        WMDM_FORMATCODE_AIFF	= 0x3007,
        WMDM_FORMATCODE_WAVE	= 0x3008,
        WMDM_FORMATCODE_MP3	= 0x3009,
        WMDM_FORMATCODE_AVI	= 0x300a,
        WMDM_FORMATCODE_MPEG	= 0x300b,
        WMDM_FORMATCODE_ASF	= 0x300c,
        WMDM_FORMATCODE_RESERVED_FIRST	= 0x300d,
        WMDM_FORMATCODE_RESERVED_LAST	= 0x37ff,
        WMDM_FORMATCODE_IMAGE_UNDEFINED	= 0x3800,
        WMDM_FORMATCODE_IMAGE_EXIF	= 0x3801,
        WMDM_FORMATCODE_IMAGE_TIFFEP	= 0x3802,
        WMDM_FORMATCODE_IMAGE_FLASHPIX	= 0x3803,
        WMDM_FORMATCODE_IMAGE_BMP	= 0x3804,
        WMDM_FORMATCODE_IMAGE_CIFF	= 0x3805,
        WMDM_FORMATCODE_IMAGE_GIF	= 0x3807,
        WMDM_FORMATCODE_IMAGE_JFIF	= 0x3808,
        WMDM_FORMATCODE_IMAGE_PCD	= 0x3809,
        WMDM_FORMATCODE_IMAGE_PICT	= 0x380a,
        WMDM_FORMATCODE_IMAGE_PNG	= 0x380b,
        WMDM_FORMATCODE_IMAGE_TIFF	= 0x380d,
        WMDM_FORMATCODE_IMAGE_TIFFIT	= 0x380e,
        WMDM_FORMATCODE_IMAGE_JP2	= 0x380f,
        WMDM_FORMATCODE_IMAGE_JPX	= 0x3810,
        WMDM_FORMATCODE_IMAGE_RESERVED_FIRST	= 0x3811,
        WMDM_FORMATCODE_IMAGE_RESERVED_LAST	= 0x3fff,
        WMDM_FORMATCODE_UNDEFINEDFIRMWARE	= 0xb802,
        WMDM_FORMATCODE_WBMP	= 0xb803,
        WMDM_FORMATCODE_JPEGXR	= 0xb804,
        WMDM_FORMATCODE_WINDOWSIMAGEFORMAT	= 0xb881,
        WMDM_FORMATCODE_UNDEFINEDAUDIO	= 0xb900,
        WMDM_FORMATCODE_WMA	= 0xb901,
        WMDM_FORMATCODE_OGG	= 0xb902,
        WMDM_FORMATCODE_AAC	= 0xb903,
        WMDM_FORMATCODE_AUDIBLE	= 0xb904,
        WMDM_FORMATCODE_FLAC	= 0xb906,
        WMDM_FORMATCODE_QCELP	= 0xb907,
        WMDM_FORMATCODE_AMR	= 0xb908,
        WMDM_FORMATCODE_UNDEFINEDVIDEO	= 0xb980,
        WMDM_FORMATCODE_WMV	= 0xb981,
        WMDM_FORMATCODE_MP4	= 0xb982,
        WMDM_FORMATCODE_MP2	= 0xb983,
        WMDM_FORMATCODE_3GP	= 0xb984,
        WMDM_FORMATCODE_3G2	= 0xb985,
        WMDM_FORMATCODE_AVCHD	= 0xb986,
        WMDM_FORMATCODE_ATSCTS	= 0xb987,
        WMDM_FORMATCODE_DVBTS	= 0xb988,
        WMDM_FORMATCODE_MKV	= 0xb989,
        WMDM_FORMATCODE_MKA	= 0xb98a,
        WMDM_FORMATCODE_MK3D	= 0xb98b,
        WMDM_FORMATCODE_UNDEFINEDCOLLECTION	= 0xba00,
        WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM	= 0xba01,
        WMDM_FORMATCODE_ABSTRACTIMAGEALBUM	= 0xba02,
        WMDM_FORMATCODE_ABSTRACTAUDIOALBUM	= 0xba03,
        WMDM_FORMATCODE_ABSTRACTVIDEOALBUM	= 0xba04,
        WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST	= 0xba05,
        WMDM_FORMATCODE_ABSTRACTCONTACTGROUP	= 0xba06,
        WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER	= 0xba07,
        WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION	= 0xba08,
        WMDM_FORMATCODE_MEDIA_CAST	= 0xba0b,
        WMDM_FORMATCODE_WPLPLAYLIST	= 0xba10,
        WMDM_FORMATCODE_M3UPLAYLIST	= 0xba11,
        WMDM_FORMATCODE_MPLPLAYLIST	= 0xba12,
        WMDM_FORMATCODE_ASXPLAYLIST	= 0xba13,
        WMDM_FORMATCODE_PLSPLAYLIST	= 0xba14,
        WMDM_FORMATCODE_UNDEFINEDDOCUMENT	= 0xba80,
        WMDM_FORMATCODE_ABSTRACTDOCUMENT	= 0xba81,
        WMDM_FORMATCODE_XMLDOCUMENT	= 0xba82,
        WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT	= 0xba83,
        WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT	= 0xba84,
        WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET	= 0xba85,
        WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT	= 0xba86,
        WMDM_FORMATCODE_UNDEFINEDMESSAGE	= 0xbb00,
        WMDM_FORMATCODE_ABSTRACTMESSAGE	= 0xbb01,
        WMDM_FORMATCODE_UNDEFINEDCONTACT	= 0xbb80,
        WMDM_FORMATCODE_ABSTRACTCONTACT	= 0xbb81,
        WMDM_FORMATCODE_VCARD2	= 0xbb82,
        WMDM_FORMATCODE_VCARD3	= 0xbb83,
        WMDM_FORMATCODE_UNDEFINEDCALENDARITEM	= 0xbe00,
        WMDM_FORMATCODE_ABSTRACTCALENDARITEM	= 0xbe01,
        WMDM_FORMATCODE_VCALENDAR1	= 0xbe02,
        WMDM_FORMATCODE_VCALENDAR2	= 0xbe03,
        WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE	= 0xbe80,
        WMDM_FORMATCODE_M4A	= 0x4d503441,
        WMDM_FORMATCODE_3GPA	= 0x33475041,
        WMDM_FORMATCODE_3G2A	= 0x33473241,
        WMDM_FORMATCODE_SECTION	= 0xbe82
    } 	WMDM_FORMATCODE;

typedef /* [v1_enum] */ 
enum _WMDM_ENUM_PROP_VALID_VALUES_FORM
    {
        WMDM_ENUM_PROP_VALID_VALUES_ANY	= 0,
        WMDM_ENUM_PROP_VALID_VALUES_RANGE	= ( WMDM_ENUM_PROP_VALID_VALUES_ANY + 1 ) ,
        WMDM_ENUM_PROP_VALID_VALUES_ENUM	= ( WMDM_ENUM_PROP_VALID_VALUES_RANGE + 1 ) 
    } 	WMDM_ENUM_PROP_VALID_VALUES_FORM;

typedef struct _WMDM_PROP_VALUES_RANGE
    {
    PROPVARIANT rangeMin;
    PROPVARIANT rangeMax;
    PROPVARIANT rangeStep;
    } 	WMDM_PROP_VALUES_RANGE;

typedef struct _WMDM_PROP_VALUES_ENUM
    {
    UINT cEnumValues;
    /* [size_is] */ PROPVARIANT *pValues;
    } 	WMDM_PROP_VALUES_ENUM;

typedef struct _WMDM_PROP_DESC
    {
    LPWSTR pwszPropName;
    WMDM_ENUM_PROP_VALID_VALUES_FORM ValidValuesForm;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ WMDM_PROP_VALUES_RANGE ValidValuesRange;
        /* [case()] */ WMDM_PROP_VALUES_ENUM EnumeratedValidValues;
        } 	ValidValues;
    } 	WMDM_PROP_DESC;

typedef struct _WMDM_PROP_CONFIG
    {
    UINT nPreference;
    UINT nPropDesc;
    /* [size_is] */ WMDM_PROP_DESC *pPropDesc;
    } 	WMDM_PROP_CONFIG;

typedef struct _WMDM_FORMAT_CAPABILITY
    {
    UINT nPropConfig;
    /* [size_is] */ WMDM_PROP_CONFIG *pConfigs;
    } 	WMDM_FORMAT_CAPABILITY;

#define	WMDM_MAC_LENGTH	( 8 )

typedef 
enum tagWMDM_FIND_SCOPE
    {
        WMDM_FIND_SCOPE_GLOBAL	= 0,
        WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN	= ( WMDM_FIND_SCOPE_GLOBAL + 1 ) 
    } 	WMDM_FIND_SCOPE;

// WMDM HRESULTS
//
//Success codes
//
#define WMDM_S_NOT_ALL_PROPERTIES_APPLIED       0x00045001L
#define WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED     0x00045002L
//
//Error codes
//
#define WMDM_E_BUSY                             0x80045000L
#define WMDM_E_INTERFACEDEAD                    0x80045001L
#define WMDM_E_INVALIDTYPE                      0x80045002L
#define WMDM_E_PROCESSFAILED                    0x80045003L
#define WMDM_E_NOTSUPPORTED                     0x80045004L
#define WMDM_E_NOTCERTIFIED                     0x80045005L
#define WMDM_E_NORIGHTS                         0x80045006L
#define WMDM_E_CALL_OUT_OF_SEQUENCE             0x80045007L
#define WMDM_E_BUFFERTOOSMALL                   0x80045008L
#define WMDM_E_MOREDATA                         0x80045009L
#define WMDM_E_MAC_CHECK_FAILED                 0x8004500AL
#define WMDM_E_USER_CANCELLED                   0x8004500BL
#define WMDM_E_SDMI_TRIGGER                     0x8004500CL
#define WMDM_E_SDMI_NOMORECOPIES                0x8004500DL
#define WMDM_E_REVOKED                          0x8004500EL
#define WMDM_E_LICENSE_NOTEXIST                 0x8004500FL
#define WMDM_E_INCORRECT_APPSEC                 0x80045010L
#define WMDM_E_INCORRECT_RIGHTS                 0x80045011L
#define WMDM_E_LICENSE_EXPIRED                  0x80045012L
#define WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE       0x80045013L
#define WMDM_E_TOO_MANY_SESSIONS                0x80045013L
// Revocation Flags
#define WMDM_WMDM_REVOKED                       0x00000001
#define WMDM_APP_REVOKED                        0x00000002
#define WMDM_SP_REVOKED                         0x00000004
#define WMDM_SCP_REVOKED                        0x00000008
// GetFormatSupport2 Flags
#define WMDM_GET_FORMAT_SUPPORT_AUDIO           0x00000001
#define WMDM_GET_FORMAT_SUPPORT_VIDEO           0x00000002
#define WMDM_GET_FORMAT_SUPPORT_FILE            0x00000004
// MDMRIGHTS Flags
#define WMDM_RIGHTS_PLAYBACKCOUNT               0x00000001
#define WMDM_RIGHTS_EXPIRATIONDATE              0x00000002
#define WMDM_RIGHTS_GROUPID                     0x00000004
#define WMDM_RIGHTS_FREESERIALIDS               0x00000008
#define WMDM_RIGHTS_NAMEDSERIALIDS              0x00000010
// Device Type Flags
#define WMDM_DEVICE_TYPE_PLAYBACK               0x00000001
#define WMDM_DEVICE_TYPE_RECORD                 0x00000002
#define WMDM_DEVICE_TYPE_DECODE                 0x00000004
#define WMDM_DEVICE_TYPE_ENCODE                 0x00000008
#define WMDM_DEVICE_TYPE_STORAGE                0x00000010
#define WMDM_DEVICE_TYPE_VIRTUAL                0x00000020
#define WMDM_DEVICE_TYPE_SDMI                   0x00000040
#define WMDM_DEVICE_TYPE_NONSDMI                0x00000080
#define WMDM_DEVICE_TYPE_NONREENTRANT           0x00000100
#define WMDM_DEVICE_TYPE_FILELISTRESYNC         0x00000200
#define WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW 0x00000400
// Device Power Source Flags
#define WMDM_POWER_CAP_BATTERY                  0x00000001
#define WMDM_POWER_CAP_EXTERNAL                 0x00000002
#define WMDM_POWER_IS_BATTERY                   0x00000004
#define WMDM_POWER_IS_EXTERNAL                  0x00000008
#define WMDM_POWER_PERCENT_AVAILABLE            0x00000010
// Device Status Flags
#define WMDM_STATUS_READY                       0x00000001
#define WMDM_STATUS_BUSY                        0x00000002
#define WMDM_STATUS_DEVICE_NOTPRESENT           0x00000004
#define WMDM_STATUS_DEVICECONTROL_PLAYING       0x00000008
#define WMDM_STATUS_DEVICECONTROL_RECORDING     0x00000010
#define WMDM_STATUS_DEVICECONTROL_PAUSED        0x00000020
#define WMDM_STATUS_DEVICECONTROL_REMOTE        0x00000040
#define WMDM_STATUS_DEVICECONTROL_STREAM        0x00000080
#define WMDM_STATUS_STORAGE_NOTPRESENT          0x00000100
#define WMDM_STATUS_STORAGE_INITIALIZING        0x00000200
#define WMDM_STATUS_STORAGE_BROKEN              0x00000400
#define WMDM_STATUS_STORAGE_NOTSUPPORTED        0x00000800
#define WMDM_STATUS_STORAGE_UNFORMATTED         0x00001000
#define WMDM_STATUS_STORAGECONTROL_INSERTING    0x00002000
#define WMDM_STATUS_STORAGECONTROL_DELETING     0x00004000
#define WMDM_STATUS_STORAGECONTROL_APPENDING    0x00008000
#define WMDM_STATUS_STORAGECONTROL_MOVING       0x00010000
#define WMDM_STATUS_STORAGECONTROL_READING      0x00020000
// Device Capabilities Flags
#define WMDM_DEVICECAP_CANPLAY                  0x00000001
#define WMDM_DEVICECAP_CANSTREAMPLAY            0x00000002
#define WMDM_DEVICECAP_CANRECORD                0x00000004
#define WMDM_DEVICECAP_CANSTREAMRECORD          0x00000008
#define WMDM_DEVICECAP_CANPAUSE                 0x00000010
#define WMDM_DEVICECAP_CANRESUME                0x00000020
#define WMDM_DEVICECAP_CANSTOP                  0x00000040
#define WMDM_DEVICECAP_CANSEEK                  0x00000080
#define WMDM_DEVICECAP_HASSECURECLOCK           0x00000100
// WMDM Seek Flags
#define WMDM_SEEK_REMOTECONTROL                 0x00000001
#define WMDM_SEEK_STREAMINGAUDIO                0x00000002
// Storage Attributes Flags
#define WMDM_STORAGE_ATTR_FILESYSTEM            0x00000001
#define WMDM_STORAGE_ATTR_REMOVABLE             0x00000002
#define WMDM_STORAGE_ATTR_NONREMOVABLE          0x00000004
#define WMDM_FILE_ATTR_FOLDER                   0x00000008
#define WMDM_FILE_ATTR_LINK                     0x00000010
#define WMDM_FILE_ATTR_FILE                     0x00000020
#define WMDM_FILE_ATTR_VIDEO                    0x00000040
#define WMDM_STORAGE_ATTR_CANEDITMETADATA       0x00000080
#define WMDM_STORAGE_ATTR_FOLDERS               0x00000100
#define WMDM_FILE_ATTR_AUDIO                    0x00001000
#define WMDM_FILE_ATTR_DATA                     0x00002000
#define WMDM_FILE_ATTR_CANPLAY                  0x00004000
#define WMDM_FILE_ATTR_CANDELETE                0x00008000
#define WMDM_FILE_ATTR_CANMOVE                  0x00010000
#define WMDM_FILE_ATTR_CANRENAME                0x00020000
#define WMDM_FILE_ATTR_CANREAD                  0x00040000
#define WMDM_FILE_ATTR_MUSIC                    0x00080000
#define WMDM_FILE_CREATE_OVERWRITE              0x00100000
#define WMDM_FILE_ATTR_AUDIOBOOK                0x00200000
#define WMDM_FILE_ATTR_HIDDEN                   0x00400000
#define WMDM_FILE_ATTR_SYSTEM                   0x00800000
#define WMDM_FILE_ATTR_READONLY                 0x01000000
#define WMDM_STORAGE_ATTR_HAS_FOLDERS           0x02000000
#define WMDM_STORAGE_ATTR_HAS_FILES             0x04000000
#define WMDM_STORAGE_IS_DEFAULT                 0x08000000
#define WMDM_STORAGE_CONTAINS_DEFAULT           0x10000000
#define WMDM_STORAGE_ATTR_VIRTUAL               0x20000000
// Storage Capabilities Flags
#define WMDM_STORAGECAP_FOLDERSINROOT           0x00000001
#define WMDM_STORAGECAP_FILESINROOT             0x00000002
#define WMDM_STORAGECAP_FOLDERSINFOLDERS        0x00000004
#define WMDM_STORAGECAP_FILESINFOLDERS          0x00000008
#define WMDM_STORAGECAP_FOLDERLIMITEXISTS       0x00000010
#define WMDM_STORAGECAP_FILELIMITEXISTS         0x00000020
#define WMDM_STORAGECAP_NOT_INITIALIZABLE       0x00000040
// WMDM Mode Flags
#define WMDM_MODE_BLOCK                         0x00000001
#define WMDM_MODE_THREAD                        0x00000002
#define WMDM_CONTENT_FILE                       0x00000004
#define WMDM_CONTENT_FOLDER                     0x00000008
#define WMDM_CONTENT_OPERATIONINTERFACE         0x00000010
#define WMDM_MODE_QUERY                         0x00000020
#define WMDM_MODE_PROGRESS                      0x00000040
#define WMDM_MODE_TRANSFER_PROTECTED            0x00000080
#define WMDM_MODE_TRANSFER_UNPROTECTED          0x00000100
#define WMDM_STORAGECONTROL_INSERTBEFORE        0x00000200
#define WMDM_STORAGECONTROL_INSERTAFTER         0x00000400
#define WMDM_STORAGECONTROL_INSERTINTO          0x00000800
#define WMDM_MODE_RECURSIVE                     0x00001000
// WMDM Rights Flags
// NON_SDMI = !SDMI_PROTECTED
// SDMI = SDMI_VALIDATED
#define WMDM_RIGHTS_PLAY_ON_PC                  0x00000001
#define WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE     0x00000002
#define WMDM_RIGHTS_COPY_TO_CD                  0x00000008
#define WMDM_RIGHTS_COPY_TO_SDMI_DEVICE         0x00000010
// WMDM Seek Flags
#define WMDM_SEEK_BEGIN                         0x00000001
#define WMDM_SEEK_CURRENT                       0x00000002
#define WMDM_SEEK_END                           0x00000008
// WMDM Device Enumeration Flags
#define DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES   0x00000001
#define ALLOW_OUTOFBAND_NOTIFICATION            0x00000002

enum WMDMMessage
    {
        WMDM_MSG_DEVICE_ARRIVAL	= 0,
        WMDM_MSG_DEVICE_REMOVAL	= ( WMDM_MSG_DEVICE_ARRIVAL + 1 ) ,
        WMDM_MSG_MEDIA_ARRIVAL	= ( WMDM_MSG_DEVICE_REMOVAL + 1 ) ,
        WMDM_MSG_MEDIA_REMOVAL	= ( WMDM_MSG_MEDIA_ARRIVAL + 1 ) 
    } ;














extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0000_v0_0_s_ifspec;

#ifndef __IWMDMMetaData_INTERFACE_DEFINED__
#define __IWMDMMetaData_INTERFACE_DEFINED__

/* interface IWMDMMetaData */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMMetaData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EC3B0663-0951-460a-9A80-0DCEED3C043C")
    IWMDMMetaData : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ WMDM_TAG_DATATYPE Type,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszTagName,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(iLength) BYTE *pValue,
            /* [in] */ UINT iLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryByName( 
            /* [string][in] */ __RPC__in_string LPCWSTR pwszTagName,
            /* [out] */ __RPC__out WMDM_TAG_DATATYPE *pType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLength) BYTE **pValue,
            /* [out] */ __RPC__out UINT *pcbLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryByIndex( 
            /* [in] */ UINT iIndex,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszName,
            /* [out] */ __RPC__out WMDM_TAG_DATATYPE *pType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLength) BYTE **ppValue,
            /* [out] */ __RPC__out UINT *pcbLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemCount( 
            /* [out] */ __RPC__out UINT *iCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMMetaDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMMetaData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMMetaData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMMetaData * This);
        
        DECLSPEC_XFGVIRT(IWMDMMetaData, AddItem)
        HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IWMDMMetaData * This,
            /* [in] */ WMDM_TAG_DATATYPE Type,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszTagName,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(iLength) BYTE *pValue,
            /* [in] */ UINT iLength);
        
        DECLSPEC_XFGVIRT(IWMDMMetaData, QueryByName)
        HRESULT ( STDMETHODCALLTYPE *QueryByName )( 
            __RPC__in IWMDMMetaData * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszTagName,
            /* [out] */ __RPC__out WMDM_TAG_DATATYPE *pType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLength) BYTE **pValue,
            /* [out] */ __RPC__out UINT *pcbLength);
        
        DECLSPEC_XFGVIRT(IWMDMMetaData, QueryByIndex)
        HRESULT ( STDMETHODCALLTYPE *QueryByIndex )( 
            __RPC__in IWMDMMetaData * This,
            /* [in] */ UINT iIndex,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszName,
            /* [out] */ __RPC__out WMDM_TAG_DATATYPE *pType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbLength) BYTE **ppValue,
            /* [out] */ __RPC__out UINT *pcbLength);
        
        DECLSPEC_XFGVIRT(IWMDMMetaData, GetItemCount)
        HRESULT ( STDMETHODCALLTYPE *GetItemCount )( 
            __RPC__in IWMDMMetaData * This,
            /* [out] */ __RPC__out UINT *iCount);
        
        END_INTERFACE
    } IWMDMMetaDataVtbl;

    interface IWMDMMetaData
    {
        CONST_VTBL struct IWMDMMetaDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMMetaData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMMetaData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMMetaData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMMetaData_AddItem(This,Type,pwszTagName,pValue,iLength)	\
    ( (This)->lpVtbl -> AddItem(This,Type,pwszTagName,pValue,iLength) ) 

#define IWMDMMetaData_QueryByName(This,pwszTagName,pType,pValue,pcbLength)	\
    ( (This)->lpVtbl -> QueryByName(This,pwszTagName,pType,pValue,pcbLength) ) 

#define IWMDMMetaData_QueryByIndex(This,iIndex,ppwszName,pType,ppValue,pcbLength)	\
    ( (This)->lpVtbl -> QueryByIndex(This,iIndex,ppwszName,pType,ppValue,pcbLength) ) 

#define IWMDMMetaData_GetItemCount(This,iCount)	\
    ( (This)->lpVtbl -> GetItemCount(This,iCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMMetaData_INTERFACE_DEFINED__ */


#ifndef __IWMDeviceManager_INTERFACE_DEFINED__
#define __IWMDeviceManager_INTERFACE_DEFINED__

/* interface IWMDeviceManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDeviceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A00-33ED-11d3-8470-00C04F79DBC0")
    IWMDeviceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRevision( 
            /* [out] */ __RPC__out DWORD *pdwRevision) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDevices( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDeviceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDeviceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetRevision)
        HRESULT ( STDMETHODCALLTYPE *GetRevision )( 
            __RPC__in IWMDeviceManager * This,
            /* [out] */ __RPC__out DWORD *pdwRevision);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IWMDeviceManager * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IWMDeviceManager * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        END_INTERFACE
    } IWMDeviceManagerVtbl;

    interface IWMDeviceManager
    {
        CONST_VTBL struct IWMDeviceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDeviceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDeviceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDeviceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDeviceManager_GetRevision(This,pdwRevision)	\
    ( (This)->lpVtbl -> GetRevision(This,pdwRevision) ) 

#define IWMDeviceManager_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IWMDeviceManager_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDeviceManager_INTERFACE_DEFINED__ */


#ifndef __IWMDeviceManager2_INTERFACE_DEFINED__
#define __IWMDeviceManager2_INTERFACE_DEFINED__

/* interface IWMDeviceManager2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDeviceManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("923E5249-8731-4c5b-9B1C-B8B60B6E46AF")
    IWMDeviceManager2 : public IWMDeviceManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceFromCanonicalName( 
            /* [string][in] */ __RPC__in_string LPCWSTR pwszCanonicalName,
            /* [out] */ __RPC__deref_out_opt IWMDMDevice **ppDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDevices2( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reinitialize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDeviceManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDeviceManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDeviceManager2 * This);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetRevision)
        HRESULT ( STDMETHODCALLTYPE *GetRevision )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [out] */ __RPC__out DWORD *pdwRevision);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, GetDeviceFromCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceFromCanonicalName )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszCanonicalName,
            /* [out] */ __RPC__deref_out_opt IWMDMDevice **ppDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, EnumDevices2)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices2 )( 
            __RPC__in IWMDeviceManager2 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, Reinitialize)
        HRESULT ( STDMETHODCALLTYPE *Reinitialize )( 
            __RPC__in IWMDeviceManager2 * This);
        
        END_INTERFACE
    } IWMDeviceManager2Vtbl;

    interface IWMDeviceManager2
    {
        CONST_VTBL struct IWMDeviceManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDeviceManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDeviceManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDeviceManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDeviceManager2_GetRevision(This,pdwRevision)	\
    ( (This)->lpVtbl -> GetRevision(This,pdwRevision) ) 

#define IWMDeviceManager2_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IWMDeviceManager2_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 


#define IWMDeviceManager2_GetDeviceFromCanonicalName(This,pwszCanonicalName,ppDevice)	\
    ( (This)->lpVtbl -> GetDeviceFromCanonicalName(This,pwszCanonicalName,ppDevice) ) 

#define IWMDeviceManager2_EnumDevices2(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices2(This,ppEnumDevice) ) 

#define IWMDeviceManager2_Reinitialize(This)	\
    ( (This)->lpVtbl -> Reinitialize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDeviceManager2_INTERFACE_DEFINED__ */


#ifndef __IWMDeviceManager3_INTERFACE_DEFINED__
#define __IWMDeviceManager3_INTERFACE_DEFINED__

/* interface IWMDeviceManager3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDeviceManager3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("af185c41-100d-46ed-be2e-9ce8c44594ef")
    IWMDeviceManager3 : public IWMDeviceManager2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDeviceEnumPreference( 
            /* [in] */ DWORD dwEnumPref) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDeviceManager3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDeviceManager3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDeviceManager3 * This);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetRevision)
        HRESULT ( STDMETHODCALLTYPE *GetRevision )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [out] */ __RPC__out DWORD *pdwRevision);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, GetDeviceFromCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceFromCanonicalName )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszCanonicalName,
            /* [out] */ __RPC__deref_out_opt IWMDMDevice **ppDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, EnumDevices2)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices2 )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager2, Reinitialize)
        HRESULT ( STDMETHODCALLTYPE *Reinitialize )( 
            __RPC__in IWMDeviceManager3 * This);
        
        DECLSPEC_XFGVIRT(IWMDeviceManager3, SetDeviceEnumPreference)
        HRESULT ( STDMETHODCALLTYPE *SetDeviceEnumPreference )( 
            __RPC__in IWMDeviceManager3 * This,
            /* [in] */ DWORD dwEnumPref);
        
        END_INTERFACE
    } IWMDeviceManager3Vtbl;

    interface IWMDeviceManager3
    {
        CONST_VTBL struct IWMDeviceManager3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDeviceManager3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDeviceManager3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDeviceManager3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDeviceManager3_GetRevision(This,pdwRevision)	\
    ( (This)->lpVtbl -> GetRevision(This,pdwRevision) ) 

#define IWMDeviceManager3_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IWMDeviceManager3_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 


#define IWMDeviceManager3_GetDeviceFromCanonicalName(This,pwszCanonicalName,ppDevice)	\
    ( (This)->lpVtbl -> GetDeviceFromCanonicalName(This,pwszCanonicalName,ppDevice) ) 

#define IWMDeviceManager3_EnumDevices2(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices2(This,ppEnumDevice) ) 

#define IWMDeviceManager3_Reinitialize(This)	\
    ( (This)->lpVtbl -> Reinitialize(This) ) 


#define IWMDeviceManager3_SetDeviceEnumPreference(This,dwEnumPref)	\
    ( (This)->lpVtbl -> SetDeviceEnumPreference(This,dwEnumPref) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDeviceManager3_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorageGlobals_INTERFACE_DEFINED__
#define __IWMDMStorageGlobals_INTERFACE_DEFINED__

/* interface IWMDMStorageGlobals */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorageGlobals;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A07-33ED-11d3-8470-00C04F79DBC0")
    IWMDMStorageGlobals : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSerialNumber( 
            /* [out] */ __RPC__out PWMDMID pSerialNum,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalSize( 
            /* [out] */ __RPC__out DWORD *pdwTotalSizeLow,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalFree( 
            /* [out] */ __RPC__out DWORD *pdwFreeLow,
            /* [out] */ __RPC__out DWORD *pdwFreeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalBad( 
            /* [out] */ __RPC__out DWORD *pdwBadLow,
            /* [out] */ __RPC__out DWORD *pdwBadHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorageGlobalsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorageGlobals * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorageGlobals * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwCapabilities);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out PWMDMID pSerialNum,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetTotalSize)
        HRESULT ( STDMETHODCALLTYPE *GetTotalSize )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeLow,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetTotalFree)
        HRESULT ( STDMETHODCALLTYPE *GetTotalFree )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwFreeLow,
            /* [out] */ __RPC__out DWORD *pdwFreeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetTotalBad)
        HRESULT ( STDMETHODCALLTYPE *GetTotalBad )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwBadLow,
            /* [out] */ __RPC__out DWORD *pdwBadHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IWMDMStorageGlobals, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IWMDMStorageGlobals * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        END_INTERFACE
    } IWMDMStorageGlobalsVtbl;

    interface IWMDMStorageGlobals
    {
        CONST_VTBL struct IWMDMStorageGlobalsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorageGlobals_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorageGlobals_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorageGlobals_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorageGlobals_GetCapabilities(This,pdwCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilities) ) 

#define IWMDMStorageGlobals_GetSerialNumber(This,pSerialNum,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNum,abMac) ) 

#define IWMDMStorageGlobals_GetTotalSize(This,pdwTotalSizeLow,pdwTotalSizeHigh)	\
    ( (This)->lpVtbl -> GetTotalSize(This,pdwTotalSizeLow,pdwTotalSizeHigh) ) 

#define IWMDMStorageGlobals_GetTotalFree(This,pdwFreeLow,pdwFreeHigh)	\
    ( (This)->lpVtbl -> GetTotalFree(This,pdwFreeLow,pdwFreeHigh) ) 

#define IWMDMStorageGlobals_GetTotalBad(This,pdwBadLow,pdwBadHigh)	\
    ( (This)->lpVtbl -> GetTotalBad(This,pdwBadLow,pdwBadHigh) ) 

#define IWMDMStorageGlobals_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IWMDMStorageGlobals_Initialize(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Initialize(This,fuMode,pProgress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorageGlobals_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorage_INTERFACE_DEFINED__
#define __IWMDMStorage_INTERFACE_DEFINED__

/* interface IWMDMStorage */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A06-33ED-11d3-8470-00C04F79DBC0")
    IWMDMStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAttributes( 
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStorageGlobals( 
            /* [out] */ __RPC__deref_out_opt IWMDMStorageGlobals **ppStorageGlobals) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDate( 
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRights( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStorage( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **pEnumStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendOpaqueCommand( 
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorage * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IWMDMStorage * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IWMDMStorage * This,
            /* [out] */ __RPC__deref_out_opt IWMDMStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IWMDMStorage * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMStorage * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IWMDMStorage * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IWMDMStorage * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IWMDMStorage * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMStorage * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **pEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMStorage * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        END_INTERFACE
    } IWMDMStorageVtbl;

    interface IWMDMStorage
    {
        CONST_VTBL struct IWMDMStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorage_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMStorage_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IWMDMStorage_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMStorage_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMStorage_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IWMDMStorage_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IWMDMStorage_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IWMDMStorage_EnumStorage(This,pEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,pEnumStorage) ) 

#define IWMDMStorage_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorage_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorage2_INTERFACE_DEFINED__
#define __IWMDMStorage2_INTERFACE_DEFINED__

/* interface IWMDMStorage2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1ED5A144-5CD5-4683-9EFF-72CBDB2D9533")
    IWMDMStorage2 : public IWMDMStorage
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStorage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAttributes2( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes2( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorage2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IWMDMStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMStorage2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IWMDMStorage2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **pEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IWMDMStorage2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IWMDMStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IWMDMStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        END_INTERFACE
    } IWMDMStorage2Vtbl;

    interface IWMDMStorage2
    {
        CONST_VTBL struct IWMDMStorage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorage2_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMStorage2_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IWMDMStorage2_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMStorage2_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMStorage2_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IWMDMStorage2_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IWMDMStorage2_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IWMDMStorage2_EnumStorage(This,pEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,pEnumStorage) ) 

#define IWMDMStorage2_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IWMDMStorage2_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IWMDMStorage2_SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat) ) 

#define IWMDMStorage2_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorage2_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorage3_INTERFACE_DEFINED__
#define __IWMDMStorage3_INTERFACE_DEFINED__

/* interface IWMDMStorage3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorage3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97717EEA-926A-464e-96A4-247B0216026E")
    IWMDMStorage3 : public IWMDMStorage2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEmptyMetadataObject( 
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEnumPreference( 
            /* [out][in] */ __RPC__inout WMDM_STORAGE_ENUM_MODE *pMode,
            /* [in] */ DWORD nViews,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(nViews) WMDMMetadataView *pViews) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorage3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorage3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorage3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IWMDMStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMStorage3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IWMDMStorage3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **pEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IWMDMStorage3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IWMDMStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IWMDMStorage3 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, CreateEmptyMetadataObject)
        HRESULT ( STDMETHODCALLTYPE *CreateEmptyMetadataObject )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, SetEnumPreference)
        HRESULT ( STDMETHODCALLTYPE *SetEnumPreference )( 
            __RPC__in IWMDMStorage3 * This,
            /* [out][in] */ __RPC__inout WMDM_STORAGE_ENUM_MODE *pMode,
            /* [in] */ DWORD nViews,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(nViews) WMDMMetadataView *pViews);
        
        END_INTERFACE
    } IWMDMStorage3Vtbl;

    interface IWMDMStorage3
    {
        CONST_VTBL struct IWMDMStorage3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorage3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorage3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorage3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorage3_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMStorage3_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IWMDMStorage3_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMStorage3_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMStorage3_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IWMDMStorage3_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IWMDMStorage3_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IWMDMStorage3_EnumStorage(This,pEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,pEnumStorage) ) 

#define IWMDMStorage3_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IWMDMStorage3_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IWMDMStorage3_SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat) ) 

#define IWMDMStorage3_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 


#define IWMDMStorage3_GetMetadata(This,ppMetadata)	\
    ( (This)->lpVtbl -> GetMetadata(This,ppMetadata) ) 

#define IWMDMStorage3_SetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> SetMetadata(This,pMetadata) ) 

#define IWMDMStorage3_CreateEmptyMetadataObject(This,ppMetadata)	\
    ( (This)->lpVtbl -> CreateEmptyMetadataObject(This,ppMetadata) ) 

#define IWMDMStorage3_SetEnumPreference(This,pMode,nViews,pViews)	\
    ( (This)->lpVtbl -> SetEnumPreference(This,pMode,nViews,pViews) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorage3_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorage4_INTERFACE_DEFINED__
#define __IWMDMStorage4_INTERFACE_DEFINED__

/* interface IWMDMStorage4 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorage4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c225bac5-a03a-40b8-9a23-91cf478c64a6")
    IWMDMStorage4 : public IWMDMStorage3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetReferences( 
            /* [in] */ DWORD dwRefs,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRefs) IWMDMStorage **ppIWMDMStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReferences( 
            /* [out] */ __RPC__out DWORD *pdwRefs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwRefs) IWMDMStorage ***pppIWMDMStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRightsWithProgress( 
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pIProgressCallback,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpecifiedMetadata( 
            /* [in] */ DWORD cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) LPCWSTR *ppwszPropNames,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindStorage( 
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParent( 
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorage4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorage4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorage4 * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMStorage4 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IWMDMStorage4 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **pEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IWMDMStorage4 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, CreateEmptyMetadataObject)
        HRESULT ( STDMETHODCALLTYPE *CreateEmptyMetadataObject )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage3, SetEnumPreference)
        HRESULT ( STDMETHODCALLTYPE *SetEnumPreference )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out][in] */ __RPC__inout WMDM_STORAGE_ENUM_MODE *pMode,
            /* [in] */ DWORD nViews,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(nViews) WMDMMetadataView *pViews);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, SetReferences)
        HRESULT ( STDMETHODCALLTYPE *SetReferences )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ DWORD dwRefs,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRefs) IWMDMStorage **ppIWMDMStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, GetReferences)
        HRESULT ( STDMETHODCALLTYPE *GetReferences )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwRefs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwRefs) IWMDMStorage ***pppIWMDMStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, GetRightsWithProgress)
        HRESULT ( STDMETHODCALLTYPE *GetRightsWithProgress )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pIProgressCallback,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, GetSpecifiedMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifiedMetadata )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ DWORD cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) LPCWSTR *ppwszPropNames,
            /* [out] */ __RPC__deref_out_opt IWMDMMetaData **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, FindStorage)
        HRESULT ( STDMETHODCALLTYPE *FindStorage )( 
            __RPC__in IWMDMStorage4 * This,
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMStorage4, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IWMDMStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        END_INTERFACE
    } IWMDMStorage4Vtbl;

    interface IWMDMStorage4
    {
        CONST_VTBL struct IWMDMStorage4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorage4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorage4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorage4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorage4_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMStorage4_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IWMDMStorage4_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMStorage4_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMStorage4_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IWMDMStorage4_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IWMDMStorage4_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IWMDMStorage4_EnumStorage(This,pEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,pEnumStorage) ) 

#define IWMDMStorage4_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IWMDMStorage4_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IWMDMStorage4_SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat) ) 

#define IWMDMStorage4_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 


#define IWMDMStorage4_GetMetadata(This,ppMetadata)	\
    ( (This)->lpVtbl -> GetMetadata(This,ppMetadata) ) 

#define IWMDMStorage4_SetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> SetMetadata(This,pMetadata) ) 

#define IWMDMStorage4_CreateEmptyMetadataObject(This,ppMetadata)	\
    ( (This)->lpVtbl -> CreateEmptyMetadataObject(This,ppMetadata) ) 

#define IWMDMStorage4_SetEnumPreference(This,pMode,nViews,pViews)	\
    ( (This)->lpVtbl -> SetEnumPreference(This,pMode,nViews,pViews) ) 


#define IWMDMStorage4_SetReferences(This,dwRefs,ppIWMDMStorage)	\
    ( (This)->lpVtbl -> SetReferences(This,dwRefs,ppIWMDMStorage) ) 

#define IWMDMStorage4_GetReferences(This,pdwRefs,pppIWMDMStorage)	\
    ( (This)->lpVtbl -> GetReferences(This,pdwRefs,pppIWMDMStorage) ) 

#define IWMDMStorage4_GetRightsWithProgress(This,pIProgressCallback,ppRights,pnRightsCount)	\
    ( (This)->lpVtbl -> GetRightsWithProgress(This,pIProgressCallback,ppRights,pnRightsCount) ) 

#define IWMDMStorage4_GetSpecifiedMetadata(This,cProperties,ppwszPropNames,ppMetadata)	\
    ( (This)->lpVtbl -> GetSpecifiedMetadata(This,cProperties,ppwszPropNames,ppMetadata) ) 

#define IWMDMStorage4_FindStorage(This,findScope,pwszUniqueID,ppStorage)	\
    ( (This)->lpVtbl -> FindStorage(This,findScope,pwszUniqueID,ppStorage) ) 

#define IWMDMStorage4_GetParent(This,ppStorage)	\
    ( (This)->lpVtbl -> GetParent(This,ppStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorage4_INTERFACE_DEFINED__ */


#ifndef __IWMDMOperation_INTERFACE_DEFINED__
#define __IWMDMOperation_INTERFACE_DEFINED__

/* interface IWMDMOperation */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A0B-33ED-11d3-8470-00C04F79DBC0")
    IWMDMOperation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginRead( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginWrite( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectName( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetObjectName( 
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectAttributes( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetObjectAttributes( 
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectTotalSize( 
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetObjectTotalSize( 
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransferObjectData( 
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( 
            /* [in] */ __RPC__in HRESULT *phCompletionCode,
            /* [in] */ __RPC__in_opt IUnknown *pNewObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMOperation * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginRead)
        HRESULT ( STDMETHODCALLTYPE *BeginRead )( 
            __RPC__in IWMDMOperation * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginWrite)
        HRESULT ( STDMETHODCALLTYPE *BeginWrite )( 
            __RPC__in IWMDMOperation * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectName)
        HRESULT ( STDMETHODCALLTYPE *GetObjectName )( 
            __RPC__in IWMDMOperation * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectName)
        HRESULT ( STDMETHODCALLTYPE *SetObjectName )( 
            __RPC__in IWMDMOperation * This,
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAttributes )( 
            __RPC__in IWMDMOperation * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetObjectAttributes )( 
            __RPC__in IWMDMOperation * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *GetObjectTotalSize )( 
            __RPC__in IWMDMOperation * This,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *SetObjectTotalSize )( 
            __RPC__in IWMDMOperation * This,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, TransferObjectData)
        HRESULT ( STDMETHODCALLTYPE *TransferObjectData )( 
            __RPC__in IWMDMOperation * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMOperation * This,
            /* [in] */ __RPC__in HRESULT *phCompletionCode,
            /* [in] */ __RPC__in_opt IUnknown *pNewObject);
        
        END_INTERFACE
    } IWMDMOperationVtbl;

    interface IWMDMOperation
    {
        CONST_VTBL struct IWMDMOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMOperation_BeginRead(This)	\
    ( (This)->lpVtbl -> BeginRead(This) ) 

#define IWMDMOperation_BeginWrite(This)	\
    ( (This)->lpVtbl -> BeginWrite(This) ) 

#define IWMDMOperation_GetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation_SetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> SetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation_GetObjectAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetObjectAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMOperation_SetObjectAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetObjectAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMOperation_GetObjectTotalSize(This,pdwSize,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetObjectTotalSize(This,pdwSize,pdwSizeHigh) ) 

#define IWMDMOperation_SetObjectTotalSize(This,dwSize,dwSizeHigh)	\
    ( (This)->lpVtbl -> SetObjectTotalSize(This,dwSize,dwSizeHigh) ) 

#define IWMDMOperation_TransferObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> TransferObjectData(This,pData,pdwSize,abMac) ) 

#define IWMDMOperation_End(This,phCompletionCode,pNewObject)	\
    ( (This)->lpVtbl -> End(This,phCompletionCode,pNewObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMOperation_INTERFACE_DEFINED__ */


#ifndef __IWMDMOperation2_INTERFACE_DEFINED__
#define __IWMDMOperation2_INTERFACE_DEFINED__

/* interface IWMDMOperation2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMOperation2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33445B48-7DF7-425c-AD8F-0FC6D82F9F75")
    IWMDMOperation2 : public IWMDMOperation
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetObjectAttributes2( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectAttributes2( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMOperation2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMOperation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMOperation2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMOperation2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginRead)
        HRESULT ( STDMETHODCALLTYPE *BeginRead )( 
            __RPC__in IWMDMOperation2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginWrite)
        HRESULT ( STDMETHODCALLTYPE *BeginWrite )( 
            __RPC__in IWMDMOperation2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectName)
        HRESULT ( STDMETHODCALLTYPE *GetObjectName )( 
            __RPC__in IWMDMOperation2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectName)
        HRESULT ( STDMETHODCALLTYPE *SetObjectName )( 
            __RPC__in IWMDMOperation2 * This,
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAttributes )( 
            __RPC__in IWMDMOperation2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetObjectAttributes )( 
            __RPC__in IWMDMOperation2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *GetObjectTotalSize )( 
            __RPC__in IWMDMOperation2 * This,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *SetObjectTotalSize )( 
            __RPC__in IWMDMOperation2 * This,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, TransferObjectData)
        HRESULT ( STDMETHODCALLTYPE *TransferObjectData )( 
            __RPC__in IWMDMOperation2 * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMOperation2 * This,
            /* [in] */ __RPC__in HRESULT *phCompletionCode,
            /* [in] */ __RPC__in_opt IUnknown *pNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMOperation2, SetObjectAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetObjectAttributes2 )( 
            __RPC__in IWMDMOperation2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation2, GetObjectAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAttributes2 )( 
            __RPC__in IWMDMOperation2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        END_INTERFACE
    } IWMDMOperation2Vtbl;

    interface IWMDMOperation2
    {
        CONST_VTBL struct IWMDMOperation2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMOperation2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMOperation2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMOperation2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMOperation2_BeginRead(This)	\
    ( (This)->lpVtbl -> BeginRead(This) ) 

#define IWMDMOperation2_BeginWrite(This)	\
    ( (This)->lpVtbl -> BeginWrite(This) ) 

#define IWMDMOperation2_GetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation2_SetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> SetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation2_GetObjectAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetObjectAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMOperation2_SetObjectAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetObjectAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMOperation2_GetObjectTotalSize(This,pdwSize,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetObjectTotalSize(This,pdwSize,pdwSizeHigh) ) 

#define IWMDMOperation2_SetObjectTotalSize(This,dwSize,dwSizeHigh)	\
    ( (This)->lpVtbl -> SetObjectTotalSize(This,dwSize,dwSizeHigh) ) 

#define IWMDMOperation2_TransferObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> TransferObjectData(This,pData,pdwSize,abMac) ) 

#define IWMDMOperation2_End(This,phCompletionCode,pNewObject)	\
    ( (This)->lpVtbl -> End(This,phCompletionCode,pNewObject) ) 


#define IWMDMOperation2_SetObjectAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetObjectAttributes2(This,dwAttributes,dwAttributesEx,pFormat,pVideoFormat) ) 

#define IWMDMOperation2_GetObjectAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetObjectAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMOperation2_INTERFACE_DEFINED__ */


#ifndef __IWMDMOperation3_INTERFACE_DEFINED__
#define __IWMDMOperation3_INTERFACE_DEFINED__

/* interface IWMDMOperation3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMOperation3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d1f9b46a-9ca8-46d8-9d0f-1ec9bae54919")
    IWMDMOperation3 : public IWMDMOperation
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferObjectDataOnClearChannel( 
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMOperation3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMOperation3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMOperation3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMOperation3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginRead)
        HRESULT ( STDMETHODCALLTYPE *BeginRead )( 
            __RPC__in IWMDMOperation3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, BeginWrite)
        HRESULT ( STDMETHODCALLTYPE *BeginWrite )( 
            __RPC__in IWMDMOperation3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectName)
        HRESULT ( STDMETHODCALLTYPE *GetObjectName )( 
            __RPC__in IWMDMOperation3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectName)
        HRESULT ( STDMETHODCALLTYPE *SetObjectName )( 
            __RPC__in IWMDMOperation3 * This,
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAttributes )( 
            __RPC__in IWMDMOperation3 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetObjectAttributes )( 
            __RPC__in IWMDMOperation3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, GetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *GetObjectTotalSize )( 
            __RPC__in IWMDMOperation3 * This,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, SetObjectTotalSize)
        HRESULT ( STDMETHODCALLTYPE *SetObjectTotalSize )( 
            __RPC__in IWMDMOperation3 * This,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwSizeHigh);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, TransferObjectData)
        HRESULT ( STDMETHODCALLTYPE *TransferObjectData )( 
            __RPC__in IWMDMOperation3 * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMOperation, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMOperation3 * This,
            /* [in] */ __RPC__in HRESULT *phCompletionCode,
            /* [in] */ __RPC__in_opt IUnknown *pNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMOperation3, TransferObjectDataOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *TransferObjectDataOnClearChannel )( 
            __RPC__in IWMDMOperation3 * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize);
        
        END_INTERFACE
    } IWMDMOperation3Vtbl;

    interface IWMDMOperation3
    {
        CONST_VTBL struct IWMDMOperation3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMOperation3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMOperation3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMOperation3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMOperation3_BeginRead(This)	\
    ( (This)->lpVtbl -> BeginRead(This) ) 

#define IWMDMOperation3_BeginWrite(This)	\
    ( (This)->lpVtbl -> BeginWrite(This) ) 

#define IWMDMOperation3_GetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation3_SetObjectName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> SetObjectName(This,pwszName,nMaxChars) ) 

#define IWMDMOperation3_GetObjectAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetObjectAttributes(This,pdwAttributes,pFormat) ) 

#define IWMDMOperation3_SetObjectAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetObjectAttributes(This,dwAttributes,pFormat) ) 

#define IWMDMOperation3_GetObjectTotalSize(This,pdwSize,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetObjectTotalSize(This,pdwSize,pdwSizeHigh) ) 

#define IWMDMOperation3_SetObjectTotalSize(This,dwSize,dwSizeHigh)	\
    ( (This)->lpVtbl -> SetObjectTotalSize(This,dwSize,dwSizeHigh) ) 

#define IWMDMOperation3_TransferObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> TransferObjectData(This,pData,pdwSize,abMac) ) 

#define IWMDMOperation3_End(This,phCompletionCode,pNewObject)	\
    ( (This)->lpVtbl -> End(This,phCompletionCode,pNewObject) ) 


#define IWMDMOperation3_TransferObjectDataOnClearChannel(This,pData,pdwSize)	\
    ( (This)->lpVtbl -> TransferObjectDataOnClearChannel(This,pData,pdwSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMOperation3_INTERFACE_DEFINED__ */


#ifndef __IWMDMProgress_INTERFACE_DEFINED__
#define __IWMDMProgress_INTERFACE_DEFINED__

/* interface IWMDMProgress */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMProgress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A0C-33ED-11d3-8470-00C04F79DBC0")
    IWMDMProgress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin( 
            /* [in] */ DWORD dwEstimatedTicks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Progress( 
            /* [in] */ DWORD dwTranspiredTicks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMProgressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMProgress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMProgress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMProgress * This);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Begin)
        HRESULT ( STDMETHODCALLTYPE *Begin )( 
            __RPC__in IWMDMProgress * This,
            /* [in] */ DWORD dwEstimatedTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Progress)
        HRESULT ( STDMETHODCALLTYPE *Progress )( 
            __RPC__in IWMDMProgress * This,
            /* [in] */ DWORD dwTranspiredTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMProgress * This);
        
        END_INTERFACE
    } IWMDMProgressVtbl;

    interface IWMDMProgress
    {
        CONST_VTBL struct IWMDMProgressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMProgress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMProgress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMProgress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMProgress_Begin(This,dwEstimatedTicks)	\
    ( (This)->lpVtbl -> Begin(This,dwEstimatedTicks) ) 

#define IWMDMProgress_Progress(This,dwTranspiredTicks)	\
    ( (This)->lpVtbl -> Progress(This,dwTranspiredTicks) ) 

#define IWMDMProgress_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMProgress_INTERFACE_DEFINED__ */


#ifndef __IWMDMProgress2_INTERFACE_DEFINED__
#define __IWMDMProgress2_INTERFACE_DEFINED__

/* interface IWMDMProgress2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMProgress2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3A43F550-B383-4e92-B04A-E6BBC660FEFC")
    IWMDMProgress2 : public IWMDMProgress
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE End2( 
            /* [in] */ HRESULT hrCompletionCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMProgress2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMProgress2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMProgress2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMProgress2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Begin)
        HRESULT ( STDMETHODCALLTYPE *Begin )( 
            __RPC__in IWMDMProgress2 * This,
            /* [in] */ DWORD dwEstimatedTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Progress)
        HRESULT ( STDMETHODCALLTYPE *Progress )( 
            __RPC__in IWMDMProgress2 * This,
            /* [in] */ DWORD dwTranspiredTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMProgress2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMProgress2, End2)
        HRESULT ( STDMETHODCALLTYPE *End2 )( 
            __RPC__in IWMDMProgress2 * This,
            /* [in] */ HRESULT hrCompletionCode);
        
        END_INTERFACE
    } IWMDMProgress2Vtbl;

    interface IWMDMProgress2
    {
        CONST_VTBL struct IWMDMProgress2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMProgress2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMProgress2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMProgress2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMProgress2_Begin(This,dwEstimatedTicks)	\
    ( (This)->lpVtbl -> Begin(This,dwEstimatedTicks) ) 

#define IWMDMProgress2_Progress(This,dwTranspiredTicks)	\
    ( (This)->lpVtbl -> Progress(This,dwTranspiredTicks) ) 

#define IWMDMProgress2_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 


#define IWMDMProgress2_End2(This,hrCompletionCode)	\
    ( (This)->lpVtbl -> End2(This,hrCompletionCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMProgress2_INTERFACE_DEFINED__ */


#ifndef __IWMDMProgress3_INTERFACE_DEFINED__
#define __IWMDMProgress3_INTERFACE_DEFINED__

/* interface IWMDMProgress3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMProgress3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("21DE01CB-3BB4-4929-B21A-17AF3F80F658")
    IWMDMProgress3 : public IWMDMProgress2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin3( 
            /* [in] */ GUID EventId,
            /* [in] */ DWORD dwEstimatedTicks,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Progress3( 
            /* [in] */ GUID EventId,
            /* [in] */ DWORD dwTranspiredTicks,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End3( 
            /* [in] */ GUID EventId,
            /* [in] */ HRESULT hrCompletionCode,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMProgress3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMProgress3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMProgress3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Begin)
        HRESULT ( STDMETHODCALLTYPE *Begin )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ DWORD dwEstimatedTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, Progress)
        HRESULT ( STDMETHODCALLTYPE *Progress )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ DWORD dwTranspiredTicks);
        
        DECLSPEC_XFGVIRT(IWMDMProgress, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IWMDMProgress3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMProgress2, End2)
        HRESULT ( STDMETHODCALLTYPE *End2 )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ HRESULT hrCompletionCode);
        
        DECLSPEC_XFGVIRT(IWMDMProgress3, Begin3)
        HRESULT ( STDMETHODCALLTYPE *Begin3 )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ GUID EventId,
            /* [in] */ DWORD dwEstimatedTicks,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext);
        
        DECLSPEC_XFGVIRT(IWMDMProgress3, Progress3)
        HRESULT ( STDMETHODCALLTYPE *Progress3 )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ GUID EventId,
            /* [in] */ DWORD dwTranspiredTicks,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext);
        
        DECLSPEC_XFGVIRT(IWMDMProgress3, End3)
        HRESULT ( STDMETHODCALLTYPE *End3 )( 
            __RPC__in IWMDMProgress3 * This,
            /* [in] */ GUID EventId,
            /* [in] */ HRESULT hrCompletionCode,
            /* [unique][out][in] */ __RPC__inout_opt OPAQUECOMMAND *pContext);
        
        END_INTERFACE
    } IWMDMProgress3Vtbl;

    interface IWMDMProgress3
    {
        CONST_VTBL struct IWMDMProgress3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMProgress3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMProgress3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMProgress3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMProgress3_Begin(This,dwEstimatedTicks)	\
    ( (This)->lpVtbl -> Begin(This,dwEstimatedTicks) ) 

#define IWMDMProgress3_Progress(This,dwTranspiredTicks)	\
    ( (This)->lpVtbl -> Progress(This,dwTranspiredTicks) ) 

#define IWMDMProgress3_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 


#define IWMDMProgress3_End2(This,hrCompletionCode)	\
    ( (This)->lpVtbl -> End2(This,hrCompletionCode) ) 


#define IWMDMProgress3_Begin3(This,EventId,dwEstimatedTicks,pContext)	\
    ( (This)->lpVtbl -> Begin3(This,EventId,dwEstimatedTicks,pContext) ) 

#define IWMDMProgress3_Progress3(This,EventId,dwTranspiredTicks,pContext)	\
    ( (This)->lpVtbl -> Progress3(This,EventId,dwTranspiredTicks,pContext) ) 

#define IWMDMProgress3_End3(This,EventId,hrCompletionCode,pContext)	\
    ( (This)->lpVtbl -> End3(This,EventId,hrCompletionCode,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMProgress3_INTERFACE_DEFINED__ */


#ifndef __IWMDMDevice_INTERFACE_DEFINED__
#define __IWMDMDevice_INTERFACE_DEFINED__

/* interface IWMDMDevice */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A02-33ED-11d3-8470-00C04F79DBC0")
    IWMDMDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetManufacturer( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [out] */ __RPC__out DWORD *pdwVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out DWORD *pdwType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSerialNumber( 
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPowerSource( 
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceIcon( 
            /* [out] */ __RPC__out ULONG *hIcon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStorage( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatSupport( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **ppFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendOpaqueCommand( 
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMDevice * This);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMDevice * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IWMDMDevice * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMDevice * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IWMDMDevice * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **ppFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMDevice * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        END_INTERFACE
    } IWMDMDeviceVtbl;

    interface IWMDMDevice
    {
        CONST_VTBL struct IWMDMDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMDevice_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMDevice_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IWMDMDevice_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IWMDMDevice_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IWMDMDevice_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IWMDMDevice_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IWMDMDevice_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IWMDMDevice_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IWMDMDevice_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IWMDMDevice_GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IWMDMDevice_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMDevice_INTERFACE_DEFINED__ */


#ifndef __IWMDMDevice2_INTERFACE_DEFINED__
#define __IWMDMDevice2_INTERFACE_DEFINED__

/* interface IWMDMDevice2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMDevice2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E34F3D37-9D67-4fc1-9252-62D28B2F8B55")
    IWMDMDevice2 : public IWMDMDevice
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStorage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatSupport2( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpecifyPropertyPages( 
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCanonicalName( 
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMDevice2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMDevice2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMDevice2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMDevice2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMDevice2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IWMDMDevice2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IWMDMDevice2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **ppFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMDevice2 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IWMDMDevice2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetFormatSupport2)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport2 )( 
            __RPC__in IWMDMDevice2 * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetSpecifyPropertyPages)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifyPropertyPages )( 
            __RPC__in IWMDMDevice2 * This,
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IWMDMDevice2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars);
        
        END_INTERFACE
    } IWMDMDevice2Vtbl;

    interface IWMDMDevice2
    {
        CONST_VTBL struct IWMDMDevice2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMDevice2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMDevice2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMDevice2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMDevice2_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMDevice2_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IWMDMDevice2_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IWMDMDevice2_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IWMDMDevice2_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IWMDMDevice2_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IWMDMDevice2_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IWMDMDevice2_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IWMDMDevice2_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IWMDMDevice2_GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IWMDMDevice2_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IWMDMDevice2_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IWMDMDevice2_GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount) ) 

#define IWMDMDevice2_GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks)	\
    ( (This)->lpVtbl -> GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks) ) 

#define IWMDMDevice2_GetCanonicalName(This,pwszPnPName,nMaxChars)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,pwszPnPName,nMaxChars) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMDevice2_INTERFACE_DEFINED__ */


#ifndef __IWMDMDevice3_INTERFACE_DEFINED__
#define __IWMDMDevice3_INTERFACE_DEFINED__

/* interface IWMDMDevice3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMDevice3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6c03e4fe-05db-4dda-9e3c-06233a6d5d65")
    IWMDMDevice3 : public IWMDMDevice2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [out] */ __RPC__out PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [in] */ __RPC__in const PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatCapability( 
            /* [in] */ WMDM_FORMATCODE format,
            /* [out] */ __RPC__out WMDM_FORMAT_CAPABILITY *pFormatSupport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceIoControl( 
            /* [in] */ DWORD dwIoControlCode,
            /* [size_is][in] */ __RPC__in_ecount_full(nInBufferSize) BYTE *lpInBuffer,
            /* [in] */ DWORD nInBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(*pnOutBufferSize) BYTE *lpOutBuffer,
            /* [out][in] */ __RPC__inout LPDWORD pnOutBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindStorage( 
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMDevice3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMDevice3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMDevice3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IWMDMDevice3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IWMDMDevice3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IWMDMDevice3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **ppFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IWMDMDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IWMDMDevice3 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IWMDMDevice3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetFormatSupport2)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport2 )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetSpecifyPropertyPages)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifyPropertyPages )( 
            __RPC__in IWMDMDevice3 * This,
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks);
        
        DECLSPEC_XFGVIRT(IWMDMDevice2, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IWMDMDevice3 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IWMDMDevice3, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [out] */ __RPC__out PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IWMDMDevice3, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [in] */ __RPC__in const PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IWMDMDevice3, GetFormatCapability)
        HRESULT ( STDMETHODCALLTYPE *GetFormatCapability )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ WMDM_FORMATCODE format,
            /* [out] */ __RPC__out WMDM_FORMAT_CAPABILITY *pFormatSupport);
        
        DECLSPEC_XFGVIRT(IWMDMDevice3, DeviceIoControl)
        HRESULT ( STDMETHODCALLTYPE *DeviceIoControl )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ DWORD dwIoControlCode,
            /* [size_is][in] */ __RPC__in_ecount_full(nInBufferSize) BYTE *lpInBuffer,
            /* [in] */ DWORD nInBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(*pnOutBufferSize) BYTE *lpOutBuffer,
            /* [out][in] */ __RPC__inout LPDWORD pnOutBufferSize);
        
        DECLSPEC_XFGVIRT(IWMDMDevice3, FindStorage)
        HRESULT ( STDMETHODCALLTYPE *FindStorage )( 
            __RPC__in IWMDMDevice3 * This,
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppStorage);
        
        END_INTERFACE
    } IWMDMDevice3Vtbl;

    interface IWMDMDevice3
    {
        CONST_VTBL struct IWMDMDevice3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMDevice3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMDevice3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMDevice3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMDevice3_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IWMDMDevice3_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IWMDMDevice3_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IWMDMDevice3_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IWMDMDevice3_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IWMDMDevice3_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IWMDMDevice3_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IWMDMDevice3_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IWMDMDevice3_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IWMDMDevice3_GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,ppFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IWMDMDevice3_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IWMDMDevice3_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IWMDMDevice3_GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount) ) 

#define IWMDMDevice3_GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks)	\
    ( (This)->lpVtbl -> GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks) ) 

#define IWMDMDevice3_GetCanonicalName(This,pwszPnPName,nMaxChars)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,pwszPnPName,nMaxChars) ) 


#define IWMDMDevice3_GetProperty(This,pwszPropName,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,pwszPropName,pValue) ) 

#define IWMDMDevice3_SetProperty(This,pwszPropName,pValue)	\
    ( (This)->lpVtbl -> SetProperty(This,pwszPropName,pValue) ) 

#define IWMDMDevice3_GetFormatCapability(This,format,pFormatSupport)	\
    ( (This)->lpVtbl -> GetFormatCapability(This,format,pFormatSupport) ) 

#define IWMDMDevice3_DeviceIoControl(This,dwIoControlCode,lpInBuffer,nInBufferSize,lpOutBuffer,pnOutBufferSize)	\
    ( (This)->lpVtbl -> DeviceIoControl(This,dwIoControlCode,lpInBuffer,nInBufferSize,lpOutBuffer,pnOutBufferSize) ) 

#define IWMDMDevice3_FindStorage(This,findScope,pwszUniqueID,ppStorage)	\
    ( (This)->lpVtbl -> FindStorage(This,findScope,pwszUniqueID,ppStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMDevice3_INTERFACE_DEFINED__ */


#ifndef __IWMDMDeviceSession_INTERFACE_DEFINED__
#define __IWMDMDeviceSession_INTERFACE_DEFINED__

/* interface IWMDMDeviceSession */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMDeviceSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82af0a65-9d96-412c-83e5-3c43e4b06cc7")
    IWMDMDeviceSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginSession( 
            /* [in] */ WMDM_SESSION_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndSession( 
            /* [in] */ WMDM_SESSION_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMDeviceSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMDeviceSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMDeviceSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMDeviceSession * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceSession, BeginSession)
        HRESULT ( STDMETHODCALLTYPE *BeginSession )( 
            __RPC__in IWMDMDeviceSession * This,
            /* [in] */ WMDM_SESSION_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceSession, EndSession)
        HRESULT ( STDMETHODCALLTYPE *EndSession )( 
            __RPC__in IWMDMDeviceSession * This,
            /* [in] */ WMDM_SESSION_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx);
        
        END_INTERFACE
    } IWMDMDeviceSessionVtbl;

    interface IWMDMDeviceSession
    {
        CONST_VTBL struct IWMDMDeviceSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMDeviceSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMDeviceSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMDeviceSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMDeviceSession_BeginSession(This,type,pCtx,dwSizeCtx)	\
    ( (This)->lpVtbl -> BeginSession(This,type,pCtx,dwSizeCtx) ) 

#define IWMDMDeviceSession_EndSession(This,type,pCtx,dwSizeCtx)	\
    ( (This)->lpVtbl -> EndSession(This,type,pCtx,dwSizeCtx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMDeviceSession_INTERFACE_DEFINED__ */


#ifndef __IWMDMEnumDevice_INTERFACE_DEFINED__
#define __IWMDMEnumDevice_INTERFACE_DEFINED__

/* interface IWMDMEnumDevice */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMEnumDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A01-33ED-11d3-8470-00C04F79DBC0")
    IWMDMEnumDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWMDMDevice **ppDevice,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMEnumDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMEnumDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IWMDMEnumDevice, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IWMDMEnumDevice * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWMDMDevice **ppDevice,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IWMDMEnumDevice, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IWMDMEnumDevice * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IWMDMEnumDevice, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IWMDMEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IWMDMEnumDevice, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWMDMEnumDevice * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumDevice **ppEnumDevice);
        
        END_INTERFACE
    } IWMDMEnumDeviceVtbl;

    interface IWMDMEnumDevice
    {
        CONST_VTBL struct IWMDMEnumDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMEnumDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMEnumDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMEnumDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMEnumDevice_Next(This,celt,ppDevice,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppDevice,pceltFetched) ) 

#define IWMDMEnumDevice_Skip(This,celt,pceltFetched)	\
    ( (This)->lpVtbl -> Skip(This,celt,pceltFetched) ) 

#define IWMDMEnumDevice_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IWMDMEnumDevice_Clone(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> Clone(This,ppEnumDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMEnumDevice_INTERFACE_DEFINED__ */


#ifndef __IWMDMDeviceControl_INTERFACE_DEFINED__
#define __IWMDMDeviceControl_INTERFACE_DEFINED__

/* interface IWMDMDeviceControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMDeviceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A04-33ED-11d3-8470-00C04F79DBC0")
    IWMDMDeviceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCapabilitiesMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Play( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Record( 
            /* [in] */ __RPC__in _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ UINT fuMode,
            /* [in] */ int nOffset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMDeviceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMDeviceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IWMDMDeviceControl * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IWMDMDeviceControl * This,
            /* [out] */ __RPC__out DWORD *pdwCapabilitiesMask);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Play)
        HRESULT ( STDMETHODCALLTYPE *Play )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Record)
        HRESULT ( STDMETHODCALLTYPE *Record )( 
            __RPC__in IWMDMDeviceControl * This,
            /* [in] */ __RPC__in _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWMDMDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMDeviceControl, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IWMDMDeviceControl * This,
            /* [in] */ UINT fuMode,
            /* [in] */ int nOffset);
        
        END_INTERFACE
    } IWMDMDeviceControlVtbl;

    interface IWMDMDeviceControl
    {
        CONST_VTBL struct IWMDMDeviceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMDeviceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMDeviceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMDeviceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMDeviceControl_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IWMDMDeviceControl_GetCapabilities(This,pdwCapabilitiesMask)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilitiesMask) ) 

#define IWMDMDeviceControl_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IWMDMDeviceControl_Record(This,pFormat)	\
    ( (This)->lpVtbl -> Record(This,pFormat) ) 

#define IWMDMDeviceControl_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IWMDMDeviceControl_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IWMDMDeviceControl_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWMDMDeviceControl_Seek(This,fuMode,nOffset)	\
    ( (This)->lpVtbl -> Seek(This,fuMode,nOffset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMDeviceControl_INTERFACE_DEFINED__ */


#ifndef __IWMDMEnumStorage_INTERFACE_DEFINED__
#define __IWMDMEnumStorage_INTERFACE_DEFINED__

/* interface IWMDMEnumStorage */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMEnumStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A05-33ED-11d3-8470-00C04F79DBC0")
    IWMDMEnumStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWMDMStorage **ppStorage,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMEnumStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMEnumStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IWMDMEnumStorage, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IWMDMEnumStorage * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWMDMStorage **ppStorage,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IWMDMEnumStorage, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IWMDMEnumStorage * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IWMDMEnumStorage, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IWMDMEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IWMDMEnumStorage, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IWMDMEnumStorage * This,
            /* [out] */ __RPC__deref_out_opt IWMDMEnumStorage **ppEnumStorage);
        
        END_INTERFACE
    } IWMDMEnumStorageVtbl;

    interface IWMDMEnumStorage
    {
        CONST_VTBL struct IWMDMEnumStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMEnumStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMEnumStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMEnumStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMEnumStorage_Next(This,celt,ppStorage,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppStorage,pceltFetched) ) 

#define IWMDMEnumStorage_Skip(This,celt,pceltFetched)	\
    ( (This)->lpVtbl -> Skip(This,celt,pceltFetched) ) 

#define IWMDMEnumStorage_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IWMDMEnumStorage_Clone(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> Clone(This,ppEnumStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMEnumStorage_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorageControl_INTERFACE_DEFINED__
#define __IWMDMStorageControl_INTERFACE_DEFINED__

/* interface IWMDMStorageControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorageControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A08-33ED-11d3-8470-00C04F79DBC0")
    IWMDMStorageControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Insert( 
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppNewObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IWMDMOperation *pOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMStorage *pTargetObject,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorageControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorageControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorageControl * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Insert)
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IWMDMOperation *pOperation);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IWMDMStorageControl * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMStorage *pTargetObject,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        END_INTERFACE
    } IWMDMStorageControlVtbl;

    interface IWMDMStorageControl
    {
        CONST_VTBL struct IWMDMStorageControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorageControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorageControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorageControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorageControl_Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject)	\
    ( (This)->lpVtbl -> Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject) ) 

#define IWMDMStorageControl_Delete(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Delete(This,fuMode,pProgress) ) 

#define IWMDMStorageControl_Rename(This,fuMode,pwszNewName,pProgress)	\
    ( (This)->lpVtbl -> Rename(This,fuMode,pwszNewName,pProgress) ) 

#define IWMDMStorageControl_Read(This,fuMode,pwszFile,pProgress,pOperation)	\
    ( (This)->lpVtbl -> Read(This,fuMode,pwszFile,pProgress,pOperation) ) 

#define IWMDMStorageControl_Move(This,fuMode,pTargetObject,pProgress)	\
    ( (This)->lpVtbl -> Move(This,fuMode,pTargetObject,pProgress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorageControl_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorageControl2_INTERFACE_DEFINED__
#define __IWMDMStorageControl2_INTERFACE_DEFINED__

/* interface IWMDMStorageControl2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorageControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("972C2E88-BD6C-4125-8E09-84F837E637B6")
    IWMDMStorageControl2 : public IWMDMStorageControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Insert2( 
            /* [in] */ UINT fuMode,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileSource,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileDest,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IWMDMStorage **ppNewObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorageControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorageControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorageControl2 * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Insert)
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IWMDMOperation *pOperation);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMStorage *pTargetObject,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl2, Insert2)
        HRESULT ( STDMETHODCALLTYPE *Insert2 )( 
            __RPC__in IWMDMStorageControl2 * This,
            /* [in] */ UINT fuMode,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileSource,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileDest,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IWMDMStorage **ppNewObject);
        
        END_INTERFACE
    } IWMDMStorageControl2Vtbl;

    interface IWMDMStorageControl2
    {
        CONST_VTBL struct IWMDMStorageControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorageControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorageControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorageControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorageControl2_Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject)	\
    ( (This)->lpVtbl -> Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject) ) 

#define IWMDMStorageControl2_Delete(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Delete(This,fuMode,pProgress) ) 

#define IWMDMStorageControl2_Rename(This,fuMode,pwszNewName,pProgress)	\
    ( (This)->lpVtbl -> Rename(This,fuMode,pwszNewName,pProgress) ) 

#define IWMDMStorageControl2_Read(This,fuMode,pwszFile,pProgress,pOperation)	\
    ( (This)->lpVtbl -> Read(This,fuMode,pwszFile,pProgress,pOperation) ) 

#define IWMDMStorageControl2_Move(This,fuMode,pTargetObject,pProgress)	\
    ( (This)->lpVtbl -> Move(This,fuMode,pTargetObject,pProgress) ) 


#define IWMDMStorageControl2_Insert2(This,fuMode,pwszFileSource,pwszFileDest,pOperation,pProgress,pUnknown,ppNewObject)	\
    ( (This)->lpVtbl -> Insert2(This,fuMode,pwszFileSource,pwszFileDest,pOperation,pProgress,pUnknown,ppNewObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorageControl2_INTERFACE_DEFINED__ */


#ifndef __IWMDMStorageControl3_INTERFACE_DEFINED__
#define __IWMDMStorageControl3_INTERFACE_DEFINED__

/* interface IWMDMStorageControl3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMStorageControl3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B3266365-D4F3-4696-8D53-BD27EC60993A")
    IWMDMStorageControl3 : public IWMDMStorageControl2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Insert3( 
            /* [in] */ UINT fuMode,
            /* [in] */ UINT fuType,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileSource,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileDest,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [unique][in] */ __RPC__in_opt IWMDMMetaData *pMetaData,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IWMDMStorage **ppNewObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMStorageControl3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMStorageControl3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMStorageControl3 * This);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Insert)
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [out] */ __RPC__deref_out_opt IWMDMStorage **ppNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [unique][in] */ __RPC__in_opt LPWSTR pwszFile,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IWMDMOperation *pOperation);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMStorage *pTargetObject,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl2, Insert2)
        HRESULT ( STDMETHODCALLTYPE *Insert2 )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileSource,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileDest,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IWMDMStorage **ppNewObject);
        
        DECLSPEC_XFGVIRT(IWMDMStorageControl3, Insert3)
        HRESULT ( STDMETHODCALLTYPE *Insert3 )( 
            __RPC__in IWMDMStorageControl3 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ UINT fuType,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileSource,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszFileDest,
            /* [unique][in] */ __RPC__in_opt IWMDMOperation *pOperation,
            /* [unique][in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [unique][in] */ __RPC__in_opt IWMDMMetaData *pMetaData,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IWMDMStorage **ppNewObject);
        
        END_INTERFACE
    } IWMDMStorageControl3Vtbl;

    interface IWMDMStorageControl3
    {
        CONST_VTBL struct IWMDMStorageControl3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMStorageControl3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMStorageControl3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMStorageControl3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMStorageControl3_Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject)	\
    ( (This)->lpVtbl -> Insert(This,fuMode,pwszFile,pOperation,pProgress,ppNewObject) ) 

#define IWMDMStorageControl3_Delete(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Delete(This,fuMode,pProgress) ) 

#define IWMDMStorageControl3_Rename(This,fuMode,pwszNewName,pProgress)	\
    ( (This)->lpVtbl -> Rename(This,fuMode,pwszNewName,pProgress) ) 

#define IWMDMStorageControl3_Read(This,fuMode,pwszFile,pProgress,pOperation)	\
    ( (This)->lpVtbl -> Read(This,fuMode,pwszFile,pProgress,pOperation) ) 

#define IWMDMStorageControl3_Move(This,fuMode,pTargetObject,pProgress)	\
    ( (This)->lpVtbl -> Move(This,fuMode,pTargetObject,pProgress) ) 


#define IWMDMStorageControl3_Insert2(This,fuMode,pwszFileSource,pwszFileDest,pOperation,pProgress,pUnknown,ppNewObject)	\
    ( (This)->lpVtbl -> Insert2(This,fuMode,pwszFileSource,pwszFileDest,pOperation,pProgress,pUnknown,ppNewObject) ) 


#define IWMDMStorageControl3_Insert3(This,fuMode,fuType,pwszFileSource,pwszFileDest,pOperation,pProgress,pMetaData,pUnknown,ppNewObject)	\
    ( (This)->lpVtbl -> Insert3(This,fuMode,fuType,pwszFileSource,pwszFileDest,pOperation,pProgress,pMetaData,pUnknown,ppNewObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMStorageControl3_INTERFACE_DEFINED__ */


#ifndef __IWMDMObjectInfo_INTERFACE_DEFINED__
#define __IWMDMObjectInfo_INTERFACE_DEFINED__

/* interface IWMDMObjectInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMObjectInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A09-33ED-11d3-8470-00C04F79DBC0")
    IWMDMObjectInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPlayLength( 
            /* [out] */ __RPC__out DWORD *pdwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlayLength( 
            /* [in] */ DWORD dwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlayOffset( 
            /* [out] */ __RPC__out DWORD *pdwOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlayOffset( 
            /* [in] */ DWORD dwOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalLength( 
            /* [out] */ __RPC__out DWORD *pdwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastPlayPosition( 
            /* [out] */ __RPC__out DWORD *pdwLastPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLongestPlayPosition( 
            /* [out] */ __RPC__out DWORD *pdwLongestPos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMObjectInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, GetPlayLength)
        HRESULT ( STDMETHODCALLTYPE *GetPlayLength )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLength);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, SetPlayLength)
        HRESULT ( STDMETHODCALLTYPE *SetPlayLength )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [in] */ DWORD dwLength);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, GetPlayOffset)
        HRESULT ( STDMETHODCALLTYPE *GetPlayOffset )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwOffset);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, SetPlayOffset)
        HRESULT ( STDMETHODCALLTYPE *SetPlayOffset )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [in] */ DWORD dwOffset);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, GetTotalLength)
        HRESULT ( STDMETHODCALLTYPE *GetTotalLength )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLength);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, GetLastPlayPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLastPlayPosition )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLastPos);
        
        DECLSPEC_XFGVIRT(IWMDMObjectInfo, GetLongestPlayPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLongestPlayPosition )( 
            __RPC__in IWMDMObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLongestPos);
        
        END_INTERFACE
    } IWMDMObjectInfoVtbl;

    interface IWMDMObjectInfo
    {
        CONST_VTBL struct IWMDMObjectInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMObjectInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMObjectInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMObjectInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMObjectInfo_GetPlayLength(This,pdwLength)	\
    ( (This)->lpVtbl -> GetPlayLength(This,pdwLength) ) 

#define IWMDMObjectInfo_SetPlayLength(This,dwLength)	\
    ( (This)->lpVtbl -> SetPlayLength(This,dwLength) ) 

#define IWMDMObjectInfo_GetPlayOffset(This,pdwOffset)	\
    ( (This)->lpVtbl -> GetPlayOffset(This,pdwOffset) ) 

#define IWMDMObjectInfo_SetPlayOffset(This,dwOffset)	\
    ( (This)->lpVtbl -> SetPlayOffset(This,dwOffset) ) 

#define IWMDMObjectInfo_GetTotalLength(This,pdwLength)	\
    ( (This)->lpVtbl -> GetTotalLength(This,pdwLength) ) 

#define IWMDMObjectInfo_GetLastPlayPosition(This,pdwLastPos)	\
    ( (This)->lpVtbl -> GetLastPlayPosition(This,pdwLastPos) ) 

#define IWMDMObjectInfo_GetLongestPlayPosition(This,pdwLongestPos)	\
    ( (This)->lpVtbl -> GetLongestPlayPosition(This,pdwLongestPos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMObjectInfo_INTERFACE_DEFINED__ */


#ifndef __IWMDMRevoked_INTERFACE_DEFINED__
#define __IWMDMRevoked_INTERFACE_DEFINED__

/* interface IWMDMRevoked */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IWMDMRevoked;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EBECCEDB-88EE-4e55-B6A4-8D9F07D696AA")
    IWMDMRevoked : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRevocationURL( 
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwBufferLen) LPWSTR *ppwszRevocationURL,
            /* [out][in] */ __RPC__inout DWORD *pdwBufferLen,
            /* [out] */ __RPC__out DWORD *pdwRevokedBitFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMRevokedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMRevoked * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMRevoked * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMRevoked * This);
        
        DECLSPEC_XFGVIRT(IWMDMRevoked, GetRevocationURL)
        HRESULT ( STDMETHODCALLTYPE *GetRevocationURL )( 
            __RPC__in IWMDMRevoked * This,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwBufferLen) LPWSTR *ppwszRevocationURL,
            /* [out][in] */ __RPC__inout DWORD *pdwBufferLen,
            /* [out] */ __RPC__out DWORD *pdwRevokedBitFlag);
        
        END_INTERFACE
    } IWMDMRevokedVtbl;

    interface IWMDMRevoked
    {
        CONST_VTBL struct IWMDMRevokedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMRevoked_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMRevoked_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMRevoked_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMRevoked_GetRevocationURL(This,ppwszRevocationURL,pdwBufferLen,pdwRevokedBitFlag)	\
    ( (This)->lpVtbl -> GetRevocationURL(This,ppwszRevocationURL,pdwBufferLen,pdwRevokedBitFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMRevoked_INTERFACE_DEFINED__ */


#ifndef __IWMDMNotification_INTERFACE_DEFINED__
#define __IWMDMNotification_INTERFACE_DEFINED__

/* interface IWMDMNotification */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWMDMNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3F5E95C0-0F43-4ed4-93D2-C89A45D59B81")
    IWMDMNotification : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WMDMMessage( 
            /* [in] */ DWORD dwMessageType,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszCanonicalName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMDMNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMDMNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMDMNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMDMNotification * This);
        
        DECLSPEC_XFGVIRT(IWMDMNotification, WMDMMessage)
        HRESULT ( STDMETHODCALLTYPE *WMDMMessage )( 
            __RPC__in IWMDMNotification * This,
            /* [in] */ DWORD dwMessageType,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszCanonicalName);
        
        END_INTERFACE
    } IWMDMNotificationVtbl;

    interface IWMDMNotification
    {
        CONST_VTBL struct IWMDMNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMDMNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMDMNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMDMNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMDMNotification_WMDMMessage(This,dwMessageType,pwszCanonicalName)	\
    ( (This)->lpVtbl -> WMDMMessage(This,dwMessageType,pwszCanonicalName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMDMNotification_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mswmdm_0000_0028 */
/* [local] */ 

// WMDM constants for wellknown meta-data tags
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMFileName = L"WMDM/FileName";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMFormatCode = L"WMDM/FormatCode";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMLastModifiedDate = L"WMDM/LastModifiedDate";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMFileCreationDate = L"WMDM/FileCreationDate";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMFileSize = L"WMDM/FileSize";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMFileAttributes = L"WMDM/FileAttributes";
//Format code: WAVE Format
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszAudioWAVECodec = L"WMDM/AudioWAVECodec";
//Format code: FOURCC code 
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszVideoFourCCCodec = L"WMDM/VideoFourCCCodec";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMTitle = L"WMDM/Title";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMAuthor = L"WMDM/Author";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMDescription = L"WMDM/Description";
//Type: BOOL, WMDM_TAG_DATATYPE: WMDM_TYPE_BOOL
static const WCHAR *g_wszWMDMIsProtected = L"WMDM/IsProtected";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMAlbumTitle = L"WMDM/AlbumTitle";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMAlbumArtist = L"WMDM/AlbumArtist";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMTrack = L"WMDM/Track";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMGenre = L"WMDM/Genre";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMTrackMood = L"WMDM/TrackMood";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAlbumCoverFormat = L"WMDM/AlbumCoverFormat";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAlbumCoverSize = L"WMDM/AlbumCoverSize";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAlbumCoverHeight = L"WMDM/AlbumCoverHeight";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAlbumCoverWidth = L"WMDM/AlbumCoverWidth";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAlbumCoverDuration = L"WMDM/AlbumCoverDuration";
//Type: BYTE*, WMDM_TAG_DATATYPE: WMDM_TYPE_BINARY
static const WCHAR *g_wszWMDMAlbumCoverData = L"WMDM/AlbumCoverData";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMYear = L"WMDM/Year";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMComposer = L"WMDM/Composer";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMCodec = L"WMDM/Codec";
static const WCHAR *g_wszWMDMDRMId = L"WMDM/DRMId";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMBitrate = L"WMDM/Bitrate";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMBitRateType = L"WMDM/BitRateType";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMSampleRate = L"WMDM/SampleRate";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMNumChannels = L"WMDM/NumChannels";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMBlockAlignment = L"WMDM/BlockAlignment";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMAudioBitDepth = L"WMDM/AudioBitDepth";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMTotalBitrate = L"WMDM/TotalBitrate";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMVideoBitrate = L"WMDM/VideoBitrate";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMFrameRate = L"WMDM/FrameRate";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMScanType = L"WMDM/ScanType";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMKeyFrameDistance = L"WMDM/KeyFrameDistance";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMBufferSize = L"WMDM/BufferSize";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMQualitySetting = L"WMDM/QualitySetting";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMEncodingProfile = L"WMDM/EncodingProfile";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMDuration = L"WMDM/Duration";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMAlbumArt = L"WMDM/AlbumArt";
//Type: BOOL, WMDM_TAG_DATATYPE: WMDM_TYPE_BOOL
static const WCHAR *g_wszWMDMBuyNow = L"WMDM/BuyNow";
//Type: BOOL, WMDM_TAG_DATATYPE: WMDM_TYPE_BOOL
static const WCHAR *g_wszWMDMNonConsumable = L"WMDM/NonConsumable";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMediaClassPrimaryID = L"WMDM/MediaClassPrimaryID";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMediaClassSecondaryID = L"WMDM/MediaClassSecondaryID";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMUserEffectiveRating = L"WMDM/UserEffectiveRating";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMUserRating = L"WMDM/UserRating";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMUserRatingOnDevice = L"WMDM/UserRatingOnDevice";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMPlayCount = L"WMDM/PlayCount";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMDevicePlayCount = L"WMDM/DevicePlayCount";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMAuthorDate = L"WMDM/AuthorDate";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMUserLastPlayTime = L"WMDM/UserLastPlayTime";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMSubTitle = L"WMDM/SubTitle";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMSubTitleDescription = L"WMDM/SubTitleDescription";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMediaCredits = L"WMDM/MediaCredits";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMediaStationName = L"WMDM/MediaStationName";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMediaOriginalChannel = L"WMDM/MediaOriginalChannel";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMMediaOriginalBroadcastDateTime = L"WMDM/MediaOriginalBroadcastDateTime";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMProviderCopyright = L"WMDM/ProviderCopyright";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMSyncID = L"WMDM/SyncID";
//Type: GUID, WMDM_TAG_DATATYPE: WMDM_TYPE_GUID
static const WCHAR *g_wszWMDMPersistentUniqueID = L"WMDM/PersistentUniqueID";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMWidth = L"WMDM/Width";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMHeight = L"WMDM/Height";
//Type: WMDMDATETIME, WMDM_TAG_DATATYPE: WMDM_TYPE_DATETIME
static const WCHAR *g_wszWMDMSyncTime = L"WMDM/SyncTime";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMParentalRating = L"WMDM/ParentalRating";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMetaGenre = L"WMDM/MetaGenre";
//Type: BOOL, WMDM_TAG_DATATYPE: WMDM_TYPE_BOOL
static const WCHAR *g_wszWMDMIsRepeat = L"WMDM/IsRepeat";
// Device properties
//PROPVARIANT vt = VT_BSTR | VT_ARRAY
static const WCHAR *g_wszWMDMSupportedDeviceProperties = L"WMDM/SupportedDeviceProperties";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMDeviceFriendlyName = L"WMDM/DeviceFriendlyName";
//PROPVARIANT vt = VT_UI4 | VT_ARRAY
static const WCHAR *g_wszWMDMFormatsSupported = L"WMDM/FormatsSupported";
//PROPVARIANT vt = VT_BOOL
static const WCHAR *g_wszWMDMFormatsSupportedAreOrdered = L"WMDM/FormatsSupportedAreOrdered";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMSyncRelationshipID = L"WMDM/SyncRelationshipID";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMDeviceModelName = L"WMDM/DeviceModelName";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMDeviceFirmwareVersion = L"WMDM/DeviceFirmwareVersion";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMDeviceVendorExtension = L"WMDM/DeviceVendorExtension";
//PROPVARIANT vt = VT_CLSID
static const WCHAR *g_wszWMDMDeviceProtocol = L"WMDM/DeviceProtocol";
//PROPVARIANT vt = VT_CLSID
static const WCHAR *g_wszWMDMDeviceServiceProviderVendor = L"WMDM/DeviceServiceProviderVendor";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMDeviceRevocationInfo = L"WMDM/DeviceRevocationInfo";
//PROPVARIANT vt = VT_BSTR
static const WCHAR *g_wszWMDMCollectionID = L"WMDM/CollectionID";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMOwner = L"WMDM/Owner";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMEditor = L"WMDM/Editor";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMWebmaster = L"WMDM/Webmaster";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMSourceURL = L"WMDM/SourceURL";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMDestinationURL = L"WMDM/DestinationURL";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMCategory = L"WMDM/Category";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMTimeBookmark = L"WMDM/TimeBookmark";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMObjectBookmark = L"WMDM/ObjectBookmark";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMByteBookmark = L"WMDM/ByteBookmark";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMDataOffset = L"WMDM/DataOffset";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMDataLength = L"WMDM/DataLength";
//Type: DWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_DWORD
static const WCHAR *g_wszWMDMDataUnits = L"WMDM/DataUnits";
//Type: QWORD, WMDM_TAG_DATATYPE: WMDM_TYPE_QWORD
static const WCHAR *g_wszWMDMTimeToLive = L"WMDM/TimeToLive";
//Type: LPCWSTR, WMDM_TAG_DATATYPE: WMDM_TYPE_STRING
static const WCHAR *g_wszWMDMMediaGuid = L"WMDM/MediaGuid";
//Type: BYTE*, WMDM_TAG_DATATYPE: WMDM_TYPE_BINARY
static const WCHAR *g_wszWPDPassthroughPropertyValues = L"WPD/PassthroughPropertyValues";
#define CCH_WMDM_PROPNAME(sz) (sizeof(sz)/sizeof(sz[0]))
union WMDMDetermineMaxPropStringLen {
WCHAR sz001[CCH_WMDM_PROPNAME(L"WMDM/DeviceFirmwareVersion")];
WCHAR sz002[CCH_WMDM_PROPNAME(L"WMDM/SupportedDeviceProperties")];
WCHAR sz003[CCH_WMDM_PROPNAME(L"WMDM/FileName")];
WCHAR sz004[CCH_WMDM_PROPNAME(L"WMDM/FormatCode")];
WCHAR sz005[CCH_WMDM_PROPNAME(L"WMDM/LastModifiedDate")];
WCHAR sz006[CCH_WMDM_PROPNAME(L"WMDM/FileSize")];
WCHAR sz007[CCH_WMDM_PROPNAME(L"WMDM/FileAttributes")];
WCHAR sz008[CCH_WMDM_PROPNAME(L"WMDM/AudioWAVECodec")];
WCHAR sz009[CCH_WMDM_PROPNAME(L"WMDM/VideoFourCCCodec")];
WCHAR sz010[CCH_WMDM_PROPNAME(L"WMDM/Title")];
WCHAR sz011[CCH_WMDM_PROPNAME(L"WMDM/Author")];
WCHAR sz012[CCH_WMDM_PROPNAME(L"WMDM/Description")];
WCHAR sz013[CCH_WMDM_PROPNAME(L"WMDM/IsProtected")];
WCHAR sz014[CCH_WMDM_PROPNAME(L"WMDM/AlbumTitle")];
WCHAR sz015[CCH_WMDM_PROPNAME(L"WMDM/AlbumArtist")];
WCHAR sz016[CCH_WMDM_PROPNAME(L"WMDM/Track")];
WCHAR sz017[CCH_WMDM_PROPNAME(L"WMDM/Genre")];
WCHAR sz018[CCH_WMDM_PROPNAME(L"WMDM/TrackMood")];
WCHAR sz019[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverFormat")];
WCHAR sz020[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverSize")];
WCHAR sz021[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverHeight")];
WCHAR sz022[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverWidth")];
WCHAR sz023[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverDuration")];
WCHAR sz024[CCH_WMDM_PROPNAME(L"WMDM/AlbumCoverData")];
WCHAR sz025[CCH_WMDM_PROPNAME(L"WMDM/Year")];
WCHAR sz026[CCH_WMDM_PROPNAME(L"WMDM/Composer")];
WCHAR sz027[CCH_WMDM_PROPNAME(L"WMDM/Codec")];
WCHAR sz028[CCH_WMDM_PROPNAME(L"WMDM/DRMId")];
WCHAR sz029[CCH_WMDM_PROPNAME(L"WMDM/Bitrate")];
WCHAR sz030[CCH_WMDM_PROPNAME(L"WMDM/BitRateType")];
WCHAR sz031[CCH_WMDM_PROPNAME(L"WMDM/SampleRate")];
WCHAR sz032[CCH_WMDM_PROPNAME(L"WMDM/NumChannels")];
WCHAR sz033[CCH_WMDM_PROPNAME(L"WMDM/BlockAlignment")];
WCHAR sz034[CCH_WMDM_PROPNAME(L"WMDM/AudioBitDepth")];
WCHAR sz035[CCH_WMDM_PROPNAME(L"WMDM/TotalBitrate")];
WCHAR sz036[CCH_WMDM_PROPNAME(L"WMDM/VideoBitrate")];
WCHAR sz037[CCH_WMDM_PROPNAME(L"WMDM/FrameRate")];
WCHAR sz041[CCH_WMDM_PROPNAME(L"WMDM/ScanType")];
WCHAR sz043[CCH_WMDM_PROPNAME(L"WMDM/KeyFrameDistance")];
WCHAR sz044[CCH_WMDM_PROPNAME(L"WMDM/BufferSize")];
WCHAR sz045[CCH_WMDM_PROPNAME(L"WMDM/QualitySetting")];
WCHAR sz046[CCH_WMDM_PROPNAME(L"WMDM/Duration")];
WCHAR sz047[CCH_WMDM_PROPNAME(L"WMDM/AlbumArt")];
WCHAR sz048[CCH_WMDM_PROPNAME(L"WMDM/BuyNow")];
WCHAR sz049[CCH_WMDM_PROPNAME(L"WMDM/MediaClassPrimaryID")];
WCHAR sz050[CCH_WMDM_PROPNAME(L"WMDM/MediaClassSecondayID")];
WCHAR sz051[CCH_WMDM_PROPNAME(L"WMDM/UserEffectiveRating")];
WCHAR sz052[CCH_WMDM_PROPNAME(L"WMDM/UserRating")];
WCHAR sz053[CCH_WMDM_PROPNAME(L"WMDM/UserRatingOnDevice")];
WCHAR sz054[CCH_WMDM_PROPNAME(L"WMDM/PlayCount")];
WCHAR sz055[CCH_WMDM_PROPNAME(L"WMDM/DevicePlayCount")];
WCHAR sz056[CCH_WMDM_PROPNAME(L"WMDM/AuthorDate")];
WCHAR sz057[CCH_WMDM_PROPNAME(L"WMDM/UserLastPlayTime")];
WCHAR sz058[CCH_WMDM_PROPNAME(L"WMDM/SubTitle")];
WCHAR sz059[CCH_WMDM_PROPNAME(L"WMDM/SubTitleDescription")];
WCHAR sz060[CCH_WMDM_PROPNAME(L"WMDM/MediaCredits")];
WCHAR sz061[CCH_WMDM_PROPNAME(L"WMDM/MediaStationName")];
WCHAR sz062[CCH_WMDM_PROPNAME(L"WMDM/MediaOriginalChannel")];
WCHAR sz063[CCH_WMDM_PROPNAME(L"WMDM/MediaOriginalBroadcastDateTime")];
WCHAR sz064[CCH_WMDM_PROPNAME(L"WMDM/ProviderCopyright")];
WCHAR sz065[CCH_WMDM_PROPNAME(L"WMDM/SyncID")];
WCHAR sz066[CCH_WMDM_PROPNAME(L"WMDM/PersistentUniqueID")];
WCHAR sz067[CCH_WMDM_PROPNAME(L"WMDM/Width")];
WCHAR sz068[CCH_WMDM_PROPNAME(L"WMDM/Height")];
WCHAR sz069[CCH_WMDM_PROPNAME(L"WMDM/SyncTime")];
WCHAR sz070[CCH_WMDM_PROPNAME(L"WMDM/ParentalRating")];
WCHAR sz071[CCH_WMDM_PROPNAME(L"WMDM/MetaGenre")];
WCHAR sz072[CCH_WMDM_PROPNAME(L"WMDM/IsRepeat")];
WCHAR sz073[CCH_WMDM_PROPNAME(L"WMDM/SupportedDeviceProperties")];
WCHAR sz074[CCH_WMDM_PROPNAME(L"WMDM/DeviceFriendlyName")];
WCHAR sz075[CCH_WMDM_PROPNAME(L"WMDM/FormatsSupported")];
WCHAR sz076[CCH_WMDM_PROPNAME(L"WMDM/SyncRelationshipID")];
WCHAR sz077[CCH_WMDM_PROPNAME(L"WMDM/DeviceModelName")];
WCHAR sz078[CCH_WMDM_PROPNAME(L"WMDM/DeviceFirmwareVersion")];
WCHAR sz079[CCH_WMDM_PROPNAME(L"WMDM/DeviceVendorExtension")];
WCHAR sz080[CCH_WMDM_PROPNAME(L"WMDM/DeviceProtocol")];
WCHAR sz081[CCH_WMDM_PROPNAME(L"WMDM/DeviceServiceProviderVendor")];
WCHAR sz082[CCH_WMDM_PROPNAME(L"WMDM/EncodingProfile")];
WCHAR sz083[CCH_WMDM_PROPNAME(L"WMDM/FormatsSupportedAreOrdered")];
WCHAR sz084[CCH_WMDM_PROPNAME(L"WMDM/DeviceRevocationInfo")];
WCHAR sz085[CCH_WMDM_PROPNAME(L"WMDM/CollectionID")];
WCHAR sz086[CCH_WMDM_PROPNAME(L"WPD/PassthroughPropertyValues")];
};
#define WMDM_MAXLEN_PROPERTYNAME (sizeof(WMDMDetermineMaxPropStringLen)/sizeof(WCHAR))
// Open Mode Flags
#define MDSP_READ                               0x00000001
#define MDSP_WRITE                              0x00000002
// Seek Flags
#define MDSP_SEEK_BOF                           0x00000001
#define MDSP_SEEK_CUR                           0x00000002
#define MDSP_SEEK_EOF                           0x00000004











extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0028_v0_0_s_ifspec;

#ifndef __IMDServiceProvider_INTERFACE_DEFINED__
#define __IMDServiceProvider_INTERFACE_DEFINED__

/* interface IMDServiceProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDServiceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A10-33ED-11d3-8470-00C04F79DBC0")
    IMDServiceProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDevices( 
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDServiceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDServiceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IMDServiceProvider * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IMDServiceProvider * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice);
        
        END_INTERFACE
    } IMDServiceProviderVtbl;

    interface IMDServiceProvider
    {
        CONST_VTBL struct IMDServiceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDServiceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDServiceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDServiceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDServiceProvider_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IMDServiceProvider_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDServiceProvider_INTERFACE_DEFINED__ */


#ifndef __IMDServiceProvider2_INTERFACE_DEFINED__
#define __IMDServiceProvider2_INTERFACE_DEFINED__

/* interface IMDServiceProvider2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDServiceProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B2FA24B7-CDA3-4694-9862-413AE1A34819")
    IMDServiceProvider2 : public IMDServiceProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateDevice( 
            /* [string][in] */ __RPC__in_string LPCWSTR pwszDevicePath,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwCount) IMDSPDevice ***pppDeviceArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDServiceProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDServiceProvider2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDServiceProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDServiceProvider2 * This);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IMDServiceProvider2 * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IMDServiceProvider2 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider2, CreateDevice)
        HRESULT ( STDMETHODCALLTYPE *CreateDevice )( 
            __RPC__in IMDServiceProvider2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszDevicePath,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwCount) IMDSPDevice ***pppDeviceArray);
        
        END_INTERFACE
    } IMDServiceProvider2Vtbl;

    interface IMDServiceProvider2
    {
        CONST_VTBL struct IMDServiceProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDServiceProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDServiceProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDServiceProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDServiceProvider2_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IMDServiceProvider2_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 


#define IMDServiceProvider2_CreateDevice(This,pwszDevicePath,pdwCount,pppDeviceArray)	\
    ( (This)->lpVtbl -> CreateDevice(This,pwszDevicePath,pdwCount,pppDeviceArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDServiceProvider2_INTERFACE_DEFINED__ */


#ifndef __IMDServiceProvider3_INTERFACE_DEFINED__
#define __IMDServiceProvider3_INTERFACE_DEFINED__

/* interface IMDServiceProvider3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDServiceProvider3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ed13ef3-a971-4d19-9f51-0e1826b2da57")
    IMDServiceProvider3 : public IMDServiceProvider2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDeviceEnumPreference( 
            /* [in] */ DWORD dwEnumPref) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDServiceProvider3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDServiceProvider3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDServiceProvider3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDServiceProvider3 * This);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, GetDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCount )( 
            __RPC__in IMDServiceProvider3 * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider, EnumDevices)
        HRESULT ( STDMETHODCALLTYPE *EnumDevices )( 
            __RPC__in IMDServiceProvider3 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider2, CreateDevice)
        HRESULT ( STDMETHODCALLTYPE *CreateDevice )( 
            __RPC__in IMDServiceProvider3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pwszDevicePath,
            /* [out] */ __RPC__out DWORD *pdwCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwCount) IMDSPDevice ***pppDeviceArray);
        
        DECLSPEC_XFGVIRT(IMDServiceProvider3, SetDeviceEnumPreference)
        HRESULT ( STDMETHODCALLTYPE *SetDeviceEnumPreference )( 
            __RPC__in IMDServiceProvider3 * This,
            /* [in] */ DWORD dwEnumPref);
        
        END_INTERFACE
    } IMDServiceProvider3Vtbl;

    interface IMDServiceProvider3
    {
        CONST_VTBL struct IMDServiceProvider3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDServiceProvider3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDServiceProvider3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDServiceProvider3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDServiceProvider3_GetDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetDeviceCount(This,pdwCount) ) 

#define IMDServiceProvider3_EnumDevices(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> EnumDevices(This,ppEnumDevice) ) 


#define IMDServiceProvider3_CreateDevice(This,pwszDevicePath,pdwCount,pppDeviceArray)	\
    ( (This)->lpVtbl -> CreateDevice(This,pwszDevicePath,pdwCount,pppDeviceArray) ) 


#define IMDServiceProvider3_SetDeviceEnumPreference(This,dwEnumPref)	\
    ( (This)->lpVtbl -> SetDeviceEnumPreference(This,dwEnumPref) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDServiceProvider3_INTERFACE_DEFINED__ */


#ifndef __IMDSPEnumDevice_INTERFACE_DEFINED__
#define __IMDSPEnumDevice_INTERFACE_DEFINED__

/* interface IMDSPEnumDevice */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPEnumDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A11-33ED-11d3-8470-00C04F79DBC0")
    IMDSPEnumDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMDSPDevice **ppDevice,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPEnumDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPEnumDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IMDSPEnumDevice, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMDSPEnumDevice * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMDSPDevice **ppDevice,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IMDSPEnumDevice, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IMDSPEnumDevice * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IMDSPEnumDevice, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMDSPEnumDevice * This);
        
        DECLSPEC_XFGVIRT(IMDSPEnumDevice, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMDSPEnumDevice * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumDevice **ppEnumDevice);
        
        END_INTERFACE
    } IMDSPEnumDeviceVtbl;

    interface IMDSPEnumDevice
    {
        CONST_VTBL struct IMDSPEnumDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPEnumDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPEnumDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPEnumDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPEnumDevice_Next(This,celt,ppDevice,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppDevice,pceltFetched) ) 

#define IMDSPEnumDevice_Skip(This,celt,pceltFetched)	\
    ( (This)->lpVtbl -> Skip(This,celt,pceltFetched) ) 

#define IMDSPEnumDevice_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMDSPEnumDevice_Clone(This,ppEnumDevice)	\
    ( (This)->lpVtbl -> Clone(This,ppEnumDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPEnumDevice_INTERFACE_DEFINED__ */


#ifndef __IMDSPDevice_INTERFACE_DEFINED__
#define __IMDSPDevice_INTERFACE_DEFINED__

/* interface IMDSPDevice */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A12-33ED-11d3-8470-00C04F79DBC0")
    IMDSPDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetManufacturer( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [out] */ __RPC__out DWORD *pdwVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out DWORD *pdwType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSerialNumber( 
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPowerSource( 
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceIcon( 
            /* [out] */ __RPC__out ULONG *hIcon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStorage( 
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatSupport( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **pFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendOpaqueCommand( 
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPDevice * This);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPDevice * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IMDSPDevice * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPDevice * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IMDSPDevice * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **pFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPDevice * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        END_INTERFACE
    } IMDSPDeviceVtbl;

    interface IMDSPDevice
    {
        CONST_VTBL struct IMDSPDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPDevice_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPDevice_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IMDSPDevice_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IMDSPDevice_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IMDSPDevice_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IMDSPDevice_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IMDSPDevice_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IMDSPDevice_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IMDSPDevice_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPDevice_GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IMDSPDevice_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPDevice_INTERFACE_DEFINED__ */


#ifndef __IMDSPDevice2_INTERFACE_DEFINED__
#define __IMDSPDevice2_INTERFACE_DEFINED__

/* interface IMDSPDevice2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPDevice2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("420D16AD-C97D-4e00-82AA-00E9F4335DDD")
    IMDSPDevice2 : public IMDSPDevice
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStorage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatSupport2( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpecifyPropertyPages( 
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCanonicalName( 
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPDevice2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPDevice2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPDevice2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPDevice2 * This);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPDevice2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IMDSPDevice2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IMDSPDevice2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **pFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPDevice2 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IMDSPDevice2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetFormatSupport2)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport2 )( 
            __RPC__in IMDSPDevice2 * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetSpecifyPropertyPages)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifyPropertyPages )( 
            __RPC__in IMDSPDevice2 * This,
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IMDSPDevice2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars);
        
        END_INTERFACE
    } IMDSPDevice2Vtbl;

    interface IMDSPDevice2
    {
        CONST_VTBL struct IMDSPDevice2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPDevice2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPDevice2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPDevice2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPDevice2_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPDevice2_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IMDSPDevice2_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IMDSPDevice2_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IMDSPDevice2_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IMDSPDevice2_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IMDSPDevice2_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IMDSPDevice2_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IMDSPDevice2_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPDevice2_GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IMDSPDevice2_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IMDSPDevice2_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IMDSPDevice2_GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount) ) 

#define IMDSPDevice2_GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks)	\
    ( (This)->lpVtbl -> GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks) ) 

#define IMDSPDevice2_GetCanonicalName(This,pwszPnPName,nMaxChars)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,pwszPnPName,nMaxChars) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPDevice2_INTERFACE_DEFINED__ */


#ifndef __IMDSPDevice3_INTERFACE_DEFINED__
#define __IMDSPDevice3_INTERFACE_DEFINED__

/* interface IMDSPDevice3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPDevice3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1a839845-fc55-487c-976f-ee38ac0e8c4e")
    IMDSPDevice3 : public IMDSPDevice2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [out] */ __RPC__out PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [in] */ __RPC__in const PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatCapability( 
            /* [in] */ WMDM_FORMATCODE format,
            /* [out] */ __RPC__out WMDM_FORMAT_CAPABILITY *pFormatSupport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceIoControl( 
            /* [in] */ DWORD dwIoControlCode,
            /* [size_is][in] */ __RPC__in_ecount_full(nInBufferSize) BYTE *lpInBuffer,
            /* [in] */ DWORD nInBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(*pnOutBufferSize) BYTE *lpOutBuffer,
            /* [out][in] */ __RPC__inout LPDWORD pnOutBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindStorage( 
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPDevice3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPDevice3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPDevice3 * This);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPDevice3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetManufacturer )( 
            __RPC__in IMDSPDevice3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out PWMDMID pSerialNumber,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetPowerSource)
        HRESULT ( STDMETHODCALLTYPE *GetPowerSource )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwPowerSource,
            /* [out] */ __RPC__out DWORD *pdwPercentRemaining);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetDeviceIcon)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceIcon )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__out ULONG *hIcon);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, GetFormatSupport)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport )( 
            __RPC__in IMDSPDevice3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormatCount) _WAVEFORMATEX **pFormatEx,
            /* [out] */ __RPC__out UINT *pnFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnMimeTypeCount) LPWSTR **pppwszMimeType,
            /* [out] */ __RPC__out UINT *pnMimeTypeCount);
        
        DECLSPEC_XFGVIRT(IMDSPDevice, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPDevice3 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IMDSPDevice3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetFormatSupport2)
        HRESULT ( STDMETHODCALLTYPE *GetFormatSupport2 )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnAudioFormatCount) _WAVEFORMATEX **ppAudioFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnAudioFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnVideoFormatCount) _VIDEOINFOHEADER **ppVideoFormatEx,
            /* [ref][out] */ __RPC__out UINT *pnVideoFormatCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFileTypeCount) WMFILECAPABILITIES **ppFileType,
            /* [ref][out] */ __RPC__out UINT *pnFileTypeCount);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetSpecifyPropertyPages)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifyPropertyPages )( 
            __RPC__in IMDSPDevice3 * This,
            /* [ref][out] */ __RPC__deref_out_opt ISpecifyPropertyPages **ppSpecifyPropPages,
            /* [size_is][size_is][ref][out] */ __RPC__deref_out_ecount_full_opt(*pcUnks) IUnknown ***pppUnknowns,
            /* [ref][out] */ __RPC__out ULONG *pcUnks);
        
        DECLSPEC_XFGVIRT(IMDSPDevice2, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IMDSPDevice3 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(nMaxChars) LPWSTR pwszPnPName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPDevice3, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [out] */ __RPC__out PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMDSPDevice3, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ __RPC__in LPCWSTR pwszPropName,
            /* [in] */ __RPC__in const PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMDSPDevice3, GetFormatCapability)
        HRESULT ( STDMETHODCALLTYPE *GetFormatCapability )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ WMDM_FORMATCODE format,
            /* [out] */ __RPC__out WMDM_FORMAT_CAPABILITY *pFormatSupport);
        
        DECLSPEC_XFGVIRT(IMDSPDevice3, DeviceIoControl)
        HRESULT ( STDMETHODCALLTYPE *DeviceIoControl )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ DWORD dwIoControlCode,
            /* [size_is][in] */ __RPC__in_ecount_full(nInBufferSize) BYTE *lpInBuffer,
            /* [in] */ DWORD nInBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(*pnOutBufferSize) BYTE *lpOutBuffer,
            /* [out][in] */ __RPC__inout LPDWORD pnOutBufferSize);
        
        DECLSPEC_XFGVIRT(IMDSPDevice3, FindStorage)
        HRESULT ( STDMETHODCALLTYPE *FindStorage )( 
            __RPC__in IMDSPDevice3 * This,
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        END_INTERFACE
    } IMDSPDevice3Vtbl;

    interface IMDSPDevice3
    {
        CONST_VTBL struct IMDSPDevice3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPDevice3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPDevice3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPDevice3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPDevice3_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPDevice3_GetManufacturer(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetManufacturer(This,pwszName,nMaxChars) ) 

#define IMDSPDevice3_GetVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersion) ) 

#define IMDSPDevice3_GetType(This,pdwType)	\
    ( (This)->lpVtbl -> GetType(This,pdwType) ) 

#define IMDSPDevice3_GetSerialNumber(This,pSerialNumber,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNumber,abMac) ) 

#define IMDSPDevice3_GetPowerSource(This,pdwPowerSource,pdwPercentRemaining)	\
    ( (This)->lpVtbl -> GetPowerSource(This,pdwPowerSource,pdwPercentRemaining) ) 

#define IMDSPDevice3_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IMDSPDevice3_GetDeviceIcon(This,hIcon)	\
    ( (This)->lpVtbl -> GetDeviceIcon(This,hIcon) ) 

#define IMDSPDevice3_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPDevice3_GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport(This,pFormatEx,pnFormatCount,pppwszMimeType,pnMimeTypeCount) ) 

#define IMDSPDevice3_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IMDSPDevice3_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IMDSPDevice3_GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount)	\
    ( (This)->lpVtbl -> GetFormatSupport2(This,dwFlags,ppAudioFormatEx,pnAudioFormatCount,ppVideoFormatEx,pnVideoFormatCount,ppFileType,pnFileTypeCount) ) 

#define IMDSPDevice3_GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks)	\
    ( (This)->lpVtbl -> GetSpecifyPropertyPages(This,ppSpecifyPropPages,pppUnknowns,pcUnks) ) 

#define IMDSPDevice3_GetCanonicalName(This,pwszPnPName,nMaxChars)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,pwszPnPName,nMaxChars) ) 


#define IMDSPDevice3_GetProperty(This,pwszPropName,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,pwszPropName,pValue) ) 

#define IMDSPDevice3_SetProperty(This,pwszPropName,pValue)	\
    ( (This)->lpVtbl -> SetProperty(This,pwszPropName,pValue) ) 

#define IMDSPDevice3_GetFormatCapability(This,format,pFormatSupport)	\
    ( (This)->lpVtbl -> GetFormatCapability(This,format,pFormatSupport) ) 

#define IMDSPDevice3_DeviceIoControl(This,dwIoControlCode,lpInBuffer,nInBufferSize,lpOutBuffer,pnOutBufferSize)	\
    ( (This)->lpVtbl -> DeviceIoControl(This,dwIoControlCode,lpInBuffer,nInBufferSize,lpOutBuffer,pnOutBufferSize) ) 

#define IMDSPDevice3_FindStorage(This,findScope,pwszUniqueID,ppStorage)	\
    ( (This)->lpVtbl -> FindStorage(This,findScope,pwszUniqueID,ppStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPDevice3_INTERFACE_DEFINED__ */


#ifndef __IMDSPDeviceControl_INTERFACE_DEFINED__
#define __IMDSPDeviceControl_INTERFACE_DEFINED__

/* interface IMDSPDeviceControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPDeviceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A14-33ED-11d3-8470-00C04F79DBC0")
    IMDSPDeviceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDCStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCapabilitiesMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Play( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Record( 
            /* [in] */ __RPC__in _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ UINT fuMode,
            /* [in] */ int nOffset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPDeviceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPDeviceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, GetDCStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDCStatus )( 
            __RPC__in IMDSPDeviceControl * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IMDSPDeviceControl * This,
            /* [out] */ __RPC__out DWORD *pdwCapabilitiesMask);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Play)
        HRESULT ( STDMETHODCALLTYPE *Play )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Record)
        HRESULT ( STDMETHODCALLTYPE *Record )( 
            __RPC__in IMDSPDeviceControl * This,
            /* [in] */ __RPC__in _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMDSPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IMDSPDeviceControl, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IMDSPDeviceControl * This,
            /* [in] */ UINT fuMode,
            /* [in] */ int nOffset);
        
        END_INTERFACE
    } IMDSPDeviceControlVtbl;

    interface IMDSPDeviceControl
    {
        CONST_VTBL struct IMDSPDeviceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPDeviceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPDeviceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPDeviceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPDeviceControl_GetDCStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetDCStatus(This,pdwStatus) ) 

#define IMDSPDeviceControl_GetCapabilities(This,pdwCapabilitiesMask)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilitiesMask) ) 

#define IMDSPDeviceControl_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IMDSPDeviceControl_Record(This,pFormat)	\
    ( (This)->lpVtbl -> Record(This,pFormat) ) 

#define IMDSPDeviceControl_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMDSPDeviceControl_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IMDSPDeviceControl_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMDSPDeviceControl_Seek(This,fuMode,nOffset)	\
    ( (This)->lpVtbl -> Seek(This,fuMode,nOffset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPDeviceControl_INTERFACE_DEFINED__ */


#ifndef __IMDSPEnumStorage_INTERFACE_DEFINED__
#define __IMDSPEnumStorage_INTERFACE_DEFINED__

/* interface IMDSPEnumStorage */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPEnumStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A15-33ED-11d3-8470-00C04F79DBC0")
    IMDSPEnumStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMDSPStorage **ppStorage,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPEnumStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPEnumStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IMDSPEnumStorage, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IMDSPEnumStorage * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMDSPStorage **ppStorage,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IMDSPEnumStorage, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IMDSPEnumStorage * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IMDSPEnumStorage, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMDSPEnumStorage * This);
        
        DECLSPEC_XFGVIRT(IMDSPEnumStorage, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMDSPEnumStorage * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        END_INTERFACE
    } IMDSPEnumStorageVtbl;

    interface IMDSPEnumStorage
    {
        CONST_VTBL struct IMDSPEnumStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPEnumStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPEnumStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPEnumStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPEnumStorage_Next(This,celt,ppStorage,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppStorage,pceltFetched) ) 

#define IMDSPEnumStorage_Skip(This,celt,pceltFetched)	\
    ( (This)->lpVtbl -> Skip(This,celt,pceltFetched) ) 

#define IMDSPEnumStorage_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IMDSPEnumStorage_Clone(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> Clone(This,ppEnumStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPEnumStorage_INTERFACE_DEFINED__ */


#ifndef __IMDSPStorage_INTERFACE_DEFINED__
#define __IMDSPStorage_INTERFACE_DEFINED__

/* interface IMDSPStorage */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A16-33ED-11d3-8470-00C04F79DBC0")
    IMDSPStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAttributes( 
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStorageGlobals( 
            /* [out] */ __RPC__deref_out_opt IMDSPStorageGlobals **ppStorageGlobals) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDate( 
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRights( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStorage( 
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStorage( 
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendOpaqueCommand( 
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPStorage * This);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IMDSPStorage * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IMDSPStorage * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IMDSPStorage * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPStorage * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IMDSPStorage * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IMDSPStorage * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IMDSPStorage * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, CreateStorage)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage )( 
            __RPC__in IMDSPStorage * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPStorage * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPStorage * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        END_INTERFACE
    } IMDSPStorageVtbl;

    interface IMDSPStorage
    {
        CONST_VTBL struct IMDSPStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPStorage_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IMDSPStorage_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IMDSPStorage_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IMDSPStorage_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPStorage_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IMDSPStorage_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IMDSPStorage_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IMDSPStorage_CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage) ) 

#define IMDSPStorage_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPStorage_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPStorage_INTERFACE_DEFINED__ */


#ifndef __IMDSPStorage2_INTERFACE_DEFINED__
#define __IMDSPStorage2_INTERFACE_DEFINED__

/* interface IMDSPStorage2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPStorage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0A5E07A5-6454-4451-9C36-1C6AE7E2B1D6")
    IMDSPStorage2 : public IMDSPStorage
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStorage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStorage2( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAttributes2( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes2( 
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPStorage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPStorage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPStorage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPStorage2 * This);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IMDSPStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPStorage2 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IMDSPStorage2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, CreateStorage)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage )( 
            __RPC__in IMDSPStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IMDSPStorage2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, CreateStorage2)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage2 )( 
            __RPC__in IMDSPStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IMDSPStorage2 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IMDSPStorage2 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        END_INTERFACE
    } IMDSPStorage2Vtbl;

    interface IMDSPStorage2
    {
        CONST_VTBL struct IMDSPStorage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPStorage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPStorage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPStorage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPStorage2_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IMDSPStorage2_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IMDSPStorage2_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IMDSPStorage2_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPStorage2_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IMDSPStorage2_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IMDSPStorage2_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IMDSPStorage2_CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage) ) 

#define IMDSPStorage2_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPStorage2_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IMDSPStorage2_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IMDSPStorage2_CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage) ) 

#define IMDSPStorage2_SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat) ) 

#define IMDSPStorage2_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPStorage2_INTERFACE_DEFINED__ */


#ifndef __IMDSPStorage3_INTERFACE_DEFINED__
#define __IMDSPStorage3_INTERFACE_DEFINED__

/* interface IMDSPStorage3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPStorage3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C669867-97ED-4a67-9706-1C5529D2A414")
    IMDSPStorage3 : public IMDSPStorage2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMetadata( 
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPStorage3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPStorage3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPStorage3 * This);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPStorage3 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IMDSPStorage3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, CreateStorage)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IMDSPStorage3 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, CreateStorage2)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage2 )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IMDSPStorage3 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage3, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IMDSPStorage3, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IMDSPStorage3 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        END_INTERFACE
    } IMDSPStorage3Vtbl;

    interface IMDSPStorage3
    {
        CONST_VTBL struct IMDSPStorage3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPStorage3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPStorage3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPStorage3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPStorage3_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IMDSPStorage3_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IMDSPStorage3_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IMDSPStorage3_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPStorage3_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IMDSPStorage3_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IMDSPStorage3_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IMDSPStorage3_CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage) ) 

#define IMDSPStorage3_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPStorage3_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IMDSPStorage3_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IMDSPStorage3_CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage) ) 

#define IMDSPStorage3_SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat) ) 

#define IMDSPStorage3_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 


#define IMDSPStorage3_GetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> GetMetadata(This,pMetadata) ) 

#define IMDSPStorage3_SetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> SetMetadata(This,pMetadata) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPStorage3_INTERFACE_DEFINED__ */


#ifndef __IMDSPStorage4_INTERFACE_DEFINED__
#define __IMDSPStorage4_INTERFACE_DEFINED__

/* interface IMDSPStorage4 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPStorage4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3133b2c4-515c-481b-b1ce-39327ecb4f74")
    IMDSPStorage4 : public IMDSPStorage3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetReferences( 
            /* [in] */ DWORD dwRefs,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRefs) IMDSPStorage **ppISPStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReferences( 
            /* [out] */ __RPC__out DWORD *pdwRefs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwRefs) IMDSPStorage ***pppISPStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStorageWithMetadata( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpecifiedMetadata( 
            /* [in] */ DWORD cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) LPCWSTR *ppwszPropNames,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindStorage( 
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParent( 
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPStorage4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPStorage4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPStorage4 * This);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SetAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetStorageGlobals)
        HRESULT ( STDMETHODCALLTYPE *GetStorageGlobals )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorageGlobals **ppStorageGlobals);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IMDSPStorage4 * This,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(nMaxChars) LPWSTR pwszName,
            /* [in] */ UINT nMaxChars);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__out PWMDMDATETIME pDateTimeUTC);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwSizeLow,
            /* [out] */ __RPC__out DWORD *pdwSizeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in IMDSPStorage4 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, CreateStorage)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, EnumStorage)
        HRESULT ( STDMETHODCALLTYPE *EnumStorage )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPEnumStorage **ppEnumStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage, SendOpaqueCommand)
        HRESULT ( STDMETHODCALLTYPE *SendOpaqueCommand )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out][in] */ __RPC__inout OPAQUECOMMAND *pCommand);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetStorage)
        HRESULT ( STDMETHODCALLTYPE *GetStorage )( 
            __RPC__in IMDSPStorage4 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszStorageName,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, CreateStorage2)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage2 )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat,
            /* [in] */ __RPC__in LPWSTR pwszName,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, SetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *SetAttributes2 )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ DWORD dwAttributesEx,
            /* [unique][in] */ __RPC__in_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][in] */ __RPC__in_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage2, GetAttributes2)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes2 )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwAttributes,
            /* [out] */ __RPC__out DWORD *pdwAttributesEx,
            /* [unique][out][in] */ __RPC__inout_opt _WAVEFORMATEX *pAudioFormat,
            /* [unique][out][in] */ __RPC__inout_opt _VIDEOINFOHEADER *pVideoFormat);
        
        DECLSPEC_XFGVIRT(IMDSPStorage3, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IMDSPStorage3, SetMetadata)
        HRESULT ( STDMETHODCALLTYPE *SetMetadata )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, SetReferences)
        HRESULT ( STDMETHODCALLTYPE *SetReferences )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwRefs,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRefs) IMDSPStorage **ppISPStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, GetReferences)
        HRESULT ( STDMETHODCALLTYPE *GetReferences )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__out DWORD *pdwRefs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwRefs) IMDSPStorage ***pppISPStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, CreateStorageWithMetadata)
        HRESULT ( STDMETHODCALLTYPE *CreateStorageWithMetadata )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata,
            /* [in] */ ULONGLONG qwFileSize,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, GetSpecifiedMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetSpecifiedMetadata )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ DWORD cProperties,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) LPCWSTR *ppwszPropNames,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pMetadata);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, FindStorage)
        HRESULT ( STDMETHODCALLTYPE *FindStorage )( 
            __RPC__in IMDSPStorage4 * This,
            /* [in] */ WMDM_FIND_SCOPE findScope,
            /* [in] */ __RPC__in LPCWSTR pwszUniqueID,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        DECLSPEC_XFGVIRT(IMDSPStorage4, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IMDSPStorage4 * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppStorage);
        
        END_INTERFACE
    } IMDSPStorage4Vtbl;

    interface IMDSPStorage4
    {
        CONST_VTBL struct IMDSPStorage4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPStorage4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPStorage4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPStorage4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPStorage4_SetAttributes(This,dwAttributes,pFormat)	\
    ( (This)->lpVtbl -> SetAttributes(This,dwAttributes,pFormat) ) 

#define IMDSPStorage4_GetStorageGlobals(This,ppStorageGlobals)	\
    ( (This)->lpVtbl -> GetStorageGlobals(This,ppStorageGlobals) ) 

#define IMDSPStorage4_GetAttributes(This,pdwAttributes,pFormat)	\
    ( (This)->lpVtbl -> GetAttributes(This,pdwAttributes,pFormat) ) 

#define IMDSPStorage4_GetName(This,pwszName,nMaxChars)	\
    ( (This)->lpVtbl -> GetName(This,pwszName,nMaxChars) ) 

#define IMDSPStorage4_GetDate(This,pDateTimeUTC)	\
    ( (This)->lpVtbl -> GetDate(This,pDateTimeUTC) ) 

#define IMDSPStorage4_GetSize(This,pdwSizeLow,pdwSizeHigh)	\
    ( (This)->lpVtbl -> GetSize(This,pdwSizeLow,pdwSizeHigh) ) 

#define IMDSPStorage4_GetRights(This,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,ppRights,pnRightsCount,abMac) ) 

#define IMDSPStorage4_CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage(This,dwAttributes,pFormat,pwszName,ppNewStorage) ) 

#define IMDSPStorage4_EnumStorage(This,ppEnumStorage)	\
    ( (This)->lpVtbl -> EnumStorage(This,ppEnumStorage) ) 

#define IMDSPStorage4_SendOpaqueCommand(This,pCommand)	\
    ( (This)->lpVtbl -> SendOpaqueCommand(This,pCommand) ) 


#define IMDSPStorage4_GetStorage(This,pszStorageName,ppStorage)	\
    ( (This)->lpVtbl -> GetStorage(This,pszStorageName,ppStorage) ) 

#define IMDSPStorage4_CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorage2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat,pwszName,qwFileSize,ppNewStorage) ) 

#define IMDSPStorage4_SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> SetAttributes2(This,dwAttributes,dwAttributesEx,pAudioFormat,pVideoFormat) ) 

#define IMDSPStorage4_GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat)	\
    ( (This)->lpVtbl -> GetAttributes2(This,pdwAttributes,pdwAttributesEx,pAudioFormat,pVideoFormat) ) 


#define IMDSPStorage4_GetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> GetMetadata(This,pMetadata) ) 

#define IMDSPStorage4_SetMetadata(This,pMetadata)	\
    ( (This)->lpVtbl -> SetMetadata(This,pMetadata) ) 


#define IMDSPStorage4_SetReferences(This,dwRefs,ppISPStorage)	\
    ( (This)->lpVtbl -> SetReferences(This,dwRefs,ppISPStorage) ) 

#define IMDSPStorage4_GetReferences(This,pdwRefs,pppISPStorage)	\
    ( (This)->lpVtbl -> GetReferences(This,pdwRefs,pppISPStorage) ) 

#define IMDSPStorage4_CreateStorageWithMetadata(This,dwAttributes,pwszName,pMetadata,qwFileSize,ppNewStorage)	\
    ( (This)->lpVtbl -> CreateStorageWithMetadata(This,dwAttributes,pwszName,pMetadata,qwFileSize,ppNewStorage) ) 

#define IMDSPStorage4_GetSpecifiedMetadata(This,cProperties,ppwszPropNames,pMetadata)	\
    ( (This)->lpVtbl -> GetSpecifiedMetadata(This,cProperties,ppwszPropNames,pMetadata) ) 

#define IMDSPStorage4_FindStorage(This,findScope,pwszUniqueID,ppStorage)	\
    ( (This)->lpVtbl -> FindStorage(This,findScope,pwszUniqueID,ppStorage) ) 

#define IMDSPStorage4_GetParent(This,ppStorage)	\
    ( (This)->lpVtbl -> GetParent(This,ppStorage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPStorage4_INTERFACE_DEFINED__ */


#ifndef __IMDSPStorageGlobals_INTERFACE_DEFINED__
#define __IMDSPStorageGlobals_INTERFACE_DEFINED__

/* interface IMDSPStorageGlobals */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPStorageGlobals;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A17-33ED-11d3-8470-00C04F79DBC0")
    IMDSPStorageGlobals : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSerialNumber( 
            /* [out] */ __RPC__out PWMDMID pSerialNum,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalSize( 
            /* [out] */ __RPC__out DWORD *pdwTotalSizeLow,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalFree( 
            /* [out] */ __RPC__out DWORD *pdwFreeLow,
            /* [out] */ __RPC__out DWORD *pdwFreeHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalBad( 
            /* [out] */ __RPC__out DWORD *pdwBadLow,
            /* [out] */ __RPC__out DWORD *pdwBadHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [out] */ __RPC__deref_out_opt IMDSPDevice **ppDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRootStorage( 
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppRoot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPStorageGlobalsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPStorageGlobals * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPStorageGlobals * This);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwCapabilities);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetSerialNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSerialNumber )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out PWMDMID pSerialNum,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetTotalSize)
        HRESULT ( STDMETHODCALLTYPE *GetTotalSize )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeLow,
            /* [out] */ __RPC__out DWORD *pdwTotalSizeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetTotalFree)
        HRESULT ( STDMETHODCALLTYPE *GetTotalFree )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwFreeLow,
            /* [out] */ __RPC__out DWORD *pdwFreeHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetTotalBad)
        HRESULT ( STDMETHODCALLTYPE *GetTotalBad )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwBadLow,
            /* [out] */ __RPC__out DWORD *pdwBadHigh);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetDevice)
        HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__deref_out_opt IMDSPDevice **ppDevice);
        
        DECLSPEC_XFGVIRT(IMDSPStorageGlobals, GetRootStorage)
        HRESULT ( STDMETHODCALLTYPE *GetRootStorage )( 
            __RPC__in IMDSPStorageGlobals * This,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppRoot);
        
        END_INTERFACE
    } IMDSPStorageGlobalsVtbl;

    interface IMDSPStorageGlobals
    {
        CONST_VTBL struct IMDSPStorageGlobalsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPStorageGlobals_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPStorageGlobals_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPStorageGlobals_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPStorageGlobals_GetCapabilities(This,pdwCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilities) ) 

#define IMDSPStorageGlobals_GetSerialNumber(This,pSerialNum,abMac)	\
    ( (This)->lpVtbl -> GetSerialNumber(This,pSerialNum,abMac) ) 

#define IMDSPStorageGlobals_GetTotalSize(This,pdwTotalSizeLow,pdwTotalSizeHigh)	\
    ( (This)->lpVtbl -> GetTotalSize(This,pdwTotalSizeLow,pdwTotalSizeHigh) ) 

#define IMDSPStorageGlobals_GetTotalFree(This,pdwFreeLow,pdwFreeHigh)	\
    ( (This)->lpVtbl -> GetTotalFree(This,pdwFreeLow,pdwFreeHigh) ) 

#define IMDSPStorageGlobals_GetTotalBad(This,pdwBadLow,pdwBadHigh)	\
    ( (This)->lpVtbl -> GetTotalBad(This,pdwBadLow,pdwBadHigh) ) 

#define IMDSPStorageGlobals_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IMDSPStorageGlobals_Initialize(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Initialize(This,fuMode,pProgress) ) 

#define IMDSPStorageGlobals_GetDevice(This,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,ppDevice) ) 

#define IMDSPStorageGlobals_GetRootStorage(This,ppRoot)	\
    ( (This)->lpVtbl -> GetRootStorage(This,ppRoot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPStorageGlobals_INTERFACE_DEFINED__ */


#ifndef __IMDSPObjectInfo_INTERFACE_DEFINED__
#define __IMDSPObjectInfo_INTERFACE_DEFINED__

/* interface IMDSPObjectInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPObjectInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A19-33ED-11d3-8470-00C04F79DBC0")
    IMDSPObjectInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPlayLength( 
            /* [out] */ __RPC__out DWORD *pdwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlayLength( 
            /* [in] */ DWORD dwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlayOffset( 
            /* [out] */ __RPC__out DWORD *pdwOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlayOffset( 
            /* [in] */ DWORD dwOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalLength( 
            /* [out] */ __RPC__out DWORD *pdwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastPlayPosition( 
            /* [out] */ __RPC__out DWORD *pdwLastPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLongestPlayPosition( 
            /* [out] */ __RPC__out DWORD *pdwLongestPos) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPObjectInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPObjectInfo * This);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, GetPlayLength)
        HRESULT ( STDMETHODCALLTYPE *GetPlayLength )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLength);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, SetPlayLength)
        HRESULT ( STDMETHODCALLTYPE *SetPlayLength )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [in] */ DWORD dwLength);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, GetPlayOffset)
        HRESULT ( STDMETHODCALLTYPE *GetPlayOffset )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwOffset);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, SetPlayOffset)
        HRESULT ( STDMETHODCALLTYPE *SetPlayOffset )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [in] */ DWORD dwOffset);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, GetTotalLength)
        HRESULT ( STDMETHODCALLTYPE *GetTotalLength )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLength);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, GetLastPlayPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLastPlayPosition )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLastPos);
        
        DECLSPEC_XFGVIRT(IMDSPObjectInfo, GetLongestPlayPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLongestPlayPosition )( 
            __RPC__in IMDSPObjectInfo * This,
            /* [out] */ __RPC__out DWORD *pdwLongestPos);
        
        END_INTERFACE
    } IMDSPObjectInfoVtbl;

    interface IMDSPObjectInfo
    {
        CONST_VTBL struct IMDSPObjectInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPObjectInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPObjectInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPObjectInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPObjectInfo_GetPlayLength(This,pdwLength)	\
    ( (This)->lpVtbl -> GetPlayLength(This,pdwLength) ) 

#define IMDSPObjectInfo_SetPlayLength(This,dwLength)	\
    ( (This)->lpVtbl -> SetPlayLength(This,dwLength) ) 

#define IMDSPObjectInfo_GetPlayOffset(This,pdwOffset)	\
    ( (This)->lpVtbl -> GetPlayOffset(This,pdwOffset) ) 

#define IMDSPObjectInfo_SetPlayOffset(This,dwOffset)	\
    ( (This)->lpVtbl -> SetPlayOffset(This,dwOffset) ) 

#define IMDSPObjectInfo_GetTotalLength(This,pdwLength)	\
    ( (This)->lpVtbl -> GetTotalLength(This,pdwLength) ) 

#define IMDSPObjectInfo_GetLastPlayPosition(This,pdwLastPos)	\
    ( (This)->lpVtbl -> GetLastPlayPosition(This,pdwLastPos) ) 

#define IMDSPObjectInfo_GetLongestPlayPosition(This,pdwLongestPos)	\
    ( (This)->lpVtbl -> GetLongestPlayPosition(This,pdwLongestPos) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPObjectInfo_INTERFACE_DEFINED__ */


#ifndef __IMDSPObject_INTERFACE_DEFINED__
#define __IMDSPObject_INTERFACE_DEFINED__

/* interface IMDSPObject */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A18-33ED-11d3-8470-00C04F79DBC0")
    IMDSPObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ UINT fuMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write( 
            /* [size_is][in] */ __RPC__in_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ UINT fuFlags,
            /* [in] */ DWORD dwOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rename( 
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IMDSPStorage *pTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPObject * This);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ UINT fuMode);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IMDSPObject * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IMDSPObject * This,
            /* [size_is][in] */ __RPC__in_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ UINT fuFlags,
            /* [in] */ DWORD dwOffset);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IMDSPObject * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IMDSPStorage *pTarget);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMDSPObject * This);
        
        END_INTERFACE
    } IMDSPObjectVtbl;

    interface IMDSPObject
    {
        CONST_VTBL struct IMDSPObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPObject_Open(This,fuMode)	\
    ( (This)->lpVtbl -> Open(This,fuMode) ) 

#define IMDSPObject_Read(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> Read(This,pData,pdwSize,abMac) ) 

#define IMDSPObject_Write(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> Write(This,pData,pdwSize,abMac) ) 

#define IMDSPObject_Delete(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Delete(This,fuMode,pProgress) ) 

#define IMDSPObject_Seek(This,fuFlags,dwOffset)	\
    ( (This)->lpVtbl -> Seek(This,fuFlags,dwOffset) ) 

#define IMDSPObject_Rename(This,pwszNewName,pProgress)	\
    ( (This)->lpVtbl -> Rename(This,pwszNewName,pProgress) ) 

#define IMDSPObject_Move(This,fuMode,pProgress,pTarget)	\
    ( (This)->lpVtbl -> Move(This,fuMode,pProgress,pTarget) ) 

#define IMDSPObject_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPObject_INTERFACE_DEFINED__ */


#ifndef __IMDSPObject2_INTERFACE_DEFINED__
#define __IMDSPObject2_INTERFACE_DEFINED__

/* interface IMDSPObject2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPObject2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3f34cd3e-5907-4341-9af9-97f4187c3aa5")
    IMDSPObject2 : public IMDSPObject
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReadOnClearChannel( 
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteOnClearChannel( 
            /* [size_is][in] */ __RPC__in_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPObject2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPObject2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPObject2 * This);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ UINT fuMode);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IMDSPObject2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IMDSPObject2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ UINT fuFlags,
            /* [in] */ DWORD dwOffset);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Rename)
        HRESULT ( STDMETHODCALLTYPE *Rename )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ __RPC__in LPWSTR pwszNewName,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IMDSPObject2 * This,
            /* [in] */ UINT fuMode,
            /* [in] */ __RPC__in_opt IWMDMProgress *pProgress,
            /* [in] */ __RPC__in_opt IMDSPStorage *pTarget);
        
        DECLSPEC_XFGVIRT(IMDSPObject, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMDSPObject2 * This);
        
        DECLSPEC_XFGVIRT(IMDSPObject2, ReadOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *ReadOnClearChannel )( 
            __RPC__in IMDSPObject2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize);
        
        DECLSPEC_XFGVIRT(IMDSPObject2, WriteOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *WriteOnClearChannel )( 
            __RPC__in IMDSPObject2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize);
        
        END_INTERFACE
    } IMDSPObject2Vtbl;

    interface IMDSPObject2
    {
        CONST_VTBL struct IMDSPObject2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPObject2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPObject2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPObject2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPObject2_Open(This,fuMode)	\
    ( (This)->lpVtbl -> Open(This,fuMode) ) 

#define IMDSPObject2_Read(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> Read(This,pData,pdwSize,abMac) ) 

#define IMDSPObject2_Write(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> Write(This,pData,pdwSize,abMac) ) 

#define IMDSPObject2_Delete(This,fuMode,pProgress)	\
    ( (This)->lpVtbl -> Delete(This,fuMode,pProgress) ) 

#define IMDSPObject2_Seek(This,fuFlags,dwOffset)	\
    ( (This)->lpVtbl -> Seek(This,fuFlags,dwOffset) ) 

#define IMDSPObject2_Rename(This,pwszNewName,pProgress)	\
    ( (This)->lpVtbl -> Rename(This,pwszNewName,pProgress) ) 

#define IMDSPObject2_Move(This,fuMode,pProgress,pTarget)	\
    ( (This)->lpVtbl -> Move(This,fuMode,pProgress,pTarget) ) 

#define IMDSPObject2_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 


#define IMDSPObject2_ReadOnClearChannel(This,pData,pdwSize)	\
    ( (This)->lpVtbl -> ReadOnClearChannel(This,pData,pdwSize) ) 

#define IMDSPObject2_WriteOnClearChannel(This,pData,pdwSize)	\
    ( (This)->lpVtbl -> WriteOnClearChannel(This,pData,pdwSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPObject2_INTERFACE_DEFINED__ */


#ifndef __IMDSPDirectTransfer_INTERFACE_DEFINED__
#define __IMDSPDirectTransfer_INTERFACE_DEFINED__

/* interface IMDSPDirectTransfer */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPDirectTransfer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c2fe57a8-9304-478c-9ee4-47e397b912d7")
    IMDSPDirectTransfer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferToDevice( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pwszSourceFilePath,
            /* [in] */ __RPC__in_opt IWMDMOperation *pSourceOperation,
            /* [in] */ UINT fuFlags,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszDestinationName,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pSourceMetaData,
            /* [in] */ __RPC__in_opt IWMDMProgress *pTransferProgress,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPDirectTransferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPDirectTransfer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPDirectTransfer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPDirectTransfer * This);
        
        DECLSPEC_XFGVIRT(IMDSPDirectTransfer, TransferToDevice)
        HRESULT ( STDMETHODCALLTYPE *TransferToDevice )( 
            __RPC__in IMDSPDirectTransfer * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pwszSourceFilePath,
            /* [in] */ __RPC__in_opt IWMDMOperation *pSourceOperation,
            /* [in] */ UINT fuFlags,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszDestinationName,
            /* [in] */ __RPC__in_opt IWMDMMetaData *pSourceMetaData,
            /* [in] */ __RPC__in_opt IWMDMProgress *pTransferProgress,
            /* [out] */ __RPC__deref_out_opt IMDSPStorage **ppNewObject);
        
        END_INTERFACE
    } IMDSPDirectTransferVtbl;

    interface IMDSPDirectTransfer
    {
        CONST_VTBL struct IMDSPDirectTransferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPDirectTransfer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPDirectTransfer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPDirectTransfer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPDirectTransfer_TransferToDevice(This,pwszSourceFilePath,pSourceOperation,fuFlags,pwszDestinationName,pSourceMetaData,pTransferProgress,ppNewObject)	\
    ( (This)->lpVtbl -> TransferToDevice(This,pwszSourceFilePath,pSourceOperation,fuFlags,pwszDestinationName,pSourceMetaData,pTransferProgress,ppNewObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPDirectTransfer_INTERFACE_DEFINED__ */


#ifndef __IMDSPRevoked_INTERFACE_DEFINED__
#define __IMDSPRevoked_INTERFACE_DEFINED__

/* interface IMDSPRevoked */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMDSPRevoked;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A4E8F2D4-3F31-464d-B53D-4FC335998184")
    IMDSPRevoked : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRevocationURL( 
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwBufferLen) LPWSTR *ppwszRevocationURL,
            /* [out][in] */ __RPC__inout DWORD *pdwBufferLen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMDSPRevokedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMDSPRevoked * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMDSPRevoked * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMDSPRevoked * This);
        
        DECLSPEC_XFGVIRT(IMDSPRevoked, GetRevocationURL)
        HRESULT ( STDMETHODCALLTYPE *GetRevocationURL )( 
            __RPC__in IMDSPRevoked * This,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwBufferLen) LPWSTR *ppwszRevocationURL,
            /* [out][in] */ __RPC__inout DWORD *pdwBufferLen);
        
        END_INTERFACE
    } IMDSPRevokedVtbl;

    interface IMDSPRevoked
    {
        CONST_VTBL struct IMDSPRevokedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMDSPRevoked_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMDSPRevoked_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMDSPRevoked_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMDSPRevoked_GetRevocationURL(This,ppwszRevocationURL,pdwBufferLen)	\
    ( (This)->lpVtbl -> GetRevocationURL(This,ppwszRevocationURL,pdwBufferLen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMDSPRevoked_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mswmdm_0000_0047 */
/* [local] */ 

// SCP Data Flags
#define WMDM_SCP_EXAMINE_EXTENSION                 0x00000001L
#define WMDM_SCP_EXAMINE_DATA                      0x00000002L
#define WMDM_SCP_DECIDE_DATA                       0x00000008L
#define WMDM_SCP_PROTECTED_OUTPUT                  0x00000010L
#define WMDM_SCP_UNPROTECTED_OUTPUT                0x00000020L
#define WMDM_SCP_RIGHTS_DATA                       0x00000040L
// SCP Transfer Flags
#define WMDM_SCP_TRANSFER_OBJECTDATA               0x00000020L
#define WMDM_SCP_NO_MORE_CHANGES                   0x00000040L
// SCP DRMINFO Flags
#define WMDM_SCP_DRMINFO_NOT_DRMPROTECTED          0x00000000L
#define WMDM_SCP_DRMINFO_V1HEADER                  0x00000001L
#define WMDM_SCP_DRMINFO_V2HEADER                  0x00000002L
#ifndef _DEFINE_SCP_EVENTID
#define _DEFINE_SCP_EVENTID
// {86248CC9-4A59-43e2-9146-48A7F3F4140C}
// this event ID is used when SCP is acquiring secure clock from server
DEFINE_GUID(SCP_EVENTID_ACQSECURECLOCK, 
0x86248cc9, 0x4a59, 0x43e2, 0x91, 0x46, 0x48, 0xa7, 0xf3, 0xf4, 0x14, 0xc);
//
// {87A507C7-B469-4386-B976-D5D1CE538A6F}
DEFINE_GUID(SCP_EVENTID_NEEDTOINDIV, 
0x87a507c7, 0xb469, 0x4386, 0xb9, 0x76, 0xd5, 0xd1, 0xce, 0x53, 0x8a, 0x6f);
// this event ID is used to notify the player the version DRM header found in the content
// {213DD287-41D2-432b-9E3F-3B4F7B3581DD}
DEFINE_GUID(SCP_EVENTID_DRMINFO, 
0x213dd287, 0x41d2, 0x432b, 0x9e, 0x3f, 0x3b, 0x4f, 0x7b, 0x35, 0x81, 0xdd);
// this parameter ID is used when notifying SCP_EVENTID_DRMINFO message
// {41D0155D-7CC7-4217-ADA9-005074624DA4}
DEFINE_GUID(SCP_PARAMID_DRMVERSION, 
0x41d0155d, 0x7cc7, 0x4217, 0xad, 0xa9, 0x00, 0x50, 0x74, 0x62, 0x4d, 0xa4);
#endif






extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0047_v0_0_s_ifspec;

#ifndef __ISCPSecureAuthenticate_INTERFACE_DEFINED__
#define __ISCPSecureAuthenticate_INTERFACE_DEFINED__

/* interface ISCPSecureAuthenticate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureAuthenticate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A0F-33ED-11d3-8470-00C04F79DBC0")
    ISCPSecureAuthenticate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSecureQuery( 
            /* [out] */ __RPC__deref_out_opt ISCPSecureQuery **ppSecureQuery) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureAuthenticateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureAuthenticate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureAuthenticate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureAuthenticate * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureAuthenticate, GetSecureQuery)
        HRESULT ( STDMETHODCALLTYPE *GetSecureQuery )( 
            __RPC__in ISCPSecureAuthenticate * This,
            /* [out] */ __RPC__deref_out_opt ISCPSecureQuery **ppSecureQuery);
        
        END_INTERFACE
    } ISCPSecureAuthenticateVtbl;

    interface ISCPSecureAuthenticate
    {
        CONST_VTBL struct ISCPSecureAuthenticateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureAuthenticate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureAuthenticate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureAuthenticate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureAuthenticate_GetSecureQuery(This,ppSecureQuery)	\
    ( (This)->lpVtbl -> GetSecureQuery(This,ppSecureQuery) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureAuthenticate_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureAuthenticate2_INTERFACE_DEFINED__
#define __ISCPSecureAuthenticate2_INTERFACE_DEFINED__

/* interface ISCPSecureAuthenticate2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureAuthenticate2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B580CFAE-1672-47e2-ACAA-44BBECBCAE5B")
    ISCPSecureAuthenticate2 : public ISCPSecureAuthenticate
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSCPSession( 
            /* [out] */ __RPC__deref_out_opt ISCPSession **ppSCPSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureAuthenticate2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureAuthenticate2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureAuthenticate2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureAuthenticate2 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureAuthenticate, GetSecureQuery)
        HRESULT ( STDMETHODCALLTYPE *GetSecureQuery )( 
            __RPC__in ISCPSecureAuthenticate2 * This,
            /* [out] */ __RPC__deref_out_opt ISCPSecureQuery **ppSecureQuery);
        
        DECLSPEC_XFGVIRT(ISCPSecureAuthenticate2, GetSCPSession)
        HRESULT ( STDMETHODCALLTYPE *GetSCPSession )( 
            __RPC__in ISCPSecureAuthenticate2 * This,
            /* [out] */ __RPC__deref_out_opt ISCPSession **ppSCPSession);
        
        END_INTERFACE
    } ISCPSecureAuthenticate2Vtbl;

    interface ISCPSecureAuthenticate2
    {
        CONST_VTBL struct ISCPSecureAuthenticate2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureAuthenticate2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureAuthenticate2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureAuthenticate2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureAuthenticate2_GetSecureQuery(This,ppSecureQuery)	\
    ( (This)->lpVtbl -> GetSecureQuery(This,ppSecureQuery) ) 


#define ISCPSecureAuthenticate2_GetSCPSession(This,ppSCPSession)	\
    ( (This)->lpVtbl -> GetSCPSession(This,ppSCPSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureAuthenticate2_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureQuery_INTERFACE_DEFINED__
#define __ISCPSecureQuery_INTERFACE_DEFINED__

/* interface ISCPSecureQuery */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A0D-33ED-11d3-8470-00C04F79DBC0")
    ISCPSecureQuery : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDataDemands( 
            /* [out] */ __RPC__out UINT *pfuFlags,
            /* [out] */ __RPC__out DWORD *pdwMinRightsData,
            /* [out] */ __RPC__out DWORD *pdwMinExamineData,
            /* [out] */ __RPC__out DWORD *pdwMinDecideData,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExamineData( 
            /* [in] */ UINT fuFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPWSTR pwszExtension,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeDecision( 
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRights( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureQuery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureQuery * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetDataDemands)
        HRESULT ( STDMETHODCALLTYPE *GetDataDemands )( 
            __RPC__in ISCPSecureQuery * This,
            /* [out] */ __RPC__out UINT *pfuFlags,
            /* [out] */ __RPC__out DWORD *pdwMinRightsData,
            /* [out] */ __RPC__out DWORD *pdwMinExamineData,
            /* [out] */ __RPC__out DWORD *pdwMinDecideData,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, ExamineData)
        HRESULT ( STDMETHODCALLTYPE *ExamineData )( 
            __RPC__in ISCPSecureQuery * This,
            /* [in] */ UINT fuFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPWSTR pwszExtension,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, MakeDecision)
        HRESULT ( STDMETHODCALLTYPE *MakeDecision )( 
            __RPC__in ISCPSecureQuery * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in ISCPSecureQuery * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        END_INTERFACE
    } ISCPSecureQueryVtbl;

    interface ISCPSecureQuery
    {
        CONST_VTBL struct ISCPSecureQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureQuery_GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac)	\
    ( (This)->lpVtbl -> GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac) ) 

#define ISCPSecureQuery_ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac)	\
    ( (This)->lpVtbl -> ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac) ) 

#define ISCPSecureQuery_MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac)	\
    ( (This)->lpVtbl -> MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac) ) 

#define ISCPSecureQuery_GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureQuery_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureQuery2_INTERFACE_DEFINED__
#define __ISCPSecureQuery2_INTERFACE_DEFINED__

/* interface ISCPSecureQuery2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureQuery2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EBE17E25-4FD7-4632-AF46-6D93D4FCC72E")
    ISCPSecureQuery2 : public ISCPSecureQuery
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MakeDecision2( 
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertAppLen) BYTE *pAppCertApp,
            /* [in] */ DWORD dwAppCertAppLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertSPLen) BYTE *pAppCertSP,
            /* [in] */ DWORD dwAppCertSPLen,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwRevocationURLLen) LPWSTR *pszRevocationURL,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRevocationURLLen,
            /* [out] */ __RPC__out DWORD *pdwRevocationBitFlag,
            /* [unique][out][in] */ __RPC__inout_opt ULONGLONG *pqwFileSize,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureQuery2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureQuery2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureQuery2 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetDataDemands)
        HRESULT ( STDMETHODCALLTYPE *GetDataDemands )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [out] */ __RPC__out UINT *pfuFlags,
            /* [out] */ __RPC__out DWORD *pdwMinRightsData,
            /* [out] */ __RPC__out DWORD *pdwMinExamineData,
            /* [out] */ __RPC__out DWORD *pdwMinDecideData,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, ExamineData)
        HRESULT ( STDMETHODCALLTYPE *ExamineData )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [in] */ UINT fuFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPWSTR pwszExtension,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, MakeDecision)
        HRESULT ( STDMETHODCALLTYPE *MakeDecision )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery2, MakeDecision2)
        HRESULT ( STDMETHODCALLTYPE *MakeDecision2 )( 
            __RPC__in ISCPSecureQuery2 * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertAppLen) BYTE *pAppCertApp,
            /* [in] */ DWORD dwAppCertAppLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertSPLen) BYTE *pAppCertSP,
            /* [in] */ DWORD dwAppCertSPLen,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwRevocationURLLen) LPWSTR *pszRevocationURL,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRevocationURLLen,
            /* [out] */ __RPC__out DWORD *pdwRevocationBitFlag,
            /* [unique][out][in] */ __RPC__inout_opt ULONGLONG *pqwFileSize,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        END_INTERFACE
    } ISCPSecureQuery2Vtbl;

    interface ISCPSecureQuery2
    {
        CONST_VTBL struct ISCPSecureQuery2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureQuery2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureQuery2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureQuery2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureQuery2_GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac)	\
    ( (This)->lpVtbl -> GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac) ) 

#define ISCPSecureQuery2_ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac)	\
    ( (This)->lpVtbl -> ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac) ) 

#define ISCPSecureQuery2_MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac)	\
    ( (This)->lpVtbl -> MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac) ) 

#define ISCPSecureQuery2_GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac) ) 


#define ISCPSecureQuery2_MakeDecision2(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange,abMac)	\
    ( (This)->lpVtbl -> MakeDecision2(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange,abMac) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureQuery2_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureExchange_INTERFACE_DEFINED__
#define __ISCPSecureExchange_INTERFACE_DEFINED__

/* interface ISCPSecureExchange */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureExchange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCB3A0E-33ED-11d3-8470-00C04F79DBC0")
    ISCPSecureExchange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferContainerData( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ObjectData( 
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransferComplete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureExchangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureExchange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureExchange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureExchange * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferContainerData)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerData )( 
            __RPC__in ISCPSecureExchange * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, ObjectData)
        HRESULT ( STDMETHODCALLTYPE *ObjectData )( 
            __RPC__in ISCPSecureExchange * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferComplete)
        HRESULT ( STDMETHODCALLTYPE *TransferComplete )( 
            __RPC__in ISCPSecureExchange * This);
        
        END_INTERFACE
    } ISCPSecureExchangeVtbl;

    interface ISCPSecureExchange
    {
        CONST_VTBL struct ISCPSecureExchangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureExchange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureExchange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureExchange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureExchange_TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac)	\
    ( (This)->lpVtbl -> TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac) ) 

#define ISCPSecureExchange_ObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> ObjectData(This,pData,pdwSize,abMac) ) 

#define ISCPSecureExchange_TransferComplete(This)	\
    ( (This)->lpVtbl -> TransferComplete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureExchange_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureExchange2_INTERFACE_DEFINED__
#define __ISCPSecureExchange2_INTERFACE_DEFINED__

/* interface ISCPSecureExchange2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureExchange2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C62FC7B-2690-483F-9D44-0A20CB35577C")
    ISCPSecureExchange2 : public ISCPSecureExchange
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferContainerData2( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureExchange2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureExchange2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureExchange2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureExchange2 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferContainerData)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerData )( 
            __RPC__in ISCPSecureExchange2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, ObjectData)
        HRESULT ( STDMETHODCALLTYPE *ObjectData )( 
            __RPC__in ISCPSecureExchange2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferComplete)
        HRESULT ( STDMETHODCALLTYPE *TransferComplete )( 
            __RPC__in ISCPSecureExchange2 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange2, TransferContainerData2)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerData2 )( 
            __RPC__in ISCPSecureExchange2 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        END_INTERFACE
    } ISCPSecureExchange2Vtbl;

    interface ISCPSecureExchange2
    {
        CONST_VTBL struct ISCPSecureExchange2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureExchange2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureExchange2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureExchange2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureExchange2_TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac)	\
    ( (This)->lpVtbl -> TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac) ) 

#define ISCPSecureExchange2_ObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> ObjectData(This,pData,pdwSize,abMac) ) 

#define ISCPSecureExchange2_TransferComplete(This)	\
    ( (This)->lpVtbl -> TransferComplete(This) ) 


#define ISCPSecureExchange2_TransferContainerData2(This,pData,dwSize,pProgressCallback,pfuReadyFlags,abMac)	\
    ( (This)->lpVtbl -> TransferContainerData2(This,pData,dwSize,pProgressCallback,pfuReadyFlags,abMac) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureExchange2_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureExchange3_INTERFACE_DEFINED__
#define __ISCPSecureExchange3_INTERFACE_DEFINED__

/* interface ISCPSecureExchange3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureExchange3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ab4e77e4-8908-4b17-bd2a-b1dbe6dd69e1")
    ISCPSecureExchange3 : public ISCPSecureExchange2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferContainerDataOnClearChannel( 
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [out] */ __RPC__out UINT *pfuReadyFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectDataOnClearChannel( 
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransferCompleteForDevice( 
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureExchange3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureExchange3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureExchange3 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferContainerData)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerData )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, ObjectData)
        HRESULT ( STDMETHODCALLTYPE *ObjectData )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange, TransferComplete)
        HRESULT ( STDMETHODCALLTYPE *TransferComplete )( 
            __RPC__in ISCPSecureExchange3 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange2, TransferContainerData2)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerData2 )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [out] */ __RPC__out UINT *pfuReadyFlags,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange3, TransferContainerDataOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *TransferContainerDataOnClearChannel )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [out] */ __RPC__out UINT *pfuReadyFlags);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange3, GetObjectDataOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *GetObjectDataOnClearChannel )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSize) BYTE *pData,
            /* [out][in] */ __RPC__inout DWORD *pdwSize);
        
        DECLSPEC_XFGVIRT(ISCPSecureExchange3, TransferCompleteForDevice)
        HRESULT ( STDMETHODCALLTYPE *TransferCompleteForDevice )( 
            __RPC__in ISCPSecureExchange3 * This,
            /* [in] */ __RPC__in_opt IMDSPDevice *pDevice);
        
        END_INTERFACE
    } ISCPSecureExchange3Vtbl;

    interface ISCPSecureExchange3
    {
        CONST_VTBL struct ISCPSecureExchange3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureExchange3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureExchange3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureExchange3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureExchange3_TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac)	\
    ( (This)->lpVtbl -> TransferContainerData(This,pData,dwSize,pfuReadyFlags,abMac) ) 

#define ISCPSecureExchange3_ObjectData(This,pData,pdwSize,abMac)	\
    ( (This)->lpVtbl -> ObjectData(This,pData,pdwSize,abMac) ) 

#define ISCPSecureExchange3_TransferComplete(This)	\
    ( (This)->lpVtbl -> TransferComplete(This) ) 


#define ISCPSecureExchange3_TransferContainerData2(This,pData,dwSize,pProgressCallback,pfuReadyFlags,abMac)	\
    ( (This)->lpVtbl -> TransferContainerData2(This,pData,dwSize,pProgressCallback,pfuReadyFlags,abMac) ) 


#define ISCPSecureExchange3_TransferContainerDataOnClearChannel(This,pDevice,pData,dwSize,pProgressCallback,pfuReadyFlags)	\
    ( (This)->lpVtbl -> TransferContainerDataOnClearChannel(This,pDevice,pData,dwSize,pProgressCallback,pfuReadyFlags) ) 

#define ISCPSecureExchange3_GetObjectDataOnClearChannel(This,pDevice,pData,pdwSize)	\
    ( (This)->lpVtbl -> GetObjectDataOnClearChannel(This,pDevice,pData,pdwSize) ) 

#define ISCPSecureExchange3_TransferCompleteForDevice(This,pDevice)	\
    ( (This)->lpVtbl -> TransferCompleteForDevice(This,pDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureExchange3_INTERFACE_DEFINED__ */


#ifndef __ISCPSession_INTERFACE_DEFINED__
#define __ISCPSession_INTERFACE_DEFINED__

/* interface ISCPSession */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88a3e6ed-eee4-4619-bbb3-fd4fb62715d1")
    ISCPSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginSession( 
            /* [in] */ __RPC__in_opt IMDSPDevice *pIDevice,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndSession( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecureQuery( 
            /* [out] */ __RPC__deref_out_opt ISCPSecureQuery **ppSecureQuery) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSession * This);
        
        DECLSPEC_XFGVIRT(ISCPSession, BeginSession)
        HRESULT ( STDMETHODCALLTYPE *BeginSession )( 
            __RPC__in ISCPSession * This,
            /* [in] */ __RPC__in_opt IMDSPDevice *pIDevice,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx);
        
        DECLSPEC_XFGVIRT(ISCPSession, EndSession)
        HRESULT ( STDMETHODCALLTYPE *EndSession )( 
            __RPC__in ISCPSession * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeCtx) BYTE *pCtx,
            /* [in] */ DWORD dwSizeCtx);
        
        DECLSPEC_XFGVIRT(ISCPSession, GetSecureQuery)
        HRESULT ( STDMETHODCALLTYPE *GetSecureQuery )( 
            __RPC__in ISCPSession * This,
            /* [out] */ __RPC__deref_out_opt ISCPSecureQuery **ppSecureQuery);
        
        END_INTERFACE
    } ISCPSessionVtbl;

    interface ISCPSession
    {
        CONST_VTBL struct ISCPSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSession_BeginSession(This,pIDevice,pCtx,dwSizeCtx)	\
    ( (This)->lpVtbl -> BeginSession(This,pIDevice,pCtx,dwSizeCtx) ) 

#define ISCPSession_EndSession(This,pCtx,dwSizeCtx)	\
    ( (This)->lpVtbl -> EndSession(This,pCtx,dwSizeCtx) ) 

#define ISCPSession_GetSecureQuery(This,ppSecureQuery)	\
    ( (This)->lpVtbl -> GetSecureQuery(This,ppSecureQuery) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSession_INTERFACE_DEFINED__ */


#ifndef __ISCPSecureQuery3_INTERFACE_DEFINED__
#define __ISCPSecureQuery3_INTERFACE_DEFINED__

/* interface ISCPSecureQuery3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCPSecureQuery3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7EDD1A2-4DAB-484b-B3C5-AD39B8B4C0B1")
    ISCPSecureQuery3 : public ISCPSecureQuery2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRightsOnClearChannel( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeDecisionOnClearChannel( 
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertAppLen) BYTE *pAppCertApp,
            /* [in] */ DWORD dwAppCertAppLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertSPLen) BYTE *pAppCertSP,
            /* [in] */ DWORD dwAppCertSPLen,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwRevocationURLLen) LPWSTR *pszRevocationURL,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRevocationURLLen,
            /* [out] */ __RPC__out DWORD *pdwRevocationBitFlag,
            /* [unique][out][in] */ __RPC__inout_opt ULONGLONG *pqwFileSize,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCPSecureQuery3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCPSecureQuery3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCPSecureQuery3 * This);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetDataDemands)
        HRESULT ( STDMETHODCALLTYPE *GetDataDemands )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [out] */ __RPC__out UINT *pfuFlags,
            /* [out] */ __RPC__out DWORD *pdwMinRightsData,
            /* [out] */ __RPC__out DWORD *pdwMinExamineData,
            /* [out] */ __RPC__out DWORD *pdwMinDecideData,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, ExamineData)
        HRESULT ( STDMETHODCALLTYPE *ExamineData )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [in] */ UINT fuFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPWSTR pwszExtension,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, MakeDecision)
        HRESULT ( STDMETHODCALLTYPE *MakeDecision )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery, GetRights)
        HRESULT ( STDMETHODCALLTYPE *GetRights )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery2, MakeDecision2)
        HRESULT ( STDMETHODCALLTYPE *MakeDecision2 )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertAppLen) BYTE *pAppCertApp,
            /* [in] */ DWORD dwAppCertAppLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertSPLen) BYTE *pAppCertSP,
            /* [in] */ DWORD dwAppCertSPLen,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwRevocationURLLen) LPWSTR *pszRevocationURL,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRevocationURLLen,
            /* [out] */ __RPC__out DWORD *pdwRevocationBitFlag,
            /* [unique][out][in] */ __RPC__inout_opt ULONGLONG *pqwFileSize,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange,
            /* [out][in] */ __RPC__inout_ecount_full(WMDM_MAC_LENGTH) BYTE abMac[ 8 ]);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery3, GetRightsOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *GetRightsOnClearChannel )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStgGlobals,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnRightsCount) PWMDMRIGHTS *ppRights,
            /* [out] */ __RPC__out UINT *pnRightsCount);
        
        DECLSPEC_XFGVIRT(ISCPSecureQuery3, MakeDecisionOnClearChannel)
        HRESULT ( STDMETHODCALLTYPE *MakeDecisionOnClearChannel )( 
            __RPC__in ISCPSecureQuery3 * This,
            /* [in] */ UINT fuFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pData,
            /* [in] */ DWORD dwSize,
            /* [in] */ DWORD dwAppSec,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSessionKeyLen) BYTE *pbSPSessionKey,
            /* [in] */ DWORD dwSessionKeyLen,
            /* [in] */ __RPC__in_opt IMDSPStorageGlobals *pStorageGlobals,
            /* [in] */ __RPC__in_opt IWMDMProgress3 *pProgressCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertAppLen) BYTE *pAppCertApp,
            /* [in] */ DWORD dwAppCertAppLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwAppCertSPLen) BYTE *pAppCertSP,
            /* [in] */ DWORD dwAppCertSPLen,
            /* [size_is][size_is][string][out][in] */ __RPC__deref_inout_ecount_full_opt_string(*pdwRevocationURLLen) LPWSTR *pszRevocationURL,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRevocationURLLen,
            /* [out] */ __RPC__out DWORD *pdwRevocationBitFlag,
            /* [unique][out][in] */ __RPC__inout_opt ULONGLONG *pqwFileSize,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown,
            /* [out] */ __RPC__deref_out_opt ISCPSecureExchange **ppExchange);
        
        END_INTERFACE
    } ISCPSecureQuery3Vtbl;

    interface ISCPSecureQuery3
    {
        CONST_VTBL struct ISCPSecureQuery3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCPSecureQuery3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCPSecureQuery3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCPSecureQuery3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCPSecureQuery3_GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac)	\
    ( (This)->lpVtbl -> GetDataDemands(This,pfuFlags,pdwMinRightsData,pdwMinExamineData,pdwMinDecideData,abMac) ) 

#define ISCPSecureQuery3_ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac)	\
    ( (This)->lpVtbl -> ExamineData(This,fuFlags,pwszExtension,pData,dwSize,abMac) ) 

#define ISCPSecureQuery3_MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac)	\
    ( (This)->lpVtbl -> MakeDecision(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,ppExchange,abMac) ) 

#define ISCPSecureQuery3_GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac)	\
    ( (This)->lpVtbl -> GetRights(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,ppRights,pnRightsCount,abMac) ) 


#define ISCPSecureQuery3_MakeDecision2(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange,abMac)	\
    ( (This)->lpVtbl -> MakeDecision2(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange,abMac) ) 


#define ISCPSecureQuery3_GetRightsOnClearChannel(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,pProgressCallback,ppRights,pnRightsCount)	\
    ( (This)->lpVtbl -> GetRightsOnClearChannel(This,pData,dwSize,pbSPSessionKey,dwSessionKeyLen,pStgGlobals,pProgressCallback,ppRights,pnRightsCount) ) 

#define ISCPSecureQuery3_MakeDecisionOnClearChannel(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pProgressCallback,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange)	\
    ( (This)->lpVtbl -> MakeDecisionOnClearChannel(This,fuFlags,pData,dwSize,dwAppSec,pbSPSessionKey,dwSessionKeyLen,pStorageGlobals,pProgressCallback,pAppCertApp,dwAppCertAppLen,pAppCertSP,dwAppCertSPLen,pszRevocationURL,pdwRevocationURLLen,pdwRevocationBitFlag,pqwFileSize,pUnknown,ppExchange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCPSecureQuery3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mswmdm_0000_0056 */
/* [local] */ 

#define SAC_MAC_LEN 8


extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0056_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0056_v0_0_s_ifspec;

#ifndef __IComponentAuthenticate_INTERFACE_DEFINED__
#define __IComponentAuthenticate_INTERFACE_DEFINED__

/* interface IComponentAuthenticate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IComponentAuthenticate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9889C00-6D2B-11d3-8496-00C04F79DBC0")
    IComponentAuthenticate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SACAuth( 
            /* [in] */ DWORD dwProtocolID,
            /* [in] */ DWORD dwPass,
            /* [size_is][in] */ __RPC__in_ecount_full(dwDataInLen) BYTE *pbDataIn,
            /* [in] */ DWORD dwDataInLen,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwDataOutLen) BYTE **ppbDataOut,
            /* [out] */ __RPC__out DWORD *pdwDataOutLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SACGetProtocols( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwProtocolCount) DWORD **ppdwProtocols,
            /* [out] */ __RPC__out DWORD *pdwProtocolCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentAuthenticateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentAuthenticate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentAuthenticate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentAuthenticate * This);
        
        DECLSPEC_XFGVIRT(IComponentAuthenticate, SACAuth)
        HRESULT ( STDMETHODCALLTYPE *SACAuth )( 
            __RPC__in IComponentAuthenticate * This,
            /* [in] */ DWORD dwProtocolID,
            /* [in] */ DWORD dwPass,
            /* [size_is][in] */ __RPC__in_ecount_full(dwDataInLen) BYTE *pbDataIn,
            /* [in] */ DWORD dwDataInLen,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwDataOutLen) BYTE **ppbDataOut,
            /* [out] */ __RPC__out DWORD *pdwDataOutLen);
        
        DECLSPEC_XFGVIRT(IComponentAuthenticate, SACGetProtocols)
        HRESULT ( STDMETHODCALLTYPE *SACGetProtocols )( 
            __RPC__in IComponentAuthenticate * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwProtocolCount) DWORD **ppdwProtocols,
            /* [out] */ __RPC__out DWORD *pdwProtocolCount);
        
        END_INTERFACE
    } IComponentAuthenticateVtbl;

    interface IComponentAuthenticate
    {
        CONST_VTBL struct IComponentAuthenticateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentAuthenticate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentAuthenticate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentAuthenticate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentAuthenticate_SACAuth(This,dwProtocolID,dwPass,pbDataIn,dwDataInLen,ppbDataOut,pdwDataOutLen)	\
    ( (This)->lpVtbl -> SACAuth(This,dwProtocolID,dwPass,pbDataIn,dwDataInLen,ppbDataOut,pdwDataOutLen) ) 

#define IComponentAuthenticate_SACGetProtocols(This,ppdwProtocols,pdwProtocolCount)	\
    ( (This)->lpVtbl -> SACGetProtocols(This,ppdwProtocols,pdwProtocolCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentAuthenticate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mswmdm_0000_0057 */
/* [local] */ 

const GUID EVENT_WMDM_CONTENT_TRANSFER = { 0x339C9BF4, 0xBCFE, 0x4ED8, { 0x94, 0xDF,  0xEA,  0xF8,  0xC2,  0x6A,  0xB6,  0x1B } };


extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0057_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0057_v0_0_s_ifspec;


#ifndef __MSWMDMLib_LIBRARY_DEFINED__
#define __MSWMDMLib_LIBRARY_DEFINED__

/* library MSWMDMLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_MSWMDMLib;

EXTERN_C const CLSID CLSID_MediaDevMgrClassFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("50040C1D-BDBF-4924-B873-F14D6C5BFD66")
MediaDevMgrClassFactory;
#endif

EXTERN_C const CLSID CLSID_MediaDevMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("25BAAD81-3560-11D3-8471-00C04F79DBC0")
MediaDevMgr;
#endif

EXTERN_C const CLSID CLSID_WMDMDevice;

#ifdef __cplusplus

class DECLSPEC_UUID("807B3CDF-357A-11d3-8471-00C04F79DBC0")
WMDMDevice;
#endif

EXTERN_C const CLSID CLSID_WMDMStorage;

#ifdef __cplusplus

class DECLSPEC_UUID("807B3CE0-357A-11d3-8471-00C04F79DBC0")
WMDMStorage;
#endif

EXTERN_C const CLSID CLSID_WMDMStorageGlobal;

#ifdef __cplusplus

class DECLSPEC_UUID("807B3CE1-357A-11d3-8471-00C04F79DBC0")
WMDMStorageGlobal;
#endif

EXTERN_C const CLSID CLSID_WMDMDeviceEnum;

#ifdef __cplusplus

class DECLSPEC_UUID("430E35AF-3971-11D3-8474-00C04F79DBC0")
WMDMDeviceEnum;
#endif

EXTERN_C const CLSID CLSID_WMDMStorageEnum;

#ifdef __cplusplus

class DECLSPEC_UUID("EB401A3B-3AF7-11d3-8474-00C04F79DBC0")
WMDMStorageEnum;
#endif
#endif /* __MSWMDMLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mswmdm_0000_0058 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0058_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mswmdm_0000_0058_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


