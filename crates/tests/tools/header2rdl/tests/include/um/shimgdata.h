

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

#ifndef __shimgdata_h__
#define __shimgdata_h__

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

#ifndef __IShellImageDataFactory_FWD_DEFINED__
#define __IShellImageDataFactory_FWD_DEFINED__
typedef interface IShellImageDataFactory IShellImageDataFactory;

#endif 	/* __IShellImageDataFactory_FWD_DEFINED__ */


#ifndef __IShellImageData_FWD_DEFINED__
#define __IShellImageData_FWD_DEFINED__
typedef interface IShellImageData IShellImageData;

#endif 	/* __IShellImageData_FWD_DEFINED__ */


#ifndef __IShellImageDataAbort_FWD_DEFINED__
#define __IShellImageDataAbort_FWD_DEFINED__
typedef interface IShellImageDataAbort IShellImageDataAbort;

#endif 	/* __IShellImageDataAbort_FWD_DEFINED__ */


#ifndef __ShellImageDataFactory_FWD_DEFINED__
#define __ShellImageDataFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellImageDataFactory ShellImageDataFactory;
#else
typedef struct ShellImageDataFactory ShellImageDataFactory;
#endif /* __cplusplus */

#endif 	/* __ShellImageDataFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oleidl.h"
#include "propidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shimgdata_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#if !defined(_GDIPLUSPIXELFORMATS_H)
typedef DWORD PixelFormat;

#endif
#if !defined(_GDIPLUSENUMS_H)
typedef DWORD InterpolationMode;

#endif
#if !defined(_GDIPLUSHEADERS_H)
typedef BYTE EncoderParameters;

typedef BYTE Image;

#endif
#define  SHIMGKEY_QUALITY    L"Compression"
#define  SHIMGKEY_RAWFORMAT  L"RawDataFormat"

#define  SHIMGDEC_DEFAULT            0x00000000
#define  SHIMGDEC_THUMBNAIL          0x00000001
#define  SHIMGDEC_LOADFULL           0x00000002
#define  E_NOTVALIDFORANIMATEDIMAGE  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x01)


extern RPC_IF_HANDLE __MIDL_itf_shimgdata_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shimgdata_0000_0000_v0_0_s_ifspec;

#ifndef __IShellImageDataFactory_INTERFACE_DEFINED__
#define __IShellImageDataFactory_INTERFACE_DEFINED__

