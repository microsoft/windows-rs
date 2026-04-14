

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

#ifndef __mlang_h__
#define __mlang_h__

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

#ifndef __IMLangStringBufW_FWD_DEFINED__
#define __IMLangStringBufW_FWD_DEFINED__
typedef interface IMLangStringBufW IMLangStringBufW;

#endif 	/* __IMLangStringBufW_FWD_DEFINED__ */


#ifndef __IMLangStringBufA_FWD_DEFINED__
#define __IMLangStringBufA_FWD_DEFINED__
typedef interface IMLangStringBufA IMLangStringBufA;

#endif 	/* __IMLangStringBufA_FWD_DEFINED__ */


#ifndef __IMLangString_FWD_DEFINED__
#define __IMLangString_FWD_DEFINED__
typedef interface IMLangString IMLangString;

#endif 	/* __IMLangString_FWD_DEFINED__ */


#ifndef __IMLangStringWStr_FWD_DEFINED__
#define __IMLangStringWStr_FWD_DEFINED__
typedef interface IMLangStringWStr IMLangStringWStr;

#endif 	/* __IMLangStringWStr_FWD_DEFINED__ */


#ifndef __IMLangStringAStr_FWD_DEFINED__
#define __IMLangStringAStr_FWD_DEFINED__
typedef interface IMLangStringAStr IMLangStringAStr;

#endif 	/* __IMLangStringAStr_FWD_DEFINED__ */


#ifndef __IMLangLineBreakConsole_FWD_DEFINED__
#define __IMLangLineBreakConsole_FWD_DEFINED__
typedef interface IMLangLineBreakConsole IMLangLineBreakConsole;

#endif 	/* __IMLangLineBreakConsole_FWD_DEFINED__ */


#ifndef __IEnumCodePage_FWD_DEFINED__
#define __IEnumCodePage_FWD_DEFINED__
typedef interface IEnumCodePage IEnumCodePage;

#endif 	/* __IEnumCodePage_FWD_DEFINED__ */


#ifndef __IEnumRfc1766_FWD_DEFINED__
#define __IEnumRfc1766_FWD_DEFINED__
typedef interface IEnumRfc1766 IEnumRfc1766;

#endif 	/* __IEnumRfc1766_FWD_DEFINED__ */


#ifndef __IEnumScript_FWD_DEFINED__
#define __IEnumScript_FWD_DEFINED__
typedef interface IEnumScript IEnumScript;

#endif 	/* __IEnumScript_FWD_DEFINED__ */


#ifndef __IMLangConvertCharset_FWD_DEFINED__
#define __IMLangConvertCharset_FWD_DEFINED__
typedef interface IMLangConvertCharset IMLangConvertCharset;

#endif 	/* __IMLangConvertCharset_FWD_DEFINED__ */


#ifndef __IMultiLanguage_FWD_DEFINED__
#define __IMultiLanguage_FWD_DEFINED__
typedef interface IMultiLanguage IMultiLanguage;

#endif 	/* __IMultiLanguage_FWD_DEFINED__ */


#ifndef __IMultiLanguage2_FWD_DEFINED__
#define __IMultiLanguage2_FWD_DEFINED__
typedef interface IMultiLanguage2 IMultiLanguage2;

#endif 	/* __IMultiLanguage2_FWD_DEFINED__ */


#ifndef __IMLangCodePages_FWD_DEFINED__
#define __IMLangCodePages_FWD_DEFINED__
typedef interface IMLangCodePages IMLangCodePages;

#endif 	/* __IMLangCodePages_FWD_DEFINED__ */


#ifndef __IMLangFontLink_FWD_DEFINED__
#define __IMLangFontLink_FWD_DEFINED__
typedef interface IMLangFontLink IMLangFontLink;

#endif 	/* __IMLangFontLink_FWD_DEFINED__ */


#ifndef __IMLangFontLink2_FWD_DEFINED__
#define __IMLangFontLink2_FWD_DEFINED__
typedef interface IMLangFontLink2 IMLangFontLink2;

#endif 	/* __IMLangFontLink2_FWD_DEFINED__ */


#ifndef __IMultiLanguage3_FWD_DEFINED__
#define __IMultiLanguage3_FWD_DEFINED__
typedef interface IMultiLanguage3 IMultiLanguage3;

#endif 	/* __IMultiLanguage3_FWD_DEFINED__ */


#ifndef __CMLangString_FWD_DEFINED__
#define __CMLangString_FWD_DEFINED__

#ifdef __cplusplus
typedef class CMLangString CMLangString;
#else
typedef struct CMLangString CMLangString;
#endif /* __cplusplus */

#endif 	/* __CMLangString_FWD_DEFINED__ */


#ifndef __CMLangConvertCharset_FWD_DEFINED__
#define __CMLangConvertCharset_FWD_DEFINED__

#ifdef __cplusplus
typedef class CMLangConvertCharset CMLangConvertCharset;
#else
typedef struct CMLangConvertCharset CMLangConvertCharset;
#endif /* __cplusplus */

#endif 	/* __CMLangConvertCharset_FWD_DEFINED__ */


#ifndef __CMultiLanguage_FWD_DEFINED__
#define __CMultiLanguage_FWD_DEFINED__

#ifdef __cplusplus
typedef class CMultiLanguage CMultiLanguage;
#else
typedef struct CMultiLanguage CMultiLanguage;
#endif /* __cplusplus */

#endif 	/* __CMultiLanguage_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mlang_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//=--------------------------------------------------------------------------=
// MLang.h                                                                    
//=--------------------------------------------------------------------------=
// Copyright (c) Microsoft Corporation. All rights reserved.
//                                                                            
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF        
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO        
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A             
// PARTICULAR PURPOSE.                                                        
//=--------------------------------------------------------------------------=
                                                                              
#pragma comment(lib,"uuid.lib")                                             
                                                                              
//----------------------------------------------------------------------------
// IMultiLanguage Interfaces.                                                 
                                                                              

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0000_v0_0_s_ifspec;

#ifndef __IMLangStringBufW_INTERFACE_DEFINED__
#define __IMLangStringBufW_INTERFACE_DEFINED__

