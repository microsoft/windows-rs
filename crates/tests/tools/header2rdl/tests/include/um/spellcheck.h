

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

#ifndef __spellcheck_h__
#define __spellcheck_h__

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

#ifndef __ISpellingError_FWD_DEFINED__
#define __ISpellingError_FWD_DEFINED__
typedef interface ISpellingError ISpellingError;

#endif 	/* __ISpellingError_FWD_DEFINED__ */


#ifndef __IEnumSpellingError_FWD_DEFINED__
#define __IEnumSpellingError_FWD_DEFINED__
typedef interface IEnumSpellingError IEnumSpellingError;

#endif 	/* __IEnumSpellingError_FWD_DEFINED__ */


#ifndef __IOptionDescription_FWD_DEFINED__
#define __IOptionDescription_FWD_DEFINED__
typedef interface IOptionDescription IOptionDescription;

#endif 	/* __IOptionDescription_FWD_DEFINED__ */


#ifndef __ISpellCheckerChangedEventHandler_FWD_DEFINED__
#define __ISpellCheckerChangedEventHandler_FWD_DEFINED__
typedef interface ISpellCheckerChangedEventHandler ISpellCheckerChangedEventHandler;

#endif 	/* __ISpellCheckerChangedEventHandler_FWD_DEFINED__ */


#ifndef __ISpellChecker_FWD_DEFINED__
#define __ISpellChecker_FWD_DEFINED__
typedef interface ISpellChecker ISpellChecker;

#endif 	/* __ISpellChecker_FWD_DEFINED__ */


#ifndef __ISpellChecker2_FWD_DEFINED__
#define __ISpellChecker2_FWD_DEFINED__
typedef interface ISpellChecker2 ISpellChecker2;

#endif 	/* __ISpellChecker2_FWD_DEFINED__ */


#ifndef __ISpellCheckerFactory_FWD_DEFINED__
#define __ISpellCheckerFactory_FWD_DEFINED__
typedef interface ISpellCheckerFactory ISpellCheckerFactory;

#endif 	/* __ISpellCheckerFactory_FWD_DEFINED__ */


#ifndef __IUserDictionariesRegistrar_FWD_DEFINED__
#define __IUserDictionariesRegistrar_FWD_DEFINED__
typedef interface IUserDictionariesRegistrar IUserDictionariesRegistrar;

#endif 	/* __IUserDictionariesRegistrar_FWD_DEFINED__ */


#ifndef __SpellCheckerFactory_FWD_DEFINED__
#define __SpellCheckerFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class SpellCheckerFactory SpellCheckerFactory;
#else
typedef struct SpellCheckerFactory SpellCheckerFactory;
#endif /* __cplusplus */

#endif 	/* __SpellCheckerFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_spellcheck_0000_0000 */
/* [local] */ 

// Copyright (c) Microsoft Corporation. All Rights Reserved.
#ifndef MIN_SPELLING_NTDDI
#define MIN_SPELLING_NTDDI NTDDI_WIN8
#endif
#if NTDDI_VERSION >= MIN_SPELLING_NTDDI
#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [v1_enum] */ 
enum WORDLIST_TYPE
    {
        WORDLIST_TYPE_IGNORE	= 0,
        WORDLIST_TYPE_ADD	= 1,
        WORDLIST_TYPE_EXCLUDE	= 2,
        WORDLIST_TYPE_AUTOCORRECT	= 3
    } 	WORDLIST_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0000_v0_0_s_ifspec;

#ifndef __ISpellingError_INTERFACE_DEFINED__
#define __ISpellingError_INTERFACE_DEFINED__

/* interface ISpellingError */
/* [unique][uuid][object] */ 

typedef /* [v1_enum] */ 
enum CORRECTIVE_ACTION
    {
        CORRECTIVE_ACTION_NONE	= 0,
        CORRECTIVE_ACTION_GET_SUGGESTIONS	= 1,
        CORRECTIVE_ACTION_REPLACE	= 2,
        CORRECTIVE_ACTION_DELETE	= 3
    } 	CORRECTIVE_ACTION;