/* interface IShellImageDataFactory */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IShellImageDataFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9be8ed5c-edab-4d75-90f3-bd5bdbb21c82")
    IShellImageDataFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateIShellImageData( 
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateImageFromFile( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateImageFromStream( 
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataFormatFromPath( 
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out GUID *pDataFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellImageDataFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellImageDataFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellImageDataFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellImageDataFactory * This);
        
        DECLSPEC_XFGVIRT(IShellImageDataFactory, CreateIShellImageData)
        HRESULT ( STDMETHODCALLTYPE *CreateIShellImageData )( 
            __RPC__in IShellImageDataFactory * This,
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg);
        
        DECLSPEC_XFGVIRT(IShellImageDataFactory, CreateImageFromFile)
        HRESULT ( STDMETHODCALLTYPE *CreateImageFromFile )( 
            __RPC__in IShellImageDataFactory * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg);
        
        DECLSPEC_XFGVIRT(IShellImageDataFactory, CreateImageFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreateImageFromStream )( 
            __RPC__in IShellImageDataFactory * This,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__deref_out_opt IShellImageData **ppshimg);
        
        DECLSPEC_XFGVIRT(IShellImageDataFactory, GetDataFormatFromPath)
        HRESULT ( STDMETHODCALLTYPE *GetDataFormatFromPath )( 
            __RPC__in IShellImageDataFactory * This,
            /* [in] */ __RPC__in LPCWSTR pszPath,
            /* [out] */ __RPC__out GUID *pDataFormat);
        
        END_INTERFACE
    } IShellImageDataFactoryVtbl;

    interface IShellImageDataFactory
    {
        CONST_VTBL struct IShellImageDataFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellImageDataFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellImageDataFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellImageDataFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellImageDataFactory_CreateIShellImageData(This,ppshimg)	\
    ( (This)->lpVtbl -> CreateIShellImageData(This,ppshimg) ) 

#define IShellImageDataFactory_CreateImageFromFile(This,pszPath,ppshimg)	\
    ( (This)->lpVtbl -> CreateImageFromFile(This,pszPath,ppshimg) ) 

#define IShellImageDataFactory_CreateImageFromStream(This,pStream,ppshimg)	\
    ( (This)->lpVtbl -> CreateImageFromStream(This,pStream,ppshimg) ) 

#define IShellImageDataFactory_GetDataFormatFromPath(This,pszPath,pDataFormat)	\
    ( (This)->lpVtbl -> GetDataFormatFromPath(This,pszPath,pDataFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellImageDataFactory_INTERFACE_DEFINED__ */


#ifndef __IShellImageData_INTERFACE_DEFINED__
#define __IShellImageData_INTERFACE_DEFINED__

/* interface IShellImageData */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IShellImageData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bfdeec12-8040-4403-a5ea-9e07dafcf530")
    IShellImageData : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cxDesired,
            /* [in] */ ULONG cyDesired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Draw( 
            /* [in] */ HDC hdc,
            /* [in] */ LPRECT prcDest,
            /* [in] */ LPRECT prcSrc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextFrame( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextPage( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrevPage( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsTransparent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAnimated( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsVector( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsMultipage( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEditable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPrintable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDecoded( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentPage( 
            /* [out] */ ULONG *pnPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPageCount( 
            /* [out] */ ULONG *pcPages) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectPage( 
            /* [in] */ ULONG iPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ SIZE *pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRawDataFormat( 
            /* [out] */ GUID *pDataFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPixelFormat( 
            /* [out] */ PixelFormat *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDelay( 
            /* [out] */ DWORD *pdwDelay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [in] */ DWORD dwMode,
            /* [out] */ IPropertySetStorage **ppPropSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ DWORD dwAngle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Scale( 
            /* [in] */ ULONG cx,
            /* [in] */ ULONG cy,
            /* [in] */ InterpolationMode hints) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DiscardEdit( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEncoderParams( 
            /* [in] */ IPropertyBag *pbagEnc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayName( 
            /* [size_is][out][in] */ LPWSTR wszName,
            /* [in] */ UINT cch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResolution( 
            /* [out] */ ULONG *puResolutionX,
            /* [out] */ ULONG *puResolutionY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEncoderParams( 
            /* [in] */ GUID *pguidFmt,
            /* [out] */ EncoderParameters **ppEncParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterAbort( 
            /* [in] */ IShellImageDataAbort *pAbort,
            /* [out] */ IShellImageDataAbort **ppAbortPrev) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloneFrame( 
            /* [out] */ Image **ppImg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReplaceFrame( 
            /* [in] */ Image *pImg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellImageDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IShellImageData * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            IShellImageData * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cxDesired,
            /* [in] */ ULONG cyDesired);
        
        DECLSPEC_XFGVIRT(IShellImageData, Draw)
        HRESULT ( STDMETHODCALLTYPE *Draw )( 
            IShellImageData * This,
            /* [in] */ HDC hdc,
            /* [in] */ LPRECT prcDest,
            /* [in] */ LPRECT prcSrc);
        
        DECLSPEC_XFGVIRT(IShellImageData, NextFrame)
        HRESULT ( STDMETHODCALLTYPE *NextFrame )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, NextPage)
        HRESULT ( STDMETHODCALLTYPE *NextPage )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, PrevPage)
        HRESULT ( STDMETHODCALLTYPE *PrevPage )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsTransparent)
        HRESULT ( STDMETHODCALLTYPE *IsTransparent )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsAnimated)
        HRESULT ( STDMETHODCALLTYPE *IsAnimated )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsVector)
        HRESULT ( STDMETHODCALLTYPE *IsVector )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsMultipage)
        HRESULT ( STDMETHODCALLTYPE *IsMultipage )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsEditable)
        HRESULT ( STDMETHODCALLTYPE *IsEditable )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsPrintable)
        HRESULT ( STDMETHODCALLTYPE *IsPrintable )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, IsDecoded)
        HRESULT ( STDMETHODCALLTYPE *IsDecoded )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetCurrentPage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentPage )( 
            IShellImageData * This,
            /* [out] */ ULONG *pnPage);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetPageCount)
        HRESULT ( STDMETHODCALLTYPE *GetPageCount )( 
            IShellImageData * This,
            /* [out] */ ULONG *pcPages);
        
        DECLSPEC_XFGVIRT(IShellImageData, SelectPage)
        HRESULT ( STDMETHODCALLTYPE *SelectPage )( 
            IShellImageData * This,
            /* [in] */ ULONG iPage);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            IShellImageData * This,
            /* [out] */ SIZE *pSize);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetRawDataFormat)
        HRESULT ( STDMETHODCALLTYPE *GetRawDataFormat )( 
            IShellImageData * This,
            /* [out] */ GUID *pDataFormat);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetPixelFormat)
        HRESULT ( STDMETHODCALLTYPE *GetPixelFormat )( 
            IShellImageData * This,
            /* [out] */ PixelFormat *pFormat);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetDelay)
        HRESULT ( STDMETHODCALLTYPE *GetDelay )( 
            IShellImageData * This,
            /* [out] */ DWORD *pdwDelay);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IShellImageData * This,
            /* [in] */ DWORD dwMode,
            /* [out] */ IPropertySetStorage **ppPropSet);
        
        DECLSPEC_XFGVIRT(IShellImageData, Rotate)
        HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            IShellImageData * This,
            /* [in] */ DWORD dwAngle);
        
        DECLSPEC_XFGVIRT(IShellImageData, Scale)
        HRESULT ( STDMETHODCALLTYPE *Scale )( 
            IShellImageData * This,
            /* [in] */ ULONG cx,
            /* [in] */ ULONG cy,
            /* [in] */ InterpolationMode hints);
        
        DECLSPEC_XFGVIRT(IShellImageData, DiscardEdit)
        HRESULT ( STDMETHODCALLTYPE *DiscardEdit )( 
            IShellImageData * This);
        
        DECLSPEC_XFGVIRT(IShellImageData, SetEncoderParams)
        HRESULT ( STDMETHODCALLTYPE *SetEncoderParams )( 
            IShellImageData * This,
            /* [in] */ IPropertyBag *pbagEnc);
        
        DECLSPEC_XFGVIRT(IShellImageData, DisplayName)
        HRESULT ( STDMETHODCALLTYPE *DisplayName )( 
            IShellImageData * This,
            /* [size_is][out][in] */ LPWSTR wszName,
            /* [in] */ UINT cch);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetResolution)
        HRESULT ( STDMETHODCALLTYPE *GetResolution )( 
            IShellImageData * This,
            /* [out] */ ULONG *puResolutionX,
            /* [out] */ ULONG *puResolutionY);
        
        DECLSPEC_XFGVIRT(IShellImageData, GetEncoderParams)
        HRESULT ( STDMETHODCALLTYPE *GetEncoderParams )( 
            IShellImageData * This,
            /* [in] */ GUID *pguidFmt,
            /* [out] */ EncoderParameters **ppEncParams);
        
        DECLSPEC_XFGVIRT(IShellImageData, RegisterAbort)
        HRESULT ( STDMETHODCALLTYPE *RegisterAbort )( 
            IShellImageData * This,
            /* [in] */ IShellImageDataAbort *pAbort,
            /* [out] */ IShellImageDataAbort **ppAbortPrev);
        
        DECLSPEC_XFGVIRT(IShellImageData, CloneFrame)
        HRESULT ( STDMETHODCALLTYPE *CloneFrame )( 
            IShellImageData * This,
            /* [out] */ Image **ppImg);
        
        DECLSPEC_XFGVIRT(IShellImageData, ReplaceFrame)
        HRESULT ( STDMETHODCALLTYPE *ReplaceFrame )( 
            IShellImageData * This,
            /* [in] */ Image *pImg);
        
        END_INTERFACE
    } IShellImageDataVtbl;

    interface IShellImageData
    {
        CONST_VTBL struct IShellImageDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellImageData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellImageData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellImageData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellImageData_Decode(This,dwFlags,cxDesired,cyDesired)	\
    ( (This)->lpVtbl -> Decode(This,dwFlags,cxDesired,cyDesired) ) 

#define IShellImageData_Draw(This,hdc,prcDest,prcSrc)	\
    ( (This)->lpVtbl -> Draw(This,hdc,prcDest,prcSrc) ) 

#define IShellImageData_NextFrame(This)	\
    ( (This)->lpVtbl -> NextFrame(This) ) 

#define IShellImageData_NextPage(This)	\
    ( (This)->lpVtbl -> NextPage(This) ) 

#define IShellImageData_PrevPage(This)	\
    ( (This)->lpVtbl -> PrevPage(This) ) 

#define IShellImageData_IsTransparent(This)	\
    ( (This)->lpVtbl -> IsTransparent(This) ) 

#define IShellImageData_IsAnimated(This)	\
    ( (This)->lpVtbl -> IsAnimated(This) ) 

#define IShellImageData_IsVector(This)	\
    ( (This)->lpVtbl -> IsVector(This) ) 

#define IShellImageData_IsMultipage(This)	\
    ( (This)->lpVtbl -> IsMultipage(This) ) 

#define IShellImageData_IsEditable(This)	\
    ( (This)->lpVtbl -> IsEditable(This) ) 

#define IShellImageData_IsPrintable(This)	\
    ( (This)->lpVtbl -> IsPrintable(This) ) 

#define IShellImageData_IsDecoded(This)	\
    ( (This)->lpVtbl -> IsDecoded(This) ) 

#define IShellImageData_GetCurrentPage(This,pnPage)	\
    ( (This)->lpVtbl -> GetCurrentPage(This,pnPage) ) 

#define IShellImageData_GetPageCount(This,pcPages)	\
    ( (This)->lpVtbl -> GetPageCount(This,pcPages) ) 

#define IShellImageData_SelectPage(This,iPage)	\
    ( (This)->lpVtbl -> SelectPage(This,iPage) ) 

#define IShellImageData_GetSize(This,pSize)	\
    ( (This)->lpVtbl -> GetSize(This,pSize) ) 

#define IShellImageData_GetRawDataFormat(This,pDataFormat)	\
    ( (This)->lpVtbl -> GetRawDataFormat(This,pDataFormat) ) 

#define IShellImageData_GetPixelFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetPixelFormat(This,pFormat) ) 

#define IShellImageData_GetDelay(This,pdwDelay)	\
    ( (This)->lpVtbl -> GetDelay(This,pdwDelay) ) 

#define IShellImageData_GetProperties(This,dwMode,ppPropSet)	\
    ( (This)->lpVtbl -> GetProperties(This,dwMode,ppPropSet) ) 

#define IShellImageData_Rotate(This,dwAngle)	\
    ( (This)->lpVtbl -> Rotate(This,dwAngle) ) 

#define IShellImageData_Scale(This,cx,cy,hints)	\
    ( (This)->lpVtbl -> Scale(This,cx,cy,hints) ) 

#define IShellImageData_DiscardEdit(This)	\
    ( (This)->lpVtbl -> DiscardEdit(This) ) 

#define IShellImageData_SetEncoderParams(This,pbagEnc)	\
    ( (This)->lpVtbl -> SetEncoderParams(This,pbagEnc) ) 

#define IShellImageData_DisplayName(This,wszName,cch)	\
    ( (This)->lpVtbl -> DisplayName(This,wszName,cch) ) 

#define IShellImageData_GetResolution(This,puResolutionX,puResolutionY)	\
    ( (This)->lpVtbl -> GetResolution(This,puResolutionX,puResolutionY) ) 

#define IShellImageData_GetEncoderParams(This,pguidFmt,ppEncParams)	\
    ( (This)->lpVtbl -> GetEncoderParams(This,pguidFmt,ppEncParams) ) 

#define IShellImageData_RegisterAbort(This,pAbort,ppAbortPrev)	\
    ( (This)->lpVtbl -> RegisterAbort(This,pAbort,ppAbortPrev) ) 

#define IShellImageData_CloneFrame(This,ppImg)	\
    ( (This)->lpVtbl -> CloneFrame(This,ppImg) ) 

#define IShellImageData_ReplaceFrame(This,pImg)	\
    ( (This)->lpVtbl -> ReplaceFrame(This,pImg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellImageData_INTERFACE_DEFINED__ */


#ifndef __IShellImageDataAbort_INTERFACE_DEFINED__
#define __IShellImageDataAbort_INTERFACE_DEFINED__

/* interface IShellImageDataAbort */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IShellImageDataAbort;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53fb8e58-50c0-4003-b4aa-0c8df28e7f3a")
    IShellImageDataAbort : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryAbort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellImageDataAbortVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellImageDataAbort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellImageDataAbort * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellImageDataAbort * This);
        
        DECLSPEC_XFGVIRT(IShellImageDataAbort, QueryAbort)
        HRESULT ( STDMETHODCALLTYPE *QueryAbort )( 
            __RPC__in IShellImageDataAbort * This);
        
        END_INTERFACE
    } IShellImageDataAbortVtbl;

    interface IShellImageDataAbort
    {
        CONST_VTBL struct IShellImageDataAbortVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellImageDataAbort_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellImageDataAbort_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellImageDataAbort_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellImageDataAbort_QueryAbort(This)	\
    ( (This)->lpVtbl -> QueryAbort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellImageDataAbort_INTERFACE_DEFINED__ */



#ifndef __ShellImageData_LIBRARY_DEFINED__
#define __ShellImageData_LIBRARY_DEFINED__

/* library ShellImageData */
/* [version][lcid][helpstring][uuid] */ 


EXTERN_C const IID LIBID_ShellImageData;

EXTERN_C const CLSID CLSID_ShellImageDataFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("66e4e4fb-f385-4dd0-8d74-a2efd1bc6178")
ShellImageDataFactory;
#endif
#endif /* __ShellImageData_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_shimgdata_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_shimgdata_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shimgdata_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


