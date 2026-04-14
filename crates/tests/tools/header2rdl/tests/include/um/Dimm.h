

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


#ifndef __dimm_h__
#define __dimm_h__

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

#ifndef __IEnumRegisterWordA_FWD_DEFINED__
#define __IEnumRegisterWordA_FWD_DEFINED__
typedef interface IEnumRegisterWordA IEnumRegisterWordA;

#endif 	/* __IEnumRegisterWordA_FWD_DEFINED__ */


#ifndef __IEnumRegisterWordW_FWD_DEFINED__
#define __IEnumRegisterWordW_FWD_DEFINED__
typedef interface IEnumRegisterWordW IEnumRegisterWordW;

#endif 	/* __IEnumRegisterWordW_FWD_DEFINED__ */


#ifndef __IEnumInputContext_FWD_DEFINED__
#define __IEnumInputContext_FWD_DEFINED__
typedef interface IEnumInputContext IEnumInputContext;

#endif 	/* __IEnumInputContext_FWD_DEFINED__ */


#ifndef __IActiveIMMRegistrar_FWD_DEFINED__
#define __IActiveIMMRegistrar_FWD_DEFINED__
typedef interface IActiveIMMRegistrar IActiveIMMRegistrar;

#endif 	/* __IActiveIMMRegistrar_FWD_DEFINED__ */


#ifndef __IActiveIMMMessagePumpOwner_FWD_DEFINED__
#define __IActiveIMMMessagePumpOwner_FWD_DEFINED__
typedef interface IActiveIMMMessagePumpOwner IActiveIMMMessagePumpOwner;

#endif 	/* __IActiveIMMMessagePumpOwner_FWD_DEFINED__ */


#ifndef __IActiveIMMApp_FWD_DEFINED__
#define __IActiveIMMApp_FWD_DEFINED__
typedef interface IActiveIMMApp IActiveIMMApp;

#endif 	/* __IActiveIMMApp_FWD_DEFINED__ */


#ifndef __IActiveIMMIME_FWD_DEFINED__
#define __IActiveIMMIME_FWD_DEFINED__
typedef interface IActiveIMMIME IActiveIMMIME;

#endif 	/* __IActiveIMMIME_FWD_DEFINED__ */


#ifndef __IActiveIME_FWD_DEFINED__
#define __IActiveIME_FWD_DEFINED__
typedef interface IActiveIME IActiveIME;

#endif 	/* __IActiveIME_FWD_DEFINED__ */


#ifndef __IActiveIME2_FWD_DEFINED__
#define __IActiveIME2_FWD_DEFINED__
typedef interface IActiveIME2 IActiveIME2;

#endif 	/* __IActiveIME2_FWD_DEFINED__ */


#ifndef __CActiveIMM_FWD_DEFINED__
#define __CActiveIMM_FWD_DEFINED__

#ifdef __cplusplus
typedef class CActiveIMM CActiveIMM;
#else
typedef struct CActiveIMM CActiveIMM;
#endif /* __cplusplus */

#endif 	/* __CActiveIMM_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dimm_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// dimm.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//--------------------------------------------------------------------------
// IActiveIMM Interfaces.

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



extern RPC_IF_HANDLE __MIDL_itf_dimm_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dimm_0000_0000_v0_0_s_ifspec;


#ifndef __ActiveIMM_LIBRARY_DEFINED__
#define __ActiveIMM_LIBRARY_DEFINED__

/* library ActiveIMM */
/* [version][lcid][helpstring][uuid] */ 

#include <imm.h>
#if 0
typedef WORD LANGID;

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0001
    {
    LPSTR lpReading;
    LPSTR lpWord;
    } 	REGISTERWORDA;

typedef /* [public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0002
    {
    LPWSTR lpReading;
    LPWSTR lpWord;
    } 	REGISTERWORDW;

typedef /* [public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0003
    {
    LONG lfHeight;
    LONG lfWidth;
    LONG lfEscapement;
    LONG lfOrientation;
    LONG lfWeight;
    BYTE lfItalic;
    BYTE lfUnderline;
    BYTE lfStrikeOut;
    BYTE lfCharSet;
    BYTE lfOutPrecision;
    BYTE lfClipPrecision;
    BYTE lfQuality;
    BYTE lfPitchAndFamily;
    CHAR lfFaceName[ 32 ];
    } 	LOGFONTA;

typedef /* [public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0004
    {
    LONG lfHeight;
    LONG lfWidth;
    LONG lfEscapement;
    LONG lfOrientation;
    LONG lfWeight;
    BYTE lfItalic;
    BYTE lfUnderline;
    BYTE lfStrikeOut;
    BYTE lfCharSet;
    BYTE lfOutPrecision;
    BYTE lfClipPrecision;
    BYTE lfQuality;
    BYTE lfPitchAndFamily;
    WCHAR lfFaceName[ 32 ];
    } 	LOGFONTW;

typedef DWORD HIMC;

typedef DWORD HIMCC;

typedef /* [public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0005
    {
    DWORD dwIndex;
    DWORD dwStyle;
    POINT ptCurrentPos;
    RECT rcArea;
    } 	CANDIDATEFORM;

typedef /* [public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0006
    {
    DWORD dwStyle;
    POINT ptCurrentPos;
    RECT rcArea;
    } 	COMPOSITIONFORM;

typedef /* [public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0007
    {
    DWORD dwSize;
    DWORD dwStyle;
    DWORD dwCount;
    DWORD dwSelection;
    DWORD dwPageStart;
    DWORD dwPageSize;
    DWORD dwOffset[ 1 ];
    } 	CANDIDATELIST;

typedef /* [public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0008
    {
    DWORD dwStyle;
    CHAR szDescription[ 32 ];
    } 	STYLEBUFA;

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0009
    {
    DWORD dwStyle;
    WCHAR szDescription[ 32 ];
    } 	STYLEBUFW;

typedef WORD ATOM;

#endif
#if (WINVER < 0x040A)
typedef /* [public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0010
    {
    UINT cbSize;
    UINT fType;
    UINT fState;
    UINT wID;
    HBITMAP hbmpChecked;
    HBITMAP hbmpUnchecked;
    DWORD dwItemData;
    CHAR szString[ 80 ];
    HBITMAP hbmpItem;
    } 	IMEMENUITEMINFOA;

typedef /* [public][public][public][public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0011
    {
    UINT cbSize;
    UINT fType;
    UINT fState;
    UINT wID;
    HBITMAP hbmpChecked;
    HBITMAP hbmpUnchecked;
    DWORD dwItemData;
    WCHAR szString[ 80 ];
    HBITMAP hbmpItem;
    } 	IMEMENUITEMINFOW;

#endif
#ifndef _DDKIMM_H_
typedef /* [public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0012
    {
    HWND hWnd;
    BOOL fOpen;
    POINT ptStatusWndPos;
    POINT ptSoftKbdPos;
    DWORD fdwConversion;
    DWORD fdwSentence;
    union 
        {
        LOGFONTA A;
        LOGFONTW W;
        } 	lfFont;
    COMPOSITIONFORM cfCompForm;
    CANDIDATEFORM cfCandForm[ 4 ];
    HIMCC hCompStr;
    HIMCC hCandInfo;
    HIMCC hGuideLine;
    HIMCC hPrivate;
    DWORD dwNumMsgBuf;
    HIMCC hMsgBuf;
    DWORD fdwInit;
    DWORD dwReserve[ 3 ];
    } 	INPUTCONTEXT;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_dimm_0000_0000_0014
    {
    DWORD dwPrivateDataSize;
    DWORD fdwProperty;
    DWORD fdwConversionCaps;
    DWORD fdwSentenceCaps;
    DWORD fdwUICaps;
    DWORD fdwSCSCaps;
    DWORD fdwSelectCaps;
    } 	IMEINFO;

#endif

EXTERN_C const IID LIBID_ActiveIMM;

#ifndef __IEnumRegisterWordA_INTERFACE_DEFINED__
#define __IEnumRegisterWordA_INTERFACE_DEFINED__

/* interface IEnumRegisterWordA */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumRegisterWordA;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08C03412-F96B-11d0-A475-00AA006BCC59")
    IEnumRegisterWordA : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out REGISTERWORDA *rgRegisterWord,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRegisterWordAVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumRegisterWordA * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumRegisterWordA * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumRegisterWordA * This);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordA, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumRegisterWordA * This,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordA, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumRegisterWordA * This,
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out REGISTERWORDA *rgRegisterWord,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordA, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumRegisterWordA * This);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordA, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumRegisterWordA * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumRegisterWordAVtbl;

    interface IEnumRegisterWordA
    {
        CONST_VTBL struct IEnumRegisterWordAVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRegisterWordA_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRegisterWordA_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRegisterWordA_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRegisterWordA_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumRegisterWordA_Next(This,ulCount,rgRegisterWord,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgRegisterWord,pcFetched) ) 

#define IEnumRegisterWordA_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRegisterWordA_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumRegisterWordA_INTERFACE_DEFINED__ */


#ifndef __IEnumRegisterWordW_INTERFACE_DEFINED__
#define __IEnumRegisterWordW_INTERFACE_DEFINED__