/* interface IMLangStringBufW */
/* [object][unique][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IMLangStringBufW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D24ACD21-BA72-11D0-B188-00AA0038C969")
    IMLangStringBufW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][out] */ 
            _Out_opt_  long *plFlags,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockBuf( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxLock,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcchBuf)  WCHAR **ppszBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockBuf( 
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWrite-cchOffset)  const WCHAR *pszBuf,
            /* [in] */ long cchOffset,
            /* [in] */ long cchWrite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Insert( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxInsert,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchDelete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangStringBufWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangStringBufW * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangStringBufW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangStringBufW * This);
        
        DECLSPEC_XFGVIRT(IMLangStringBufW, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IMLangStringBufW * This,
            /* [annotation][out] */ 
            _Out_opt_  long *plFlags,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf);
        
        DECLSPEC_XFGVIRT(IMLangStringBufW, LockBuf)
        HRESULT ( STDMETHODCALLTYPE *LockBuf )( 
            IMLangStringBufW * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxLock,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcchBuf)  WCHAR **ppszBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf);
        
        DECLSPEC_XFGVIRT(IMLangStringBufW, UnlockBuf)
        HRESULT ( STDMETHODCALLTYPE *UnlockBuf )( 
            IMLangStringBufW * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWrite-cchOffset)  const WCHAR *pszBuf,
            /* [in] */ long cchOffset,
            /* [in] */ long cchWrite);
        
        DECLSPEC_XFGVIRT(IMLangStringBufW, Insert)
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            IMLangStringBufW * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxInsert,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual);
        
        DECLSPEC_XFGVIRT(IMLangStringBufW, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IMLangStringBufW * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchDelete);
        
        END_INTERFACE
    } IMLangStringBufWVtbl;

    interface IMLangStringBufW
    {
        CONST_VTBL struct IMLangStringBufWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangStringBufW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangStringBufW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangStringBufW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangStringBufW_GetStatus(This,plFlags,pcchBuf)	\
    ( (This)->lpVtbl -> GetStatus(This,plFlags,pcchBuf) ) 

#define IMLangStringBufW_LockBuf(This,cchOffset,cchMaxLock,ppszBuf,pcchBuf)	\
    ( (This)->lpVtbl -> LockBuf(This,cchOffset,cchMaxLock,ppszBuf,pcchBuf) ) 

#define IMLangStringBufW_UnlockBuf(This,pszBuf,cchOffset,cchWrite)	\
    ( (This)->lpVtbl -> UnlockBuf(This,pszBuf,cchOffset,cchWrite) ) 

#define IMLangStringBufW_Insert(This,cchOffset,cchMaxInsert,pcchActual)	\
    ( (This)->lpVtbl -> Insert(This,cchOffset,cchMaxInsert,pcchActual) ) 

#define IMLangStringBufW_Delete(This,cchOffset,cchDelete)	\
    ( (This)->lpVtbl -> Delete(This,cchOffset,cchDelete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangStringBufW_INTERFACE_DEFINED__ */


#ifndef __IMLangStringBufA_INTERFACE_DEFINED__
#define __IMLangStringBufA_INTERFACE_DEFINED__

/* interface IMLangStringBufA */
/* [object][unique][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IMLangStringBufA;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D24ACD23-BA72-11D0-B188-00AA0038C969")
    IMLangStringBufA : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][out] */ 
            _Out_opt_  long *plFlags,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockBuf( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxLock,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcchBuf)  CHAR **ppszBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockBuf( 
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWrite-cchOffset)  const CHAR *pszBuf,
            /* [in] */ long cchOffset,
            /* [in] */ long cchWrite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Insert( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxInsert,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ long cchOffset,
            /* [in] */ long cchDelete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangStringBufAVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangStringBufA * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangStringBufA * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangStringBufA * This);
        
        DECLSPEC_XFGVIRT(IMLangStringBufA, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IMLangStringBufA * This,
            /* [annotation][out] */ 
            _Out_opt_  long *plFlags,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf);
        
        DECLSPEC_XFGVIRT(IMLangStringBufA, LockBuf)
        HRESULT ( STDMETHODCALLTYPE *LockBuf )( 
            IMLangStringBufA * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxLock,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcchBuf)  CHAR **ppszBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchBuf);
        
        DECLSPEC_XFGVIRT(IMLangStringBufA, UnlockBuf)
        HRESULT ( STDMETHODCALLTYPE *UnlockBuf )( 
            IMLangStringBufA * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWrite-cchOffset)  const CHAR *pszBuf,
            /* [in] */ long cchOffset,
            /* [in] */ long cchWrite);
        
        DECLSPEC_XFGVIRT(IMLangStringBufA, Insert)
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            IMLangStringBufA * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchMaxInsert,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual);
        
        DECLSPEC_XFGVIRT(IMLangStringBufA, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            IMLangStringBufA * This,
            /* [in] */ long cchOffset,
            /* [in] */ long cchDelete);
        
        END_INTERFACE
    } IMLangStringBufAVtbl;

    interface IMLangStringBufA
    {
        CONST_VTBL struct IMLangStringBufAVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangStringBufA_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangStringBufA_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangStringBufA_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangStringBufA_GetStatus(This,plFlags,pcchBuf)	\
    ( (This)->lpVtbl -> GetStatus(This,plFlags,pcchBuf) ) 

#define IMLangStringBufA_LockBuf(This,cchOffset,cchMaxLock,ppszBuf,pcchBuf)	\
    ( (This)->lpVtbl -> LockBuf(This,cchOffset,cchMaxLock,ppszBuf,pcchBuf) ) 

#define IMLangStringBufA_UnlockBuf(This,pszBuf,cchOffset,cchWrite)	\
    ( (This)->lpVtbl -> UnlockBuf(This,pszBuf,cchOffset,cchWrite) ) 

#define IMLangStringBufA_Insert(This,cchOffset,cchMaxInsert,pcchActual)	\
    ( (This)->lpVtbl -> Insert(This,cchOffset,cchMaxInsert,pcchActual) ) 

#define IMLangStringBufA_Delete(This,cchOffset,cchDelete)	\
    ( (This)->lpVtbl -> Delete(This,cchOffset,cchDelete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangStringBufA_INTERFACE_DEFINED__ */


#ifndef __IMLangString_INTERFACE_DEFINED__
#define __IMLangString_INTERFACE_DEFINED__

/* interface IMLangString */
/* [object][unique][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IMLangString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C04D65CE-B70D-11D0-B188-00AA0038C969")
    IMLangString : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Sync( 
            /* [in] */ BOOL fNoAccess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [annotation][retval][out] */ 
            _Out_opt_  long *plLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMLStr( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMLStr( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [annotation][in] */ 
            _In_  const IID *piid,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppDestMLStr,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestPos,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangString * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangString * This);
        
        DECLSPEC_XFGVIRT(IMLangString, Sync)
        HRESULT ( STDMETHODCALLTYPE *Sync )( 
            IMLangString * This,
            /* [in] */ BOOL fNoAccess);
        
        DECLSPEC_XFGVIRT(IMLangString, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IMLangString * This,
            /* [annotation][retval][out] */ 
            _Out_opt_  long *plLen);
        
        DECLSPEC_XFGVIRT(IMLangString, SetMLStr)
        HRESULT ( STDMETHODCALLTYPE *SetMLStr )( 
            IMLangString * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen);
        
        DECLSPEC_XFGVIRT(IMLangString, GetMLStr)
        HRESULT ( STDMETHODCALLTYPE *GetMLStr )( 
            IMLangString * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [annotation][in] */ 
            _In_  const IID *piid,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppDestMLStr,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestPos,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        END_INTERFACE
    } IMLangStringVtbl;

    interface IMLangString
    {
        CONST_VTBL struct IMLangStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangString_Sync(This,fNoAccess)	\
    ( (This)->lpVtbl -> Sync(This,fNoAccess) ) 

#define IMLangString_GetLength(This,plLen)	\
    ( (This)->lpVtbl -> GetLength(This,plLen) ) 

#define IMLangString_SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen)	\
    ( (This)->lpVtbl -> SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen) ) 

#define IMLangString_GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen)	\
    ( (This)->lpVtbl -> GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangString_INTERFACE_DEFINED__ */


#ifndef __IMLangStringWStr_INTERFACE_DEFINED__
#define __IMLangStringWStr_INTERFACE_DEFINED__

/* interface IMLangStringWStr */
/* [object][unique][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IMLangStringWStr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C04D65D0-B70D-11D0-B188-00AA0038C969")
    IMLangStringWStr : public IMLangString
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetWStr( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][size_is][string][in] */ 
            _In_reads_(cchSrc)  LPCWSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrBufW( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][unique][in] */ 
            _In_opt_  IMLangStringBufW *pSrcBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWStr( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(cchDest)  LPWSTR pszDest,
            /* [in] */ long cchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrBufW( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Outptr_  IMLangStringBufW **ppDestBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockWStr( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long lFlags,
            /* [in] */ long cchRequest,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_opt_result_buffer_(*pcchDest)  LPWSTR *ppszDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockWStr( 
            /* [annotation][size_is][string][in] */ 
            _In_reads_(cchSrc)  LPCWSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLocale( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ LCID locale) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocale( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plocale,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocalePos,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocaleLen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangStringWStrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangStringWStr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangStringWStr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangStringWStr * This);
        
        DECLSPEC_XFGVIRT(IMLangString, Sync)
        HRESULT ( STDMETHODCALLTYPE *Sync )( 
            IMLangStringWStr * This,
            /* [in] */ BOOL fNoAccess);
        
        DECLSPEC_XFGVIRT(IMLangString, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IMLangStringWStr * This,
            /* [annotation][retval][out] */ 
            _Out_opt_  long *plLen);
        
        DECLSPEC_XFGVIRT(IMLangString, SetMLStr)
        HRESULT ( STDMETHODCALLTYPE *SetMLStr )( 
            IMLangStringWStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen);
        
        DECLSPEC_XFGVIRT(IMLangString, GetMLStr)
        HRESULT ( STDMETHODCALLTYPE *GetMLStr )( 
            IMLangStringWStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [annotation][in] */ 
            _In_  const IID *piid,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppDestMLStr,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestPos,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, SetWStr)
        HRESULT ( STDMETHODCALLTYPE *SetWStr )( 
            IMLangStringWStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][size_is][string][in] */ 
            _In_reads_(cchSrc)  LPCWSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, SetStrBufW)
        HRESULT ( STDMETHODCALLTYPE *SetStrBufW )( 
            IMLangStringWStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][unique][in] */ 
            _In_opt_  IMLangStringBufW *pSrcBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, GetWStr)
        HRESULT ( STDMETHODCALLTYPE *GetWStr )( 
            IMLangStringWStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(cchDest)  LPWSTR pszDest,
            /* [in] */ long cchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, GetStrBufW)
        HRESULT ( STDMETHODCALLTYPE *GetStrBufW )( 
            IMLangStringWStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Outptr_  IMLangStringBufW **ppDestBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, LockWStr)
        HRESULT ( STDMETHODCALLTYPE *LockWStr )( 
            IMLangStringWStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long lFlags,
            /* [in] */ long cchRequest,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_opt_result_buffer_(*pcchDest)  LPWSTR *ppszDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, UnlockWStr)
        HRESULT ( STDMETHODCALLTYPE *UnlockWStr )( 
            IMLangStringWStr * This,
            /* [annotation][size_is][string][in] */ 
            _In_reads_(cchSrc)  LPCWSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, SetLocale)
        HRESULT ( STDMETHODCALLTYPE *SetLocale )( 
            IMLangStringWStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ LCID locale);
        
        DECLSPEC_XFGVIRT(IMLangStringWStr, GetLocale)
        HRESULT ( STDMETHODCALLTYPE *GetLocale )( 
            IMLangStringWStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plocale,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocalePos,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocaleLen);
        
        END_INTERFACE
    } IMLangStringWStrVtbl;

    interface IMLangStringWStr
    {
        CONST_VTBL struct IMLangStringWStrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangStringWStr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangStringWStr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangStringWStr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangStringWStr_Sync(This,fNoAccess)	\
    ( (This)->lpVtbl -> Sync(This,fNoAccess) ) 

#define IMLangStringWStr_GetLength(This,plLen)	\
    ( (This)->lpVtbl -> GetLength(This,plLen) ) 

#define IMLangStringWStr_SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen)	\
    ( (This)->lpVtbl -> SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen) ) 

#define IMLangStringWStr_GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen)	\
    ( (This)->lpVtbl -> GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen) ) 


#define IMLangStringWStr_SetWStr(This,lDestPos,lDestLen,pszSrc,cchSrc,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> SetWStr(This,lDestPos,lDestLen,pszSrc,cchSrc,pcchActual,plActualLen) ) 

#define IMLangStringWStr_SetStrBufW(This,lDestPos,lDestLen,pSrcBuf,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> SetStrBufW(This,lDestPos,lDestLen,pSrcBuf,pcchActual,plActualLen) ) 

#define IMLangStringWStr_GetWStr(This,lSrcPos,lSrcLen,pszDest,cchDest,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> GetWStr(This,lSrcPos,lSrcLen,pszDest,cchDest,pcchActual,plActualLen) ) 

#define IMLangStringWStr_GetStrBufW(This,lSrcPos,lSrcMaxLen,ppDestBuf,plDestLen)	\
    ( (This)->lpVtbl -> GetStrBufW(This,lSrcPos,lSrcMaxLen,ppDestBuf,plDestLen) ) 

#define IMLangStringWStr_LockWStr(This,lSrcPos,lSrcLen,lFlags,cchRequest,ppszDest,pcchDest,plDestLen)	\
    ( (This)->lpVtbl -> LockWStr(This,lSrcPos,lSrcLen,lFlags,cchRequest,ppszDest,pcchDest,plDestLen) ) 

#define IMLangStringWStr_UnlockWStr(This,pszSrc,cchSrc,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> UnlockWStr(This,pszSrc,cchSrc,pcchActual,plActualLen) ) 

#define IMLangStringWStr_SetLocale(This,lDestPos,lDestLen,locale)	\
    ( (This)->lpVtbl -> SetLocale(This,lDestPos,lDestLen,locale) ) 

#define IMLangStringWStr_GetLocale(This,lSrcPos,lSrcMaxLen,plocale,plLocalePos,plLocaleLen)	\
    ( (This)->lpVtbl -> GetLocale(This,lSrcPos,lSrcMaxLen,plocale,plLocalePos,plLocaleLen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangStringWStr_INTERFACE_DEFINED__ */


#ifndef __IMLangStringAStr_INTERFACE_DEFINED__
#define __IMLangStringAStr_INTERFACE_DEFINED__

/* interface IMLangStringAStr */
/* [object][unique][helpstring][uuid][local] */ 


EXTERN_C const IID IID_IMLangStringAStr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C04D65D2-B70D-11D0-B188-00AA0038C969")
    IMLangStringAStr : public IMLangString
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAStr( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ UINT uCodePage,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  LPCSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrBufA( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ UINT uCodePage,
            /* [annotation][in] */ 
            _In_opt_  IMLangStringBufA *pSrcBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAStr( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ UINT uCodePageIn,
            /* [annotation][out] */ 
            _Reserved_  UINT *puCodePageOut,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(cchDest)  LPSTR pszDest,
            /* [in] */ long cchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrBufA( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puDestCodePage,
            /* [annotation][out] */ 
            _Outptr_  IMLangStringBufA **ppDestBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockAStr( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long lFlags,
            /* [in] */ UINT uCodePageIn,
            /* [in] */ long cchRequest,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puCodePageOut,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_opt_result_buffer_(*pcchDest)  LPSTR *ppszDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockAStr( 
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  LPCSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLocale( 
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ LCID locale) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocale( 
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plocale,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocalePos,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocaleLen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangStringAStrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangStringAStr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangStringAStr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangStringAStr * This);
        
        DECLSPEC_XFGVIRT(IMLangString, Sync)
        HRESULT ( STDMETHODCALLTYPE *Sync )( 
            IMLangStringAStr * This,
            /* [in] */ BOOL fNoAccess);
        
        DECLSPEC_XFGVIRT(IMLangString, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IMLangStringAStr * This,
            /* [annotation][retval][out] */ 
            _Out_opt_  long *plLen);
        
        DECLSPEC_XFGVIRT(IMLangString, SetMLStr)
        HRESULT ( STDMETHODCALLTYPE *SetMLStr )( 
            IMLangStringAStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen);
        
        DECLSPEC_XFGVIRT(IMLangString, GetMLStr)
        HRESULT ( STDMETHODCALLTYPE *GetMLStr )( 
            IMLangStringAStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [annotation][in] */ 
            _In_  const IID *piid,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppDestMLStr,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestPos,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, SetAStr)
        HRESULT ( STDMETHODCALLTYPE *SetAStr )( 
            IMLangStringAStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ UINT uCodePage,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  LPCSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, SetStrBufA)
        HRESULT ( STDMETHODCALLTYPE *SetStrBufA )( 
            IMLangStringAStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ UINT uCodePage,
            /* [annotation][in] */ 
            _In_opt_  IMLangStringBufA *pSrcBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, GetAStr)
        HRESULT ( STDMETHODCALLTYPE *GetAStr )( 
            IMLangStringAStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ UINT uCodePageIn,
            /* [annotation][out] */ 
            _Reserved_  UINT *puCodePageOut,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(cchDest)  LPSTR pszDest,
            /* [in] */ long cchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, GetStrBufA)
        HRESULT ( STDMETHODCALLTYPE *GetStrBufA )( 
            IMLangStringAStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puDestCodePage,
            /* [annotation][out] */ 
            _Outptr_  IMLangStringBufA **ppDestBuf,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, LockAStr)
        HRESULT ( STDMETHODCALLTYPE *LockAStr )( 
            IMLangStringAStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long lFlags,
            /* [in] */ UINT uCodePageIn,
            /* [in] */ long cchRequest,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puCodePageOut,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_opt_result_buffer_(*pcchDest)  LPSTR *ppszDest,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchDest,
            /* [annotation][out] */ 
            _Out_opt_  long *plDestLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, UnlockAStr)
        HRESULT ( STDMETHODCALLTYPE *UnlockAStr )( 
            IMLangStringAStr * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  LPCSTR pszSrc,
            /* [in] */ long cchSrc,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchActual,
            /* [annotation][out] */ 
            _Out_opt_  long *plActualLen);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, SetLocale)
        HRESULT ( STDMETHODCALLTYPE *SetLocale )( 
            IMLangStringAStr * This,
            /* [in] */ long lDestPos,
            /* [in] */ long lDestLen,
            /* [in] */ LCID locale);
        
        DECLSPEC_XFGVIRT(IMLangStringAStr, GetLocale)
        HRESULT ( STDMETHODCALLTYPE *GetLocale )( 
            IMLangStringAStr * This,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcMaxLen,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plocale,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocalePos,
            /* [annotation][out] */ 
            _Out_opt_  long *plLocaleLen);
        
        END_INTERFACE
    } IMLangStringAStrVtbl;

    interface IMLangStringAStr
    {
        CONST_VTBL struct IMLangStringAStrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangStringAStr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangStringAStr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangStringAStr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangStringAStr_Sync(This,fNoAccess)	\
    ( (This)->lpVtbl -> Sync(This,fNoAccess) ) 

#define IMLangStringAStr_GetLength(This,plLen)	\
    ( (This)->lpVtbl -> GetLength(This,plLen) ) 

#define IMLangStringAStr_SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen)	\
    ( (This)->lpVtbl -> SetMLStr(This,lDestPos,lDestLen,pSrcMLStr,lSrcPos,lSrcLen) ) 

#define IMLangStringAStr_GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen)	\
    ( (This)->lpVtbl -> GetMLStr(This,lSrcPos,lSrcLen,pUnkOuter,dwClsContext,piid,ppDestMLStr,plDestPos,plDestLen) ) 


#define IMLangStringAStr_SetAStr(This,lDestPos,lDestLen,uCodePage,pszSrc,cchSrc,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> SetAStr(This,lDestPos,lDestLen,uCodePage,pszSrc,cchSrc,pcchActual,plActualLen) ) 

#define IMLangStringAStr_SetStrBufA(This,lDestPos,lDestLen,uCodePage,pSrcBuf,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> SetStrBufA(This,lDestPos,lDestLen,uCodePage,pSrcBuf,pcchActual,plActualLen) ) 

#define IMLangStringAStr_GetAStr(This,lSrcPos,lSrcLen,uCodePageIn,puCodePageOut,pszDest,cchDest,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> GetAStr(This,lSrcPos,lSrcLen,uCodePageIn,puCodePageOut,pszDest,cchDest,pcchActual,plActualLen) ) 

#define IMLangStringAStr_GetStrBufA(This,lSrcPos,lSrcMaxLen,puDestCodePage,ppDestBuf,plDestLen)	\
    ( (This)->lpVtbl -> GetStrBufA(This,lSrcPos,lSrcMaxLen,puDestCodePage,ppDestBuf,plDestLen) ) 

#define IMLangStringAStr_LockAStr(This,lSrcPos,lSrcLen,lFlags,uCodePageIn,cchRequest,puCodePageOut,ppszDest,pcchDest,plDestLen)	\
    ( (This)->lpVtbl -> LockAStr(This,lSrcPos,lSrcLen,lFlags,uCodePageIn,cchRequest,puCodePageOut,ppszDest,pcchDest,plDestLen) ) 

#define IMLangStringAStr_UnlockAStr(This,pszSrc,cchSrc,pcchActual,plActualLen)	\
    ( (This)->lpVtbl -> UnlockAStr(This,pszSrc,cchSrc,pcchActual,plActualLen) ) 

#define IMLangStringAStr_SetLocale(This,lDestPos,lDestLen,locale)	\
    ( (This)->lpVtbl -> SetLocale(This,lDestPos,lDestLen,locale) ) 

#define IMLangStringAStr_GetLocale(This,lSrcPos,lSrcMaxLen,plocale,plLocalePos,plLocaleLen)	\
    ( (This)->lpVtbl -> GetLocale(This,lSrcPos,lSrcMaxLen,plocale,plLocalePos,plLocaleLen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangStringAStr_INTERFACE_DEFINED__ */


#ifndef __IMLangLineBreakConsole_INTERFACE_DEFINED__
#define __IMLangLineBreakConsole_INTERFACE_DEFINED__

/* interface IMLangLineBreakConsole */
/* [object][local][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IMLangLineBreakConsole;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5BE2EE1-BFD7-11D0-B188-00AA0038C969")
    IMLangLineBreakConsole : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BreakLineML( 
            /* [annotation][in] */ 
            _In_  IMLangString *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long cMinColumns,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *plLineLen,
            /* [annotation][out] */ 
            _Out_opt_  long *plSkipLen) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BreakLineW( 
            /* [in] */ LCID locale,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchLine,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchSkip) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BreakLineA( 
            /* [in] */ LCID locale,
            /* [in] */ UINT uCodePage,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const CHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchLine,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchSkip) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangLineBreakConsoleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangLineBreakConsole * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangLineBreakConsole * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangLineBreakConsole * This);
        
        DECLSPEC_XFGVIRT(IMLangLineBreakConsole, BreakLineML)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BreakLineML )( 
            IMLangLineBreakConsole * This,
            /* [annotation][in] */ 
            _In_  IMLangString *pSrcMLStr,
            /* [in] */ long lSrcPos,
            /* [in] */ long lSrcLen,
            /* [in] */ long cMinColumns,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *plLineLen,
            /* [annotation][out] */ 
            _Out_opt_  long *plSkipLen);
        
        DECLSPEC_XFGVIRT(IMLangLineBreakConsole, BreakLineW)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BreakLineW )( 
            IMLangLineBreakConsole * This,
            /* [in] */ LCID locale,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchLine,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchSkip);
        
        DECLSPEC_XFGVIRT(IMLangLineBreakConsole, BreakLineA)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BreakLineA )( 
            IMLangLineBreakConsole * This,
            /* [in] */ LCID locale,
            /* [in] */ UINT uCodePage,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const CHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ long cMaxColumns,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchLine,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchSkip);
        
        END_INTERFACE
    } IMLangLineBreakConsoleVtbl;

    interface IMLangLineBreakConsole
    {
        CONST_VTBL struct IMLangLineBreakConsoleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangLineBreakConsole_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangLineBreakConsole_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangLineBreakConsole_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangLineBreakConsole_BreakLineML(This,pSrcMLStr,lSrcPos,lSrcLen,cMinColumns,cMaxColumns,plLineLen,plSkipLen)	\
    ( (This)->lpVtbl -> BreakLineML(This,pSrcMLStr,lSrcPos,lSrcLen,cMinColumns,cMaxColumns,plLineLen,plSkipLen) ) 

#define IMLangLineBreakConsole_BreakLineW(This,locale,pszSrc,cchSrc,cMaxColumns,pcchLine,pcchSkip)	\
    ( (This)->lpVtbl -> BreakLineW(This,locale,pszSrc,cchSrc,cMaxColumns,pcchLine,pcchSkip) ) 

#define IMLangLineBreakConsole_BreakLineA(This,locale,uCodePage,pszSrc,cchSrc,cMaxColumns,pcchLine,pcchSkip)	\
    ( (This)->lpVtbl -> BreakLineA(This,locale,uCodePage,pszSrc,cchSrc,cMaxColumns,pcchLine,pcchSkip) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangLineBreakConsole_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mlang_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0006_v0_0_s_ifspec;

#ifndef __IEnumCodePage_INTERFACE_DEFINED__
#define __IEnumCodePage_INTERFACE_DEFINED__

/* interface IEnumCodePage */
/* [local][unique][uuid][object] */ 

#define	MAX_MIMECP_NAME	( 64 )

#define	MAX_MIMECSET_NAME	( 50 )

#define	MAX_MIMEFACE_NAME	( 32 )

typedef 
enum tagMIMECONTF
    {
        MIMECONTF_MAILNEWS	= 0x1,
        MIMECONTF_BROWSER	= 0x2,
        MIMECONTF_MINIMAL	= 0x4,
        MIMECONTF_IMPORT	= 0x8,
        MIMECONTF_SAVABLE_MAILNEWS	= 0x100,
        MIMECONTF_SAVABLE_BROWSER	= 0x200,
        MIMECONTF_EXPORT	= 0x400,
        MIMECONTF_PRIVCONVERTER	= 0x10000,
        MIMECONTF_VALID	= 0x20000,
        MIMECONTF_VALID_NLS	= 0x40000,
        MIMECONTF_MIME_IE4	= 0x10000000,
        MIMECONTF_MIME_LATEST	= 0x20000000,
        MIMECONTF_MIME_REGISTRY	= 0x40000000
    } 	MIMECONTF;

typedef struct tagMIMECPINFO
    {
    DWORD dwFlags;
    UINT uiCodePage;
    UINT uiFamilyCodePage;
    WCHAR wszDescription[ 64 ];
    WCHAR wszWebCharset[ 50 ];
    WCHAR wszHeaderCharset[ 50 ];
    WCHAR wszBodyCharset[ 50 ];
    WCHAR wszFixedWidthFont[ 32 ];
    WCHAR wszProportionalFont[ 32 ];
    BYTE bGDICharset;
    } 	MIMECPINFO;

typedef struct tagMIMECPINFO *PMIMECPINFO;

typedef struct tagMIMECSETINFO
    {
    UINT uiCodePage;
    UINT uiInternetEncoding;
    WCHAR wszCharset[ 50 ];
    } 	MIMECSETINFO;

typedef struct tagMIMECSETINFO *PMIMECSETINFO;

typedef /* [unique] */ IEnumCodePage *LPENUMCODEPAGE;


EXTERN_C const IID IID_IEnumCodePage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("275c23e3-3747-11d0-9fea-00aa003f8646")
    IEnumCodePage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumCodePage **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCodePageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumCodePage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumCodePage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumCodePage * This);
        
        DECLSPEC_XFGVIRT(IEnumCodePage, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumCodePage * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumCodePage **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumCodePage, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumCodePage * This,
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumCodePage, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumCodePage * This);
        
        DECLSPEC_XFGVIRT(IEnumCodePage, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumCodePage * This,
            /* [in] */ ULONG celt);
        
        END_INTERFACE
    } IEnumCodePageVtbl;

    interface IEnumCodePage
    {
        CONST_VTBL struct IEnumCodePageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCodePage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCodePage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCodePage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCodePage_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumCodePage_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumCodePage_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCodePage_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCodePage_INTERFACE_DEFINED__ */


#ifndef __IEnumRfc1766_INTERFACE_DEFINED__
#define __IEnumRfc1766_INTERFACE_DEFINED__

/* interface IEnumRfc1766 */
/* [local][unique][uuid][object] */ 

#define	MAX_RFC1766_NAME	( 6 )

#define	MAX_LOCALE_NAME	( 32 )

typedef struct tagRFC1766INFO
    {
    LCID lcid;
    WCHAR wszRfc1766[ 6 ];
    WCHAR wszLocaleName[ 32 ];
    } 	RFC1766INFO;

typedef struct tagRFC1766INFO *PRFC1766INFO;

typedef /* [unique] */ IEnumRfc1766 *LPENUMRFC1766;


EXTERN_C const IID IID_IEnumRfc1766;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3dc39d1d-c030-11d0-b81b-00c04fc9b31f")
    IEnumRfc1766 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumRfc1766 **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRfc1766Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumRfc1766 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumRfc1766 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumRfc1766 * This);
        
        DECLSPEC_XFGVIRT(IEnumRfc1766, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumRfc1766 * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumRfc1766 **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumRfc1766, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumRfc1766 * This,
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumRfc1766, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumRfc1766 * This);
        
        DECLSPEC_XFGVIRT(IEnumRfc1766, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumRfc1766 * This,
            /* [in] */ ULONG celt);
        
        END_INTERFACE
    } IEnumRfc1766Vtbl;

    interface IEnumRfc1766
    {
        CONST_VTBL struct IEnumRfc1766Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRfc1766_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRfc1766_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRfc1766_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRfc1766_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumRfc1766_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumRfc1766_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRfc1766_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumRfc1766_INTERFACE_DEFINED__ */


#ifndef __IEnumScript_INTERFACE_DEFINED__
#define __IEnumScript_INTERFACE_DEFINED__

/* interface IEnumScript */
/* [local][unique][uuid][object] */ 

#define	MAX_SCRIPT_NAME	( 48 )

typedef BYTE SCRIPT_ID;

typedef __int64 SCRIPT_IDS;

typedef 
enum tagSCRIPTCONTF
    {
        sidDefault	= 0,
        sidMerge	= ( sidDefault + 1 ) ,
        sidAsciiSym	= ( sidMerge + 1 ) ,
        sidAsciiLatin	= ( sidAsciiSym + 1 ) ,
        sidLatin	= ( sidAsciiLatin + 1 ) ,
        sidGreek	= ( sidLatin + 1 ) ,
        sidCyrillic	= ( sidGreek + 1 ) ,
        sidArmenian	= ( sidCyrillic + 1 ) ,
        sidHebrew	= ( sidArmenian + 1 ) ,
        sidArabic	= ( sidHebrew + 1 ) ,
        sidDevanagari	= ( sidArabic + 1 ) ,
        sidBengali	= ( sidDevanagari + 1 ) ,
        sidGurmukhi	= ( sidBengali + 1 ) ,
        sidGujarati	= ( sidGurmukhi + 1 ) ,
        sidOriya	= ( sidGujarati + 1 ) ,
        sidTamil	= ( sidOriya + 1 ) ,
        sidTelugu	= ( sidTamil + 1 ) ,
        sidKannada	= ( sidTelugu + 1 ) ,
        sidMalayalam	= ( sidKannada + 1 ) ,
        sidThai	= ( sidMalayalam + 1 ) ,
        sidLao	= ( sidThai + 1 ) ,
        sidTibetan	= ( sidLao + 1 ) ,
        sidGeorgian	= ( sidTibetan + 1 ) ,
        sidHangul	= ( sidGeorgian + 1 ) ,
        sidKana	= ( sidHangul + 1 ) ,
        sidBopomofo	= ( sidKana + 1 ) ,
        sidHan	= ( sidBopomofo + 1 ) ,
        sidEthiopic	= ( sidHan + 1 ) ,
        sidCanSyllabic	= ( sidEthiopic + 1 ) ,
        sidCherokee	= ( sidCanSyllabic + 1 ) ,
        sidYi	= ( sidCherokee + 1 ) ,
        sidBraille	= ( sidYi + 1 ) ,
        sidRunic	= ( sidBraille + 1 ) ,
        sidOgham	= ( sidRunic + 1 ) ,
        sidSinhala	= ( sidOgham + 1 ) ,
        sidSyriac	= ( sidSinhala + 1 ) ,
        sidBurmese	= ( sidSyriac + 1 ) ,
        sidKhmer	= ( sidBurmese + 1 ) ,
        sidThaana	= ( sidKhmer + 1 ) ,
        sidMongolian	= ( sidThaana + 1 ) ,
        sidUserDefined	= ( sidMongolian + 1 ) ,
        sidLim	= ( sidUserDefined + 1 ) ,
        sidFEFirst	= sidHangul,
        sidFELast	= sidHan
    } 	SCRIPTCONTF;

typedef struct tagSCRIPTINFO
    {
    SCRIPT_ID ScriptId;
    UINT uiCodePage;
    WCHAR wszDescription[ 48 ];
    WCHAR wszFixedWidthFont[ 32 ];
    WCHAR wszProportionalFont[ 32 ];
    } 	SCRIPTINFO;

typedef struct tagSCRIPTINFO *PSCRIPTINFO;

typedef /* [unique] */ IEnumScript *LPENUMScript;


EXTERN_C const IID IID_IEnumScript;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE5F1430-388B-11d2-8380-00C04F8F5DA1")
    IEnumScript : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumScript **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PSCRIPTINFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumScriptVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumScript * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumScript * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumScript * This);
        
        DECLSPEC_XFGVIRT(IEnumScript, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumScript * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumScript **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumScript, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumScript * This,
            /* [in] */ ULONG celt,
            /* [annotation][out] */ 
            _Out_  PSCRIPTINFO rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumScript, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumScript * This);
        
        DECLSPEC_XFGVIRT(IEnumScript, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumScript * This,
            /* [in] */ ULONG celt);
        
        END_INTERFACE
    } IEnumScriptVtbl;

    interface IEnumScript
    {
        CONST_VTBL struct IEnumScriptVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumScript_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumScript_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumScript_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumScript_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumScript_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumScript_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumScript_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumScript_INTERFACE_DEFINED__ */


#ifndef __IMLangConvertCharset_INTERFACE_DEFINED__
#define __IMLangConvertCharset_INTERFACE_DEFINED__

/* interface IMLangConvertCharset */
/* [local][unique][uuid][object] */ 

typedef 
enum tagMLCONVCHARF
    {
        MLCONVCHARF_AUTODETECT	= 1,
        MLCONVCHARF_ENTITIZE	= 2,
        MLCONVCHARF_NCR_ENTITIZE	= 2,
        MLCONVCHARF_NAME_ENTITIZE	= 4,
        MLCONVCHARF_USEDEFCHAR	= 8,
        MLCONVCHARF_NOBESTFITCHARS	= 16,
        MLCONVCHARF_DETECTJPN	= 32
    } 	MLCONVCHAR;

typedef 
enum tagMLCPF
    {
        MLDETECTF_MAILNEWS	= 0x1,
        MLDETECTF_BROWSER	= 0x2,
        MLDETECTF_VALID	= 0x4,
        MLDETECTF_VALID_NLS	= 0x8,
        MLDETECTF_PRESERVE_ORDER	= 0x10,
        MLDETECTF_PREFERRED_ONLY	= 0x20,
        MLDETECTF_FILTER_SPECIALCHAR	= 0x40,
        MLDETECTF_EURO_UTF8	= 0x80
    } 	MLCP;

typedef /* [unique] */ IMLangConvertCharset *LPMLANGCONVERTCHARSET;


EXTERN_C const IID IID_IMLangConvertCharset;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d66d6f98-cdaa-11d0-b822-00c04fc9b31f")
    IMLangConvertCharset : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceCodePage( 
            /* [annotation][out] */ 
            _Out_  UINT *puiSrcCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDestinationCodePage( 
            /* [annotation][out] */ 
            _Out_  UINT *puiDstCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoConversion( 
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoConversionToUnicode( 
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoConversionFromUnicode( 
            /* [annotation][in] */ 
            _In_reads_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangConvertCharsetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangConvertCharset * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangConvertCharset * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangConvertCharset * This);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMLangConvertCharset * This,
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, GetSourceCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetSourceCodePage )( 
            IMLangConvertCharset * This,
            /* [annotation][out] */ 
            _Out_  UINT *puiSrcCodePage);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, GetDestinationCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetDestinationCodePage )( 
            IMLangConvertCharset * This,
            /* [annotation][out] */ 
            _Out_  UINT *puiDstCodePage);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IMLangConvertCharset * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwProperty);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, DoConversion)
        HRESULT ( STDMETHODCALLTYPE *DoConversion )( 
            IMLangConvertCharset * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, DoConversionToUnicode)
        HRESULT ( STDMETHODCALLTYPE *DoConversionToUnicode )( 
            IMLangConvertCharset * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMLangConvertCharset, DoConversionFromUnicode)
        HRESULT ( STDMETHODCALLTYPE *DoConversionFromUnicode )( 
            IMLangConvertCharset * This,
            /* [annotation][in] */ 
            _In_reads_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        END_INTERFACE
    } IMLangConvertCharsetVtbl;

    interface IMLangConvertCharset
    {
        CONST_VTBL struct IMLangConvertCharsetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangConvertCharset_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangConvertCharset_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangConvertCharset_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangConvertCharset_Initialize(This,uiSrcCodePage,uiDstCodePage,dwProperty)	\
    ( (This)->lpVtbl -> Initialize(This,uiSrcCodePage,uiDstCodePage,dwProperty) ) 

#define IMLangConvertCharset_GetSourceCodePage(This,puiSrcCodePage)	\
    ( (This)->lpVtbl -> GetSourceCodePage(This,puiSrcCodePage) ) 

#define IMLangConvertCharset_GetDestinationCodePage(This,puiDstCodePage)	\
    ( (This)->lpVtbl -> GetDestinationCodePage(This,puiDstCodePage) ) 

#define IMLangConvertCharset_GetProperty(This,pdwProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,pdwProperty) ) 

#define IMLangConvertCharset_DoConversion(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> DoConversion(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMLangConvertCharset_DoConversionToUnicode(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> DoConversionToUnicode(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMLangConvertCharset_DoConversionFromUnicode(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> DoConversionFromUnicode(This,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangConvertCharset_INTERFACE_DEFINED__ */


#ifndef __IMultiLanguage_INTERFACE_DEFINED__
#define __IMultiLanguage_INTERFACE_DEFINED__

/* interface IMultiLanguage */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ IMultiLanguage *LPMULTILANGUAGE;


EXTERN_C const IID IID_IMultiLanguage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("275c23e1-3747-11d0-9fea-00aa003f8646")
    IMultiLanguage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfCodePageInfo( 
            /* [annotation][out] */ 
            _Out_  UINT *pcCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageInfo( 
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO pCodePageInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFamilyCodePage( 
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puiFamilyCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCodePages( 
            /* [in] */ DWORD grfFlags,
            /* [annotation][out] */ 
            _Outptr_  IEnumCodePage **ppEnumCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCharsetInfo( 
            /* [in] */ BSTR Charset,
            /* [annotation][out] */ 
            _Out_  PMIMECSETINFO pCharsetInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsConvertible( 
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertString( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_opt_(*pcDstSize, *pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringToUnicode( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pcDstSize, *pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringFromUnicode( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_opt_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_opt_(*pcDstSize, *pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringReset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRfc1766FromLcid( 
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLcidFromRfc1766( 
            /* [annotation][out] */ 
            _Out_  LCID *pLocale,
            /* [annotation][in] */ 
            _In_  BSTR bstrRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRfc1766( 
            /* [annotation][out] */ 
            _Outptr_  IEnumRfc1766 **ppEnumRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRfc1766Info( 
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO pRfc1766Info) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateConvertCharset( 
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty,
            /* [annotation][out] */ 
            _Outptr_  IMLangConvertCharset **ppMLangConvertCharset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiLanguageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMultiLanguage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMultiLanguage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMultiLanguage * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetNumberOfCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfCodePageInfo )( 
            IMultiLanguage * This,
            /* [annotation][out] */ 
            _Out_  UINT *pcCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageInfo )( 
            IMultiLanguage * This,
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO pCodePageInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetFamilyCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetFamilyCodePage )( 
            IMultiLanguage * This,
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puiFamilyCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, EnumCodePages)
        HRESULT ( STDMETHODCALLTYPE *EnumCodePages )( 
            IMultiLanguage * This,
            /* [in] */ DWORD grfFlags,
            /* [annotation][out] */ 
            _Outptr_  IEnumCodePage **ppEnumCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetCharsetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCharsetInfo )( 
            IMultiLanguage * This,
            /* [in] */ BSTR Charset,
            /* [annotation][out] */ 
            _Out_  PMIMECSETINFO pCharsetInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, IsConvertible)
        HRESULT ( STDMETHODCALLTYPE *IsConvertible )( 
            IMultiLanguage * This,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, ConvertString)
        HRESULT ( STDMETHODCALLTYPE *ConvertString )( 
            IMultiLanguage * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_opt_(*pcDstSize, *pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, ConvertStringToUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringToUnicode )( 
            IMultiLanguage * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pcDstSize, *pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, ConvertStringFromUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringFromUnicode )( 
            IMultiLanguage * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_opt_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_opt_(*pcDstSize, *pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, ConvertStringReset)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringReset )( 
            IMultiLanguage * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetRfc1766FromLcid)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766FromLcid )( 
            IMultiLanguage * This,
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetLcidFromRfc1766)
        HRESULT ( STDMETHODCALLTYPE *GetLcidFromRfc1766 )( 
            IMultiLanguage * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLocale,
            /* [annotation][in] */ 
            _In_  BSTR bstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, EnumRfc1766)
        HRESULT ( STDMETHODCALLTYPE *EnumRfc1766 )( 
            IMultiLanguage * This,
            /* [annotation][out] */ 
            _Outptr_  IEnumRfc1766 **ppEnumRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, GetRfc1766Info)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766Info )( 
            IMultiLanguage * This,
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO pRfc1766Info);
        
        DECLSPEC_XFGVIRT(IMultiLanguage, CreateConvertCharset)
        HRESULT ( STDMETHODCALLTYPE *CreateConvertCharset )( 
            IMultiLanguage * This,
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty,
            /* [annotation][out] */ 
            _Outptr_  IMLangConvertCharset **ppMLangConvertCharset);
        
        END_INTERFACE
    } IMultiLanguageVtbl;

    interface IMultiLanguage
    {
        CONST_VTBL struct IMultiLanguageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiLanguage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiLanguage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiLanguage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiLanguage_GetNumberOfCodePageInfo(This,pcCodePage)	\
    ( (This)->lpVtbl -> GetNumberOfCodePageInfo(This,pcCodePage) ) 

#define IMultiLanguage_GetCodePageInfo(This,uiCodePage,pCodePageInfo)	\
    ( (This)->lpVtbl -> GetCodePageInfo(This,uiCodePage,pCodePageInfo) ) 

#define IMultiLanguage_GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage)	\
    ( (This)->lpVtbl -> GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage) ) 

#define IMultiLanguage_EnumCodePages(This,grfFlags,ppEnumCodePage)	\
    ( (This)->lpVtbl -> EnumCodePages(This,grfFlags,ppEnumCodePage) ) 

#define IMultiLanguage_GetCharsetInfo(This,Charset,pCharsetInfo)	\
    ( (This)->lpVtbl -> GetCharsetInfo(This,Charset,pCharsetInfo) ) 

#define IMultiLanguage_IsConvertible(This,dwSrcEncoding,dwDstEncoding)	\
    ( (This)->lpVtbl -> IsConvertible(This,dwSrcEncoding,dwDstEncoding) ) 

#define IMultiLanguage_ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage_ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage_ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage_ConvertStringReset(This)	\
    ( (This)->lpVtbl -> ConvertStringReset(This) ) 

#define IMultiLanguage_GetRfc1766FromLcid(This,Locale,pbstrRfc1766)	\
    ( (This)->lpVtbl -> GetRfc1766FromLcid(This,Locale,pbstrRfc1766) ) 

#define IMultiLanguage_GetLcidFromRfc1766(This,pLocale,bstrRfc1766)	\
    ( (This)->lpVtbl -> GetLcidFromRfc1766(This,pLocale,bstrRfc1766) ) 

#define IMultiLanguage_EnumRfc1766(This,ppEnumRfc1766)	\
    ( (This)->lpVtbl -> EnumRfc1766(This,ppEnumRfc1766) ) 

#define IMultiLanguage_GetRfc1766Info(This,Locale,pRfc1766Info)	\
    ( (This)->lpVtbl -> GetRfc1766Info(This,Locale,pRfc1766Info) ) 

#define IMultiLanguage_CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset)	\
    ( (This)->lpVtbl -> CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiLanguage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mlang_0000_0011 */
/* [local] */ 

// dwfIODControl definitions for ValidateCodePageEx()
#define CPIOD_PEEK          0x40000000L
#define CPIOD_FORCE_PROMPT  0x80000000L


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0011_v0_0_s_ifspec;

#ifndef __IMultiLanguage2_INTERFACE_DEFINED__
#define __IMultiLanguage2_INTERFACE_DEFINED__

/* interface IMultiLanguage2 */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ IMultiLanguage2 *LPMULTILANGUAGE2;

typedef 
enum tagMLDETECTCP
    {
        MLDETECTCP_NONE	= 0,
        MLDETECTCP_7BIT	= 1,
        MLDETECTCP_8BIT	= 2,
        MLDETECTCP_DBCS	= 4,
        MLDETECTCP_HTML	= 8,
        MLDETECTCP_NUMBER	= 16
    } 	MLDETECTCP;

typedef struct tagDetectEncodingInfo
    {
    UINT nLangID;
    UINT nCodePage;
    INT nDocPercent;
    INT nConfidence;
    } 	DetectEncodingInfo;

typedef struct tagDetectEncodingInfo *pDetectEncodingInfo;

typedef 
enum tagSCRIPTFONTCONTF
    {
        SCRIPTCONTF_FIXED_FONT	= 0x1,
        SCRIPTCONTF_PROPORTIONAL_FONT	= 0x2,
        SCRIPTCONTF_SCRIPT_USER	= 0x10000,
        SCRIPTCONTF_SCRIPT_HIDE	= 0x20000,
        SCRIPTCONTF_SCRIPT_SYSTEM	= 0x40000
    } 	SCRIPTFONTCONTF;

typedef struct tagSCRIPFONTINFO
    {
    SCRIPT_IDS scripts;
    WCHAR wszFont[ 32 ];
    } 	SCRIPTFONTINFO;

typedef struct tagSCRIPFONTINFO *PSCRIPTFONTINFO;


EXTERN_C const IID IID_IMultiLanguage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCCFC164-2B38-11d2-B7EC-00C04F8F5D9A")
    IMultiLanguage2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfCodePageInfo( 
            /* [annotation][out] */ 
            _Out_  UINT *pcCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageInfo( 
            /* [in] */ UINT uiCodePage,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO pCodePageInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFamilyCodePage( 
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puiFamilyCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCodePages( 
            /* [in] */ DWORD grfFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumCodePage **ppEnumCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCharsetInfo( 
            /* [in] */ BSTR Charset,
            /* [annotation][out] */ 
            _Out_  PMIMECSETINFO pCharsetInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsConvertible( 
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertString( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringToUnicode( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_opt_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringFromUnicode( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_opt_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringReset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRfc1766FromLcid( 
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLcidFromRfc1766( 
            /* [annotation][out] */ 
            _Out_  LCID *pLocale,
            /* [annotation][in] */ 
            _In_  BSTR bstrRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRfc1766( 
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumRfc1766 **ppEnumRfc1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRfc1766Info( 
            /* [in] */ LCID Locale,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO pRfc1766Info) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateConvertCharset( 
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty,
            /* [annotation][out] */ 
            _Outptr_  IMLangConvertCharset **ppMLangConvertCharset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringInIStream( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][in] */ 
            _In_  IStream *pstmOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringToUnicodeEx( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertStringFromUnicodeEx( 
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetectCodepageInIStream( 
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetectInputCodepage( 
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_  INT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ValidateCodePage( 
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageDescription( 
            /* [in] */ UINT uiCodePage,
            /* [in] */ LCID lcid,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchWideChar)  LPWSTR lpWideCharStr,
            /* [in] */ int cchWideChar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCodePageInstallable( 
            /* [in] */ UINT uiCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMimeDBSource( 
            /* [in] */ MIMECONTF dwSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfScripts( 
            /* [annotation][out] */ 
            _Out_  UINT *pnScripts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumScripts( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumScript **ppEnumScript) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ValidateCodePageEx( 
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd,
            /* [in] */ DWORD dwfIODControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiLanguage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMultiLanguage2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMultiLanguage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMultiLanguage2 * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetNumberOfCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfCodePageInfo )( 
            IMultiLanguage2 * This,
            /* [annotation][out] */ 
            _Out_  UINT *pcCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageInfo )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO pCodePageInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetFamilyCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetFamilyCodePage )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puiFamilyCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumCodePages)
        HRESULT ( STDMETHODCALLTYPE *EnumCodePages )( 
            IMultiLanguage2 * This,
            /* [in] */ DWORD grfFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumCodePage **ppEnumCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCharsetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCharsetInfo )( 
            IMultiLanguage2 * This,
            /* [in] */ BSTR Charset,
            /* [annotation][out] */ 
            _Out_  PMIMECSETINFO pCharsetInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, IsConvertible)
        HRESULT ( STDMETHODCALLTYPE *IsConvertible )( 
            IMultiLanguage2 * This,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertString)
        HRESULT ( STDMETHODCALLTYPE *ConvertString )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringToUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringToUnicode )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_opt_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringFromUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringFromUnicode )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_opt_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringReset)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringReset )( 
            IMultiLanguage2 * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetRfc1766FromLcid)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766FromLcid )( 
            IMultiLanguage2 * This,
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetLcidFromRfc1766)
        HRESULT ( STDMETHODCALLTYPE *GetLcidFromRfc1766 )( 
            IMultiLanguage2 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLocale,
            /* [annotation][in] */ 
            _In_  BSTR bstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumRfc1766)
        HRESULT ( STDMETHODCALLTYPE *EnumRfc1766 )( 
            IMultiLanguage2 * This,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumRfc1766 **ppEnumRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetRfc1766Info)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766Info )( 
            IMultiLanguage2 * This,
            /* [in] */ LCID Locale,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO pRfc1766Info);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, CreateConvertCharset)
        HRESULT ( STDMETHODCALLTYPE *CreateConvertCharset )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty,
            /* [annotation][out] */ 
            _Outptr_  IMLangConvertCharset **ppMLangConvertCharset);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringInIStream)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringInIStream )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][in] */ 
            _In_  IStream *pstmOut);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringToUnicodeEx)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringToUnicodeEx )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringFromUnicodeEx)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringFromUnicodeEx )( 
            IMultiLanguage2 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, DetectCodepageInIStream)
        HRESULT ( STDMETHODCALLTYPE *DetectCodepageInIStream )( 
            IMultiLanguage2 * This,
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, DetectInputCodepage)
        HRESULT ( STDMETHODCALLTYPE *DetectInputCodepage )( 
            IMultiLanguage2 * This,
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_  INT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ValidateCodePage)
        HRESULT ( STDMETHODCALLTYPE *ValidateCodePage )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCodePageDescription)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageDescription )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ LCID lcid,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchWideChar)  LPWSTR lpWideCharStr,
            /* [in] */ int cchWideChar);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, IsCodePageInstallable)
        HRESULT ( STDMETHODCALLTYPE *IsCodePageInstallable )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, SetMimeDBSource)
        HRESULT ( STDMETHODCALLTYPE *SetMimeDBSource )( 
            IMultiLanguage2 * This,
            /* [in] */ MIMECONTF dwSource);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetNumberOfScripts)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfScripts )( 
            IMultiLanguage2 * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnScripts);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumScripts)
        HRESULT ( STDMETHODCALLTYPE *EnumScripts )( 
            IMultiLanguage2 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumScript **ppEnumScript);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ValidateCodePageEx)
        HRESULT ( STDMETHODCALLTYPE *ValidateCodePageEx )( 
            IMultiLanguage2 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd,
            /* [in] */ DWORD dwfIODControl);
        
        END_INTERFACE
    } IMultiLanguage2Vtbl;

    interface IMultiLanguage2
    {
        CONST_VTBL struct IMultiLanguage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiLanguage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiLanguage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiLanguage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiLanguage2_GetNumberOfCodePageInfo(This,pcCodePage)	\
    ( (This)->lpVtbl -> GetNumberOfCodePageInfo(This,pcCodePage) ) 

#define IMultiLanguage2_GetCodePageInfo(This,uiCodePage,LangId,pCodePageInfo)	\
    ( (This)->lpVtbl -> GetCodePageInfo(This,uiCodePage,LangId,pCodePageInfo) ) 

#define IMultiLanguage2_GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage)	\
    ( (This)->lpVtbl -> GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage) ) 

#define IMultiLanguage2_EnumCodePages(This,grfFlags,LangId,ppEnumCodePage)	\
    ( (This)->lpVtbl -> EnumCodePages(This,grfFlags,LangId,ppEnumCodePage) ) 

#define IMultiLanguage2_GetCharsetInfo(This,Charset,pCharsetInfo)	\
    ( (This)->lpVtbl -> GetCharsetInfo(This,Charset,pCharsetInfo) ) 

#define IMultiLanguage2_IsConvertible(This,dwSrcEncoding,dwDstEncoding)	\
    ( (This)->lpVtbl -> IsConvertible(This,dwSrcEncoding,dwDstEncoding) ) 

#define IMultiLanguage2_ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage2_ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage2_ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage2_ConvertStringReset(This)	\
    ( (This)->lpVtbl -> ConvertStringReset(This) ) 

#define IMultiLanguage2_GetRfc1766FromLcid(This,Locale,pbstrRfc1766)	\
    ( (This)->lpVtbl -> GetRfc1766FromLcid(This,Locale,pbstrRfc1766) ) 

#define IMultiLanguage2_GetLcidFromRfc1766(This,pLocale,bstrRfc1766)	\
    ( (This)->lpVtbl -> GetLcidFromRfc1766(This,pLocale,bstrRfc1766) ) 

#define IMultiLanguage2_EnumRfc1766(This,LangId,ppEnumRfc1766)	\
    ( (This)->lpVtbl -> EnumRfc1766(This,LangId,ppEnumRfc1766) ) 

#define IMultiLanguage2_GetRfc1766Info(This,Locale,LangId,pRfc1766Info)	\
    ( (This)->lpVtbl -> GetRfc1766Info(This,Locale,LangId,pRfc1766Info) ) 

#define IMultiLanguage2_CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset)	\
    ( (This)->lpVtbl -> CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset) ) 

#define IMultiLanguage2_ConvertStringInIStream(This,pdwMode,dwFlag,lpFallBack,dwSrcEncoding,dwDstEncoding,pstmIn,pstmOut)	\
    ( (This)->lpVtbl -> ConvertStringInIStream(This,pdwMode,dwFlag,lpFallBack,dwSrcEncoding,dwDstEncoding,pstmIn,pstmOut) ) 

#define IMultiLanguage2_ConvertStringToUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack)	\
    ( (This)->lpVtbl -> ConvertStringToUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack) ) 

#define IMultiLanguage2_ConvertStringFromUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack)	\
    ( (This)->lpVtbl -> ConvertStringFromUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack) ) 

#define IMultiLanguage2_DetectCodepageInIStream(This,dwFlag,dwPrefWinCodePage,pstmIn,lpEncoding,pnScores)	\
    ( (This)->lpVtbl -> DetectCodepageInIStream(This,dwFlag,dwPrefWinCodePage,pstmIn,lpEncoding,pnScores) ) 

#define IMultiLanguage2_DetectInputCodepage(This,dwFlag,dwPrefWinCodePage,pSrcStr,pcSrcSize,lpEncoding,pnScores)	\
    ( (This)->lpVtbl -> DetectInputCodepage(This,dwFlag,dwPrefWinCodePage,pSrcStr,pcSrcSize,lpEncoding,pnScores) ) 

#define IMultiLanguage2_ValidateCodePage(This,uiCodePage,hwnd)	\
    ( (This)->lpVtbl -> ValidateCodePage(This,uiCodePage,hwnd) ) 

#define IMultiLanguage2_GetCodePageDescription(This,uiCodePage,lcid,lpWideCharStr,cchWideChar)	\
    ( (This)->lpVtbl -> GetCodePageDescription(This,uiCodePage,lcid,lpWideCharStr,cchWideChar) ) 

#define IMultiLanguage2_IsCodePageInstallable(This,uiCodePage)	\
    ( (This)->lpVtbl -> IsCodePageInstallable(This,uiCodePage) ) 

#define IMultiLanguage2_SetMimeDBSource(This,dwSource)	\
    ( (This)->lpVtbl -> SetMimeDBSource(This,dwSource) ) 

#define IMultiLanguage2_GetNumberOfScripts(This,pnScripts)	\
    ( (This)->lpVtbl -> GetNumberOfScripts(This,pnScripts) ) 

#define IMultiLanguage2_EnumScripts(This,dwFlags,LangId,ppEnumScript)	\
    ( (This)->lpVtbl -> EnumScripts(This,dwFlags,LangId,ppEnumScript) ) 

#define IMultiLanguage2_ValidateCodePageEx(This,uiCodePage,hwnd,dwfIODControl)	\
    ( (This)->lpVtbl -> ValidateCodePageEx(This,uiCodePage,hwnd,dwfIODControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiLanguage2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mlang_0000_0012 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0012_v0_0_s_ifspec;

#ifndef __IMLangCodePages_INTERFACE_DEFINED__
#define __IMLangCodePages_INTERFACE_DEFINED__

/* interface IMLangCodePages */
/* [object][local][unique][helpstring][uuid] */ 

typedef /* [unique] */ IMLangCodePages *PMLANGCODEPAGES;


EXTERN_C const IID IID_IMLangCodePages;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("359F3443-BD4A-11D0-B188-00AA0038C969")
    IMLangCodePages : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCharCodePages( 
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStrCodePages( 
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ DWORD dwPriorityCodePages,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchCodePages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CodePageToCodePages( 
            /* [in] */ UINT uCodePage,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CodePagesToCodePage( 
            /* [in] */ DWORD dwCodePages,
            /* [in] */ UINT uDefaultCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puCodePage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangCodePagesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangCodePages * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangCodePages * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangCodePages * This);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetCharCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCharCodePages )( 
            IMLangCodePages * This,
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetStrCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStrCodePages )( 
            IMLangCodePages * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ DWORD dwPriorityCodePages,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePageToCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePageToCodePages )( 
            IMLangCodePages * This,
            /* [in] */ UINT uCodePage,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePagesToCodePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePagesToCodePage )( 
            IMLangCodePages * This,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ UINT uDefaultCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puCodePage);
        
        END_INTERFACE
    } IMLangCodePagesVtbl;

    interface IMLangCodePages
    {
        CONST_VTBL struct IMLangCodePagesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangCodePages_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangCodePages_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangCodePages_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangCodePages_GetCharCodePages(This,chSrc,pdwCodePages)	\
    ( (This)->lpVtbl -> GetCharCodePages(This,chSrc,pdwCodePages) ) 

#define IMLangCodePages_GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages)	\
    ( (This)->lpVtbl -> GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages) ) 

#define IMLangCodePages_CodePageToCodePages(This,uCodePage,pdwCodePages)	\
    ( (This)->lpVtbl -> CodePageToCodePages(This,uCodePage,pdwCodePages) ) 

#define IMLangCodePages_CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage)	\
    ( (This)->lpVtbl -> CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangCodePages_INTERFACE_DEFINED__ */


#ifndef __IMLangFontLink_INTERFACE_DEFINED__
#define __IMLangFontLink_INTERFACE_DEFINED__

/* interface IMLangFontLink */
/* [object][local][unique][helpstring][uuid] */ 

typedef /* [unique] */ IMLangFontLink *PMLANGFONTLINK;


EXTERN_C const IID IID_IMLangFontLink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("359F3441-BD4A-11D0-B188-00AA0038C969")
    IMLangFontLink : public IMLangCodePages
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFontCodePages( 
            /* [in] */ HDC hDC,
            /* [in] */ HFONT hFont,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MapFont( 
            /* [in] */ HDC hDC,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ HFONT hSrcFont,
            /* [annotation][out] */ 
            _Out_opt_  HFONT *phDestFont) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReleaseFont( 
            /* [in] */ HFONT hFont) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ResetFontMapping( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangFontLinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangFontLink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangFontLink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangFontLink * This);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetCharCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCharCodePages )( 
            IMLangFontLink * This,
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetStrCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStrCodePages )( 
            IMLangFontLink * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ DWORD dwPriorityCodePages,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePageToCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePageToCodePages )( 
            IMLangFontLink * This,
            /* [in] */ UINT uCodePage,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePagesToCodePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePagesToCodePage )( 
            IMLangFontLink * This,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ UINT uDefaultCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puCodePage);
        
        DECLSPEC_XFGVIRT(IMLangFontLink, GetFontCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFontCodePages )( 
            IMLangFontLink * This,
            /* [in] */ HDC hDC,
            /* [in] */ HFONT hFont,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangFontLink, MapFont)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MapFont )( 
            IMLangFontLink * This,
            /* [in] */ HDC hDC,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ HFONT hSrcFont,
            /* [annotation][out] */ 
            _Out_opt_  HFONT *phDestFont);
        
        DECLSPEC_XFGVIRT(IMLangFontLink, ReleaseFont)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReleaseFont )( 
            IMLangFontLink * This,
            /* [in] */ HFONT hFont);
        
        DECLSPEC_XFGVIRT(IMLangFontLink, ResetFontMapping)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ResetFontMapping )( 
            IMLangFontLink * This);
        
        END_INTERFACE
    } IMLangFontLinkVtbl;

    interface IMLangFontLink
    {
        CONST_VTBL struct IMLangFontLinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangFontLink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangFontLink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangFontLink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangFontLink_GetCharCodePages(This,chSrc,pdwCodePages)	\
    ( (This)->lpVtbl -> GetCharCodePages(This,chSrc,pdwCodePages) ) 

#define IMLangFontLink_GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages)	\
    ( (This)->lpVtbl -> GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages) ) 

#define IMLangFontLink_CodePageToCodePages(This,uCodePage,pdwCodePages)	\
    ( (This)->lpVtbl -> CodePageToCodePages(This,uCodePage,pdwCodePages) ) 

#define IMLangFontLink_CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage)	\
    ( (This)->lpVtbl -> CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage) ) 


#define IMLangFontLink_GetFontCodePages(This,hDC,hFont,pdwCodePages)	\
    ( (This)->lpVtbl -> GetFontCodePages(This,hDC,hFont,pdwCodePages) ) 

#define IMLangFontLink_MapFont(This,hDC,dwCodePages,hSrcFont,phDestFont)	\
    ( (This)->lpVtbl -> MapFont(This,hDC,dwCodePages,hSrcFont,phDestFont) ) 

#define IMLangFontLink_ReleaseFont(This,hFont)	\
    ( (This)->lpVtbl -> ReleaseFont(This,hFont) ) 

#define IMLangFontLink_ResetFontMapping(This)	\
    ( (This)->lpVtbl -> ResetFontMapping(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangFontLink_INTERFACE_DEFINED__ */


#ifndef __IMLangFontLink2_INTERFACE_DEFINED__
#define __IMLangFontLink2_INTERFACE_DEFINED__

/* interface IMLangFontLink2 */
/* [object][local][unique][helpstring][uuid] */ 

typedef struct tagUNICODERANGE
    {
    WCHAR wcFrom;
    WCHAR wcTo;
    } 	UNICODERANGE;

typedef /* [unique] */ IMLangFontLink2 *PMLANGFONTLINK2;


EXTERN_C const IID IID_IMLangFontLink2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCCFC162-2B38-11d2-B7EC-00C04F8F5D9A")
    IMLangFontLink2 : public IMLangCodePages
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFontCodePages( 
            /* [in] */ HDC hDC,
            /* [in] */ HFONT hFont,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReleaseFont( 
            /* [in] */ HFONT hFont) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ResetFontMapping( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MapFont( 
            /* [in] */ HDC hDC,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_opt_  HFONT *pFont) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFontUnicodeRanges( 
            /* [in] */ HDC hDC,
            /* [annotation][out][in] */ 
            _In_  UINT *puiRanges,
            /* [annotation][out] */ 
            _Out_opt_  UNICODERANGE *pUranges) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetScriptFontInfo( 
            /* [in] */ SCRIPT_ID sid,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out][in] */ 
            _Inout_  UINT *puiFonts,
            /* [annotation][out] */ 
            _Out_opt_  SCRIPTFONTINFO *pScriptFont) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CodePageToScriptID( 
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  SCRIPT_ID *pSid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMLangFontLink2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMLangFontLink2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMLangFontLink2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMLangFontLink2 * This);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetCharCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCharCodePages )( 
            IMLangFontLink2 * This,
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, GetStrCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStrCodePages )( 
            IMLangFontLink2 * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchSrc)  const WCHAR *pszSrc,
            /* [in] */ long cchSrc,
            /* [in] */ DWORD dwPriorityCodePages,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages,
            /* [annotation][out] */ 
            _Out_opt_  long *pcchCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePageToCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePageToCodePages )( 
            IMLangFontLink2 * This,
            /* [in] */ UINT uCodePage,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangCodePages, CodePagesToCodePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePagesToCodePage )( 
            IMLangFontLink2 * This,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ UINT uDefaultCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puCodePage);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, GetFontCodePages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFontCodePages )( 
            IMLangFontLink2 * This,
            /* [in] */ HDC hDC,
            /* [in] */ HFONT hFont,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwCodePages);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, ReleaseFont)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReleaseFont )( 
            IMLangFontLink2 * This,
            /* [in] */ HFONT hFont);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, ResetFontMapping)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ResetFontMapping )( 
            IMLangFontLink2 * This);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, MapFont)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MapFont )( 
            IMLangFontLink2 * This,
            /* [in] */ HDC hDC,
            /* [in] */ DWORD dwCodePages,
            /* [in] */ WCHAR chSrc,
            /* [annotation][out] */ 
            _Out_opt_  HFONT *pFont);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, GetFontUnicodeRanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFontUnicodeRanges )( 
            IMLangFontLink2 * This,
            /* [in] */ HDC hDC,
            /* [annotation][out][in] */ 
            _In_  UINT *puiRanges,
            /* [annotation][out] */ 
            _Out_opt_  UNICODERANGE *pUranges);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, GetScriptFontInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetScriptFontInfo )( 
            IMLangFontLink2 * This,
            /* [in] */ SCRIPT_ID sid,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out][in] */ 
            _Inout_  UINT *puiFonts,
            /* [annotation][out] */ 
            _Out_opt_  SCRIPTFONTINFO *pScriptFont);
        
        DECLSPEC_XFGVIRT(IMLangFontLink2, CodePageToScriptID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CodePageToScriptID )( 
            IMLangFontLink2 * This,
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  SCRIPT_ID *pSid);
        
        END_INTERFACE
    } IMLangFontLink2Vtbl;

    interface IMLangFontLink2
    {
        CONST_VTBL struct IMLangFontLink2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMLangFontLink2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMLangFontLink2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMLangFontLink2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMLangFontLink2_GetCharCodePages(This,chSrc,pdwCodePages)	\
    ( (This)->lpVtbl -> GetCharCodePages(This,chSrc,pdwCodePages) ) 

#define IMLangFontLink2_GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages)	\
    ( (This)->lpVtbl -> GetStrCodePages(This,pszSrc,cchSrc,dwPriorityCodePages,pdwCodePages,pcchCodePages) ) 

#define IMLangFontLink2_CodePageToCodePages(This,uCodePage,pdwCodePages)	\
    ( (This)->lpVtbl -> CodePageToCodePages(This,uCodePage,pdwCodePages) ) 

#define IMLangFontLink2_CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage)	\
    ( (This)->lpVtbl -> CodePagesToCodePage(This,dwCodePages,uDefaultCodePage,puCodePage) ) 


#define IMLangFontLink2_GetFontCodePages(This,hDC,hFont,pdwCodePages)	\
    ( (This)->lpVtbl -> GetFontCodePages(This,hDC,hFont,pdwCodePages) ) 

#define IMLangFontLink2_ReleaseFont(This,hFont)	\
    ( (This)->lpVtbl -> ReleaseFont(This,hFont) ) 

#define IMLangFontLink2_ResetFontMapping(This)	\
    ( (This)->lpVtbl -> ResetFontMapping(This) ) 

#define IMLangFontLink2_MapFont(This,hDC,dwCodePages,chSrc,pFont)	\
    ( (This)->lpVtbl -> MapFont(This,hDC,dwCodePages,chSrc,pFont) ) 

#define IMLangFontLink2_GetFontUnicodeRanges(This,hDC,puiRanges,pUranges)	\
    ( (This)->lpVtbl -> GetFontUnicodeRanges(This,hDC,puiRanges,pUranges) ) 

#define IMLangFontLink2_GetScriptFontInfo(This,sid,dwFlags,puiFonts,pScriptFont)	\
    ( (This)->lpVtbl -> GetScriptFontInfo(This,sid,dwFlags,puiFonts,pScriptFont) ) 

#define IMLangFontLink2_CodePageToScriptID(This,uiCodePage,pSid)	\
    ( (This)->lpVtbl -> CodePageToScriptID(This,uiCodePage,pSid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMLangFontLink2_INTERFACE_DEFINED__ */


#ifndef __IMultiLanguage3_INTERFACE_DEFINED__
#define __IMultiLanguage3_INTERFACE_DEFINED__

/* interface IMultiLanguage3 */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ IMultiLanguage3 *LPMULTILANGUAGE3;


EXTERN_C const IID IID_IMultiLanguage3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4e5868ab-b157-4623-9acc-6a1d9caebe04")
    IMultiLanguage3 : public IMultiLanguage2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DetectOutboundCodePage( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWideChar)  LPCWSTR lpWideCharStr,
            /* [in] */ UINT cchWideChar,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(nPreferredCodePages)  const UINT *puiPreferredCodePages,
            /* [in] */ UINT nPreferredCodePages,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(*pnDetectedCodePages, *pnDetectedCodePages)  UINT *puiDetectedCodePages,
            /* [annotation][out][in] */ 
            _Inout_  UINT *pnDetectedCodePages,
            /* [annotation][string][unique][in] */ 
            _In_opt_  LPCWSTR lpSpecialChar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetectOutboundCodePageInIStream( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IStream *pStrIn,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(nPreferredCodePages)  const UINT *puiPreferredCodePages,
            /* [in] */ UINT nPreferredCodePages,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(*pnDetectedCodePages, *pnDetectedCodePages)  UINT *puiDetectedCodePages,
            /* [annotation][out][in] */ 
            _Inout_  UINT *pnDetectedCodePages,
            /* [annotation][string][unique][in] */ 
            _In_opt_  LPCWSTR lpSpecialChar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiLanguage3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMultiLanguage3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMultiLanguage3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMultiLanguage3 * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetNumberOfCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfCodePageInfo )( 
            IMultiLanguage3 * This,
            /* [annotation][out] */ 
            _Out_  UINT *pcCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCodePageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageInfo )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PMIMECPINFO pCodePageInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetFamilyCodePage)
        HRESULT ( STDMETHODCALLTYPE *GetFamilyCodePage )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage,
            /* [annotation][out] */ 
            _Out_  UINT *puiFamilyCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumCodePages)
        HRESULT ( STDMETHODCALLTYPE *EnumCodePages )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD grfFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumCodePage **ppEnumCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCharsetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCharsetInfo )( 
            IMultiLanguage3 * This,
            /* [in] */ BSTR Charset,
            /* [annotation][out] */ 
            _Out_  PMIMECSETINFO pCharsetInfo);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, IsConvertible)
        HRESULT ( STDMETHODCALLTYPE *IsConvertible )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertString)
        HRESULT ( STDMETHODCALLTYPE *ConvertString )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  BYTE *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  BYTE *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringToUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringToUnicode )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_opt_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_opt_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringFromUnicode)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringFromUnicode )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_opt_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringReset)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringReset )( 
            IMultiLanguage3 * This);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetRfc1766FromLcid)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766FromLcid )( 
            IMultiLanguage3 * This,
            /* [in] */ LCID Locale,
            /* [annotation][out] */ 
            _Outptr_  BSTR *pbstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetLcidFromRfc1766)
        HRESULT ( STDMETHODCALLTYPE *GetLcidFromRfc1766 )( 
            IMultiLanguage3 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLocale,
            /* [annotation][in] */ 
            _In_  BSTR bstrRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumRfc1766)
        HRESULT ( STDMETHODCALLTYPE *EnumRfc1766 )( 
            IMultiLanguage3 * This,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumRfc1766 **ppEnumRfc1766);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetRfc1766Info)
        HRESULT ( STDMETHODCALLTYPE *GetRfc1766Info )( 
            IMultiLanguage3 * This,
            /* [in] */ LCID Locale,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Out_  PRFC1766INFO pRfc1766Info);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, CreateConvertCharset)
        HRESULT ( STDMETHODCALLTYPE *CreateConvertCharset )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiSrcCodePage,
            /* [in] */ UINT uiDstCodePage,
            /* [in] */ DWORD dwProperty,
            /* [annotation][out] */ 
            _Outptr_  IMLangConvertCharset **ppMLangConvertCharset);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringInIStream)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringInIStream )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack,
            /* [in] */ DWORD dwSrcEncoding,
            /* [in] */ DWORD dwDstEncoding,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][in] */ 
            _In_  IStream *pstmOut);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringToUnicodeEx)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringToUnicodeEx )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_(*pcDstSize)  WCHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ConvertStringFromUnicodeEx)
        HRESULT ( STDMETHODCALLTYPE *ConvertStringFromUnicodeEx )( 
            IMultiLanguage3 * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pdwMode,
            /* [in] */ DWORD dwEncoding,
            /* [annotation][in] */ 
            _In_reads_(*pcSrcSize)  WCHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_writes_bytes_(*pcDstSize)  CHAR *pDstStr,
            /* [annotation][out][in] */ 
            _Inout_opt_  UINT *pcDstSize,
            /* [in] */ DWORD dwFlag,
            /* [annotation][in] */ 
            _In_opt_  WCHAR *lpFallBack);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, DetectCodepageInIStream)
        HRESULT ( STDMETHODCALLTYPE *DetectCodepageInIStream )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_  IStream *pstmIn,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, DetectInputCodepage)
        HRESULT ( STDMETHODCALLTYPE *DetectInputCodepage )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwFlag,
            /* [in] */ DWORD dwPrefWinCodePage,
            /* [annotation][in] */ 
            _In_reads_bytes_(*pcSrcSize)  CHAR *pSrcStr,
            /* [annotation][out][in] */ 
            _Inout_  INT *pcSrcSize,
            /* [annotation][out] */ 
            _Out_  DetectEncodingInfo *lpEncoding,
            /* [annotation][out][in] */ 
            _Inout_  INT *pnScores);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ValidateCodePage)
        HRESULT ( STDMETHODCALLTYPE *ValidateCodePage )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetCodePageDescription)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageDescription )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ LCID lcid,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchWideChar)  LPWSTR lpWideCharStr,
            /* [in] */ int cchWideChar);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, IsCodePageInstallable)
        HRESULT ( STDMETHODCALLTYPE *IsCodePageInstallable )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, SetMimeDBSource)
        HRESULT ( STDMETHODCALLTYPE *SetMimeDBSource )( 
            IMultiLanguage3 * This,
            /* [in] */ MIMECONTF dwSource);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, GetNumberOfScripts)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfScripts )( 
            IMultiLanguage3 * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnScripts);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, EnumScripts)
        HRESULT ( STDMETHODCALLTYPE *EnumScripts )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LANGID LangId,
            /* [annotation][out] */ 
            _Outptr_  IEnumScript **ppEnumScript);
        
        DECLSPEC_XFGVIRT(IMultiLanguage2, ValidateCodePageEx)
        HRESULT ( STDMETHODCALLTYPE *ValidateCodePageEx )( 
            IMultiLanguage3 * This,
            /* [in] */ UINT uiCodePage,
            /* [in] */ HWND hwnd,
            /* [in] */ DWORD dwfIODControl);
        
        DECLSPEC_XFGVIRT(IMultiLanguage3, DetectOutboundCodePage)
        HRESULT ( STDMETHODCALLTYPE *DetectOutboundCodePage )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][size_is][in] */ 
            _In_reads_(cchWideChar)  LPCWSTR lpWideCharStr,
            /* [in] */ UINT cchWideChar,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(nPreferredCodePages)  const UINT *puiPreferredCodePages,
            /* [in] */ UINT nPreferredCodePages,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(*pnDetectedCodePages, *pnDetectedCodePages)  UINT *puiDetectedCodePages,
            /* [annotation][out][in] */ 
            _Inout_  UINT *pnDetectedCodePages,
            /* [annotation][string][unique][in] */ 
            _In_opt_  LPCWSTR lpSpecialChar);
        
        DECLSPEC_XFGVIRT(IMultiLanguage3, DetectOutboundCodePageInIStream)
        HRESULT ( STDMETHODCALLTYPE *DetectOutboundCodePageInIStream )( 
            IMultiLanguage3 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IStream *pStrIn,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(nPreferredCodePages)  const UINT *puiPreferredCodePages,
            /* [in] */ UINT nPreferredCodePages,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(*pnDetectedCodePages, *pnDetectedCodePages)  UINT *puiDetectedCodePages,
            /* [annotation][out][in] */ 
            _Inout_  UINT *pnDetectedCodePages,
            /* [annotation][string][unique][in] */ 
            _In_opt_  LPCWSTR lpSpecialChar);
        
        END_INTERFACE
    } IMultiLanguage3Vtbl;

    interface IMultiLanguage3
    {
        CONST_VTBL struct IMultiLanguage3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiLanguage3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiLanguage3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiLanguage3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiLanguage3_GetNumberOfCodePageInfo(This,pcCodePage)	\
    ( (This)->lpVtbl -> GetNumberOfCodePageInfo(This,pcCodePage) ) 

#define IMultiLanguage3_GetCodePageInfo(This,uiCodePage,LangId,pCodePageInfo)	\
    ( (This)->lpVtbl -> GetCodePageInfo(This,uiCodePage,LangId,pCodePageInfo) ) 

#define IMultiLanguage3_GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage)	\
    ( (This)->lpVtbl -> GetFamilyCodePage(This,uiCodePage,puiFamilyCodePage) ) 

#define IMultiLanguage3_EnumCodePages(This,grfFlags,LangId,ppEnumCodePage)	\
    ( (This)->lpVtbl -> EnumCodePages(This,grfFlags,LangId,ppEnumCodePage) ) 

#define IMultiLanguage3_GetCharsetInfo(This,Charset,pCharsetInfo)	\
    ( (This)->lpVtbl -> GetCharsetInfo(This,Charset,pCharsetInfo) ) 

#define IMultiLanguage3_IsConvertible(This,dwSrcEncoding,dwDstEncoding)	\
    ( (This)->lpVtbl -> IsConvertible(This,dwSrcEncoding,dwDstEncoding) ) 

#define IMultiLanguage3_ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertString(This,pdwMode,dwSrcEncoding,dwDstEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage3_ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringToUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage3_ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize)	\
    ( (This)->lpVtbl -> ConvertStringFromUnicode(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize) ) 

#define IMultiLanguage3_ConvertStringReset(This)	\
    ( (This)->lpVtbl -> ConvertStringReset(This) ) 

#define IMultiLanguage3_GetRfc1766FromLcid(This,Locale,pbstrRfc1766)	\
    ( (This)->lpVtbl -> GetRfc1766FromLcid(This,Locale,pbstrRfc1766) ) 

#define IMultiLanguage3_GetLcidFromRfc1766(This,pLocale,bstrRfc1766)	\
    ( (This)->lpVtbl -> GetLcidFromRfc1766(This,pLocale,bstrRfc1766) ) 

#define IMultiLanguage3_EnumRfc1766(This,LangId,ppEnumRfc1766)	\
    ( (This)->lpVtbl -> EnumRfc1766(This,LangId,ppEnumRfc1766) ) 

#define IMultiLanguage3_GetRfc1766Info(This,Locale,LangId,pRfc1766Info)	\
    ( (This)->lpVtbl -> GetRfc1766Info(This,Locale,LangId,pRfc1766Info) ) 

#define IMultiLanguage3_CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset)	\
    ( (This)->lpVtbl -> CreateConvertCharset(This,uiSrcCodePage,uiDstCodePage,dwProperty,ppMLangConvertCharset) ) 

#define IMultiLanguage3_ConvertStringInIStream(This,pdwMode,dwFlag,lpFallBack,dwSrcEncoding,dwDstEncoding,pstmIn,pstmOut)	\
    ( (This)->lpVtbl -> ConvertStringInIStream(This,pdwMode,dwFlag,lpFallBack,dwSrcEncoding,dwDstEncoding,pstmIn,pstmOut) ) 

#define IMultiLanguage3_ConvertStringToUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack)	\
    ( (This)->lpVtbl -> ConvertStringToUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack) ) 

