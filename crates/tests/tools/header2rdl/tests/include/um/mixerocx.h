

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

#ifndef __mixerocx_h__
#define __mixerocx_h__

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

#ifndef __IMixerOCXNotify_FWD_DEFINED__
#define __IMixerOCXNotify_FWD_DEFINED__
typedef interface IMixerOCXNotify IMixerOCXNotify;

#endif 	/* __IMixerOCXNotify_FWD_DEFINED__ */


#ifndef __IMixerOCX_FWD_DEFINED__
#define __IMixerOCX_FWD_DEFINED__
typedef interface IMixerOCX IMixerOCX;

#endif 	/* __IMixerOCX_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mixerocx_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_mixerocx_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mixerocx_0000_0000_v0_0_s_ifspec;

#ifndef __IMixerOCXNotify_INTERFACE_DEFINED__
#define __IMixerOCXNotify_INTERFACE_DEFINED__

/* interface IMixerOCXNotify */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMixerOCXNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81A3BD31-DEE1-11d1-8508-00A0C91F9CA0")
    IMixerOCXNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnInvalidateRect( 
            /* [in] */ __RPC__in LPCRECT lpcRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            /* [in] */ ULONG ulStatusFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDataChange( 
            /* [in] */ ULONG ulDataFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMixerOCXNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMixerOCXNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMixerOCXNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMixerOCXNotify * This);
        
        DECLSPEC_XFGVIRT(IMixerOCXNotify, OnInvalidateRect)
        HRESULT ( STDMETHODCALLTYPE *OnInvalidateRect )( 
            __RPC__in IMixerOCXNotify * This,
            /* [in] */ __RPC__in LPCRECT lpcRect);
        
        DECLSPEC_XFGVIRT(IMixerOCXNotify, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in IMixerOCXNotify * This,
            /* [in] */ ULONG ulStatusFlags);
        
        DECLSPEC_XFGVIRT(IMixerOCXNotify, OnDataChange)
        HRESULT ( STDMETHODCALLTYPE *OnDataChange )( 
            __RPC__in IMixerOCXNotify * This,
            /* [in] */ ULONG ulDataFlags);
        
        END_INTERFACE
    } IMixerOCXNotifyVtbl;

    interface IMixerOCXNotify
    {
        CONST_VTBL struct IMixerOCXNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMixerOCXNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMixerOCXNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMixerOCXNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMixerOCXNotify_OnInvalidateRect(This,lpcRect)	\
    ( (This)->lpVtbl -> OnInvalidateRect(This,lpcRect) ) 

#define IMixerOCXNotify_OnStatusChange(This,ulStatusFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,ulStatusFlags) ) 

#define IMixerOCXNotify_OnDataChange(This,ulDataFlags)	\
    ( (This)->lpVtbl -> OnDataChange(This,ulDataFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMixerOCXNotify_INTERFACE_DEFINED__ */


#ifndef __IMixerOCX_INTERFACE_DEFINED__
#define __IMixerOCX_INTERFACE_DEFINED__

/* interface IMixerOCX */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMixerOCX;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81A3BD32-DEE1-11d1-8508-00A0C91F9CA0")
    IMixerOCX : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDisplayChange( 
            /* [in] */ ULONG ulBitsPerPixel,
            /* [in] */ ULONG ulScreenWidth,
            /* [in] */ ULONG ulScreenHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAspectRatio( 
            /* [out] */ __RPC__out LPDWORD pdwPictAspectRatioX,
            /* [out] */ __RPC__out LPDWORD pdwPictAspectRatioY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoSize( 
            /* [out] */ __RPC__out LPDWORD pdwVideoWidth,
            /* [out] */ __RPC__out LPDWORD pdwVideoHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__deref_out_opt LPDWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDraw( 
            /* [in] */ __RPC__in HDC hdcDraw,
            /* [in] */ __RPC__in LPCRECT prcDraw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDrawRegion( 
            /* [in] */ __RPC__in LPPOINT lpptTopLeftSC,
            /* [in] */ __RPC__in LPCRECT prcDrawCC,
            /* [in] */ __RPC__in LPCRECT lprcClip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ __RPC__in_opt IMixerOCXNotify *pmdns) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnAdvise( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMixerOCXVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMixerOCX * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMixerOCX * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMixerOCX * This);
        
        DECLSPEC_XFGVIRT(IMixerOCX, OnDisplayChange)
        HRESULT ( STDMETHODCALLTYPE *OnDisplayChange )( 
            __RPC__in IMixerOCX * This,
            /* [in] */ ULONG ulBitsPerPixel,
            /* [in] */ ULONG ulScreenWidth,
            /* [in] */ ULONG ulScreenHeight);
        
        DECLSPEC_XFGVIRT(IMixerOCX, GetAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetAspectRatio )( 
            __RPC__in IMixerOCX * This,
            /* [out] */ __RPC__out LPDWORD pdwPictAspectRatioX,
            /* [out] */ __RPC__out LPDWORD pdwPictAspectRatioY);
        
        DECLSPEC_XFGVIRT(IMixerOCX, GetVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetVideoSize )( 
            __RPC__in IMixerOCX * This,
            /* [out] */ __RPC__out LPDWORD pdwVideoWidth,
            /* [out] */ __RPC__out LPDWORD pdwVideoHeight);
        
        DECLSPEC_XFGVIRT(IMixerOCX, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IMixerOCX * This,
            /* [out] */ __RPC__deref_out_opt LPDWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMixerOCX, OnDraw)
        HRESULT ( STDMETHODCALLTYPE *OnDraw )( 
            __RPC__in IMixerOCX * This,
            /* [in] */ __RPC__in HDC hdcDraw,
            /* [in] */ __RPC__in LPCRECT prcDraw);
        
        DECLSPEC_XFGVIRT(IMixerOCX, SetDrawRegion)
        HRESULT ( STDMETHODCALLTYPE *SetDrawRegion )( 
            __RPC__in IMixerOCX * This,
            /* [in] */ __RPC__in LPPOINT lpptTopLeftSC,
            /* [in] */ __RPC__in LPCRECT prcDrawCC,
            /* [in] */ __RPC__in LPCRECT lprcClip);
        
        DECLSPEC_XFGVIRT(IMixerOCX, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IMixerOCX * This,
            /* [in] */ __RPC__in_opt IMixerOCXNotify *pmdns);
        
        DECLSPEC_XFGVIRT(IMixerOCX, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            __RPC__in IMixerOCX * This);
        
        END_INTERFACE
    } IMixerOCXVtbl;

    interface IMixerOCX
    {
        CONST_VTBL struct IMixerOCXVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMixerOCX_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMixerOCX_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMixerOCX_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMixerOCX_OnDisplayChange(This,ulBitsPerPixel,ulScreenWidth,ulScreenHeight)	\
    ( (This)->lpVtbl -> OnDisplayChange(This,ulBitsPerPixel,ulScreenWidth,ulScreenHeight) ) 

#define IMixerOCX_GetAspectRatio(This,pdwPictAspectRatioX,pdwPictAspectRatioY)	\
    ( (This)->lpVtbl -> GetAspectRatio(This,pdwPictAspectRatioX,pdwPictAspectRatioY) ) 

#define IMixerOCX_GetVideoSize(This,pdwVideoWidth,pdwVideoHeight)	\
    ( (This)->lpVtbl -> GetVideoSize(This,pdwVideoWidth,pdwVideoHeight) ) 

#define IMixerOCX_GetStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus) ) 

#define IMixerOCX_OnDraw(This,hdcDraw,prcDraw)	\
    ( (This)->lpVtbl -> OnDraw(This,hdcDraw,prcDraw) ) 

#define IMixerOCX_SetDrawRegion(This,lpptTopLeftSC,prcDrawCC,lprcClip)	\
    ( (This)->lpVtbl -> SetDrawRegion(This,lpptTopLeftSC,prcDrawCC,lprcClip) ) 

#define IMixerOCX_Advise(This,pmdns)	\
    ( (This)->lpVtbl -> Advise(This,pmdns) ) 

#define IMixerOCX_UnAdvise(This)	\
    ( (This)->lpVtbl -> UnAdvise(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMixerOCX_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mixerocx_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mixerocx_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mixerocx_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HDC_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * ); 
void                      __RPC_USER  HDC_UserFree(     __RPC__in unsigned long *, __RPC__in HDC * ); 

unsigned long             __RPC_USER  HDC_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * ); 
void                      __RPC_USER  HDC_UserFree64(     __RPC__in unsigned long *, __RPC__in HDC * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


