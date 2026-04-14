

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
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

#ifndef __mfvirtualcamera_h__
#define __mfvirtualcamera_h__

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

#ifndef __IMFCameraSyncObject_FWD_DEFINED__
#define __IMFCameraSyncObject_FWD_DEFINED__
typedef interface IMFCameraSyncObject IMFCameraSyncObject;

#endif 	/* __IMFCameraSyncObject_FWD_DEFINED__ */


#ifndef __IMFVirtualCamera_FWD_DEFINED__
#define __IMFVirtualCamera_FWD_DEFINED__
typedef interface IMFVirtualCamera IMFVirtualCamera;

#endif 	/* __IMFVirtualCamera_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"
#include "mfidl.h"
#include "devpropdef.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfvirtualcamera_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// {6EDC630D-C2E3-43B7-B2D1-20525A1AF120}, 3
DEFINE_DEVPROPKEY(DEVPKEY_DeviceInterface_IsVirtualCamera,                       0x6EDC630D, 0xC2E3, 0x43B7, 0xB2, 0xD1, 0x20, 0x52, 0x5A, 0x1A, 0xF1, 0x20, 3);    // DEVPROP_TYPE_BOOLEAN
/// {6EDC630D-C2E3-43B7-B2D1-20525A1AF120}, 4
DEFINE_DEVPROPKEY(DEVPKEY_DeviceInterface_IsWindowsCameraEffectAvailable,        0x6EDC630D, 0xC2E3, 0x43B7, 0xB2, 0xD1, 0x20, 0x52, 0x5A, 0x1A, 0xF1, 0x20, 4);    // DEVPROP_TYPE_BOOLEAN
/// {6EDC630D-C2E3-43B7-B2D1-20525A1AF120}, 5
DEFINE_DEVPROPKEY(DEVPKEY_DeviceInterface_VirtualCameraAssociatedCameras,        0x6EDC630D, 0xC2E3, 0x43B7, 0xB2, 0xD1, 0x20, 0x52, 0x5A, 0x1A, 0xF1, 0x20, 5);    // DEVPROP_TYPE_STRING_LIST
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
#ifndef _DEVPROPDEF_H_
typedef void *DEVPROPKEY;

typedef ULONG DEVPROPTYPE;

#endif
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_ENABLE_MS_CAMERA_EFFECTS,                     0x28A5531A, 0x57DD, 0x4FD5, 0xAA, 0xA7, 0x38, 0x5A, 0xBF, 0x57, 0xD7, 0x85);
EXTERN_GUID(MF_VIRTUALCAMERA_ASSOCIATED_CAMERA_SOURCES,                          0x1BB79E7C, 0x5D83, 0x438C, 0x94, 0xD8, 0xE5, 0xF0, 0xDF, 0x6D, 0x32, 0x79);
EXTERN_GUID(MF_VIRTUALCAMERA_PROVIDE_ASSOCIATED_CAMERA_SOURCES,                  0xF0273718, 0x4A4D, 0x4AC5, 0xA1, 0x5D, 0x30, 0x5E, 0xB5, 0xE9, 0x06, 0x67);
EXTERN_GUID(MF_VIRTUALCAMERA_CONFIGURATION_APP_PACKAGE_FAMILY_NAME,              0x658ABE51, 0x8044, 0x462E, 0x97, 0xEA, 0xE6, 0x76, 0xFD, 0x72, 0x05, 0x5F);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_INITIALIZE,                 0xE52C4DFF, 0xE46D, 0x4D0B, 0xBC, 0x75, 0xDD, 0xD4, 0xC8, 0x72, 0x3F, 0x96);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_START,                      0xB1EEB989, 0xB456, 0x4F4A, 0xAE, 0x40, 0x07, 0x9C, 0x28, 0xE2, 0x4A, 0xF8);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_STOP,                       0xB7FE7A61, 0xFE91, 0x415E, 0x86, 0x08, 0xD3, 0x7D, 0xED, 0xB1, 0xA5, 0x8B);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_UNINITIALIZE,               0xA0EBABA7, 0xA422, 0x4E33, 0x84, 0x01, 0xB3, 0x7D, 0x28, 0x00, 0xAA, 0x67);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_PIPELINE_SHUTDOWN,                 0x45A81B31, 0x43F8, 0x4E5D, 0x8C, 0xE2, 0x22, 0xDC, 0xE0, 0x26, 0x99, 0x6D);
EXTERN_GUID(MF_FRAMESERVER_VCAMEVENT_EXTENDED_CUSTOM_EVENT,                      0x6E59489C, 0x47D3, 0x4467, 0x83, 0xEF, 0x12, 0xD3, 0x4E, 0x87, 0x16, 0x65);
typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0001
    {
        MFVirtualCameraType_SoftwareCameraSource	= 0
    } 	MFVirtualCameraType;