#define IMultiLanguage3_ConvertStringFromUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack)	\
    ( (This)->lpVtbl -> ConvertStringFromUnicodeEx(This,pdwMode,dwEncoding,pSrcStr,pcSrcSize,pDstStr,pcDstSize,dwFlag,lpFallBack) ) 

#define IMultiLanguage3_DetectCodepageInIStream(This,dwFlag,dwPrefWinCodePage,pstmIn,lpEncoding,pnScores)	\
    ( (This)->lpVtbl -> DetectCodepageInIStream(This,dwFlag,dwPrefWinCodePage,pstmIn,lpEncoding,pnScores) ) 

#define IMultiLanguage3_DetectInputCodepage(This,dwFlag,dwPrefWinCodePage,pSrcStr,pcSrcSize,lpEncoding,pnScores)	\
    ( (This)->lpVtbl -> DetectInputCodepage(This,dwFlag,dwPrefWinCodePage,pSrcStr,pcSrcSize,lpEncoding,pnScores) ) 

#define IMultiLanguage3_ValidateCodePage(This,uiCodePage,hwnd)	\
    ( (This)->lpVtbl -> ValidateCodePage(This,uiCodePage,hwnd) ) 

#define IMultiLanguage3_GetCodePageDescription(This,uiCodePage,lcid,lpWideCharStr,cchWideChar)	\
    ( (This)->lpVtbl -> GetCodePageDescription(This,uiCodePage,lcid,lpWideCharStr,cchWideChar) ) 

