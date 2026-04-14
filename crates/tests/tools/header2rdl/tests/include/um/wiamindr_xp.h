

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

#ifndef __wiamindr_xp_h__
#define __wiamindr_xp_h__

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

#ifndef __IWiaMiniDrv_FWD_DEFINED__
#define __IWiaMiniDrv_FWD_DEFINED__
typedef interface IWiaMiniDrv IWiaMiniDrv;

#endif 	/* __IWiaMiniDrv_FWD_DEFINED__ */


#ifndef __IWiaMiniDrvCallBack_FWD_DEFINED__
#define __IWiaMiniDrvCallBack_FWD_DEFINED__
typedef interface IWiaMiniDrvCallBack IWiaMiniDrvCallBack;

#endif 	/* __IWiaMiniDrvCallBack_FWD_DEFINED__ */


#ifndef __IWiaDrvItem_FWD_DEFINED__
#define __IWiaDrvItem_FWD_DEFINED__
typedef interface IWiaDrvItem IWiaDrvItem;

#endif 	/* __IWiaDrvItem_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "propidl.h"
#include "wia_xp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wiamindr_xp_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)





typedef struct _MINIDRV_TRANSFER_CONTEXT
    {
    LONG lSize;
    LONG lWidthInPixels;
    LONG lLines;
    LONG lDepth;
    LONG lXRes;
    LONG lYRes;
    LONG lCompression;
    GUID guidFormatID;
    LONG tymed;
    LONG_PTR hFile;
    LONG cbOffset;
    LONG lBufferSize;
    LONG lActiveBuffer;
    LONG lNumBuffers;
    BYTE *pBaseBuffer;
    BYTE *pTransferBuffer;
    BOOL bTransferDataCB;
    BOOL bClassDrvAllocBuf;
    LONG_PTR lClientAddress;
    IWiaMiniDrvCallBack *pIWiaMiniDrvCallBack;
    LONG lImageSize;
    LONG lHeaderSize;
    LONG lItemSize;
    LONG cbWidthInBytes;
    LONG lPage;
    LONG lCurIfdOffset;
    LONG lPrevIfdOffset;
    } 	MINIDRV_TRANSFER_CONTEXT;

typedef struct _MINIDRV_TRANSFER_CONTEXT *PMINIDRV_TRANSFER_CONTEXT;

typedef struct _WIA_DEV_CAP_DRV
    {
    GUID *guid;
    ULONG ulFlags;
    LPOLESTR wszName;
    LPOLESTR wszDescription;
    LPOLESTR wszIcon;
    } 	WIA_DEV_CAP_DRV;

typedef struct _WIA_DEV_CAP_DRV *PWIA_DEV_CAP_DRV;



extern RPC_IF_HANDLE __MIDL_itf_wiamindr_xp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wiamindr_xp_0000_0000_v0_0_s_ifspec;

#ifndef __IWiaMiniDrv_INTERFACE_DEFINED__
#define __IWiaMiniDrv_INTERFACE_DEFINED__