typedef enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0001 *PMFVirtualCameraType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0002
    {
        MFVirtualCameraLifetime_Session	= 0,
        MFVirtualCameraLifetime_System	= ( MFVirtualCameraLifetime_Session + 1 ) 
    } 	MFVirtualCameraLifetime;

typedef enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0002 *PMFVirtualCameraLifetime;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0003
    {
        MFVirtualCameraAccess_CurrentUser	= 0,
        MFVirtualCameraAccess_AllUsers	= ( MFVirtualCameraAccess_CurrentUser + 1 ) 
    } 	MFVirtualCameraAccess;

typedef enum __MIDL___MIDL_itf_mfvirtualcamera_0000_0000_0003 *PMFVirtualCameraAccess;



extern RPC_IF_HANDLE __MIDL_itf_mfvirtualcamera_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfvirtualcamera_0000_0000_v0_0_s_ifspec;

#ifndef __IMFCameraSyncObject_INTERFACE_DEFINED__
#define __IMFCameraSyncObject_INTERFACE_DEFINED__

/* interface IMFCameraSyncObject */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraSyncObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6338B23A-3042-49D2-A3EA-EC0FED815407")
    IMFCameraSyncObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WaitOnSignal( 
            /* [annotation][in] */ 
            _In_  DWORD timeOutInMs) = 0;
        
        virtual void STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraSyncObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraSyncObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraSyncObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraSyncObject * This);
        
        DECLSPEC_XFGVIRT(IMFCameraSyncObject, WaitOnSignal)
        HRESULT ( STDMETHODCALLTYPE *WaitOnSignal )( 
            IMFCameraSyncObject * This,
            /* [annotation][in] */ 
            _In_  DWORD timeOutInMs);
        
        DECLSPEC_XFGVIRT(IMFCameraSyncObject, Shutdown)
        void ( STDMETHODCALLTYPE *Shutdown )( 
            IMFCameraSyncObject * This);
        
        END_INTERFACE
    } IMFCameraSyncObjectVtbl;

    interface IMFCameraSyncObject
    {
        CONST_VTBL struct IMFCameraSyncObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraSyncObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraSyncObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraSyncObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraSyncObject_WaitOnSignal(This,timeOutInMs)	\
    ( (This)->lpVtbl -> WaitOnSignal(This,timeOutInMs) ) 

#define IMFCameraSyncObject_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraSyncObject_INTERFACE_DEFINED__ */


#ifndef __IMFVirtualCamera_INTERFACE_DEFINED__
#define __IMFVirtualCamera_INTERFACE_DEFINED__

/* interface IMFVirtualCamera */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVirtualCamera;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1C08A864-EF6C-4C75-AF59-5F2D68DA9563")
    IMFVirtualCamera : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddDeviceSourceInfo( 
            /* [annotation][in] */ 
            _In_z_  LPCWSTR DeviceSourceInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddProperty( 
            /* [annotation][in] */ 
            _In_  const DEVPROPKEY *pKey,
            /* [annotation][in] */ 
            _In_  DEVPROPTYPE Type,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbData)  const BYTE *pbData,
            /* [annotation][in] */ 
            _In_  ULONG cbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRegistryEntry( 
            /* [annotation][in] */ 
            _In_z_  LPCWSTR EntryName,
            /* [annotation][in] */ 
            _In_opt_z_  LPCWSTR SubkeyPath,
            /* [annotation][in] */ 
            _In_  DWORD dwRegType,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbData)  const BYTE *pbData,
            /* [annotation][in] */ 
            _In_  ULONG cbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [annotation][in] */ 
            _In_opt_  IMFAsyncCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaSource( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppMediaSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendCameraProperty( 
            /* [annotation][in] */ 
            _In_  REFGUID propertySet,
            /* [annotation][in] */ 
            _In_  ULONG propertyId,
            /* [annotation][in] */ 
            _In_  ULONG propertyFlags,
            /* [annotation][in] */ 
            _Inout_updates_bytes_opt_(propertyPayloadLength)  void *propertyPayload,
            /* [annotation][in] */ 
            _In_  ULONG propertyPayloadLength,
            /* [annotation][in] */ 
            _Inout_updates_bytes_to_opt_(dataLength,*dataWritten)  void *data,
            /* [annotation][in] */ 
            _In_  ULONG dataLength,
            /* [annotation][out] */ 
            _Out_  ULONG *dataWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSyncEvent( 
            /* [annotation][in] */ 
            _In_  REFGUID kseventSet,
            /* [annotation][in] */ 
            _In_  ULONG kseventId,
            /* [annotation][in] */ 
            _In_  ULONG kseventFlags,
            /* [annotation][system_handle][in] */ 
            _In_  HANDLE eventHandle,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraSyncObject **cameraSyncObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSyncSemaphore( 
            /* [annotation][in] */ 
            _In_  REFGUID kseventSet,
            /* [annotation][in] */ 
            _In_  ULONG kseventId,
            /* [annotation][in] */ 
            _In_  ULONG kseventFlags,
            /* [annotation][system_handle][in] */ 
            _In_  HANDLE semaphoreHandle,
            /* [annotation][in] */ 
            _In_  LONG semaphoreAdjustment,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraSyncObject **cameraSyncObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVirtualCameraVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVirtualCamera * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFVirtualCamera * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFVirtualCamera * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFVirtualCamera * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFVirtualCamera * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFVirtualCamera * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFVirtualCamera * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, AddDeviceSourceInfo)
        HRESULT ( STDMETHODCALLTYPE *AddDeviceSourceInfo )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR DeviceSourceInfo);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, AddProperty)
        HRESULT ( STDMETHODCALLTYPE *AddProperty )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_  const DEVPROPKEY *pKey,
            /* [annotation][in] */ 
            _In_  DEVPROPTYPE Type,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbData)  const BYTE *pbData,
            /* [annotation][in] */ 
            _In_  ULONG cbData);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, AddRegistryEntry)
        HRESULT ( STDMETHODCALLTYPE *AddRegistryEntry )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR EntryName,
            /* [annotation][in] */ 
            _In_opt_z_  LPCWSTR SubkeyPath,
            /* [annotation][in] */ 
            _In_  DWORD dwRegType,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbData)  const BYTE *pbData,
            /* [annotation][in] */ 
            _In_  ULONG cbData);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_opt_  IMFAsyncCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IMFVirtualCamera * This);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, GetMediaSource)
        HRESULT ( STDMETHODCALLTYPE *GetMediaSource )( 
            IMFVirtualCamera * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppMediaSource);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, SendCameraProperty)
        HRESULT ( STDMETHODCALLTYPE *SendCameraProperty )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_  REFGUID propertySet,
            /* [annotation][in] */ 
            _In_  ULONG propertyId,
            /* [annotation][in] */ 
            _In_  ULONG propertyFlags,
            /* [annotation][in] */ 
            _Inout_updates_bytes_opt_(propertyPayloadLength)  void *propertyPayload,
            /* [annotation][in] */ 
            _In_  ULONG propertyPayloadLength,
            /* [annotation][in] */ 
            _Inout_updates_bytes_to_opt_(dataLength,*dataWritten)  void *data,
            /* [annotation][in] */ 
            _In_  ULONG dataLength,
            /* [annotation][out] */ 
            _Out_  ULONG *dataWritten);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, CreateSyncEvent)
        HRESULT ( STDMETHODCALLTYPE *CreateSyncEvent )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_  REFGUID kseventSet,
            /* [annotation][in] */ 
            _In_  ULONG kseventId,
            /* [annotation][in] */ 
            _In_  ULONG kseventFlags,
            /* [annotation][system_handle][in] */ 
            _In_  HANDLE eventHandle,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraSyncObject **cameraSyncObject);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, CreateSyncSemaphore)
        HRESULT ( STDMETHODCALLTYPE *CreateSyncSemaphore )( 
            IMFVirtualCamera * This,
            /* [annotation][in] */ 
            _In_  REFGUID kseventSet,
            /* [annotation][in] */ 
            _In_  ULONG kseventId,
            /* [annotation][in] */ 
            _In_  ULONG kseventFlags,
            /* [annotation][system_handle][in] */ 
            _In_  HANDLE semaphoreHandle,
            /* [annotation][in] */ 
            _In_  LONG semaphoreAdjustment,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraSyncObject **cameraSyncObject);
        
        DECLSPEC_XFGVIRT(IMFVirtualCamera, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFVirtualCamera * This);
        
        END_INTERFACE
    } IMFVirtualCameraVtbl;

    interface IMFVirtualCamera
    {
        CONST_VTBL struct IMFVirtualCameraVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVirtualCamera_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVirtualCamera_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVirtualCamera_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVirtualCamera_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFVirtualCamera_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFVirtualCamera_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFVirtualCamera_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFVirtualCamera_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFVirtualCamera_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFVirtualCamera_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFVirtualCamera_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFVirtualCamera_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFVirtualCamera_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFVirtualCamera_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFVirtualCamera_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFVirtualCamera_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFVirtualCamera_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFVirtualCamera_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFVirtualCamera_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFVirtualCamera_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFVirtualCamera_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFVirtualCamera_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFVirtualCamera_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFVirtualCamera_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFVirtualCamera_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFVirtualCamera_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFVirtualCamera_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFVirtualCamera_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFVirtualCamera_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFVirtualCamera_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFVirtualCamera_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFVirtualCamera_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFVirtualCamera_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFVirtualCamera_AddDeviceSourceInfo(This,DeviceSourceInfo)	\
    ( (This)->lpVtbl -> AddDeviceSourceInfo(This,DeviceSourceInfo) ) 

#define IMFVirtualCamera_AddProperty(This,pKey,Type,pbData,cbData)	\
    ( (This)->lpVtbl -> AddProperty(This,pKey,Type,pbData,cbData) ) 

#define IMFVirtualCamera_AddRegistryEntry(This,EntryName,SubkeyPath,dwRegType,pbData,cbData)	\
    ( (This)->lpVtbl -> AddRegistryEntry(This,EntryName,SubkeyPath,dwRegType,pbData,cbData) ) 

#define IMFVirtualCamera_Start(This,pCallback)	\
    ( (This)->lpVtbl -> Start(This,pCallback) ) 

#define IMFVirtualCamera_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFVirtualCamera_Remove(This)	\
    ( (This)->lpVtbl -> Remove(This) ) 

#define IMFVirtualCamera_GetMediaSource(This,ppMediaSource)	\
    ( (This)->lpVtbl -> GetMediaSource(This,ppMediaSource) ) 

#define IMFVirtualCamera_SendCameraProperty(This,propertySet,propertyId,propertyFlags,propertyPayload,propertyPayloadLength,data,dataLength,dataWritten)	\
    ( (This)->lpVtbl -> SendCameraProperty(This,propertySet,propertyId,propertyFlags,propertyPayload,propertyPayloadLength,data,dataLength,dataWritten) ) 

#define IMFVirtualCamera_CreateSyncEvent(This,kseventSet,kseventId,kseventFlags,eventHandle,cameraSyncObject)	\
    ( (This)->lpVtbl -> CreateSyncEvent(This,kseventSet,kseventId,kseventFlags,eventHandle,cameraSyncObject) ) 

#define IMFVirtualCamera_CreateSyncSemaphore(This,kseventSet,kseventId,kseventFlags,semaphoreHandle,semaphoreAdjustment,cameraSyncObject)	\
    ( (This)->lpVtbl -> CreateSyncSemaphore(This,kseventSet,kseventId,kseventFlags,semaphoreHandle,semaphoreAdjustment,cameraSyncObject) ) 

#define IMFVirtualCamera_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVirtualCamera_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfvirtualcamera_0000_0002 */
/* [local] */ 


STDAPI
MFCreateVirtualCamera(
    _In_ MFVirtualCameraType type,
    _In_ MFVirtualCameraLifetime lifetime,
    _In_ MFVirtualCameraAccess access,
    _In_z_ LPCWSTR friendlyName,
    _In_z_ LPCWSTR sourceId,
    _In_reads_opt_(categoryCount) const GUID* categories,
    _In_ ULONG categoryCount,
    _COM_Outptr_ IMFVirtualCamera** virtualCamera
    );


STDAPI
MFIsVirtualCameraTypeSupported(
    _In_ MFVirtualCameraType type,
    _Out_ BOOL* supported
    );

#endif // (WINVER >= NTDDI_WIN10_CO)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfvirtualcamera_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfvirtualcamera_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