#define IMultiLanguage3_IsCodePageInstallable(This,uiCodePage)	\
    ( (This)->lpVtbl -> IsCodePageInstallable(This,uiCodePage) ) 

#define IMultiLanguage3_SetMimeDBSource(This,dwSource)	\
    ( (This)->lpVtbl -> SetMimeDBSource(This,dwSource) ) 

#define IMultiLanguage3_GetNumberOfScripts(This,pnScripts)	\
    ( (This)->lpVtbl -> GetNumberOfScripts(This,pnScripts) ) 

#define IMultiLanguage3_EnumScripts(This,dwFlags,LangId,ppEnumScript)	\
    ( (This)->lpVtbl -> EnumScripts(This,dwFlags,LangId,ppEnumScript) ) 

#define IMultiLanguage3_ValidateCodePageEx(This,uiCodePage,hwnd,dwfIODControl)	\
    ( (This)->lpVtbl -> ValidateCodePageEx(This,uiCodePage,hwnd,dwfIODControl) ) 


#define IMultiLanguage3_DetectOutboundCodePage(This,dwFlags,lpWideCharStr,cchWideChar,puiPreferredCodePages,nPreferredCodePages,puiDetectedCodePages,pnDetectedCodePages,lpSpecialChar)	\
    ( (This)->lpVtbl -> DetectOutboundCodePage(This,dwFlags,lpWideCharStr,cchWideChar,puiPreferredCodePages,nPreferredCodePages,puiDetectedCodePages,pnDetectedCodePages,lpSpecialChar) ) 