/* interface IWiaMiniDrv */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaMiniDrv;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d8cdee14-3c6c-11d2-9a35-00c04fa36145")
    IWiaMiniDrv : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvInitializeWia( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0000,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0001,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaMiniDrv0002,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaMiniDrv0003,
            /* [in] */ __RPC__in_opt IUnknown *__MIDL__IWiaMiniDrv0004,
            /* [in] */ __RPC__in_opt IUnknown *__MIDL__IWiaMiniDrv0005,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaMiniDrv0006,
            /* [out] */ __RPC__deref_out_opt IUnknown **__MIDL__IWiaMiniDrv0007,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0008) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvAcquireItemData( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0009,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0010,
            /* [out][in] */ __RPC__inout PMINIDRV_TRANSFER_CONTEXT __MIDL__IWiaMiniDrv0011,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0012) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvInitItemProperties( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0013,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0014,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0015) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvValidateItemProperties( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0016,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0017,
            /* [in] */ ULONG __MIDL__IWiaMiniDrv0018,
            /* [in] */ __RPC__in const PROPSPEC *__MIDL__IWiaMiniDrv0019,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0020) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvWriteItemProperties( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0021,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0022,
            /* [in] */ __RPC__in PMINIDRV_TRANSFER_CONTEXT __MIDL__IWiaMiniDrv0023,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0024) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvReadItemProperties( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0025,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0026,
            /* [in] */ ULONG __MIDL__IWiaMiniDrv0027,
            /* [in] */ __RPC__in const PROPSPEC *__MIDL__IWiaMiniDrv0028,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0029) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvLockWiaDevice( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0030,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0031,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0032) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvUnLockWiaDevice( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0033,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0034,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0035) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvAnalyzeItem( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0036,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0037,
            /* [in] */ __RPC__in LONG *__MIDL__IWiaMiniDrv0038) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvGetDeviceErrorStr( 
            /* [in] */ LONG __MIDL__IWiaMiniDrv0039,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0040,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *__MIDL__IWiaMiniDrv0041,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0042) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvDeviceCommand( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0043,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0044,
            /* [in] */ __RPC__in const GUID *__MIDL__IWiaMiniDrv0045,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaMiniDrv0046,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0047) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvGetCapabilities( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0048,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0049,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0050,
            /* [out] */ __RPC__deref_out_opt WIA_DEV_CAP_DRV **__MIDL__IWiaMiniDrv0051,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0052) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvDeleteItem( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0053,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0054,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0055) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvFreeDrvItemContext( 
            /* [in] */ LONG __MIDL__IWiaMiniDrv0056,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0057,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0058) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvGetWiaFormatInfo( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0059,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0060,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0061,
            /* [out] */ __RPC__deref_out_opt WIA_FORMAT_INFO **__MIDL__IWiaMiniDrv0062,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0063) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvNotifyPnpEvent( 
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [in] */ __RPC__in BSTR bstrDeviceID,
            /* [in] */ ULONG ulReserved) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE drvUnInitializeWia( 
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0064) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaMiniDrvVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaMiniDrv * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaMiniDrv * This);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvInitializeWia)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvInitializeWia )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0000,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0001,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaMiniDrv0002,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaMiniDrv0003,
            /* [in] */ __RPC__in_opt IUnknown *__MIDL__IWiaMiniDrv0004,
            /* [in] */ __RPC__in_opt IUnknown *__MIDL__IWiaMiniDrv0005,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaMiniDrv0006,
            /* [out] */ __RPC__deref_out_opt IUnknown **__MIDL__IWiaMiniDrv0007,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0008);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvAcquireItemData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvAcquireItemData )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0009,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0010,
            /* [out][in] */ __RPC__inout PMINIDRV_TRANSFER_CONTEXT __MIDL__IWiaMiniDrv0011,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0012);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvInitItemProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvInitItemProperties )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0013,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0014,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0015);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvValidateItemProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvValidateItemProperties )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0016,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0017,
            /* [in] */ ULONG __MIDL__IWiaMiniDrv0018,
            /* [in] */ __RPC__in const PROPSPEC *__MIDL__IWiaMiniDrv0019,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0020);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvWriteItemProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvWriteItemProperties )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0021,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0022,
            /* [in] */ __RPC__in PMINIDRV_TRANSFER_CONTEXT __MIDL__IWiaMiniDrv0023,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0024);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvReadItemProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvReadItemProperties )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0025,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0026,
            /* [in] */ ULONG __MIDL__IWiaMiniDrv0027,
            /* [in] */ __RPC__in const PROPSPEC *__MIDL__IWiaMiniDrv0028,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0029);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvLockWiaDevice)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvLockWiaDevice )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0030,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0031,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0032);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvUnLockWiaDevice)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvUnLockWiaDevice )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0033,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0034,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0035);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvAnalyzeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvAnalyzeItem )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0036,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0037,
            /* [in] */ __RPC__in LONG *__MIDL__IWiaMiniDrv0038);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvGetDeviceErrorStr)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvGetDeviceErrorStr )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0039,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0040,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *__MIDL__IWiaMiniDrv0041,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0042);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvDeviceCommand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvDeviceCommand )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0043,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0044,
            /* [in] */ __RPC__in const GUID *__MIDL__IWiaMiniDrv0045,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaMiniDrv0046,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0047);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvGetCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvGetCapabilities )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0048,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0049,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0050,
            /* [out] */ __RPC__deref_out_opt WIA_DEV_CAP_DRV **__MIDL__IWiaMiniDrv0051,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0052);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvDeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvDeleteItem )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0053,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0054,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0055);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvFreeDrvItemContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvFreeDrvItemContext )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0056,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0057,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0058);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvGetWiaFormatInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvGetWiaFormatInfo )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0059,
            /* [in] */ LONG __MIDL__IWiaMiniDrv0060,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0061,
            /* [out] */ __RPC__deref_out_opt WIA_FORMAT_INFO **__MIDL__IWiaMiniDrv0062,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaMiniDrv0063);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvNotifyPnpEvent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvNotifyPnpEvent )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [in] */ __RPC__in BSTR bstrDeviceID,
            /* [in] */ ULONG ulReserved);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrv, drvUnInitializeWia)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *drvUnInitializeWia )( 
            __RPC__in IWiaMiniDrv * This,
            /* [in] */ __RPC__in BYTE *__MIDL__IWiaMiniDrv0064);
        
        END_INTERFACE
    } IWiaMiniDrvVtbl;

    interface IWiaMiniDrv
    {
        CONST_VTBL struct IWiaMiniDrvVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaMiniDrv_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaMiniDrv_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaMiniDrv_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaMiniDrv_drvInitializeWia(This,__MIDL__IWiaMiniDrv0000,__MIDL__IWiaMiniDrv0001,__MIDL__IWiaMiniDrv0002,__MIDL__IWiaMiniDrv0003,__MIDL__IWiaMiniDrv0004,__MIDL__IWiaMiniDrv0005,__MIDL__IWiaMiniDrv0006,__MIDL__IWiaMiniDrv0007,__MIDL__IWiaMiniDrv0008)	\
    ( (This)->lpVtbl -> drvInitializeWia(This,__MIDL__IWiaMiniDrv0000,__MIDL__IWiaMiniDrv0001,__MIDL__IWiaMiniDrv0002,__MIDL__IWiaMiniDrv0003,__MIDL__IWiaMiniDrv0004,__MIDL__IWiaMiniDrv0005,__MIDL__IWiaMiniDrv0006,__MIDL__IWiaMiniDrv0007,__MIDL__IWiaMiniDrv0008) ) 

#define IWiaMiniDrv_drvAcquireItemData(This,__MIDL__IWiaMiniDrv0009,__MIDL__IWiaMiniDrv0010,__MIDL__IWiaMiniDrv0011,__MIDL__IWiaMiniDrv0012)	\
    ( (This)->lpVtbl -> drvAcquireItemData(This,__MIDL__IWiaMiniDrv0009,__MIDL__IWiaMiniDrv0010,__MIDL__IWiaMiniDrv0011,__MIDL__IWiaMiniDrv0012) ) 

#define IWiaMiniDrv_drvInitItemProperties(This,__MIDL__IWiaMiniDrv0013,__MIDL__IWiaMiniDrv0014,__MIDL__IWiaMiniDrv0015)	\
    ( (This)->lpVtbl -> drvInitItemProperties(This,__MIDL__IWiaMiniDrv0013,__MIDL__IWiaMiniDrv0014,__MIDL__IWiaMiniDrv0015) ) 

#define IWiaMiniDrv_drvValidateItemProperties(This,__MIDL__IWiaMiniDrv0016,__MIDL__IWiaMiniDrv0017,__MIDL__IWiaMiniDrv0018,__MIDL__IWiaMiniDrv0019,__MIDL__IWiaMiniDrv0020)	\
    ( (This)->lpVtbl -> drvValidateItemProperties(This,__MIDL__IWiaMiniDrv0016,__MIDL__IWiaMiniDrv0017,__MIDL__IWiaMiniDrv0018,__MIDL__IWiaMiniDrv0019,__MIDL__IWiaMiniDrv0020) ) 

#define IWiaMiniDrv_drvWriteItemProperties(This,__MIDL__IWiaMiniDrv0021,__MIDL__IWiaMiniDrv0022,__MIDL__IWiaMiniDrv0023,__MIDL__IWiaMiniDrv0024)	\
    ( (This)->lpVtbl -> drvWriteItemProperties(This,__MIDL__IWiaMiniDrv0021,__MIDL__IWiaMiniDrv0022,__MIDL__IWiaMiniDrv0023,__MIDL__IWiaMiniDrv0024) ) 

#define IWiaMiniDrv_drvReadItemProperties(This,__MIDL__IWiaMiniDrv0025,__MIDL__IWiaMiniDrv0026,__MIDL__IWiaMiniDrv0027,__MIDL__IWiaMiniDrv0028,__MIDL__IWiaMiniDrv0029)	\
    ( (This)->lpVtbl -> drvReadItemProperties(This,__MIDL__IWiaMiniDrv0025,__MIDL__IWiaMiniDrv0026,__MIDL__IWiaMiniDrv0027,__MIDL__IWiaMiniDrv0028,__MIDL__IWiaMiniDrv0029) ) 

#define IWiaMiniDrv_drvLockWiaDevice(This,__MIDL__IWiaMiniDrv0030,__MIDL__IWiaMiniDrv0031,__MIDL__IWiaMiniDrv0032)	\
    ( (This)->lpVtbl -> drvLockWiaDevice(This,__MIDL__IWiaMiniDrv0030,__MIDL__IWiaMiniDrv0031,__MIDL__IWiaMiniDrv0032) ) 

#define IWiaMiniDrv_drvUnLockWiaDevice(This,__MIDL__IWiaMiniDrv0033,__MIDL__IWiaMiniDrv0034,__MIDL__IWiaMiniDrv0035)	\
    ( (This)->lpVtbl -> drvUnLockWiaDevice(This,__MIDL__IWiaMiniDrv0033,__MIDL__IWiaMiniDrv0034,__MIDL__IWiaMiniDrv0035) ) 

#define IWiaMiniDrv_drvAnalyzeItem(This,__MIDL__IWiaMiniDrv0036,__MIDL__IWiaMiniDrv0037,__MIDL__IWiaMiniDrv0038)	\
    ( (This)->lpVtbl -> drvAnalyzeItem(This,__MIDL__IWiaMiniDrv0036,__MIDL__IWiaMiniDrv0037,__MIDL__IWiaMiniDrv0038) ) 

#define IWiaMiniDrv_drvGetDeviceErrorStr(This,__MIDL__IWiaMiniDrv0039,__MIDL__IWiaMiniDrv0040,__MIDL__IWiaMiniDrv0041,__MIDL__IWiaMiniDrv0042)	\
    ( (This)->lpVtbl -> drvGetDeviceErrorStr(This,__MIDL__IWiaMiniDrv0039,__MIDL__IWiaMiniDrv0040,__MIDL__IWiaMiniDrv0041,__MIDL__IWiaMiniDrv0042) ) 

#define IWiaMiniDrv_drvDeviceCommand(This,__MIDL__IWiaMiniDrv0043,__MIDL__IWiaMiniDrv0044,__MIDL__IWiaMiniDrv0045,__MIDL__IWiaMiniDrv0046,__MIDL__IWiaMiniDrv0047)	\
    ( (This)->lpVtbl -> drvDeviceCommand(This,__MIDL__IWiaMiniDrv0043,__MIDL__IWiaMiniDrv0044,__MIDL__IWiaMiniDrv0045,__MIDL__IWiaMiniDrv0046,__MIDL__IWiaMiniDrv0047) ) 

#define IWiaMiniDrv_drvGetCapabilities(This,__MIDL__IWiaMiniDrv0048,__MIDL__IWiaMiniDrv0049,__MIDL__IWiaMiniDrv0050,__MIDL__IWiaMiniDrv0051,__MIDL__IWiaMiniDrv0052)	\
    ( (This)->lpVtbl -> drvGetCapabilities(This,__MIDL__IWiaMiniDrv0048,__MIDL__IWiaMiniDrv0049,__MIDL__IWiaMiniDrv0050,__MIDL__IWiaMiniDrv0051,__MIDL__IWiaMiniDrv0052) ) 

#define IWiaMiniDrv_drvDeleteItem(This,__MIDL__IWiaMiniDrv0053,__MIDL__IWiaMiniDrv0054,__MIDL__IWiaMiniDrv0055)	\
    ( (This)->lpVtbl -> drvDeleteItem(This,__MIDL__IWiaMiniDrv0053,__MIDL__IWiaMiniDrv0054,__MIDL__IWiaMiniDrv0055) ) 

#define IWiaMiniDrv_drvFreeDrvItemContext(This,__MIDL__IWiaMiniDrv0056,__MIDL__IWiaMiniDrv0057,__MIDL__IWiaMiniDrv0058)	\
    ( (This)->lpVtbl -> drvFreeDrvItemContext(This,__MIDL__IWiaMiniDrv0056,__MIDL__IWiaMiniDrv0057,__MIDL__IWiaMiniDrv0058) ) 

#define IWiaMiniDrv_drvGetWiaFormatInfo(This,__MIDL__IWiaMiniDrv0059,__MIDL__IWiaMiniDrv0060,__MIDL__IWiaMiniDrv0061,__MIDL__IWiaMiniDrv0062,__MIDL__IWiaMiniDrv0063)	\
    ( (This)->lpVtbl -> drvGetWiaFormatInfo(This,__MIDL__IWiaMiniDrv0059,__MIDL__IWiaMiniDrv0060,__MIDL__IWiaMiniDrv0061,__MIDL__IWiaMiniDrv0062,__MIDL__IWiaMiniDrv0063) ) 

#define IWiaMiniDrv_drvNotifyPnpEvent(This,pEventGUID,bstrDeviceID,ulReserved)	\
    ( (This)->lpVtbl -> drvNotifyPnpEvent(This,pEventGUID,bstrDeviceID,ulReserved) ) 

#define IWiaMiniDrv_drvUnInitializeWia(This,__MIDL__IWiaMiniDrv0064)	\
    ( (This)->lpVtbl -> drvUnInitializeWia(This,__MIDL__IWiaMiniDrv0064) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaMiniDrv_INTERFACE_DEFINED__ */


#ifndef __IWiaMiniDrvCallBack_INTERFACE_DEFINED__
#define __IWiaMiniDrvCallBack_INTERFACE_DEFINED__

/* interface IWiaMiniDrvCallBack */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaMiniDrvCallBack;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33a57d5a-3de8-11d2-9a36-00c04fa36145")
    IWiaMiniDrvCallBack : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MiniDrvCallback( 
            /* [in] */ LONG lReason,
            /* [in] */ LONG lStatus,
            /* [in] */ LONG lPercentComplete,
            /* [in] */ LONG lOffset,
            /* [in] */ LONG lLength,
            /* [in] */ __RPC__in PMINIDRV_TRANSFER_CONTEXT pTranCtx,
            /* [in] */ LONG lReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaMiniDrvCallBackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaMiniDrvCallBack * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaMiniDrvCallBack * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaMiniDrvCallBack * This);
        
        DECLSPEC_XFGVIRT(IWiaMiniDrvCallBack, MiniDrvCallback)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MiniDrvCallback )( 
            __RPC__in IWiaMiniDrvCallBack * This,
            /* [in] */ LONG lReason,
            /* [in] */ LONG lStatus,
            /* [in] */ LONG lPercentComplete,
            /* [in] */ LONG lOffset,
            /* [in] */ LONG lLength,
            /* [in] */ __RPC__in PMINIDRV_TRANSFER_CONTEXT pTranCtx,
            /* [in] */ LONG lReserved);
        
        END_INTERFACE
    } IWiaMiniDrvCallBackVtbl;

    interface IWiaMiniDrvCallBack
    {
        CONST_VTBL struct IWiaMiniDrvCallBackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaMiniDrvCallBack_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaMiniDrvCallBack_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaMiniDrvCallBack_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaMiniDrvCallBack_MiniDrvCallback(This,lReason,lStatus,lPercentComplete,lOffset,lLength,pTranCtx,lReserved)	\
    ( (This)->lpVtbl -> MiniDrvCallback(This,lReason,lStatus,lPercentComplete,lOffset,lLength,pTranCtx,lReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaMiniDrvCallBack_INTERFACE_DEFINED__ */


#ifndef __IWiaDrvItem_INTERFACE_DEFINED__
#define __IWiaDrvItem_INTERFACE_DEFINED__

/* interface IWiaDrvItem */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaDrvItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f02b5c5-b00c-11d2-a094-00c04f72dc3c")
    IWiaDrvItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetItemFlags( 
            /* [out] */ __RPC__out LONG *__MIDL__IWiaDrvItem0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceSpecContext( 
            /* [out] */ __RPC__deref_out_opt BYTE **__MIDL__IWiaDrvItem0001) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullItemName( 
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0002) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemName( 
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0003) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddItemToFolder( 
            /* [in] */ __RPC__in_opt IWiaDrvItem *__MIDL__IWiaDrvItem0004) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlinkItemTree( 
            /* [in] */ LONG __MIDL__IWiaDrvItem0005) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveItemFromFolder( 
            /* [in] */ LONG __MIDL__IWiaDrvItem0006) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindItemByName( 
            /* [in] */ LONG __MIDL__IWiaDrvItem0007,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaDrvItem0008,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0009) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindChildItemByName( 
            /* [in] */ __RPC__in BSTR __MIDL__IWiaDrvItem0010,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0011) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentItem( 
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0012) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFirstChildItem( 
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0013) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextSiblingItem( 
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0014) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DumpItemData( 
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0015) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaDrvItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaDrvItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaDrvItem * This);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetItemFlags)
        HRESULT ( STDMETHODCALLTYPE *GetItemFlags )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__out LONG *__MIDL__IWiaDrvItem0000);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetDeviceSpecContext)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceSpecContext )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt BYTE **__MIDL__IWiaDrvItem0001);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetFullItemName)
        HRESULT ( STDMETHODCALLTYPE *GetFullItemName )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0002);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetItemName)
        HRESULT ( STDMETHODCALLTYPE *GetItemName )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0003);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, AddItemToFolder)
        HRESULT ( STDMETHODCALLTYPE *AddItemToFolder )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ __RPC__in_opt IWiaDrvItem *__MIDL__IWiaDrvItem0004);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, UnlinkItemTree)
        HRESULT ( STDMETHODCALLTYPE *UnlinkItemTree )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ LONG __MIDL__IWiaDrvItem0005);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, RemoveItemFromFolder)
        HRESULT ( STDMETHODCALLTYPE *RemoveItemFromFolder )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ LONG __MIDL__IWiaDrvItem0006);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, FindItemByName)
        HRESULT ( STDMETHODCALLTYPE *FindItemByName )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ LONG __MIDL__IWiaDrvItem0007,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaDrvItem0008,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0009);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, FindChildItemByName)
        HRESULT ( STDMETHODCALLTYPE *FindChildItemByName )( 
            __RPC__in IWiaDrvItem * This,
            /* [in] */ __RPC__in BSTR __MIDL__IWiaDrvItem0010,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0011);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetParentItem)
        HRESULT ( STDMETHODCALLTYPE *GetParentItem )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0012);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetFirstChildItem)
        HRESULT ( STDMETHODCALLTYPE *GetFirstChildItem )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0013);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, GetNextSiblingItem)
        HRESULT ( STDMETHODCALLTYPE *GetNextSiblingItem )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt IWiaDrvItem **__MIDL__IWiaDrvItem0014);
        
        DECLSPEC_XFGVIRT(IWiaDrvItem, DumpItemData)
        HRESULT ( STDMETHODCALLTYPE *DumpItemData )( 
            __RPC__in IWiaDrvItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *__MIDL__IWiaDrvItem0015);
        
        END_INTERFACE
    } IWiaDrvItemVtbl;

    interface IWiaDrvItem
    {
        CONST_VTBL struct IWiaDrvItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaDrvItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaDrvItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaDrvItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaDrvItem_GetItemFlags(This,__MIDL__IWiaDrvItem0000)	\
    ( (This)->lpVtbl -> GetItemFlags(This,__MIDL__IWiaDrvItem0000) ) 

#define IWiaDrvItem_GetDeviceSpecContext(This,__MIDL__IWiaDrvItem0001)	\
    ( (This)->lpVtbl -> GetDeviceSpecContext(This,__MIDL__IWiaDrvItem0001) ) 

#define IWiaDrvItem_GetFullItemName(This,__MIDL__IWiaDrvItem0002)	\
    ( (This)->lpVtbl -> GetFullItemName(This,__MIDL__IWiaDrvItem0002) ) 

#define IWiaDrvItem_GetItemName(This,__MIDL__IWiaDrvItem0003)	\
    ( (This)->lpVtbl -> GetItemName(This,__MIDL__IWiaDrvItem0003) ) 

#define IWiaDrvItem_AddItemToFolder(This,__MIDL__IWiaDrvItem0004)	\
    ( (This)->lpVtbl -> AddItemToFolder(This,__MIDL__IWiaDrvItem0004) ) 

#define IWiaDrvItem_UnlinkItemTree(This,__MIDL__IWiaDrvItem0005)	\
    ( (This)->lpVtbl -> UnlinkItemTree(This,__MIDL__IWiaDrvItem0005) ) 

#define IWiaDrvItem_RemoveItemFromFolder(This,__MIDL__IWiaDrvItem0006)	\
    ( (This)->lpVtbl -> RemoveItemFromFolder(This,__MIDL__IWiaDrvItem0006) ) 

#define IWiaDrvItem_FindItemByName(This,__MIDL__IWiaDrvItem0007,__MIDL__IWiaDrvItem0008,__MIDL__IWiaDrvItem0009)	\
    ( (This)->lpVtbl -> FindItemByName(This,__MIDL__IWiaDrvItem0007,__MIDL__IWiaDrvItem0008,__MIDL__IWiaDrvItem0009) ) 

#define IWiaDrvItem_FindChildItemByName(This,__MIDL__IWiaDrvItem0010,__MIDL__IWiaDrvItem0011)	\
    ( (This)->lpVtbl -> FindChildItemByName(This,__MIDL__IWiaDrvItem0010,__MIDL__IWiaDrvItem0011) ) 

#define IWiaDrvItem_GetParentItem(This,__MIDL__IWiaDrvItem0012)	\
    ( (This)->lpVtbl -> GetParentItem(This,__MIDL__IWiaDrvItem0012) ) 

#define IWiaDrvItem_GetFirstChildItem(This,__MIDL__IWiaDrvItem0013)	\
    ( (This)->lpVtbl -> GetFirstChildItem(This,__MIDL__IWiaDrvItem0013) ) 

#define IWiaDrvItem_GetNextSiblingItem(This,__MIDL__IWiaDrvItem0014)	\
    ( (This)->lpVtbl -> GetNextSiblingItem(This,__MIDL__IWiaDrvItem0014) ) 

#define IWiaDrvItem_DumpItemData(This,__MIDL__IWiaDrvItem0015)	\
    ( (This)->lpVtbl -> DumpItemData(This,__MIDL__IWiaDrvItem0015) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaDrvItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wiamindr_xp_0000_0003 */
/* [local] */ 

typedef struct _WIA_PROPERTY_INFO
    {
    ULONG lAccessFlags;
    VARTYPE vt;
    union 
        {
        struct 
            {
            LONG Min;
            LONG Nom;
            LONG Max;
            LONG Inc;
            } 	Range;
        struct 
            {
            DOUBLE Min;
            DOUBLE Nom;
            DOUBLE Max;
            DOUBLE Inc;
            } 	RangeFloat;
        struct 
            {
            LONG cNumList;
            LONG Nom;
            /* [size_is] */ BYTE *pList;
            } 	List;
        struct 
            {
            LONG cNumList;
            DOUBLE Nom;
            /* [size_is] */ BYTE *pList;
            } 	ListFloat;
        struct 
            {
            LONG cNumList;
            GUID Nom;
            /* [size_is] */ GUID *pList;
            } 	ListGuid;
        struct 
            {
            LONG cNumList;
            BSTR Nom;
            /* [size_is] */ BSTR *pList;
            } 	ListBStr;
        struct 
            {
            LONG Nom;
            LONG ValidBits;
            } 	Flag;
        struct 
            {
            LONG Dummy;
            } 	None;
        } 	ValidVal;
    } 	WIA_PROPERTY_INFO;

typedef struct _WIA_PROPERTY_INFO *PWIA_PROPERTY_INFO;

typedef struct _WIA_PROPERTY_CONTEXT
    {
    ULONG cProps;
    /* [size_is] */ PROPID *pProps;
    /* [size_is] */ BOOL *pChanged;
    } 	WIA_PROPERTY_CONTEXT;

typedef struct _WIA_PROPERTY_CONTEXT *PWIA_PROPERTY_CONTEXT;

typedef struct _WIAS_CHANGED_VALUE_INFO
    {
    BOOL bChanged;
    LONG vt;
    union 
        {
        LONG lVal;
        FLOAT fltVal;
        BSTR bstrVal;
        GUID guidVal;
        } 	Old;
    union 
        {
        LONG lVal;
        FLOAT fltVal;
        BSTR bstrVal;
        GUID guidVal;
        } 	Current;
    } 	WIAS_CHANGED_VALUE_INFO;

typedef struct _WIAS_CHANGED_VALUE_INFO *PWIAS_CHANGED_VALUE_INFO;

typedef struct _WIAS_DOWN_SAMPLE_INFO
    {
    ULONG ulOriginalWidth;
    ULONG ulOriginalHeight;
    ULONG ulBitsPerPixel;
    ULONG ulXRes;
    ULONG ulYRes;
    ULONG ulDownSampledWidth;
    ULONG ulDownSampledHeight;
    ULONG ulActualSize;
    ULONG ulDestBufSize;
    ULONG ulSrcBufSize;
    /* [size_is] */ BYTE *pSrcBuffer;
    /* [size_is] */ BYTE *pDestBuffer;
    } 	WIAS_DOWN_SAMPLE_INFO;

typedef struct _WIAS_DOWN_SAMPLE_INFO *PWIAS_DOWN_SAMPLE_INFO;

typedef struct _WIAS_ENDORSER_VALUE
    {
    LPWSTR wszTokenName;
    LPWSTR wszValue;
    } 	WIAS_ENDORSER_VALUE;

typedef struct _WIAS_ENDORSER_VALUE *PWIAS_ENDORSER_VALUE;

typedef struct _WIAS_ENDORSER_INFO
    {
    ULONG ulPageCount;
    ULONG ulNumEndorserValues;
    /* [size_is] */ WIAS_ENDORSER_VALUE *pEndorserValues;
    } 	WIAS_ENDORSER_INFO;

typedef struct _WIAS_ENDORSER_INFO *PWIAS_ENDORSER_INFO;

#include "wiamdef.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wiamindr_xp_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wiamindr_xp_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