/* interface IEnumRegisterWordW */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumRegisterWordW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4955DD31-B159-11d0-8FCF-00AA006BCC59")
    IEnumRegisterWordW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out REGISTERWORDW *rgRegisterWord,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRegisterWordWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumRegisterWordW * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumRegisterWordW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumRegisterWordW * This);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordW, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumRegisterWordW * This,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordW, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumRegisterWordW * This,
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out REGISTERWORDW *rgRegisterWord,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordW, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumRegisterWordW * This);
        
        DECLSPEC_XFGVIRT(IEnumRegisterWordW, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumRegisterWordW * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumRegisterWordWVtbl;

    interface IEnumRegisterWordW
    {
        CONST_VTBL struct IEnumRegisterWordWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRegisterWordW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRegisterWordW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRegisterWordW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRegisterWordW_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumRegisterWordW_Next(This,ulCount,rgRegisterWord,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgRegisterWord,pcFetched) ) 

#define IEnumRegisterWordW_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRegisterWordW_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumRegisterWordW_INTERFACE_DEFINED__ */


#ifndef __IEnumInputContext_INTERFACE_DEFINED__
#define __IEnumInputContext_INTERFACE_DEFINED__

/* interface IEnumInputContext */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumInputContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09b5eab0-f997-11d1-93d4-0060b067b86e")
    IEnumInputContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out HIMC *rgInputContext,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumInputContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumInputContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumInputContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumInputContext * This);
        
        DECLSPEC_XFGVIRT(IEnumInputContext, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumInputContext * This,
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumInputContext, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumInputContext * This,
            /* [in] */ ULONG ulCount,
            /* [out] */ __RPC__out HIMC *rgInputContext,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumInputContext, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumInputContext * This);
        
        DECLSPEC_XFGVIRT(IEnumInputContext, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumInputContext * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumInputContextVtbl;

    interface IEnumInputContext
    {
        CONST_VTBL struct IEnumInputContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumInputContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumInputContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumInputContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumInputContext_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumInputContext_Next(This,ulCount,rgInputContext,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgInputContext,pcFetched) ) 

#define IEnumInputContext_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumInputContext_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumInputContext_INTERFACE_DEFINED__ */


#ifndef __IActiveIMMRegistrar_INTERFACE_DEFINED__
#define __IActiveIMMRegistrar_INTERFACE_DEFINED__

/* interface IActiveIMMRegistrar */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIMMRegistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b3458082-bd00-11d1-939b-0060b067b86e")
    IActiveIMMRegistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterIME( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID lgid,
            /* [in] */ __RPC__in LPCWSTR pszIconFile,
            /* [in] */ __RPC__in LPCWSTR pszDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterIME( 
            /* [in] */ __RPC__in REFCLSID rclsid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIMMRegistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIMMRegistrar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIMMRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIMMRegistrar * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMRegistrar, RegisterIME)
        HRESULT ( STDMETHODCALLTYPE *RegisterIME )( 
            __RPC__in IActiveIMMRegistrar * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID lgid,
            /* [in] */ __RPC__in LPCWSTR pszIconFile,
            /* [in] */ __RPC__in LPCWSTR pszDesc);
        
        DECLSPEC_XFGVIRT(IActiveIMMRegistrar, UnregisterIME)
        HRESULT ( STDMETHODCALLTYPE *UnregisterIME )( 
            __RPC__in IActiveIMMRegistrar * This,
            /* [in] */ __RPC__in REFCLSID rclsid);
        
        END_INTERFACE
    } IActiveIMMRegistrarVtbl;

    interface IActiveIMMRegistrar
    {
        CONST_VTBL struct IActiveIMMRegistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIMMRegistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIMMRegistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIMMRegistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIMMRegistrar_RegisterIME(This,rclsid,lgid,pszIconFile,pszDesc)	\
    ( (This)->lpVtbl -> RegisterIME(This,rclsid,lgid,pszIconFile,pszDesc) ) 

#define IActiveIMMRegistrar_UnregisterIME(This,rclsid)	\
    ( (This)->lpVtbl -> UnregisterIME(This,rclsid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIMMRegistrar_INTERFACE_DEFINED__ */


#ifndef __IActiveIMMMessagePumpOwner_INTERFACE_DEFINED__
#define __IActiveIMMMessagePumpOwner_INTERFACE_DEFINED__

/* interface IActiveIMMMessagePumpOwner */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIMMMessagePumpOwner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b5cf2cfa-8aeb-11d1-9364-0060b067b86e")
    IActiveIMMMessagePumpOwner : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTranslateMessage( 
            /* [in] */ __RPC__in const MSG *pMsg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( 
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIMMMessagePumpOwnerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIMMMessagePumpOwner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIMMMessagePumpOwner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIMMMessagePumpOwner * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMMessagePumpOwner, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IActiveIMMMessagePumpOwner * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMMessagePumpOwner, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IActiveIMMMessagePumpOwner * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMMessagePumpOwner, OnTranslateMessage)
        HRESULT ( STDMETHODCALLTYPE *OnTranslateMessage )( 
            __RPC__in IActiveIMMMessagePumpOwner * This,
            /* [in] */ __RPC__in const MSG *pMsg);
        
        DECLSPEC_XFGVIRT(IActiveIMMMessagePumpOwner, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IActiveIMMMessagePumpOwner * This,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IActiveIMMMessagePumpOwner, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IActiveIMMMessagePumpOwner * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } IActiveIMMMessagePumpOwnerVtbl;

    interface IActiveIMMMessagePumpOwner
    {
        CONST_VTBL struct IActiveIMMMessagePumpOwnerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIMMMessagePumpOwner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIMMMessagePumpOwner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIMMMessagePumpOwner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIMMMessagePumpOwner_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IActiveIMMMessagePumpOwner_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#define IActiveIMMMessagePumpOwner_OnTranslateMessage(This,pMsg)	\
    ( (This)->lpVtbl -> OnTranslateMessage(This,pMsg) ) 

#define IActiveIMMMessagePumpOwner_Pause(This,pdwCookie)	\
    ( (This)->lpVtbl -> Pause(This,pdwCookie) ) 

#define IActiveIMMMessagePumpOwner_Resume(This,dwCookie)	\
    ( (This)->lpVtbl -> Resume(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIMMMessagePumpOwner_INTERFACE_DEFINED__ */


#ifndef __IActiveIMMApp_INTERFACE_DEFINED__
#define __IActiveIMMApp_INTERFACE_DEFINED__

/* interface IActiveIMMApp */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIMMApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08c0e040-62d1-11d1-9326-0060b067b86e")
    IActiveIMMApp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AssociateContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIME,
            /* [out] */ __RPC__out HIMC *phPrev) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConfigureIMEA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDA *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConfigureIMEW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateContext( 
            /* [out] */ __RPC__out HIMC *phIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyContext( 
            /* [in] */ HIMC hIME) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRegisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **pEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRegisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **pEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EscapeA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EscapeW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListCountA( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListCountW( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out CANDIDATEFORM *pCandidate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionFontA( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTA *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionFontW( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTW *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionStringA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionStringW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionWindow( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out COMPOSITIONFORM *pCompForm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out HIMC *phIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionListA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionListW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionStatus( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pfdwConversion,
            /* [out] */ __RPC__out DWORD *pfdwSentence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultIMEWnd( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__deref_out_opt HWND *phDefWnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuideLineA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuideLineW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPWSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMEFileNameA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMEFileNameW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpenStatus( 
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ DWORD fdwIndex,
            /* [out] */ __RPC__out DWORD *pdwProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisterWordStyleA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFA *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisterWordStyleW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatusWindowPos( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out POINT *pptPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVirtualKey( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out UINT *puVirtualKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallIMEA( 
            /* [in] */ __RPC__in LPSTR szIMEFileName,
            /* [in] */ __RPC__in LPSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallIMEW( 
            /* [in] */ __RPC__in LPWSTR szIMEFileName,
            /* [in] */ __RPC__in LPWSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsIME( 
            /* [in] */ __RPC__in HKL hKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUIMessageA( 
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUIMessageW( 
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyIME( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCandidateWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in CANDIDATEFORM *pCandidate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionFontA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTA *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionFontW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTW *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionStringA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionStringW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in COMPOSITIONFORM *pCompForm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConversionStatus( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD fdwConversion,
            /* [in] */ DWORD fdwSentence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpenStatus( 
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fOpen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStatusWindowPos( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in POINT *pptPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SimulateHotKey( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwHotKeyID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szUnregister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szUnregister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [in] */ BOOL fRestoreLayout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDefWindowProc( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT Msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FilterClientWindows( 
            /* [in] */ __RPC__in ATOM *aaClassList,
            /* [in] */ UINT uSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out UINT *uCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLangId( 
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out LANGID *plid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AssociateContextEx( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableIME( 
            /* [in] */ DWORD idThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImeMenuItemsA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOA *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOA *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImeMenuItemsW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOW *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOW *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumInputContext( 
            /* [in] */ DWORD idThread,
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIMMAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIMMApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIMMApp * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, AssociateContext)
        HRESULT ( STDMETHODCALLTYPE *AssociateContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIME,
            /* [out] */ __RPC__out HIMC *phPrev);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, ConfigureIMEA)
        HRESULT ( STDMETHODCALLTYPE *ConfigureIMEA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDA *pData);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, ConfigureIMEW)
        HRESULT ( STDMETHODCALLTYPE *ConfigureIMEW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pData);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, CreateContext)
        HRESULT ( STDMETHODCALLTYPE *CreateContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [out] */ __RPC__out HIMC *phIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, DestroyContext)
        HRESULT ( STDMETHODCALLTYPE *DestroyContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIME);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, EnumRegisterWordA)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWordA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **pEnum);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, EnumRegisterWordW)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWordW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **pEnum);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, EscapeA)
        HRESULT ( STDMETHODCALLTYPE *EscapeA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, EscapeW)
        HRESULT ( STDMETHODCALLTYPE *EscapeW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCandidateListA)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCandidateListW)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCandidateListCountA)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListCountA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCandidateListCountW)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListCountW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCandidateWindow)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateWindow )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out CANDIDATEFORM *pCandidate);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCompositionFontA)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionFontA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTA *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCompositionFontW)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionFontW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTW *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCompositionStringA)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionStringA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCompositionStringW)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionStringW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCompositionWindow)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionWindow )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out COMPOSITIONFORM *pCompForm);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out HIMC *phIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetConversionListA)
        HRESULT ( STDMETHODCALLTYPE *GetConversionListA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetConversionListW)
        HRESULT ( STDMETHODCALLTYPE *GetConversionListW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetConversionStatus)
        HRESULT ( STDMETHODCALLTYPE *GetConversionStatus )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pfdwConversion,
            /* [out] */ __RPC__out DWORD *pfdwSentence);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetDefaultIMEWnd)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultIMEWnd )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__deref_out_opt HWND *phDefWnd);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetDescriptionA)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetDescriptionW)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetGuideLineA)
        HRESULT ( STDMETHODCALLTYPE *GetGuideLineA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetGuideLineW)
        HRESULT ( STDMETHODCALLTYPE *GetGuideLineW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPWSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetIMEFileNameA)
        HRESULT ( STDMETHODCALLTYPE *GetIMEFileNameA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetIMEFileNameW)
        HRESULT ( STDMETHODCALLTYPE *GetIMEFileNameW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetOpenStatus)
        HRESULT ( STDMETHODCALLTYPE *GetOpenStatus )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ DWORD fdwIndex,
            /* [out] */ __RPC__out DWORD *pdwProperty);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetRegisterWordStyleA)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyleA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFA *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetRegisterWordStyleW)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyleW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetStatusWindowPos)
        HRESULT ( STDMETHODCALLTYPE *GetStatusWindowPos )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out POINT *pptPos);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetVirtualKey)
        HRESULT ( STDMETHODCALLTYPE *GetVirtualKey )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out UINT *puVirtualKey);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, InstallIMEA)
        HRESULT ( STDMETHODCALLTYPE *InstallIMEA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in LPSTR szIMEFileName,
            /* [in] */ __RPC__in LPSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, InstallIMEW)
        HRESULT ( STDMETHODCALLTYPE *InstallIMEW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in LPWSTR szIMEFileName,
            /* [in] */ __RPC__in LPWSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, IsIME)
        HRESULT ( STDMETHODCALLTYPE *IsIME )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, IsUIMessageA)
        HRESULT ( STDMETHODCALLTYPE *IsUIMessageA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, IsUIMessageW)
        HRESULT ( STDMETHODCALLTYPE *IsUIMessageW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, NotifyIME)
        HRESULT ( STDMETHODCALLTYPE *NotifyIME )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, RegisterWordA)
        HRESULT ( STDMETHODCALLTYPE *RegisterWordA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, RegisterWordW)
        HRESULT ( STDMETHODCALLTYPE *RegisterWordW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, ReleaseContext)
        HRESULT ( STDMETHODCALLTYPE *ReleaseContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCandidateWindow)
        HRESULT ( STDMETHODCALLTYPE *SetCandidateWindow )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in CANDIDATEFORM *pCandidate);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCompositionFontA)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionFontA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTA *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCompositionFontW)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionFontW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTW *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCompositionStringA)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionStringA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCompositionStringW)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionStringW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetCompositionWindow)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionWindow )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in COMPOSITIONFORM *pCompForm);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetConversionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetConversionStatus )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD fdwConversion,
            /* [in] */ DWORD fdwSentence);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetOpenStatus)
        HRESULT ( STDMETHODCALLTYPE *SetOpenStatus )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fOpen);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SetStatusWindowPos)
        HRESULT ( STDMETHODCALLTYPE *SetStatusWindowPos )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in POINT *pptPos);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, SimulateHotKey)
        HRESULT ( STDMETHODCALLTYPE *SimulateHotKey )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwHotKeyID);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, UnregisterWordA)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWordA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szUnregister);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, UnregisterWordW)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWordW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szUnregister);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ BOOL fRestoreLayout);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            __RPC__in IActiveIMMApp * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, OnDefWindowProc)
        HRESULT ( STDMETHODCALLTYPE *OnDefWindowProc )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT Msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, FilterClientWindows)
        HRESULT ( STDMETHODCALLTYPE *FilterClientWindows )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in ATOM *aaClassList,
            /* [in] */ UINT uSize);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetCodePageA)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out UINT *uCodePage);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetLangId)
        HRESULT ( STDMETHODCALLTYPE *GetLangId )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out LANGID *plid);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, AssociateContextEx)
        HRESULT ( STDMETHODCALLTYPE *AssociateContextEx )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, DisableIME)
        HRESULT ( STDMETHODCALLTYPE *DisableIME )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ DWORD idThread);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetImeMenuItemsA)
        HRESULT ( STDMETHODCALLTYPE *GetImeMenuItemsA )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOA *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOA *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, GetImeMenuItemsW)
        HRESULT ( STDMETHODCALLTYPE *GetImeMenuItemsW )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOW *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOW *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMApp, EnumInputContext)
        HRESULT ( STDMETHODCALLTYPE *EnumInputContext )( 
            __RPC__in IActiveIMMApp * This,
            /* [in] */ DWORD idThread,
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum);
        
        END_INTERFACE
    } IActiveIMMAppVtbl;

    interface IActiveIMMApp
    {
        CONST_VTBL struct IActiveIMMAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIMMApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIMMApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIMMApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIMMApp_AssociateContext(This,hWnd,hIME,phPrev)	\
    ( (This)->lpVtbl -> AssociateContext(This,hWnd,hIME,phPrev) ) 

#define IActiveIMMApp_ConfigureIMEA(This,hKL,hWnd,dwMode,pData)	\
    ( (This)->lpVtbl -> ConfigureIMEA(This,hKL,hWnd,dwMode,pData) ) 

#define IActiveIMMApp_ConfigureIMEW(This,hKL,hWnd,dwMode,pData)	\
    ( (This)->lpVtbl -> ConfigureIMEW(This,hKL,hWnd,dwMode,pData) ) 

#define IActiveIMMApp_CreateContext(This,phIMC)	\
    ( (This)->lpVtbl -> CreateContext(This,phIMC) ) 

#define IActiveIMMApp_DestroyContext(This,hIME)	\
    ( (This)->lpVtbl -> DestroyContext(This,hIME) ) 

#define IActiveIMMApp_EnumRegisterWordA(This,hKL,szReading,dwStyle,szRegister,pData,pEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWordA(This,hKL,szReading,dwStyle,szRegister,pData,pEnum) ) 

#define IActiveIMMApp_EnumRegisterWordW(This,hKL,szReading,dwStyle,szRegister,pData,pEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWordW(This,hKL,szReading,dwStyle,szRegister,pData,pEnum) ) 

#define IActiveIMMApp_EscapeA(This,hKL,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> EscapeA(This,hKL,hIMC,uEscape,pData,plResult) ) 

#define IActiveIMMApp_EscapeW(This,hKL,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> EscapeW(This,hKL,hIMC,uEscape,pData,plResult) ) 

#define IActiveIMMApp_GetCandidateListA(This,hIMC,dwIndex,uBufLen,pCandList,puCopied)	\
    ( (This)->lpVtbl -> GetCandidateListA(This,hIMC,dwIndex,uBufLen,pCandList,puCopied) ) 

#define IActiveIMMApp_GetCandidateListW(This,hIMC,dwIndex,uBufLen,pCandList,puCopied)	\
    ( (This)->lpVtbl -> GetCandidateListW(This,hIMC,dwIndex,uBufLen,pCandList,puCopied) ) 

#define IActiveIMMApp_GetCandidateListCountA(This,hIMC,pdwListSize,pdwBufLen)	\
    ( (This)->lpVtbl -> GetCandidateListCountA(This,hIMC,pdwListSize,pdwBufLen) ) 

#define IActiveIMMApp_GetCandidateListCountW(This,hIMC,pdwListSize,pdwBufLen)	\
    ( (This)->lpVtbl -> GetCandidateListCountW(This,hIMC,pdwListSize,pdwBufLen) ) 

#define IActiveIMMApp_GetCandidateWindow(This,hIMC,dwIndex,pCandidate)	\
    ( (This)->lpVtbl -> GetCandidateWindow(This,hIMC,dwIndex,pCandidate) ) 

#define IActiveIMMApp_GetCompositionFontA(This,hIMC,plf)	\
    ( (This)->lpVtbl -> GetCompositionFontA(This,hIMC,plf) ) 

#define IActiveIMMApp_GetCompositionFontW(This,hIMC,plf)	\
    ( (This)->lpVtbl -> GetCompositionFontW(This,hIMC,plf) ) 

#define IActiveIMMApp_GetCompositionStringA(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf)	\
    ( (This)->lpVtbl -> GetCompositionStringA(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf) ) 

#define IActiveIMMApp_GetCompositionStringW(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf)	\
    ( (This)->lpVtbl -> GetCompositionStringW(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf) ) 

#define IActiveIMMApp_GetCompositionWindow(This,hIMC,pCompForm)	\
    ( (This)->lpVtbl -> GetCompositionWindow(This,hIMC,pCompForm) ) 

#define IActiveIMMApp_GetContext(This,hWnd,phIMC)	\
    ( (This)->lpVtbl -> GetContext(This,hWnd,phIMC) ) 

#define IActiveIMMApp_GetConversionListA(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied)	\
    ( (This)->lpVtbl -> GetConversionListA(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied) ) 

#define IActiveIMMApp_GetConversionListW(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied)	\
    ( (This)->lpVtbl -> GetConversionListW(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied) ) 

#define IActiveIMMApp_GetConversionStatus(This,hIMC,pfdwConversion,pfdwSentence)	\
    ( (This)->lpVtbl -> GetConversionStatus(This,hIMC,pfdwConversion,pfdwSentence) ) 

#define IActiveIMMApp_GetDefaultIMEWnd(This,hWnd,phDefWnd)	\
    ( (This)->lpVtbl -> GetDefaultIMEWnd(This,hWnd,phDefWnd) ) 

#define IActiveIMMApp_GetDescriptionA(This,hKL,uBufLen,szDescription,puCopied)	\
    ( (This)->lpVtbl -> GetDescriptionA(This,hKL,uBufLen,szDescription,puCopied) ) 

#define IActiveIMMApp_GetDescriptionW(This,hKL,uBufLen,szDescription,puCopied)	\
    ( (This)->lpVtbl -> GetDescriptionW(This,hKL,uBufLen,szDescription,puCopied) ) 

#define IActiveIMMApp_GetGuideLineA(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult)	\
    ( (This)->lpVtbl -> GetGuideLineA(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult) ) 

#define IActiveIMMApp_GetGuideLineW(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult)	\
    ( (This)->lpVtbl -> GetGuideLineW(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult) ) 

#define IActiveIMMApp_GetIMEFileNameA(This,hKL,uBufLen,szFileName,puCopied)	\
    ( (This)->lpVtbl -> GetIMEFileNameA(This,hKL,uBufLen,szFileName,puCopied) ) 

#define IActiveIMMApp_GetIMEFileNameW(This,hKL,uBufLen,szFileName,puCopied)	\
    ( (This)->lpVtbl -> GetIMEFileNameW(This,hKL,uBufLen,szFileName,puCopied) ) 

#define IActiveIMMApp_GetOpenStatus(This,hIMC)	\
    ( (This)->lpVtbl -> GetOpenStatus(This,hIMC) ) 

#define IActiveIMMApp_GetProperty(This,hKL,fdwIndex,pdwProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,hKL,fdwIndex,pdwProperty) ) 

#define IActiveIMMApp_GetRegisterWordStyleA(This,hKL,nItem,pStyleBuf,puCopied)	\
    ( (This)->lpVtbl -> GetRegisterWordStyleA(This,hKL,nItem,pStyleBuf,puCopied) ) 

#define IActiveIMMApp_GetRegisterWordStyleW(This,hKL,nItem,pStyleBuf,puCopied)	\
    ( (This)->lpVtbl -> GetRegisterWordStyleW(This,hKL,nItem,pStyleBuf,puCopied) ) 

#define IActiveIMMApp_GetStatusWindowPos(This,hIMC,pptPos)	\
    ( (This)->lpVtbl -> GetStatusWindowPos(This,hIMC,pptPos) ) 

#define IActiveIMMApp_GetVirtualKey(This,hWnd,puVirtualKey)	\
    ( (This)->lpVtbl -> GetVirtualKey(This,hWnd,puVirtualKey) ) 

#define IActiveIMMApp_InstallIMEA(This,szIMEFileName,szLayoutText,phKL)	\
    ( (This)->lpVtbl -> InstallIMEA(This,szIMEFileName,szLayoutText,phKL) ) 

#define IActiveIMMApp_InstallIMEW(This,szIMEFileName,szLayoutText,phKL)	\
    ( (This)->lpVtbl -> InstallIMEW(This,szIMEFileName,szLayoutText,phKL) ) 

#define IActiveIMMApp_IsIME(This,hKL)	\
    ( (This)->lpVtbl -> IsIME(This,hKL) ) 

#define IActiveIMMApp_IsUIMessageA(This,hWndIME,msg,wParam,lParam)	\
    ( (This)->lpVtbl -> IsUIMessageA(This,hWndIME,msg,wParam,lParam) ) 

#define IActiveIMMApp_IsUIMessageW(This,hWndIME,msg,wParam,lParam)	\
    ( (This)->lpVtbl -> IsUIMessageW(This,hWndIME,msg,wParam,lParam) ) 

#define IActiveIMMApp_NotifyIME(This,hIMC,dwAction,dwIndex,dwValue)	\
    ( (This)->lpVtbl -> NotifyIME(This,hIMC,dwAction,dwIndex,dwValue) ) 

#define IActiveIMMApp_RegisterWordA(This,hKL,szReading,dwStyle,szRegister)	\
    ( (This)->lpVtbl -> RegisterWordA(This,hKL,szReading,dwStyle,szRegister) ) 

#define IActiveIMMApp_RegisterWordW(This,hKL,szReading,dwStyle,szRegister)	\
    ( (This)->lpVtbl -> RegisterWordW(This,hKL,szReading,dwStyle,szRegister) ) 

#define IActiveIMMApp_ReleaseContext(This,hWnd,hIMC)	\
    ( (This)->lpVtbl -> ReleaseContext(This,hWnd,hIMC) ) 

#define IActiveIMMApp_SetCandidateWindow(This,hIMC,pCandidate)	\
    ( (This)->lpVtbl -> SetCandidateWindow(This,hIMC,pCandidate) ) 

#define IActiveIMMApp_SetCompositionFontA(This,hIMC,plf)	\
    ( (This)->lpVtbl -> SetCompositionFontA(This,hIMC,plf) ) 

#define IActiveIMMApp_SetCompositionFontW(This,hIMC,plf)	\
    ( (This)->lpVtbl -> SetCompositionFontW(This,hIMC,plf) ) 

#define IActiveIMMApp_SetCompositionStringA(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionStringA(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIMMApp_SetCompositionStringW(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionStringW(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIMMApp_SetCompositionWindow(This,hIMC,pCompForm)	\
    ( (This)->lpVtbl -> SetCompositionWindow(This,hIMC,pCompForm) ) 

#define IActiveIMMApp_SetConversionStatus(This,hIMC,fdwConversion,fdwSentence)	\
    ( (This)->lpVtbl -> SetConversionStatus(This,hIMC,fdwConversion,fdwSentence) ) 

#define IActiveIMMApp_SetOpenStatus(This,hIMC,fOpen)	\
    ( (This)->lpVtbl -> SetOpenStatus(This,hIMC,fOpen) ) 

#define IActiveIMMApp_SetStatusWindowPos(This,hIMC,pptPos)	\
    ( (This)->lpVtbl -> SetStatusWindowPos(This,hIMC,pptPos) ) 

#define IActiveIMMApp_SimulateHotKey(This,hWnd,dwHotKeyID)	\
    ( (This)->lpVtbl -> SimulateHotKey(This,hWnd,dwHotKeyID) ) 

#define IActiveIMMApp_UnregisterWordA(This,hKL,szReading,dwStyle,szUnregister)	\
    ( (This)->lpVtbl -> UnregisterWordA(This,hKL,szReading,dwStyle,szUnregister) ) 

#define IActiveIMMApp_UnregisterWordW(This,hKL,szReading,dwStyle,szUnregister)	\
    ( (This)->lpVtbl -> UnregisterWordW(This,hKL,szReading,dwStyle,szUnregister) ) 

#define IActiveIMMApp_Activate(This,fRestoreLayout)	\
    ( (This)->lpVtbl -> Activate(This,fRestoreLayout) ) 

#define IActiveIMMApp_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define IActiveIMMApp_OnDefWindowProc(This,hWnd,Msg,wParam,lParam,plResult)	\
    ( (This)->lpVtbl -> OnDefWindowProc(This,hWnd,Msg,wParam,lParam,plResult) ) 

#define IActiveIMMApp_FilterClientWindows(This,aaClassList,uSize)	\
    ( (This)->lpVtbl -> FilterClientWindows(This,aaClassList,uSize) ) 

#define IActiveIMMApp_GetCodePageA(This,hKL,uCodePage)	\
    ( (This)->lpVtbl -> GetCodePageA(This,hKL,uCodePage) ) 

#define IActiveIMMApp_GetLangId(This,hKL,plid)	\
    ( (This)->lpVtbl -> GetLangId(This,hKL,plid) ) 

#define IActiveIMMApp_AssociateContextEx(This,hWnd,hIMC,dwFlags)	\
    ( (This)->lpVtbl -> AssociateContextEx(This,hWnd,hIMC,dwFlags) ) 

#define IActiveIMMApp_DisableIME(This,idThread)	\
    ( (This)->lpVtbl -> DisableIME(This,idThread) ) 

#define IActiveIMMApp_GetImeMenuItemsA(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult)	\
    ( (This)->lpVtbl -> GetImeMenuItemsA(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult) ) 

#define IActiveIMMApp_GetImeMenuItemsW(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult)	\
    ( (This)->lpVtbl -> GetImeMenuItemsW(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult) ) 

#define IActiveIMMApp_EnumInputContext(This,idThread,ppEnum)	\
    ( (This)->lpVtbl -> EnumInputContext(This,idThread,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIMMApp_INTERFACE_DEFINED__ */


#ifndef __IActiveIMMIME_INTERFACE_DEFINED__
#define __IActiveIMMIME_INTERFACE_DEFINED__

/* interface IActiveIMMIME */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIMMIME;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("08C03411-F96B-11d0-A475-00AA006BCC59")
    IActiveIMMIME : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AssociateContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIME,
            /* [out] */ __RPC__out HIMC *phPrev) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConfigureIMEA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDA *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConfigureIMEW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateContext( 
            /* [out] */ __RPC__out HIMC *phIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyContext( 
            /* [in] */ HIMC hIME) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRegisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **pEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRegisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **pEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EscapeA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EscapeW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListCountA( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateListCountW( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCandidateWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out CANDIDATEFORM *pCandidate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionFontA( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTA *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionFontW( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTW *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionStringA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionStringW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompositionWindow( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out COMPOSITIONFORM *pCompForm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out HIMC *phIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionListA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionListW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionStatus( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pfdwConversion,
            /* [out] */ __RPC__out DWORD *pfdwSentence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultIMEWnd( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__deref_out_opt HWND *phDefWnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuideLineA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuideLineW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPWSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMEFileNameA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMEFileNameW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpenStatus( 
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ DWORD fdwIndex,
            /* [out] */ __RPC__out DWORD *pdwProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisterWordStyleA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFA *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisterWordStyleW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatusWindowPos( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out POINT *pptPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVirtualKey( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out UINT *puVirtualKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallIMEA( 
            /* [in] */ __RPC__in LPSTR szIMEFileName,
            /* [in] */ __RPC__in LPSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallIMEW( 
            /* [in] */ __RPC__in LPWSTR szIMEFileName,
            /* [in] */ __RPC__in LPWSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsIME( 
            /* [in] */ __RPC__in HKL hKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUIMessageA( 
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUIMessageW( 
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyIME( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseContext( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCandidateWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in CANDIDATEFORM *pCandidate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionFontA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTA *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionFontW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTW *plf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionStringA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionStringW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionWindow( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in COMPOSITIONFORM *pCompForm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConversionStatus( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD fdwConversion,
            /* [in] */ DWORD fdwSentence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpenStatus( 
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fOpen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStatusWindowPos( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in POINT *pptPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SimulateHotKey( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwHotKeyID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWordA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szUnregister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWordW( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szUnregister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateMessage( 
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockIMC( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__deref_out_opt INPUTCONTEXT **ppIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockIMC( 
            /* [in] */ HIMC hIMC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMCLockCount( 
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwLockCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateIMCC( 
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out HIMCC *phIMCC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyIMCC( 
            /* [in] */ HIMCC hIMCC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockIMCC( 
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockIMCC( 
            /* [in] */ HIMCC hIMCC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReSizeIMCC( 
            /* [in] */ HIMCC hIMCC,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out HIMCC *phIMCC) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMCCSize( 
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__out DWORD *pdwSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIMCCLockCount( 
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__out DWORD *pdwLockCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHotKey( 
            /* [in] */ DWORD dwHotKeyID,
            /* [out] */ __RPC__out UINT *puModifiers,
            /* [out] */ __RPC__out UINT *puVKey,
            /* [out] */ __RPC__deref_out_opt HKL *phKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHotKey( 
            /* [in] */ DWORD dwHotKeyID,
            /* [in] */ UINT uModifiers,
            /* [in] */ UINT uVKey,
            /* [in] */ __RPC__in HKL hKL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSoftKeyboard( 
            /* [in] */ UINT uType,
            /* [in] */ __RPC__in HWND hOwner,
            /* [in] */ int x,
            /* [in] */ int y,
            /* [out] */ __RPC__deref_out_opt HWND *phSoftKbdWnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroySoftKeyboard( 
            /* [in] */ __RPC__in HWND hSoftKbdWnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowSoftKeyboard( 
            /* [in] */ __RPC__in HWND hSoftKbdWnd,
            /* [in] */ int nCmdShow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageA( 
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out UINT *uCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLangId( 
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out LANGID *plid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE KeybdEvent( 
            /* [in] */ LANGID lgidIME,
            /* [in] */ BYTE bVk,
            /* [in] */ BYTE bScan,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwExtraInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockModal( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockModal( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AssociateContextEx( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableIME( 
            /* [in] */ DWORD idThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImeMenuItemsA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOA *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOA *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImeMenuItemsW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOW *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOW *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumInputContext( 
            /* [in] */ DWORD idThread,
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestMessageA( 
            /* [in] */ HIMC hIMC,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestMessageW( 
            /* [in] */ HIMC hIMC,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendIMCA( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendIMCW( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSleeping( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIMMIMEVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIMMIME * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIMMIME * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, AssociateContext)
        HRESULT ( STDMETHODCALLTYPE *AssociateContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIME,
            /* [out] */ __RPC__out HIMC *phPrev);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, ConfigureIMEA)
        HRESULT ( STDMETHODCALLTYPE *ConfigureIMEA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDA *pData);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, ConfigureIMEW)
        HRESULT ( STDMETHODCALLTYPE *ConfigureIMEW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pData);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, CreateContext)
        HRESULT ( STDMETHODCALLTYPE *CreateContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [out] */ __RPC__out HIMC *phIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, DestroyContext)
        HRESULT ( STDMETHODCALLTYPE *DestroyContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIME);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, EnumRegisterWordA)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWordA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordA **pEnum);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, EnumRegisterWordW)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWordW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **pEnum);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, EscapeA)
        HRESULT ( STDMETHODCALLTYPE *EscapeA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, EscapeW)
        HRESULT ( STDMETHODCALLTYPE *EscapeW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout LPVOID pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCandidateListA)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCandidateListW)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pCandList,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCandidateListCountA)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListCountA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCandidateListCountW)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateListCountW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwListSize,
            /* [out] */ __RPC__out DWORD *pdwBufLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCandidateWindow)
        HRESULT ( STDMETHODCALLTYPE *GetCandidateWindow )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out CANDIDATEFORM *pCandidate);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCompositionFontA)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionFontA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTA *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCompositionFontW)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionFontW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out LOGFONTW *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCompositionStringA)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionStringA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCompositionStringW)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionStringW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LONG *plCopied,
            /* [out] */ __RPC__out LPVOID pBuf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCompositionWindow)
        HRESULT ( STDMETHODCALLTYPE *GetCompositionWindow )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out COMPOSITIONFORM *pCompForm);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out HIMC *phIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetConversionListA)
        HRESULT ( STDMETHODCALLTYPE *GetConversionListA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetConversionListW)
        HRESULT ( STDMETHODCALLTYPE *GetConversionListW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR pSrc,
            /* [in] */ UINT uBufLen,
            /* [in] */ UINT uFlag,
            /* [out] */ __RPC__out CANDIDATELIST *pDst,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetConversionStatus)
        HRESULT ( STDMETHODCALLTYPE *GetConversionStatus )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pfdwConversion,
            /* [out] */ __RPC__out DWORD *pfdwSentence);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetDefaultIMEWnd)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultIMEWnd )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__deref_out_opt HWND *phDefWnd);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetDescriptionA)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetDescriptionW)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szDescription,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetGuideLineA)
        HRESULT ( STDMETHODCALLTYPE *GetGuideLineA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetGuideLineW)
        HRESULT ( STDMETHODCALLTYPE *GetGuideLineW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwBufLen,
            /* [out] */ __RPC__out LPWSTR pBuf,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetIMEFileNameA)
        HRESULT ( STDMETHODCALLTYPE *GetIMEFileNameA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetIMEFileNameW)
        HRESULT ( STDMETHODCALLTYPE *GetIMEFileNameW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out LPWSTR szFileName,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetOpenStatus)
        HRESULT ( STDMETHODCALLTYPE *GetOpenStatus )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ DWORD fdwIndex,
            /* [out] */ __RPC__out DWORD *pdwProperty);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetRegisterWordStyleA)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyleA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFA *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetRegisterWordStyleW)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyleW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetStatusWindowPos)
        HRESULT ( STDMETHODCALLTYPE *GetStatusWindowPos )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out POINT *pptPos);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetVirtualKey)
        HRESULT ( STDMETHODCALLTYPE *GetVirtualKey )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [out] */ __RPC__out UINT *puVirtualKey);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, InstallIMEA)
        HRESULT ( STDMETHODCALLTYPE *InstallIMEA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in LPSTR szIMEFileName,
            /* [in] */ __RPC__in LPSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, InstallIMEW)
        HRESULT ( STDMETHODCALLTYPE *InstallIMEW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in LPWSTR szIMEFileName,
            /* [in] */ __RPC__in LPWSTR szLayoutText,
            /* [out] */ __RPC__deref_out_opt HKL *phKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, IsIME)
        HRESULT ( STDMETHODCALLTYPE *IsIME )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, IsUIMessageA)
        HRESULT ( STDMETHODCALLTYPE *IsUIMessageA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, IsUIMessageW)
        HRESULT ( STDMETHODCALLTYPE *IsUIMessageW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWndIME,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, NotifyIME)
        HRESULT ( STDMETHODCALLTYPE *NotifyIME )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, RegisterWordA)
        HRESULT ( STDMETHODCALLTYPE *RegisterWordA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szRegister);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, RegisterWordW)
        HRESULT ( STDMETHODCALLTYPE *RegisterWordW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, ReleaseContext)
        HRESULT ( STDMETHODCALLTYPE *ReleaseContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCandidateWindow)
        HRESULT ( STDMETHODCALLTYPE *SetCandidateWindow )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in CANDIDATEFORM *pCandidate);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCompositionFontA)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionFontA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTA *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCompositionFontW)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionFontW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LOGFONTW *plf);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCompositionStringA)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionStringA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCompositionStringW)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionStringW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in LPVOID pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in LPVOID pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetCompositionWindow)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionWindow )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in COMPOSITIONFORM *pCompForm);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetConversionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetConversionStatus )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD fdwConversion,
            /* [in] */ DWORD fdwSentence);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetOpenStatus)
        HRESULT ( STDMETHODCALLTYPE *SetOpenStatus )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fOpen);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetStatusWindowPos)
        HRESULT ( STDMETHODCALLTYPE *SetStatusWindowPos )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in POINT *pptPos);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SimulateHotKey)
        HRESULT ( STDMETHODCALLTYPE *SimulateHotKey )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwHotKeyID);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, UnregisterWordA)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWordA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPSTR szUnregister);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, UnregisterWordW)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWordW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szUnregister);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GenerateMessage)
        HRESULT ( STDMETHODCALLTYPE *GenerateMessage )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, LockIMC)
        HRESULT ( STDMETHODCALLTYPE *LockIMC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__deref_out_opt INPUTCONTEXT **ppIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, UnlockIMC)
        HRESULT ( STDMETHODCALLTYPE *UnlockIMC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetIMCLockCount)
        HRESULT ( STDMETHODCALLTYPE *GetIMCLockCount )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwLockCount);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, CreateIMCC)
        HRESULT ( STDMETHODCALLTYPE *CreateIMCC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out HIMCC *phIMCC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, DestroyIMCC)
        HRESULT ( STDMETHODCALLTYPE *DestroyIMCC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, LockIMCC)
        HRESULT ( STDMETHODCALLTYPE *LockIMCC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, UnlockIMCC)
        HRESULT ( STDMETHODCALLTYPE *UnlockIMCC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, ReSizeIMCC)
        HRESULT ( STDMETHODCALLTYPE *ReSizeIMCC )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out HIMCC *phIMCC);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetIMCCSize)
        HRESULT ( STDMETHODCALLTYPE *GetIMCCSize )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__out DWORD *pdwSize);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetIMCCLockCount)
        HRESULT ( STDMETHODCALLTYPE *GetIMCCLockCount )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMCC hIMCC,
            /* [out] */ __RPC__out DWORD *pdwLockCount);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetHotKey)
        HRESULT ( STDMETHODCALLTYPE *GetHotKey )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ DWORD dwHotKeyID,
            /* [out] */ __RPC__out UINT *puModifiers,
            /* [out] */ __RPC__out UINT *puVKey,
            /* [out] */ __RPC__deref_out_opt HKL *phKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SetHotKey)
        HRESULT ( STDMETHODCALLTYPE *SetHotKey )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ DWORD dwHotKeyID,
            /* [in] */ UINT uModifiers,
            /* [in] */ UINT uVKey,
            /* [in] */ __RPC__in HKL hKL);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, CreateSoftKeyboard)
        HRESULT ( STDMETHODCALLTYPE *CreateSoftKeyboard )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ UINT uType,
            /* [in] */ __RPC__in HWND hOwner,
            /* [in] */ int x,
            /* [in] */ int y,
            /* [out] */ __RPC__deref_out_opt HWND *phSoftKbdWnd);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, DestroySoftKeyboard)
        HRESULT ( STDMETHODCALLTYPE *DestroySoftKeyboard )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hSoftKbdWnd);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, ShowSoftKeyboard)
        HRESULT ( STDMETHODCALLTYPE *ShowSoftKeyboard )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hSoftKbdWnd,
            /* [in] */ int nCmdShow);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetCodePageA)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out UINT *uCodePage);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetLangId)
        HRESULT ( STDMETHODCALLTYPE *GetLangId )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [out] */ __RPC__out LANGID *plid);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, KeybdEvent)
        HRESULT ( STDMETHODCALLTYPE *KeybdEvent )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ LANGID lgidIME,
            /* [in] */ BYTE bVk,
            /* [in] */ BYTE bScan,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwExtraInfo);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, LockModal)
        HRESULT ( STDMETHODCALLTYPE *LockModal )( 
            __RPC__in IActiveIMMIME * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, UnlockModal)
        HRESULT ( STDMETHODCALLTYPE *UnlockModal )( 
            __RPC__in IActiveIMMIME * This);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, AssociateContextEx)
        HRESULT ( STDMETHODCALLTYPE *AssociateContextEx )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, DisableIME)
        HRESULT ( STDMETHODCALLTYPE *DisableIME )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ DWORD idThread);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetImeMenuItemsA)
        HRESULT ( STDMETHODCALLTYPE *GetImeMenuItemsA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOA *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOA *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, GetImeMenuItemsW)
        HRESULT ( STDMETHODCALLTYPE *GetImeMenuItemsW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in IMEMENUITEMINFOW *pImeParentMenu,
            /* [out] */ __RPC__out IMEMENUITEMINFOW *pImeMenu,
            /* [in] */ DWORD dwSize,
            /* [out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, EnumInputContext)
        HRESULT ( STDMETHODCALLTYPE *EnumInputContext )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ DWORD idThread,
            /* [out] */ __RPC__deref_out_opt IEnumInputContext **ppEnum);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, RequestMessageA)
        HRESULT ( STDMETHODCALLTYPE *RequestMessageA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, RequestMessageW)
        HRESULT ( STDMETHODCALLTYPE *RequestMessageW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SendIMCA)
        HRESULT ( STDMETHODCALLTYPE *SendIMCA )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, SendIMCW)
        HRESULT ( STDMETHODCALLTYPE *SendIMCW )( 
            __RPC__in IActiveIMMIME * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIMMIME, IsSleeping)
        HRESULT ( STDMETHODCALLTYPE *IsSleeping )( 
            __RPC__in IActiveIMMIME * This);
        
        END_INTERFACE
    } IActiveIMMIMEVtbl;

    interface IActiveIMMIME
    {
        CONST_VTBL struct IActiveIMMIMEVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIMMIME_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIMMIME_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIMMIME_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIMMIME_AssociateContext(This,hWnd,hIME,phPrev)	\
    ( (This)->lpVtbl -> AssociateContext(This,hWnd,hIME,phPrev) ) 

#define IActiveIMMIME_ConfigureIMEA(This,hKL,hWnd,dwMode,pData)	\
    ( (This)->lpVtbl -> ConfigureIMEA(This,hKL,hWnd,dwMode,pData) ) 

#define IActiveIMMIME_ConfigureIMEW(This,hKL,hWnd,dwMode,pData)	\
    ( (This)->lpVtbl -> ConfigureIMEW(This,hKL,hWnd,dwMode,pData) ) 

#define IActiveIMMIME_CreateContext(This,phIMC)	\
    ( (This)->lpVtbl -> CreateContext(This,phIMC) ) 

#define IActiveIMMIME_DestroyContext(This,hIME)	\
    ( (This)->lpVtbl -> DestroyContext(This,hIME) ) 

#define IActiveIMMIME_EnumRegisterWordA(This,hKL,szReading,dwStyle,szRegister,pData,pEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWordA(This,hKL,szReading,dwStyle,szRegister,pData,pEnum) ) 

#define IActiveIMMIME_EnumRegisterWordW(This,hKL,szReading,dwStyle,szRegister,pData,pEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWordW(This,hKL,szReading,dwStyle,szRegister,pData,pEnum) ) 

#define IActiveIMMIME_EscapeA(This,hKL,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> EscapeA(This,hKL,hIMC,uEscape,pData,plResult) ) 

#define IActiveIMMIME_EscapeW(This,hKL,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> EscapeW(This,hKL,hIMC,uEscape,pData,plResult) ) 

#define IActiveIMMIME_GetCandidateListA(This,hIMC,dwIndex,uBufLen,pCandList,puCopied)	\
    ( (This)->lpVtbl -> GetCandidateListA(This,hIMC,dwIndex,uBufLen,pCandList,puCopied) ) 

#define IActiveIMMIME_GetCandidateListW(This,hIMC,dwIndex,uBufLen,pCandList,puCopied)	\
    ( (This)->lpVtbl -> GetCandidateListW(This,hIMC,dwIndex,uBufLen,pCandList,puCopied) ) 

#define IActiveIMMIME_GetCandidateListCountA(This,hIMC,pdwListSize,pdwBufLen)	\
    ( (This)->lpVtbl -> GetCandidateListCountA(This,hIMC,pdwListSize,pdwBufLen) ) 

#define IActiveIMMIME_GetCandidateListCountW(This,hIMC,pdwListSize,pdwBufLen)	\
    ( (This)->lpVtbl -> GetCandidateListCountW(This,hIMC,pdwListSize,pdwBufLen) ) 

#define IActiveIMMIME_GetCandidateWindow(This,hIMC,dwIndex,pCandidate)	\
    ( (This)->lpVtbl -> GetCandidateWindow(This,hIMC,dwIndex,pCandidate) ) 

#define IActiveIMMIME_GetCompositionFontA(This,hIMC,plf)	\
    ( (This)->lpVtbl -> GetCompositionFontA(This,hIMC,plf) ) 

#define IActiveIMMIME_GetCompositionFontW(This,hIMC,plf)	\
    ( (This)->lpVtbl -> GetCompositionFontW(This,hIMC,plf) ) 

#define IActiveIMMIME_GetCompositionStringA(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf)	\
    ( (This)->lpVtbl -> GetCompositionStringA(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf) ) 

#define IActiveIMMIME_GetCompositionStringW(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf)	\
    ( (This)->lpVtbl -> GetCompositionStringW(This,hIMC,dwIndex,dwBufLen,plCopied,pBuf) ) 

#define IActiveIMMIME_GetCompositionWindow(This,hIMC,pCompForm)	\
    ( (This)->lpVtbl -> GetCompositionWindow(This,hIMC,pCompForm) ) 

#define IActiveIMMIME_GetContext(This,hWnd,phIMC)	\
    ( (This)->lpVtbl -> GetContext(This,hWnd,phIMC) ) 

#define IActiveIMMIME_GetConversionListA(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied)	\
    ( (This)->lpVtbl -> GetConversionListA(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied) ) 

#define IActiveIMMIME_GetConversionListW(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied)	\
    ( (This)->lpVtbl -> GetConversionListW(This,hKL,hIMC,pSrc,uBufLen,uFlag,pDst,puCopied) ) 

#define IActiveIMMIME_GetConversionStatus(This,hIMC,pfdwConversion,pfdwSentence)	\
    ( (This)->lpVtbl -> GetConversionStatus(This,hIMC,pfdwConversion,pfdwSentence) ) 

#define IActiveIMMIME_GetDefaultIMEWnd(This,hWnd,phDefWnd)	\
    ( (This)->lpVtbl -> GetDefaultIMEWnd(This,hWnd,phDefWnd) ) 

#define IActiveIMMIME_GetDescriptionA(This,hKL,uBufLen,szDescription,puCopied)	\
    ( (This)->lpVtbl -> GetDescriptionA(This,hKL,uBufLen,szDescription,puCopied) ) 

#define IActiveIMMIME_GetDescriptionW(This,hKL,uBufLen,szDescription,puCopied)	\
    ( (This)->lpVtbl -> GetDescriptionW(This,hKL,uBufLen,szDescription,puCopied) ) 

#define IActiveIMMIME_GetGuideLineA(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult)	\
    ( (This)->lpVtbl -> GetGuideLineA(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult) ) 

#define IActiveIMMIME_GetGuideLineW(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult)	\
    ( (This)->lpVtbl -> GetGuideLineW(This,hIMC,dwIndex,dwBufLen,pBuf,pdwResult) ) 

#define IActiveIMMIME_GetIMEFileNameA(This,hKL,uBufLen,szFileName,puCopied)	\
    ( (This)->lpVtbl -> GetIMEFileNameA(This,hKL,uBufLen,szFileName,puCopied) ) 

#define IActiveIMMIME_GetIMEFileNameW(This,hKL,uBufLen,szFileName,puCopied)	\
    ( (This)->lpVtbl -> GetIMEFileNameW(This,hKL,uBufLen,szFileName,puCopied) ) 

#define IActiveIMMIME_GetOpenStatus(This,hIMC)	\
    ( (This)->lpVtbl -> GetOpenStatus(This,hIMC) ) 

#define IActiveIMMIME_GetProperty(This,hKL,fdwIndex,pdwProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,hKL,fdwIndex,pdwProperty) ) 

#define IActiveIMMIME_GetRegisterWordStyleA(This,hKL,nItem,pStyleBuf,puCopied)	\
    ( (This)->lpVtbl -> GetRegisterWordStyleA(This,hKL,nItem,pStyleBuf,puCopied) ) 

#define IActiveIMMIME_GetRegisterWordStyleW(This,hKL,nItem,pStyleBuf,puCopied)	\
    ( (This)->lpVtbl -> GetRegisterWordStyleW(This,hKL,nItem,pStyleBuf,puCopied) ) 

#define IActiveIMMIME_GetStatusWindowPos(This,hIMC,pptPos)	\
    ( (This)->lpVtbl -> GetStatusWindowPos(This,hIMC,pptPos) ) 

#define IActiveIMMIME_GetVirtualKey(This,hWnd,puVirtualKey)	\
    ( (This)->lpVtbl -> GetVirtualKey(This,hWnd,puVirtualKey) ) 

#define IActiveIMMIME_InstallIMEA(This,szIMEFileName,szLayoutText,phKL)	\
    ( (This)->lpVtbl -> InstallIMEA(This,szIMEFileName,szLayoutText,phKL) ) 

#define IActiveIMMIME_InstallIMEW(This,szIMEFileName,szLayoutText,phKL)	\
    ( (This)->lpVtbl -> InstallIMEW(This,szIMEFileName,szLayoutText,phKL) ) 

#define IActiveIMMIME_IsIME(This,hKL)	\
    ( (This)->lpVtbl -> IsIME(This,hKL) ) 

#define IActiveIMMIME_IsUIMessageA(This,hWndIME,msg,wParam,lParam)	\
    ( (This)->lpVtbl -> IsUIMessageA(This,hWndIME,msg,wParam,lParam) ) 

#define IActiveIMMIME_IsUIMessageW(This,hWndIME,msg,wParam,lParam)	\
    ( (This)->lpVtbl -> IsUIMessageW(This,hWndIME,msg,wParam,lParam) ) 

#define IActiveIMMIME_NotifyIME(This,hIMC,dwAction,dwIndex,dwValue)	\
    ( (This)->lpVtbl -> NotifyIME(This,hIMC,dwAction,dwIndex,dwValue) ) 

#define IActiveIMMIME_RegisterWordA(This,hKL,szReading,dwStyle,szRegister)	\
    ( (This)->lpVtbl -> RegisterWordA(This,hKL,szReading,dwStyle,szRegister) ) 

#define IActiveIMMIME_RegisterWordW(This,hKL,szReading,dwStyle,szRegister)	\
    ( (This)->lpVtbl -> RegisterWordW(This,hKL,szReading,dwStyle,szRegister) ) 

#define IActiveIMMIME_ReleaseContext(This,hWnd,hIMC)	\
    ( (This)->lpVtbl -> ReleaseContext(This,hWnd,hIMC) ) 

#define IActiveIMMIME_SetCandidateWindow(This,hIMC,pCandidate)	\
    ( (This)->lpVtbl -> SetCandidateWindow(This,hIMC,pCandidate) ) 

#define IActiveIMMIME_SetCompositionFontA(This,hIMC,plf)	\
    ( (This)->lpVtbl -> SetCompositionFontA(This,hIMC,plf) ) 

#define IActiveIMMIME_SetCompositionFontW(This,hIMC,plf)	\
    ( (This)->lpVtbl -> SetCompositionFontW(This,hIMC,plf) ) 

#define IActiveIMMIME_SetCompositionStringA(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionStringA(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIMMIME_SetCompositionStringW(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionStringW(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIMMIME_SetCompositionWindow(This,hIMC,pCompForm)	\
    ( (This)->lpVtbl -> SetCompositionWindow(This,hIMC,pCompForm) ) 

#define IActiveIMMIME_SetConversionStatus(This,hIMC,fdwConversion,fdwSentence)	\
    ( (This)->lpVtbl -> SetConversionStatus(This,hIMC,fdwConversion,fdwSentence) ) 

#define IActiveIMMIME_SetOpenStatus(This,hIMC,fOpen)	\
    ( (This)->lpVtbl -> SetOpenStatus(This,hIMC,fOpen) ) 

#define IActiveIMMIME_SetStatusWindowPos(This,hIMC,pptPos)	\
    ( (This)->lpVtbl -> SetStatusWindowPos(This,hIMC,pptPos) ) 

#define IActiveIMMIME_SimulateHotKey(This,hWnd,dwHotKeyID)	\
    ( (This)->lpVtbl -> SimulateHotKey(This,hWnd,dwHotKeyID) ) 

#define IActiveIMMIME_UnregisterWordA(This,hKL,szReading,dwStyle,szUnregister)	\
    ( (This)->lpVtbl -> UnregisterWordA(This,hKL,szReading,dwStyle,szUnregister) ) 

#define IActiveIMMIME_UnregisterWordW(This,hKL,szReading,dwStyle,szUnregister)	\
    ( (This)->lpVtbl -> UnregisterWordW(This,hKL,szReading,dwStyle,szUnregister) ) 

#define IActiveIMMIME_GenerateMessage(This,hIMC)	\
    ( (This)->lpVtbl -> GenerateMessage(This,hIMC) ) 

#define IActiveIMMIME_LockIMC(This,hIMC,ppIMC)	\
    ( (This)->lpVtbl -> LockIMC(This,hIMC,ppIMC) ) 

#define IActiveIMMIME_UnlockIMC(This,hIMC)	\
    ( (This)->lpVtbl -> UnlockIMC(This,hIMC) ) 

#define IActiveIMMIME_GetIMCLockCount(This,hIMC,pdwLockCount)	\
    ( (This)->lpVtbl -> GetIMCLockCount(This,hIMC,pdwLockCount) ) 

#define IActiveIMMIME_CreateIMCC(This,dwSize,phIMCC)	\
    ( (This)->lpVtbl -> CreateIMCC(This,dwSize,phIMCC) ) 

#define IActiveIMMIME_DestroyIMCC(This,hIMCC)	\
    ( (This)->lpVtbl -> DestroyIMCC(This,hIMCC) ) 

#define IActiveIMMIME_LockIMCC(This,hIMCC,ppv)	\
    ( (This)->lpVtbl -> LockIMCC(This,hIMCC,ppv) ) 

#define IActiveIMMIME_UnlockIMCC(This,hIMCC)	\
    ( (This)->lpVtbl -> UnlockIMCC(This,hIMCC) ) 

#define IActiveIMMIME_ReSizeIMCC(This,hIMCC,dwSize,phIMCC)	\
    ( (This)->lpVtbl -> ReSizeIMCC(This,hIMCC,dwSize,phIMCC) ) 

#define IActiveIMMIME_GetIMCCSize(This,hIMCC,pdwSize)	\
    ( (This)->lpVtbl -> GetIMCCSize(This,hIMCC,pdwSize) ) 

#define IActiveIMMIME_GetIMCCLockCount(This,hIMCC,pdwLockCount)	\
    ( (This)->lpVtbl -> GetIMCCLockCount(This,hIMCC,pdwLockCount) ) 

#define IActiveIMMIME_GetHotKey(This,dwHotKeyID,puModifiers,puVKey,phKL)	\
    ( (This)->lpVtbl -> GetHotKey(This,dwHotKeyID,puModifiers,puVKey,phKL) ) 

#define IActiveIMMIME_SetHotKey(This,dwHotKeyID,uModifiers,uVKey,hKL)	\
    ( (This)->lpVtbl -> SetHotKey(This,dwHotKeyID,uModifiers,uVKey,hKL) ) 

#define IActiveIMMIME_CreateSoftKeyboard(This,uType,hOwner,x,y,phSoftKbdWnd)	\
    ( (This)->lpVtbl -> CreateSoftKeyboard(This,uType,hOwner,x,y,phSoftKbdWnd) ) 

#define IActiveIMMIME_DestroySoftKeyboard(This,hSoftKbdWnd)	\
    ( (This)->lpVtbl -> DestroySoftKeyboard(This,hSoftKbdWnd) ) 

#define IActiveIMMIME_ShowSoftKeyboard(This,hSoftKbdWnd,nCmdShow)	\
    ( (This)->lpVtbl -> ShowSoftKeyboard(This,hSoftKbdWnd,nCmdShow) ) 

#define IActiveIMMIME_GetCodePageA(This,hKL,uCodePage)	\
    ( (This)->lpVtbl -> GetCodePageA(This,hKL,uCodePage) ) 

#define IActiveIMMIME_GetLangId(This,hKL,plid)	\
    ( (This)->lpVtbl -> GetLangId(This,hKL,plid) ) 

#define IActiveIMMIME_KeybdEvent(This,lgidIME,bVk,bScan,dwFlags,dwExtraInfo)	\
    ( (This)->lpVtbl -> KeybdEvent(This,lgidIME,bVk,bScan,dwFlags,dwExtraInfo) ) 

#define IActiveIMMIME_LockModal(This)	\
    ( (This)->lpVtbl -> LockModal(This) ) 

#define IActiveIMMIME_UnlockModal(This)	\
    ( (This)->lpVtbl -> UnlockModal(This) ) 

#define IActiveIMMIME_AssociateContextEx(This,hWnd,hIMC,dwFlags)	\
    ( (This)->lpVtbl -> AssociateContextEx(This,hWnd,hIMC,dwFlags) ) 

#define IActiveIMMIME_DisableIME(This,idThread)	\
    ( (This)->lpVtbl -> DisableIME(This,idThread) ) 

#define IActiveIMMIME_GetImeMenuItemsA(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult)	\
    ( (This)->lpVtbl -> GetImeMenuItemsA(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult) ) 

#define IActiveIMMIME_GetImeMenuItemsW(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult)	\
    ( (This)->lpVtbl -> GetImeMenuItemsW(This,hIMC,dwFlags,dwType,pImeParentMenu,pImeMenu,dwSize,pdwResult) ) 

#define IActiveIMMIME_EnumInputContext(This,idThread,ppEnum)	\
    ( (This)->lpVtbl -> EnumInputContext(This,idThread,ppEnum) ) 

#define IActiveIMMIME_RequestMessageA(This,hIMC,wParam,lParam,plResult)	\
    ( (This)->lpVtbl -> RequestMessageA(This,hIMC,wParam,lParam,plResult) ) 

#define IActiveIMMIME_RequestMessageW(This,hIMC,wParam,lParam,plResult)	\
    ( (This)->lpVtbl -> RequestMessageW(This,hIMC,wParam,lParam,plResult) ) 

#define IActiveIMMIME_SendIMCA(This,hWnd,uMsg,wParam,lParam,plResult)	\
    ( (This)->lpVtbl -> SendIMCA(This,hWnd,uMsg,wParam,lParam,plResult) ) 

#define IActiveIMMIME_SendIMCW(This,hWnd,uMsg,wParam,lParam,plResult)	\
    ( (This)->lpVtbl -> SendIMCW(This,hWnd,uMsg,wParam,lParam,plResult) ) 

#define IActiveIMMIME_IsSleeping(This)	\
    ( (This)->lpVtbl -> IsSleeping(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIMMIME_INTERFACE_DEFINED__ */


#ifndef __IActiveIME_INTERFACE_DEFINED__
#define __IActiveIME_INTERFACE_DEFINED__

/* interface IActiveIME */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIME;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6FE20962-D077-11d0-8FE7-00AA006BCC59")
    IActiveIME : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Inquire( 
            /* [in] */ DWORD dwSystemInfoFlags,
            /* [out] */ __RPC__out IMEINFO *pIMEInfo,
            /* [out] */ __RPC__out LPWSTR szWndClass,
            /* [out] */ __RPC__out DWORD *pdwPrivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConversionList( 
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR szSource,
            /* [in] */ UINT uFlag,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pDest,
            /* [out] */ __RPC__out UINT *puCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Configure( 
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pRegisterWord) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Destroy( 
            /* [in] */ UINT uReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Escape( 
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout void *pData,
            /* [out] */ __RPC__out LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActiveContext( 
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessKey( 
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uVirKey,
            /* [in] */ DWORD lParam,
            /* [in] */ __RPC__in BYTE *pbKeyState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Select( 
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fSelect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompositionString( 
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in void *pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in void *pRead,
            /* [in] */ DWORD dwReadLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ToAsciiEx( 
            /* [in] */ UINT uVirKey,
            /* [in] */ UINT uScanCode,
            /* [in] */ __RPC__in BYTE *pbKeyState,
            /* [in] */ UINT fuState,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwTransBuf,
            /* [out] */ __RPC__out UINT *puSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterWord( 
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWord( 
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisterWordStyle( 
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puBufSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRegisterWord( 
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodePageA( 
            /* [out] */ __RPC__out UINT *uCodePage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLangId( 
            /* [out] */ __RPC__out LANGID *plid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIMEVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIME * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIME * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIME * This);
        
        DECLSPEC_XFGVIRT(IActiveIME, Inquire)
        HRESULT ( STDMETHODCALLTYPE *Inquire )( 
            __RPC__in IActiveIME * This,
            /* [in] */ DWORD dwSystemInfoFlags,
            /* [out] */ __RPC__out IMEINFO *pIMEInfo,
            /* [out] */ __RPC__out LPWSTR szWndClass,
            /* [out] */ __RPC__out DWORD *pdwPrivate);
        
        DECLSPEC_XFGVIRT(IActiveIME, ConversionList)
        HRESULT ( STDMETHODCALLTYPE *ConversionList )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR szSource,
            /* [in] */ UINT uFlag,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pDest,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIME, Configure)
        HRESULT ( STDMETHODCALLTYPE *Configure )( 
            __RPC__in IActiveIME * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pRegisterWord);
        
        DECLSPEC_XFGVIRT(IActiveIME, Destroy)
        HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IActiveIME * This,
            /* [in] */ UINT uReserved);
        
        DECLSPEC_XFGVIRT(IActiveIME, Escape)
        HRESULT ( STDMETHODCALLTYPE *Escape )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout void *pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIME, SetActiveContext)
        HRESULT ( STDMETHODCALLTYPE *SetActiveContext )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fFlag);
        
        DECLSPEC_XFGVIRT(IActiveIME, ProcessKey)
        HRESULT ( STDMETHODCALLTYPE *ProcessKey )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uVirKey,
            /* [in] */ DWORD lParam,
            /* [in] */ __RPC__in BYTE *pbKeyState);
        
        DECLSPEC_XFGVIRT(IActiveIME, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IActiveIME, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fSelect);
        
        DECLSPEC_XFGVIRT(IActiveIME, SetCompositionString)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionString )( 
            __RPC__in IActiveIME * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in void *pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in void *pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIME, ToAsciiEx)
        HRESULT ( STDMETHODCALLTYPE *ToAsciiEx )( 
            __RPC__in IActiveIME * This,
            /* [in] */ UINT uVirKey,
            /* [in] */ UINT uScanCode,
            /* [in] */ __RPC__in BYTE *pbKeyState,
            /* [in] */ UINT fuState,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwTransBuf,
            /* [out] */ __RPC__out UINT *puSize);
        
        DECLSPEC_XFGVIRT(IActiveIME, RegisterWord)
        HRESULT ( STDMETHODCALLTYPE *RegisterWord )( 
            __RPC__in IActiveIME * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString);
        
        DECLSPEC_XFGVIRT(IActiveIME, UnregisterWord)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWord )( 
            __RPC__in IActiveIME * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetRegisterWordStyle)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyle )( 
            __RPC__in IActiveIME * This,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puBufSize);
        
        DECLSPEC_XFGVIRT(IActiveIME, EnumRegisterWord)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWord )( 
            __RPC__in IActiveIME * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **ppEnum);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetCodePageA)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageA )( 
            __RPC__in IActiveIME * This,
            /* [out] */ __RPC__out UINT *uCodePage);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetLangId)
        HRESULT ( STDMETHODCALLTYPE *GetLangId )( 
            __RPC__in IActiveIME * This,
            /* [out] */ __RPC__out LANGID *plid);
        
        END_INTERFACE
    } IActiveIMEVtbl;

    interface IActiveIME
    {
        CONST_VTBL struct IActiveIMEVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIME_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIME_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIME_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIME_Inquire(This,dwSystemInfoFlags,pIMEInfo,szWndClass,pdwPrivate)	\
    ( (This)->lpVtbl -> Inquire(This,dwSystemInfoFlags,pIMEInfo,szWndClass,pdwPrivate) ) 

#define IActiveIME_ConversionList(This,hIMC,szSource,uFlag,uBufLen,pDest,puCopied)	\
    ( (This)->lpVtbl -> ConversionList(This,hIMC,szSource,uFlag,uBufLen,pDest,puCopied) ) 

#define IActiveIME_Configure(This,hKL,hWnd,dwMode,pRegisterWord)	\
    ( (This)->lpVtbl -> Configure(This,hKL,hWnd,dwMode,pRegisterWord) ) 

#define IActiveIME_Destroy(This,uReserved)	\
    ( (This)->lpVtbl -> Destroy(This,uReserved) ) 

#define IActiveIME_Escape(This,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> Escape(This,hIMC,uEscape,pData,plResult) ) 

#define IActiveIME_SetActiveContext(This,hIMC,fFlag)	\
    ( (This)->lpVtbl -> SetActiveContext(This,hIMC,fFlag) ) 

#define IActiveIME_ProcessKey(This,hIMC,uVirKey,lParam,pbKeyState)	\
    ( (This)->lpVtbl -> ProcessKey(This,hIMC,uVirKey,lParam,pbKeyState) ) 

#define IActiveIME_Notify(This,hIMC,dwAction,dwIndex,dwValue)	\
    ( (This)->lpVtbl -> Notify(This,hIMC,dwAction,dwIndex,dwValue) ) 

#define IActiveIME_Select(This,hIMC,fSelect)	\
    ( (This)->lpVtbl -> Select(This,hIMC,fSelect) ) 

#define IActiveIME_SetCompositionString(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionString(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIME_ToAsciiEx(This,uVirKey,uScanCode,pbKeyState,fuState,hIMC,pdwTransBuf,puSize)	\
    ( (This)->lpVtbl -> ToAsciiEx(This,uVirKey,uScanCode,pbKeyState,fuState,hIMC,pdwTransBuf,puSize) ) 

#define IActiveIME_RegisterWord(This,szReading,dwStyle,szString)	\
    ( (This)->lpVtbl -> RegisterWord(This,szReading,dwStyle,szString) ) 

#define IActiveIME_UnregisterWord(This,szReading,dwStyle,szString)	\
    ( (This)->lpVtbl -> UnregisterWord(This,szReading,dwStyle,szString) ) 

#define IActiveIME_GetRegisterWordStyle(This,nItem,pStyleBuf,puBufSize)	\
    ( (This)->lpVtbl -> GetRegisterWordStyle(This,nItem,pStyleBuf,puBufSize) ) 

#define IActiveIME_EnumRegisterWord(This,szReading,dwStyle,szRegister,pData,ppEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWord(This,szReading,dwStyle,szRegister,pData,ppEnum) ) 

#define IActiveIME_GetCodePageA(This,uCodePage)	\
    ( (This)->lpVtbl -> GetCodePageA(This,uCodePage) ) 

#define IActiveIME_GetLangId(This,plid)	\
    ( (This)->lpVtbl -> GetLangId(This,plid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIME_INTERFACE_DEFINED__ */


#ifndef __IActiveIME2_INTERFACE_DEFINED__
#define __IActiveIME2_INTERFACE_DEFINED__

/* interface IActiveIME2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveIME2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e1c4bf0e-2d53-11d2-93e1-0060b067b86e")
    IActiveIME2 : public IActiveIME
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Sleep( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unsleep( 
            /* [in] */ BOOL fDead) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveIME2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveIME2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveIME2 * This);
        
        DECLSPEC_XFGVIRT(IActiveIME, Inquire)
        HRESULT ( STDMETHODCALLTYPE *Inquire )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ DWORD dwSystemInfoFlags,
            /* [out] */ __RPC__out IMEINFO *pIMEInfo,
            /* [out] */ __RPC__out LPWSTR szWndClass,
            /* [out] */ __RPC__out DWORD *pdwPrivate);
        
        DECLSPEC_XFGVIRT(IActiveIME, ConversionList)
        HRESULT ( STDMETHODCALLTYPE *ConversionList )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ __RPC__in LPWSTR szSource,
            /* [in] */ UINT uFlag,
            /* [in] */ UINT uBufLen,
            /* [out] */ __RPC__out CANDIDATELIST *pDest,
            /* [out] */ __RPC__out UINT *puCopied);
        
        DECLSPEC_XFGVIRT(IActiveIME, Configure)
        HRESULT ( STDMETHODCALLTYPE *Configure )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ __RPC__in HKL hKL,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ DWORD dwMode,
            /* [in] */ __RPC__in REGISTERWORDW *pRegisterWord);
        
        DECLSPEC_XFGVIRT(IActiveIME, Destroy)
        HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ UINT uReserved);
        
        DECLSPEC_XFGVIRT(IActiveIME, Escape)
        HRESULT ( STDMETHODCALLTYPE *Escape )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uEscape,
            /* [out][in] */ __RPC__inout void *pData,
            /* [out] */ __RPC__out LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IActiveIME, SetActiveContext)
        HRESULT ( STDMETHODCALLTYPE *SetActiveContext )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fFlag);
        
        DECLSPEC_XFGVIRT(IActiveIME, ProcessKey)
        HRESULT ( STDMETHODCALLTYPE *ProcessKey )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ UINT uVirKey,
            /* [in] */ DWORD lParam,
            /* [in] */ __RPC__in BYTE *pbKeyState);
        
        DECLSPEC_XFGVIRT(IActiveIME, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwAction,
            /* [in] */ DWORD dwIndex,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IActiveIME, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ BOOL fSelect);
        
        DECLSPEC_XFGVIRT(IActiveIME, SetCompositionString)
        HRESULT ( STDMETHODCALLTYPE *SetCompositionString )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ HIMC hIMC,
            /* [in] */ DWORD dwIndex,
            /* [in] */ __RPC__in void *pComp,
            /* [in] */ DWORD dwCompLen,
            /* [in] */ __RPC__in void *pRead,
            /* [in] */ DWORD dwReadLen);
        
        DECLSPEC_XFGVIRT(IActiveIME, ToAsciiEx)
        HRESULT ( STDMETHODCALLTYPE *ToAsciiEx )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ UINT uVirKey,
            /* [in] */ UINT uScanCode,
            /* [in] */ __RPC__in BYTE *pbKeyState,
            /* [in] */ UINT fuState,
            /* [in] */ HIMC hIMC,
            /* [out] */ __RPC__out DWORD *pdwTransBuf,
            /* [out] */ __RPC__out UINT *puSize);
        
        DECLSPEC_XFGVIRT(IActiveIME, RegisterWord)
        HRESULT ( STDMETHODCALLTYPE *RegisterWord )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString);
        
        DECLSPEC_XFGVIRT(IActiveIME, UnregisterWord)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWord )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szString);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetRegisterWordStyle)
        HRESULT ( STDMETHODCALLTYPE *GetRegisterWordStyle )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ UINT nItem,
            /* [out] */ __RPC__out STYLEBUFW *pStyleBuf,
            /* [out] */ __RPC__out UINT *puBufSize);
        
        DECLSPEC_XFGVIRT(IActiveIME, EnumRegisterWord)
        HRESULT ( STDMETHODCALLTYPE *EnumRegisterWord )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ __RPC__in LPWSTR szReading,
            /* [in] */ DWORD dwStyle,
            /* [in] */ __RPC__in LPWSTR szRegister,
            /* [in] */ __RPC__in LPVOID pData,
            /* [out] */ __RPC__deref_out_opt IEnumRegisterWordW **ppEnum);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetCodePageA)
        HRESULT ( STDMETHODCALLTYPE *GetCodePageA )( 
            __RPC__in IActiveIME2 * This,
            /* [out] */ __RPC__out UINT *uCodePage);
        
        DECLSPEC_XFGVIRT(IActiveIME, GetLangId)
        HRESULT ( STDMETHODCALLTYPE *GetLangId )( 
            __RPC__in IActiveIME2 * This,
            /* [out] */ __RPC__out LANGID *plid);
        
        DECLSPEC_XFGVIRT(IActiveIME2, Sleep)
        HRESULT ( STDMETHODCALLTYPE *Sleep )( 
            __RPC__in IActiveIME2 * This);
        
        DECLSPEC_XFGVIRT(IActiveIME2, Unsleep)
        HRESULT ( STDMETHODCALLTYPE *Unsleep )( 
            __RPC__in IActiveIME2 * This,
            /* [in] */ BOOL fDead);
        
        END_INTERFACE
    } IActiveIME2Vtbl;

    interface IActiveIME2
    {
        CONST_VTBL struct IActiveIME2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveIME2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveIME2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveIME2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveIME2_Inquire(This,dwSystemInfoFlags,pIMEInfo,szWndClass,pdwPrivate)	\
    ( (This)->lpVtbl -> Inquire(This,dwSystemInfoFlags,pIMEInfo,szWndClass,pdwPrivate) ) 

#define IActiveIME2_ConversionList(This,hIMC,szSource,uFlag,uBufLen,pDest,puCopied)	\
    ( (This)->lpVtbl -> ConversionList(This,hIMC,szSource,uFlag,uBufLen,pDest,puCopied) ) 

#define IActiveIME2_Configure(This,hKL,hWnd,dwMode,pRegisterWord)	\
    ( (This)->lpVtbl -> Configure(This,hKL,hWnd,dwMode,pRegisterWord) ) 

#define IActiveIME2_Destroy(This,uReserved)	\
    ( (This)->lpVtbl -> Destroy(This,uReserved) ) 

#define IActiveIME2_Escape(This,hIMC,uEscape,pData,plResult)	\
    ( (This)->lpVtbl -> Escape(This,hIMC,uEscape,pData,plResult) ) 

#define IActiveIME2_SetActiveContext(This,hIMC,fFlag)	\
    ( (This)->lpVtbl -> SetActiveContext(This,hIMC,fFlag) ) 

#define IActiveIME2_ProcessKey(This,hIMC,uVirKey,lParam,pbKeyState)	\
    ( (This)->lpVtbl -> ProcessKey(This,hIMC,uVirKey,lParam,pbKeyState) ) 

#define IActiveIME2_Notify(This,hIMC,dwAction,dwIndex,dwValue)	\
    ( (This)->lpVtbl -> Notify(This,hIMC,dwAction,dwIndex,dwValue) ) 

#define IActiveIME2_Select(This,hIMC,fSelect)	\
    ( (This)->lpVtbl -> Select(This,hIMC,fSelect) ) 

#define IActiveIME2_SetCompositionString(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen)	\
    ( (This)->lpVtbl -> SetCompositionString(This,hIMC,dwIndex,pComp,dwCompLen,pRead,dwReadLen) ) 

#define IActiveIME2_ToAsciiEx(This,uVirKey,uScanCode,pbKeyState,fuState,hIMC,pdwTransBuf,puSize)	\
    ( (This)->lpVtbl -> ToAsciiEx(This,uVirKey,uScanCode,pbKeyState,fuState,hIMC,pdwTransBuf,puSize) ) 

#define IActiveIME2_RegisterWord(This,szReading,dwStyle,szString)	\
    ( (This)->lpVtbl -> RegisterWord(This,szReading,dwStyle,szString) ) 

#define IActiveIME2_UnregisterWord(This,szReading,dwStyle,szString)	\
    ( (This)->lpVtbl -> UnregisterWord(This,szReading,dwStyle,szString) ) 

#define IActiveIME2_GetRegisterWordStyle(This,nItem,pStyleBuf,puBufSize)	\
    ( (This)->lpVtbl -> GetRegisterWordStyle(This,nItem,pStyleBuf,puBufSize) ) 

#define IActiveIME2_EnumRegisterWord(This,szReading,dwStyle,szRegister,pData,ppEnum)	\
    ( (This)->lpVtbl -> EnumRegisterWord(This,szReading,dwStyle,szRegister,pData,ppEnum) ) 

#define IActiveIME2_GetCodePageA(This,uCodePage)	\
    ( (This)->lpVtbl -> GetCodePageA(This,uCodePage) ) 

#define IActiveIME2_GetLangId(This,plid)	\
    ( (This)->lpVtbl -> GetLangId(This,plid) ) 


#define IActiveIME2_Sleep(This)	\
    ( (This)->lpVtbl -> Sleep(This) ) 

#define IActiveIME2_Unsleep(This,fDead)	\
    ( (This)->lpVtbl -> Unsleep(This,fDead) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveIME2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CActiveIMM;

#ifdef __cplusplus

class DECLSPEC_UUID("4955DD33-B159-11d0-8FCF-00AA006BCC59")
CActiveIMM;
#endif
#endif /* __ActiveIMM_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_dimm_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_dimm_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dimm_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