#define IMultiLanguage3_DetectOutboundCodePageInIStream(This,dwFlags,pStrIn,puiPreferredCodePages,nPreferredCodePages,puiDetectedCodePages,pnDetectedCodePages,lpSpecialChar)	\
    ( (This)->lpVtbl -> DetectOutboundCodePageInIStream(This,dwFlags,pStrIn,puiPreferredCodePages,nPreferredCodePages,puiDetectedCodePages,pnDetectedCodePages,lpSpecialChar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiLanguage3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mlang_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0016_v0_0_s_ifspec;


#ifndef __MultiLanguage_LIBRARY_DEFINED__
#define __MultiLanguage_LIBRARY_DEFINED__

/* library MultiLanguage */
/* [version][lcid][helpstring][uuid] */ 

typedef WORD LANGID;

typedef 
enum tagMLSTR_FLAGS
    {
        MLSTR_READ	= 1,
        MLSTR_WRITE	= 2
    } 	MLSTR_FLAGS;


EXTERN_C const IID LIBID_MultiLanguage;

EXTERN_C const CLSID CLSID_CMLangString;

#ifdef __cplusplus

class DECLSPEC_UUID("C04D65CF-B70D-11D0-B188-00AA0038C969")
CMLangString;
#endif

EXTERN_C const CLSID CLSID_CMLangConvertCharset;

#ifdef __cplusplus

class DECLSPEC_UUID("d66d6f99-cdaa-11d0-b822-00c04fc9b31f")
CMLangConvertCharset;
#endif

EXTERN_C const CLSID CLSID_CMultiLanguage;

#ifdef __cplusplus

class DECLSPEC_UUID("275c23e2-3747-11d0-9fea-00aa003f8646")
CMultiLanguage;
#endif
#endif /* __MultiLanguage_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mlang_0000_0017 */
/* [local] */ 

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _MLANG_H_API_DEF_                                                     
#define _MLANG_H_API_DEF_                                                     
                                                                              
// APIs prototypes                                                            
STDAPI LcidToRfc1766A(_In_ LCID Locale, _Out_writes_bytes_(iMaxLength) LPSTR pszRfc1766, _In_ int iMaxLength);         
STDAPI LcidToRfc1766W(_In_ LCID Locale, _Out_writes_(nChar) LPWSTR pwszRfc1766, _In_ int nChar);             
#ifdef UNICODE                                                                
#define LcidToRfc1766        LcidToRfc1766W                                   
#else                                                                         
#define LcidToRfc1766        LcidToRfc1766A                                   
#endif                                                                        
STDAPI Rfc1766ToLcidA(_Out_ LCID *pLocale, _In_ LPCSTR pszRfc1766);           
STDAPI Rfc1766ToLcidW(_Out_ LCID *pLocale, _In_ LPCWSTR pszRfc1766);          
#ifdef UNICODE                                                                
#define Rfc1766ToLcid        Rfc1766ToLcidW                                   
#else                                                                         
#define Rfc1766ToLcid        Rfc1766ToLcidA                                   
#endif                                                                        
                                                                              
STDAPI IsConvertINetStringAvailable(DWORD dwSrcEncoding, DWORD dwDstEncoding);
STDAPI ConvertINetString(_Inout_opt_ LPDWORD lpdwMode, _In_ DWORD dwSrcEncoding, _In_ DWORD dwDstEncoding, _In_reads_bytes_opt_(*lpnSrcSize) LPCSTR lpSrcStr, _Inout_opt_ LPINT lpnSrcSize, _Out_writes_bytes_to_opt_(*lpnDstSize, *lpnDstSize) LPSTR lpDstStr, _Inout_opt_ LPINT lpnDstSize); 
STDAPI ConvertINetMultiByteToUnicode(_Inout_opt_ LPDWORD lpdwMode, _In_ DWORD dwEncoding, _In_reads_bytes_opt_(*lpnMultiCharCount) LPCSTR lpSrcStr, _Inout_opt_ LPINT lpnMultiCharCount, _Out_writes_to_opt_(*lpnWideCharCount, *lpnWideCharCount) LPWSTR lpDstStr, _Inout_opt_ LPINT lpnWideCharCount); 
STDAPI ConvertINetUnicodeToMultiByte(_Inout_opt_ LPDWORD lpdwMode, _In_ DWORD dwEncoding, _In_reads_opt_(*lpnWideCharCount) LPCWSTR lpSrcStr, _Inout_opt_ LPINT lpnWideCharCount, _Out_writes_bytes_to_opt_(*lpnMultiCharCount, *lpnMultiCharCount) LPSTR lpDstStr, _Inout_opt_ LPINT lpnMultiCharCount); 
                                                                              
#endif // _MLANG_H_API_DEF_                                                   
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mlang_0000_0017_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