EXTERN_C const IID IID_ISpellingError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7C82D61-FBE8-4B47-9B27-6C0D2E0DE0A3")
    ISpellingError : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StartIndex( 
            /* [retval][out] */ __RPC__out ULONG *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Length( 
            /* [retval][out] */ __RPC__out ULONG *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CorrectiveAction( 
            /* [retval][out] */ __RPC__out CORRECTIVE_ACTION *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Replacement( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpellingErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpellingError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpellingError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpellingError * This);
        
        DECLSPEC_XFGVIRT(ISpellingError, get_StartIndex)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartIndex )( 
            __RPC__in ISpellingError * This,
            /* [retval][out] */ __RPC__out ULONG *value);
        
        DECLSPEC_XFGVIRT(ISpellingError, get_Length)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Length )( 
            __RPC__in ISpellingError * This,
            /* [retval][out] */ __RPC__out ULONG *value);
        
        DECLSPEC_XFGVIRT(ISpellingError, get_CorrectiveAction)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CorrectiveAction )( 
            __RPC__in ISpellingError * This,
            /* [retval][out] */ __RPC__out CORRECTIVE_ACTION *value);
        
        DECLSPEC_XFGVIRT(ISpellingError, get_Replacement)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Replacement )( 
            __RPC__in ISpellingError * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        END_INTERFACE
    } ISpellingErrorVtbl;

    interface ISpellingError
    {
        CONST_VTBL struct ISpellingErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpellingError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpellingError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpellingError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpellingError_get_StartIndex(This,value)	\
    ( (This)->lpVtbl -> get_StartIndex(This,value) ) 

#define ISpellingError_get_Length(This,value)	\
    ( (This)->lpVtbl -> get_Length(This,value) ) 

#define ISpellingError_get_CorrectiveAction(This,value)	\
    ( (This)->lpVtbl -> get_CorrectiveAction(This,value) ) 

#define ISpellingError_get_Replacement(This,value)	\
    ( (This)->lpVtbl -> get_Replacement(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpellingError_INTERFACE_DEFINED__ */


#ifndef __IEnumSpellingError_INTERFACE_DEFINED__
#define __IEnumSpellingError_INTERFACE_DEFINED__

/* interface IEnumSpellingError */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumSpellingError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("803E3BD4-2828-4410-8290-418D1D73C762")
    IEnumSpellingError : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__deref_out_opt ISpellingError **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSpellingErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSpellingError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSpellingError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSpellingError * This);
        
        DECLSPEC_XFGVIRT(IEnumSpellingError, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumSpellingError * This,
            /* [retval][out] */ __RPC__deref_out_opt ISpellingError **value);
        
        END_INTERFACE
    } IEnumSpellingErrorVtbl;

    interface IEnumSpellingError
    {
        CONST_VTBL struct IEnumSpellingErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSpellingError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSpellingError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSpellingError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSpellingError_Next(This,value)	\
    ( (This)->lpVtbl -> Next(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumSpellingError_INTERFACE_DEFINED__ */


#ifndef __IOptionDescription_INTERFACE_DEFINED__
#define __IOptionDescription_INTERFACE_DEFINED__

/* interface IOptionDescription */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IOptionDescription;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("432E5F85-35CF-4606-A801-6F70277E1D7A")
    IOptionDescription : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Heading( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Labels( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOptionDescriptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOptionDescription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOptionDescription * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOptionDescription * This);
        
        DECLSPEC_XFGVIRT(IOptionDescription, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IOptionDescription * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IOptionDescription, get_Heading)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Heading )( 
            __RPC__in IOptionDescription * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IOptionDescription, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IOptionDescription * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(IOptionDescription, get_Labels)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Labels )( 
            __RPC__in IOptionDescription * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        END_INTERFACE
    } IOptionDescriptionVtbl;

    interface IOptionDescription
    {
        CONST_VTBL struct IOptionDescriptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOptionDescription_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOptionDescription_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOptionDescription_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOptionDescription_get_Id(This,value)	\
    ( (This)->lpVtbl -> get_Id(This,value) ) 

#define IOptionDescription_get_Heading(This,value)	\
    ( (This)->lpVtbl -> get_Heading(This,value) ) 

#define IOptionDescription_get_Description(This,value)	\
    ( (This)->lpVtbl -> get_Description(This,value) ) 

#define IOptionDescription_get_Labels(This,value)	\
    ( (This)->lpVtbl -> get_Labels(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOptionDescription_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spellcheck_0000_0003 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0003_v0_0_s_ifspec;

#ifndef __ISpellCheckerChangedEventHandler_INTERFACE_DEFINED__
#define __ISpellCheckerChangedEventHandler_INTERFACE_DEFINED__

/* interface ISpellCheckerChangedEventHandler */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISpellCheckerChangedEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0B83A5B0-792F-4EAB-9799-ACF52C5ED08A")
    ISpellCheckerChangedEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ __RPC__in_opt ISpellChecker *sender) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpellCheckerChangedEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpellCheckerChangedEventHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpellCheckerChangedEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpellCheckerChangedEventHandler * This);
        
        DECLSPEC_XFGVIRT(ISpellCheckerChangedEventHandler, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in ISpellCheckerChangedEventHandler * This,
            /* [in] */ __RPC__in_opt ISpellChecker *sender);
        
        END_INTERFACE
    } ISpellCheckerChangedEventHandlerVtbl;

    interface ISpellCheckerChangedEventHandler
    {
        CONST_VTBL struct ISpellCheckerChangedEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpellCheckerChangedEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpellCheckerChangedEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpellCheckerChangedEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpellCheckerChangedEventHandler_Invoke(This,sender)	\
    ( (This)->lpVtbl -> Invoke(This,sender) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpellCheckerChangedEventHandler_INTERFACE_DEFINED__ */


#ifndef __ISpellChecker_INTERFACE_DEFINED__
#define __ISpellChecker_INTERFACE_DEFINED__

/* interface ISpellChecker */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISpellChecker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B6FD0B71-E2BC-4653-8D05-F197E412770B")
    ISpellChecker : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LanguageTag( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Check( 
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Suggest( 
            /* [in] */ __RPC__in LPCWSTR word,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in LPCWSTR word) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Ignore( 
            /* [in] */ __RPC__in LPCWSTR word) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AutoCorrect( 
            /* [in] */ __RPC__in LPCWSTR from,
            /* [in] */ __RPC__in LPCWSTR to) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptionValue( 
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__out BYTE *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OptionIds( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LocalizedName( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE add_SpellCheckerChanged( 
            /* [in] */ __RPC__in_opt ISpellCheckerChangedEventHandler *handler,
            /* [retval][out] */ __RPC__out DWORD *eventCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE remove_SpellCheckerChanged( 
            /* [in] */ DWORD eventCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptionDescription( 
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__deref_out_opt IOptionDescription **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ComprehensiveCheck( 
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpellCheckerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpellChecker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpellChecker * This);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_LanguageTag)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LanguageTag )( 
            __RPC__in ISpellChecker * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Check)
        HRESULT ( STDMETHODCALLTYPE *Check )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Suggest)
        HRESULT ( STDMETHODCALLTYPE *Suggest )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR word,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR word);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Ignore)
        HRESULT ( STDMETHODCALLTYPE *Ignore )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR word);
        
        DECLSPEC_XFGVIRT(ISpellChecker, AutoCorrect)
        HRESULT ( STDMETHODCALLTYPE *AutoCorrect )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR from,
            /* [in] */ __RPC__in LPCWSTR to);
        
        DECLSPEC_XFGVIRT(ISpellChecker, GetOptionValue)
        HRESULT ( STDMETHODCALLTYPE *GetOptionValue )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__out BYTE *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_OptionIds)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OptionIds )( 
            __RPC__in ISpellChecker * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in ISpellChecker * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_LocalizedName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalizedName )( 
            __RPC__in ISpellChecker * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, add_SpellCheckerChanged)
        HRESULT ( STDMETHODCALLTYPE *add_SpellCheckerChanged )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in_opt ISpellCheckerChangedEventHandler *handler,
            /* [retval][out] */ __RPC__out DWORD *eventCookie);
        
        DECLSPEC_XFGVIRT(ISpellChecker, remove_SpellCheckerChanged)
        HRESULT ( STDMETHODCALLTYPE *remove_SpellCheckerChanged )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ DWORD eventCookie);
        
        DECLSPEC_XFGVIRT(ISpellChecker, GetOptionDescription)
        HRESULT ( STDMETHODCALLTYPE *GetOptionDescription )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__deref_out_opt IOptionDescription **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, ComprehensiveCheck)
        HRESULT ( STDMETHODCALLTYPE *ComprehensiveCheck )( 
            __RPC__in ISpellChecker * This,
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value);
        
        END_INTERFACE
    } ISpellCheckerVtbl;

    interface ISpellChecker
    {
        CONST_VTBL struct ISpellCheckerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpellChecker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpellChecker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpellChecker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpellChecker_get_LanguageTag(This,value)	\
    ( (This)->lpVtbl -> get_LanguageTag(This,value) ) 

#define ISpellChecker_Check(This,text,value)	\
    ( (This)->lpVtbl -> Check(This,text,value) ) 

#define ISpellChecker_Suggest(This,word,value)	\
    ( (This)->lpVtbl -> Suggest(This,word,value) ) 

#define ISpellChecker_Add(This,word)	\
    ( (This)->lpVtbl -> Add(This,word) ) 

#define ISpellChecker_Ignore(This,word)	\
    ( (This)->lpVtbl -> Ignore(This,word) ) 

#define ISpellChecker_AutoCorrect(This,from,to)	\
    ( (This)->lpVtbl -> AutoCorrect(This,from,to) ) 

#define ISpellChecker_GetOptionValue(This,optionId,value)	\
    ( (This)->lpVtbl -> GetOptionValue(This,optionId,value) ) 

#define ISpellChecker_get_OptionIds(This,value)	\
    ( (This)->lpVtbl -> get_OptionIds(This,value) ) 

#define ISpellChecker_get_Id(This,value)	\
    ( (This)->lpVtbl -> get_Id(This,value) ) 

#define ISpellChecker_get_LocalizedName(This,value)	\
    ( (This)->lpVtbl -> get_LocalizedName(This,value) ) 

#define ISpellChecker_add_SpellCheckerChanged(This,handler,eventCookie)	\
    ( (This)->lpVtbl -> add_SpellCheckerChanged(This,handler,eventCookie) ) 

#define ISpellChecker_remove_SpellCheckerChanged(This,eventCookie)	\
    ( (This)->lpVtbl -> remove_SpellCheckerChanged(This,eventCookie) ) 

#define ISpellChecker_GetOptionDescription(This,optionId,value)	\
    ( (This)->lpVtbl -> GetOptionDescription(This,optionId,value) ) 

#define ISpellChecker_ComprehensiveCheck(This,text,value)	\
    ( (This)->lpVtbl -> ComprehensiveCheck(This,text,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpellChecker_INTERFACE_DEFINED__ */


#ifndef __ISpellChecker2_INTERFACE_DEFINED__
#define __ISpellChecker2_INTERFACE_DEFINED__

/* interface ISpellChecker2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISpellChecker2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E7ED1C71-87F7-4378-A840-C9200DACEE47")
    ISpellChecker2 : public ISpellChecker
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in LPCWSTR word) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpellChecker2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpellChecker2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpellChecker2 * This);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_LanguageTag)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LanguageTag )( 
            __RPC__in ISpellChecker2 * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Check)
        HRESULT ( STDMETHODCALLTYPE *Check )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Suggest)
        HRESULT ( STDMETHODCALLTYPE *Suggest )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR word,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR word);
        
        DECLSPEC_XFGVIRT(ISpellChecker, Ignore)
        HRESULT ( STDMETHODCALLTYPE *Ignore )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR word);
        
        DECLSPEC_XFGVIRT(ISpellChecker, AutoCorrect)
        HRESULT ( STDMETHODCALLTYPE *AutoCorrect )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR from,
            /* [in] */ __RPC__in LPCWSTR to);
        
        DECLSPEC_XFGVIRT(ISpellChecker, GetOptionValue)
        HRESULT ( STDMETHODCALLTYPE *GetOptionValue )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__out BYTE *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_OptionIds)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OptionIds )( 
            __RPC__in ISpellChecker2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in ISpellChecker2 * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, get_LocalizedName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalizedName )( 
            __RPC__in ISpellChecker2 * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, add_SpellCheckerChanged)
        HRESULT ( STDMETHODCALLTYPE *add_SpellCheckerChanged )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in_opt ISpellCheckerChangedEventHandler *handler,
            /* [retval][out] */ __RPC__out DWORD *eventCookie);
        
        DECLSPEC_XFGVIRT(ISpellChecker, remove_SpellCheckerChanged)
        HRESULT ( STDMETHODCALLTYPE *remove_SpellCheckerChanged )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ DWORD eventCookie);
        
        DECLSPEC_XFGVIRT(ISpellChecker, GetOptionDescription)
        HRESULT ( STDMETHODCALLTYPE *GetOptionDescription )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR optionId,
            /* [retval][out] */ __RPC__deref_out_opt IOptionDescription **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker, ComprehensiveCheck)
        HRESULT ( STDMETHODCALLTYPE *ComprehensiveCheck )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR text,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSpellingError **value);
        
        DECLSPEC_XFGVIRT(ISpellChecker2, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ISpellChecker2 * This,
            /* [in] */ __RPC__in LPCWSTR word);
        
        END_INTERFACE
    } ISpellChecker2Vtbl;

    interface ISpellChecker2
    {
        CONST_VTBL struct ISpellChecker2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpellChecker2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpellChecker2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpellChecker2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpellChecker2_get_LanguageTag(This,value)	\
    ( (This)->lpVtbl -> get_LanguageTag(This,value) ) 

#define ISpellChecker2_Check(This,text,value)	\
    ( (This)->lpVtbl -> Check(This,text,value) ) 

#define ISpellChecker2_Suggest(This,word,value)	\
    ( (This)->lpVtbl -> Suggest(This,word,value) ) 

#define ISpellChecker2_Add(This,word)	\
    ( (This)->lpVtbl -> Add(This,word) ) 

#define ISpellChecker2_Ignore(This,word)	\
    ( (This)->lpVtbl -> Ignore(This,word) ) 

#define ISpellChecker2_AutoCorrect(This,from,to)	\
    ( (This)->lpVtbl -> AutoCorrect(This,from,to) ) 

#define ISpellChecker2_GetOptionValue(This,optionId,value)	\
    ( (This)->lpVtbl -> GetOptionValue(This,optionId,value) ) 

#define ISpellChecker2_get_OptionIds(This,value)	\
    ( (This)->lpVtbl -> get_OptionIds(This,value) ) 

#define ISpellChecker2_get_Id(This,value)	\
    ( (This)->lpVtbl -> get_Id(This,value) ) 

#define ISpellChecker2_get_LocalizedName(This,value)	\
    ( (This)->lpVtbl -> get_LocalizedName(This,value) ) 

#define ISpellChecker2_add_SpellCheckerChanged(This,handler,eventCookie)	\
    ( (This)->lpVtbl -> add_SpellCheckerChanged(This,handler,eventCookie) ) 

#define ISpellChecker2_remove_SpellCheckerChanged(This,eventCookie)	\
    ( (This)->lpVtbl -> remove_SpellCheckerChanged(This,eventCookie) ) 

#define ISpellChecker2_GetOptionDescription(This,optionId,value)	\
    ( (This)->lpVtbl -> GetOptionDescription(This,optionId,value) ) 

#define ISpellChecker2_ComprehensiveCheck(This,text,value)	\
    ( (This)->lpVtbl -> ComprehensiveCheck(This,text,value) ) 


#define ISpellChecker2_Remove(This,word)	\
    ( (This)->lpVtbl -> Remove(This,word) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpellChecker2_INTERFACE_DEFINED__ */


#ifndef __ISpellCheckerFactory_INTERFACE_DEFINED__
#define __ISpellCheckerFactory_INTERFACE_DEFINED__

/* interface ISpellCheckerFactory */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISpellCheckerFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E018A9D-2415-4677-BF08-794EA61F94BB")
    ISpellCheckerFactory : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SupportedLanguages( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSupported( 
            /* [in] */ __RPC__in LPCWSTR languageTag,
            /* [retval][out] */ __RPC__out BOOL *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSpellChecker( 
            /* [in] */ __RPC__in LPCWSTR languageTag,
            /* [retval][out] */ __RPC__deref_out_opt ISpellChecker **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpellCheckerFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpellCheckerFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpellCheckerFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpellCheckerFactory * This);
        
        DECLSPEC_XFGVIRT(ISpellCheckerFactory, get_SupportedLanguages)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedLanguages )( 
            __RPC__in ISpellCheckerFactory * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumString **value);
        
        DECLSPEC_XFGVIRT(ISpellCheckerFactory, IsSupported)
        HRESULT ( STDMETHODCALLTYPE *IsSupported )( 
            __RPC__in ISpellCheckerFactory * This,
            /* [in] */ __RPC__in LPCWSTR languageTag,
            /* [retval][out] */ __RPC__out BOOL *value);
        
        DECLSPEC_XFGVIRT(ISpellCheckerFactory, CreateSpellChecker)
        HRESULT ( STDMETHODCALLTYPE *CreateSpellChecker )( 
            __RPC__in ISpellCheckerFactory * This,
            /* [in] */ __RPC__in LPCWSTR languageTag,
            /* [retval][out] */ __RPC__deref_out_opt ISpellChecker **value);
        
        END_INTERFACE
    } ISpellCheckerFactoryVtbl;

    interface ISpellCheckerFactory
    {
        CONST_VTBL struct ISpellCheckerFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpellCheckerFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpellCheckerFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpellCheckerFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpellCheckerFactory_get_SupportedLanguages(This,value)	\
    ( (This)->lpVtbl -> get_SupportedLanguages(This,value) ) 

#define ISpellCheckerFactory_IsSupported(This,languageTag,value)	\
    ( (This)->lpVtbl -> IsSupported(This,languageTag,value) ) 

#define ISpellCheckerFactory_CreateSpellChecker(This,languageTag,value)	\
    ( (This)->lpVtbl -> CreateSpellChecker(This,languageTag,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpellCheckerFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spellcheck_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0007_v0_0_s_ifspec;

#ifndef __IUserDictionariesRegistrar_INTERFACE_DEFINED__
#define __IUserDictionariesRegistrar_INTERFACE_DEFINED__

/* interface IUserDictionariesRegistrar */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IUserDictionariesRegistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA176B85-0E12-4844-8E1A-EEF1DA77F586")
    IUserDictionariesRegistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterUserDictionary( 
            /* [in] */ __RPC__in LPCWSTR dictionaryPath,
            /* [in] */ __RPC__in LPCWSTR languageTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterUserDictionary( 
            /* [in] */ __RPC__in LPCWSTR dictionaryPath,
            /* [in] */ __RPC__in LPCWSTR languageTag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserDictionariesRegistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUserDictionariesRegistrar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUserDictionariesRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUserDictionariesRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUserDictionariesRegistrar, RegisterUserDictionary)
        HRESULT ( STDMETHODCALLTYPE *RegisterUserDictionary )( 
            __RPC__in IUserDictionariesRegistrar * This,
            /* [in] */ __RPC__in LPCWSTR dictionaryPath,
            /* [in] */ __RPC__in LPCWSTR languageTag);
        
        DECLSPEC_XFGVIRT(IUserDictionariesRegistrar, UnregisterUserDictionary)
        HRESULT ( STDMETHODCALLTYPE *UnregisterUserDictionary )( 
            __RPC__in IUserDictionariesRegistrar * This,
            /* [in] */ __RPC__in LPCWSTR dictionaryPath,
            /* [in] */ __RPC__in LPCWSTR languageTag);
        
        END_INTERFACE
    } IUserDictionariesRegistrarVtbl;

    interface IUserDictionariesRegistrar
    {
        CONST_VTBL struct IUserDictionariesRegistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserDictionariesRegistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserDictionariesRegistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserDictionariesRegistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserDictionariesRegistrar_RegisterUserDictionary(This,dictionaryPath,languageTag)	\
    ( (This)->lpVtbl -> RegisterUserDictionary(This,dictionaryPath,languageTag) ) 

#define IUserDictionariesRegistrar_UnregisterUserDictionary(This,dictionaryPath,languageTag)	\
    ( (This)->lpVtbl -> UnregisterUserDictionary(This,dictionaryPath,languageTag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserDictionariesRegistrar_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spellcheck_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0008_v0_0_s_ifspec;


#ifndef __MsSpellCheckLib_LIBRARY_DEFINED__
#define __MsSpellCheckLib_LIBRARY_DEFINED__

/* library MsSpellCheckLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_MsSpellCheckLib;

EXTERN_C const CLSID CLSID_SpellCheckerFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("7AB36653-1796-484B-BDFA-E74F1DB7C1DC")
SpellCheckerFactory;
#endif
#endif /* __MsSpellCheckLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_spellcheck_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= MIN_SPELLING_NTDDI)


extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spellcheck_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


