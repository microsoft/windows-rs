

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

#ifndef __windows2Egraphics2Eimaging2Einterop_h__
#define __windows2Egraphics2Eimaging2Einterop_h__

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

#ifndef __ISoftwareBitmapNative_FWD_DEFINED__
#define __ISoftwareBitmapNative_FWD_DEFINED__
typedef interface ISoftwareBitmapNative ISoftwareBitmapNative;

#endif 	/* __ISoftwareBitmapNative_FWD_DEFINED__ */


#ifndef __ISoftwareBitmapNativeFactory_FWD_DEFINED__
#define __ISoftwareBitmapNativeFactory_FWD_DEFINED__
typedef interface ISoftwareBitmapNativeFactory ISoftwareBitmapNativeFactory;

#endif 	/* __ISoftwareBitmapNativeFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "Inspectable.h"
#include "mfobjects.h"
#include "Wincodec.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
// {84E65691-8602-4A84-BE46-708BE9CD4B74}
extern const __declspec(selectany) CLSID CLSID_SoftwareBitmapNativeFactory = { 0x84e65691, 0x8602, 0x4a84, { 0xbe, 0x46, 0x70, 0x8b, 0xe9, 0xcd, 0x4b, 0x74 } };


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __ISoftwareBitmapNative_INTERFACE_DEFINED__
#define __ISoftwareBitmapNative_INTERFACE_DEFINED__

/* interface ISoftwareBitmapNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISoftwareBitmapNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94BC8415-04EA-4B2E-AF13-4DE95AA898EB")
    ISoftwareBitmapNative : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISoftwareBitmapNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISoftwareBitmapNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISoftwareBitmapNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISoftwareBitmapNative * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            ISoftwareBitmapNative * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            ISoftwareBitmapNative * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            ISoftwareBitmapNative * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(ISoftwareBitmapNative, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            ISoftwareBitmapNative * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } ISoftwareBitmapNativeVtbl;

    interface ISoftwareBitmapNative
    {
        CONST_VTBL struct ISoftwareBitmapNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISoftwareBitmapNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISoftwareBitmapNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISoftwareBitmapNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISoftwareBitmapNative_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define ISoftwareBitmapNative_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define ISoftwareBitmapNative_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define ISoftwareBitmapNative_GetData(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetData(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISoftwareBitmapNative_INTERFACE_DEFINED__ */


#ifndef __ISoftwareBitmapNativeFactory_INTERFACE_DEFINED__
#define __ISoftwareBitmapNativeFactory_INTERFACE_DEFINED__

/* interface ISoftwareBitmapNativeFactory */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISoftwareBitmapNativeFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C3C181EC-2914-4791-AF02-02D224A10B43")
    ISoftwareBitmapNativeFactory : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromWICBitmap( 
            /* [annotation][in] */ 
            _In_  IWICBitmap *data,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFromMF2DBuffer2( 
            /* [annotation][in] */ 
            _In_  IMF2DBuffer2 *data,
            /* [annotation][in] */ 
            _In_  REFGUID subtype,
            /* [annotation][in] */ 
            _In_  UINT32 width,
            /* [annotation][in] */ 
            _In_  UINT32 height,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoArea *minDisplayAperture,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISoftwareBitmapNativeFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISoftwareBitmapNativeFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISoftwareBitmapNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISoftwareBitmapNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            ISoftwareBitmapNativeFactory * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            ISoftwareBitmapNativeFactory * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            ISoftwareBitmapNativeFactory * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(ISoftwareBitmapNativeFactory, CreateFromWICBitmap)
        HRESULT ( STDMETHODCALLTYPE *CreateFromWICBitmap )( 
            ISoftwareBitmapNativeFactory * This,
            /* [annotation][in] */ 
            _In_  IWICBitmap *data,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(ISoftwareBitmapNativeFactory, CreateFromMF2DBuffer2)
        HRESULT ( STDMETHODCALLTYPE *CreateFromMF2DBuffer2 )( 
            ISoftwareBitmapNativeFactory * This,
            /* [annotation][in] */ 
            _In_  IMF2DBuffer2 *data,
            /* [annotation][in] */ 
            _In_  REFGUID subtype,
            /* [annotation][in] */ 
            _In_  UINT32 width,
            /* [annotation][in] */ 
            _In_  UINT32 height,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoArea *minDisplayAperture,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } ISoftwareBitmapNativeFactoryVtbl;

    interface ISoftwareBitmapNativeFactory
    {
        CONST_VTBL struct ISoftwareBitmapNativeFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISoftwareBitmapNativeFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISoftwareBitmapNativeFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISoftwareBitmapNativeFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISoftwareBitmapNativeFactory_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define ISoftwareBitmapNativeFactory_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define ISoftwareBitmapNativeFactory_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define ISoftwareBitmapNativeFactory_CreateFromWICBitmap(This,data,forceReadOnly,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFromWICBitmap(This,data,forceReadOnly,riid,ppv) ) 

#define ISoftwareBitmapNativeFactory_CreateFromMF2DBuffer2(This,data,subtype,width,height,forceReadOnly,minDisplayAperture,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFromMF2DBuffer2(This,data,subtype,width,height,forceReadOnly,minDisplayAperture,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISoftwareBitmapNativeFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Eimaging2Einterop_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


