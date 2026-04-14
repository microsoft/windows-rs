

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

#ifndef __indexsrv_h__
#define __indexsrv_h__

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

#ifndef __IPhraseSink_FWD_DEFINED__
#define __IPhraseSink_FWD_DEFINED__
typedef interface IPhraseSink IPhraseSink;

#endif 	/* __IPhraseSink_FWD_DEFINED__ */


#ifndef __IWordSink_FWD_DEFINED__
#define __IWordSink_FWD_DEFINED__
typedef interface IWordSink IWordSink;

#endif 	/* __IWordSink_FWD_DEFINED__ */


#ifndef __IWordBreaker_FWD_DEFINED__
#define __IWordBreaker_FWD_DEFINED__
typedef interface IWordBreaker IWordBreaker;

#endif 	/* __IWordBreaker_FWD_DEFINED__ */


#ifndef __IWordFormSink_FWD_DEFINED__
#define __IWordFormSink_FWD_DEFINED__
typedef interface IWordFormSink IWordFormSink;

#endif 	/* __IWordFormSink_FWD_DEFINED__ */


#ifndef __IStemmer_FWD_DEFINED__
#define __IStemmer_FWD_DEFINED__
typedef interface IStemmer IStemmer;

#endif 	/* __IStemmer_FWD_DEFINED__ */


#ifndef __ISimpleCommandCreator_FWD_DEFINED__
#define __ISimpleCommandCreator_FWD_DEFINED__
typedef interface ISimpleCommandCreator ISimpleCommandCreator;

#endif 	/* __ISimpleCommandCreator_FWD_DEFINED__ */


#ifndef __IColumnMapper_FWD_DEFINED__
#define __IColumnMapper_FWD_DEFINED__
typedef interface IColumnMapper IColumnMapper;

#endif 	/* __IColumnMapper_FWD_DEFINED__ */


#ifndef __IColumnMapperCreator_FWD_DEFINED__
#define __IColumnMapperCreator_FWD_DEFINED__
typedef interface IColumnMapperCreator IColumnMapperCreator;

#endif 	/* __IColumnMapperCreator_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "filter.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_indexsrv_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0000_v0_0_s_ifspec;

#ifndef __IPhraseSink_INTERFACE_DEFINED__
#define __IPhraseSink_INTERFACE_DEFINED__

/* interface IPhraseSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPhraseSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CC906FF0-C058-101A-B554-08002B33B0E6")
    IPhraseSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutSmallPhrase( 
            /* [size_is][in] */ const WCHAR *pwcNoun,
            /* [in] */ ULONG cwcNoun,
            /* [size_is][in] */ const WCHAR *pwcModifier,
            /* [in] */ ULONG cwcModifier,
            /* [in] */ ULONG ulAttachmentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutPhrase( 
            /* [size_is][in] */ const WCHAR *pwcPhrase,
            /* [in] */ ULONG cwcPhrase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPhraseSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPhraseSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPhraseSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPhraseSink * This);
        
        DECLSPEC_XFGVIRT(IPhraseSink, PutSmallPhrase)
        HRESULT ( STDMETHODCALLTYPE *PutSmallPhrase )( 
            IPhraseSink * This,
            /* [size_is][in] */ const WCHAR *pwcNoun,
            /* [in] */ ULONG cwcNoun,
            /* [size_is][in] */ const WCHAR *pwcModifier,
            /* [in] */ ULONG cwcModifier,
            /* [in] */ ULONG ulAttachmentType);
        
        DECLSPEC_XFGVIRT(IPhraseSink, PutPhrase)
        HRESULT ( STDMETHODCALLTYPE *PutPhrase )( 
            IPhraseSink * This,
            /* [size_is][in] */ const WCHAR *pwcPhrase,
            /* [in] */ ULONG cwcPhrase);
        
        END_INTERFACE
    } IPhraseSinkVtbl;

    interface IPhraseSink
    {
        CONST_VTBL struct IPhraseSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPhraseSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPhraseSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPhraseSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPhraseSink_PutSmallPhrase(This,pwcNoun,cwcNoun,pwcModifier,cwcModifier,ulAttachmentType)	\
    ( (This)->lpVtbl -> PutSmallPhrase(This,pwcNoun,cwcNoun,pwcModifier,cwcModifier,ulAttachmentType) ) 

#define IPhraseSink_PutPhrase(This,pwcPhrase,cwcPhrase)	\
    ( (This)->lpVtbl -> PutPhrase(This,pwcPhrase,cwcPhrase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPhraseSink_INTERFACE_DEFINED__ */


#ifndef __IWordSink_INTERFACE_DEFINED__
#define __IWordSink_INTERFACE_DEFINED__

/* interface IWordSink */
/* [unique][uuid][object][local] */ 

#ifndef _tagWORDREP_BREAK_TYPE_DEFINED
typedef 
enum tagWORDREP_BREAK_TYPE
    {
        WORDREP_BREAK_EOW	= 0,
        WORDREP_BREAK_EOS	= 1,
        WORDREP_BREAK_EOP	= 2,
        WORDREP_BREAK_EOC	= 3
    } 	WORDREP_BREAK_TYPE;

#define _tagWORDREP_BREAK_TYPE_DEFINED
#define _WORDREP_BREAK_TYPE_DEFINED
#endif

EXTERN_C const IID IID_IWordSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CC907054-C058-101A-B554-08002B33B0E6")
    IWordSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutWord( 
            /* [in] */ ULONG cwc,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwcSrcLen,
            /* [in] */ ULONG cwcSrcPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutAltWord( 
            /* [in] */ ULONG cwc,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwcSrcLen,
            /* [in] */ ULONG cwcSrcPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartAltPhrase( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndAltPhrase( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutBreak( 
            /* [in] */ WORDREP_BREAK_TYPE breakType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWordSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWordSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWordSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWordSink * This);
        
        DECLSPEC_XFGVIRT(IWordSink, PutWord)
        HRESULT ( STDMETHODCALLTYPE *PutWord )( 
            IWordSink * This,
            /* [in] */ ULONG cwc,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwcSrcLen,
            /* [in] */ ULONG cwcSrcPos);
        
        DECLSPEC_XFGVIRT(IWordSink, PutAltWord)
        HRESULT ( STDMETHODCALLTYPE *PutAltWord )( 
            IWordSink * This,
            /* [in] */ ULONG cwc,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwcSrcLen,
            /* [in] */ ULONG cwcSrcPos);
        
        DECLSPEC_XFGVIRT(IWordSink, StartAltPhrase)
        HRESULT ( STDMETHODCALLTYPE *StartAltPhrase )( 
            IWordSink * This);
        
        DECLSPEC_XFGVIRT(IWordSink, EndAltPhrase)
        HRESULT ( STDMETHODCALLTYPE *EndAltPhrase )( 
            IWordSink * This);
        
        DECLSPEC_XFGVIRT(IWordSink, PutBreak)
        HRESULT ( STDMETHODCALLTYPE *PutBreak )( 
            IWordSink * This,
            /* [in] */ WORDREP_BREAK_TYPE breakType);
        
        END_INTERFACE
    } IWordSinkVtbl;

    interface IWordSink
    {
        CONST_VTBL struct IWordSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWordSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWordSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWordSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWordSink_PutWord(This,cwc,pwcInBuf,cwcSrcLen,cwcSrcPos)	\
    ( (This)->lpVtbl -> PutWord(This,cwc,pwcInBuf,cwcSrcLen,cwcSrcPos) ) 

#define IWordSink_PutAltWord(This,cwc,pwcInBuf,cwcSrcLen,cwcSrcPos)	\
    ( (This)->lpVtbl -> PutAltWord(This,cwc,pwcInBuf,cwcSrcLen,cwcSrcPos) ) 

#define IWordSink_StartAltPhrase(This)	\
    ( (This)->lpVtbl -> StartAltPhrase(This) ) 

#define IWordSink_EndAltPhrase(This)	\
    ( (This)->lpVtbl -> EndAltPhrase(This) ) 

#define IWordSink_PutBreak(This,breakType)	\
    ( (This)->lpVtbl -> PutBreak(This,breakType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWordSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_indexsrv_0000_0002 */
/* [local] */ 

#ifndef _tagTEXT_SOURCE_DEFINED

typedef HRESULT ( __stdcall *PFNFILLTEXTBUFFER )( 
    struct tagTEXT_SOURCE *pTextSource);

typedef struct tagTEXT_SOURCE
    {
    PFNFILLTEXTBUFFER pfnFillTextBuffer;
    const WCHAR *awcBuffer;
    ULONG iEnd;
    ULONG iCur;
    } 	TEXT_SOURCE;

#define _tagTEXT_SOURCE_DEFINED
#define _TEXT_SOURCE_DEFINED
#endif


extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0002_v0_0_s_ifspec;

#ifndef __IWordBreaker_INTERFACE_DEFINED__
#define __IWordBreaker_INTERFACE_DEFINED__

/* interface IWordBreaker */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IWordBreaker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D53552C8-77E3-101A-B552-08002B33B0E6")
    IWordBreaker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ BOOL fQuery,
            /* [in] */ ULONG ulMaxTokenSize,
            /* [out] */ BOOL *pfLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BreakText( 
            /* [in] */ TEXT_SOURCE *pTextSource,
            /* [in] */ IWordSink *pWordSink,
            /* [in] */ IPhraseSink *pPhraseSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ComposePhrase( 
            /* [size_is][in] */ const WCHAR *pwcNoun,
            /* [in] */ ULONG cwcNoun,
            /* [size_is][in] */ const WCHAR *pwcModifier,
            /* [in] */ ULONG cwcModifier,
            /* [in] */ ULONG ulAttachmentType,
            /* [size_is][out] */ WCHAR *pwcPhrase,
            /* [out][in] */ ULONG *pcwcPhrase) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseToUse( 
            /* [string][out] */ const WCHAR **ppwcsLicense) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWordBreakerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWordBreaker * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWordBreaker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWordBreaker * This);
        
        DECLSPEC_XFGVIRT(IWordBreaker, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            IWordBreaker * This,
            /* [in] */ BOOL fQuery,
            /* [in] */ ULONG ulMaxTokenSize,
            /* [out] */ BOOL *pfLicense);
        
        DECLSPEC_XFGVIRT(IWordBreaker, BreakText)
        HRESULT ( STDMETHODCALLTYPE *BreakText )( 
            IWordBreaker * This,
            /* [in] */ TEXT_SOURCE *pTextSource,
            /* [in] */ IWordSink *pWordSink,
            /* [in] */ IPhraseSink *pPhraseSink);
        
        DECLSPEC_XFGVIRT(IWordBreaker, ComposePhrase)
        HRESULT ( STDMETHODCALLTYPE *ComposePhrase )( 
            IWordBreaker * This,
            /* [size_is][in] */ const WCHAR *pwcNoun,
            /* [in] */ ULONG cwcNoun,
            /* [size_is][in] */ const WCHAR *pwcModifier,
            /* [in] */ ULONG cwcModifier,
            /* [in] */ ULONG ulAttachmentType,
            /* [size_is][out] */ WCHAR *pwcPhrase,
            /* [out][in] */ ULONG *pcwcPhrase);
        
        DECLSPEC_XFGVIRT(IWordBreaker, GetLicenseToUse)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseToUse )( 
            IWordBreaker * This,
            /* [string][out] */ const WCHAR **ppwcsLicense);
        
        END_INTERFACE
    } IWordBreakerVtbl;

    interface IWordBreaker
    {
        CONST_VTBL struct IWordBreakerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWordBreaker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWordBreaker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWordBreaker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWordBreaker_Init(This,fQuery,ulMaxTokenSize,pfLicense)	\
    ( (This)->lpVtbl -> Init(This,fQuery,ulMaxTokenSize,pfLicense) ) 

#define IWordBreaker_BreakText(This,pTextSource,pWordSink,pPhraseSink)	\
    ( (This)->lpVtbl -> BreakText(This,pTextSource,pWordSink,pPhraseSink) ) 

#define IWordBreaker_ComposePhrase(This,pwcNoun,cwcNoun,pwcModifier,cwcModifier,ulAttachmentType,pwcPhrase,pcwcPhrase)	\
    ( (This)->lpVtbl -> ComposePhrase(This,pwcNoun,cwcNoun,pwcModifier,cwcModifier,ulAttachmentType,pwcPhrase,pcwcPhrase) ) 

#define IWordBreaker_GetLicenseToUse(This,ppwcsLicense)	\
    ( (This)->lpVtbl -> GetLicenseToUse(This,ppwcsLicense) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWordBreaker_INTERFACE_DEFINED__ */


#ifndef __IWordFormSink_INTERFACE_DEFINED__
#define __IWordFormSink_INTERFACE_DEFINED__

/* interface IWordFormSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IWordFormSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fe77c330-7f42-11ce-be57-00aa0051fe20")
    IWordFormSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutAltWord( 
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutWord( 
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWordFormSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWordFormSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWordFormSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWordFormSink * This);
        
        DECLSPEC_XFGVIRT(IWordFormSink, PutAltWord)
        HRESULT ( STDMETHODCALLTYPE *PutAltWord )( 
            IWordFormSink * This,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc);
        
        DECLSPEC_XFGVIRT(IWordFormSink, PutWord)
        HRESULT ( STDMETHODCALLTYPE *PutWord )( 
            IWordFormSink * This,
            /* [size_is][in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc);
        
        END_INTERFACE
    } IWordFormSinkVtbl;

    interface IWordFormSink
    {
        CONST_VTBL struct IWordFormSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWordFormSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWordFormSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWordFormSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWordFormSink_PutAltWord(This,pwcInBuf,cwc)	\
    ( (This)->lpVtbl -> PutAltWord(This,pwcInBuf,cwc) ) 

#define IWordFormSink_PutWord(This,pwcInBuf,cwc)	\
    ( (This)->lpVtbl -> PutWord(This,pwcInBuf,cwc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWordFormSink_INTERFACE_DEFINED__ */


#ifndef __IStemmer_INTERFACE_DEFINED__
#define __IStemmer_INTERFACE_DEFINED__

/* interface IStemmer */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IStemmer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("efbaf140-7f42-11ce-be57-00aa0051fe20")
    IStemmer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ ULONG ulMaxTokenSize,
            /* [out] */ BOOL *pfLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateWordForms( 
            /* [in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc,
            /* [in] */ IWordFormSink *pStemSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseToUse( 
            /* [string][out] */ const WCHAR **ppwcsLicense) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStemmerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStemmer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStemmer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStemmer * This);
        
        DECLSPEC_XFGVIRT(IStemmer, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            IStemmer * This,
            /* [in] */ ULONG ulMaxTokenSize,
            /* [out] */ BOOL *pfLicense);
        
        DECLSPEC_XFGVIRT(IStemmer, GenerateWordForms)
        HRESULT ( STDMETHODCALLTYPE *GenerateWordForms )( 
            IStemmer * This,
            /* [in] */ const WCHAR *pwcInBuf,
            /* [in] */ ULONG cwc,
            /* [in] */ IWordFormSink *pStemSink);
        
        DECLSPEC_XFGVIRT(IStemmer, GetLicenseToUse)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseToUse )( 
            IStemmer * This,
            /* [string][out] */ const WCHAR **ppwcsLicense);
        
        END_INTERFACE
    } IStemmerVtbl;

    interface IStemmer
    {
        CONST_VTBL struct IStemmerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStemmer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStemmer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStemmer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStemmer_Init(This,ulMaxTokenSize,pfLicense)	\
    ( (This)->lpVtbl -> Init(This,ulMaxTokenSize,pfLicense) ) 

#define IStemmer_GenerateWordForms(This,pwcInBuf,cwc,pStemSink)	\
    ( (This)->lpVtbl -> GenerateWordForms(This,pwcInBuf,cwc,pStemSink) ) 

#define IStemmer_GetLicenseToUse(This,ppwcsLicense)	\
    ( (This)->lpVtbl -> GetLicenseToUse(This,ppwcsLicense) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStemmer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_indexsrv_0000_0005 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0005_v0_0_s_ifspec;

#ifndef __ISimpleCommandCreator_INTERFACE_DEFINED__
#define __ISimpleCommandCreator_INTERFACE_DEFINED__

/* interface ISimpleCommandCreator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ISimpleCommandCreator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e341ab7-02d0-11d1-900c-00a0c9063796")
    ISimpleCommandCreator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateICommand( 
            IUnknown **ppIUnknown,
            IUnknown *pOuterUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE VerifyCatalog( 
            const WCHAR *pwszMachine,
            const WCHAR *pwszCatalogName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultCatalog( 
            WCHAR *pwszCatalogName,
            ULONG cwcIn,
            ULONG *pcwcOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISimpleCommandCreatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISimpleCommandCreator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISimpleCommandCreator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISimpleCommandCreator * This);
        
        DECLSPEC_XFGVIRT(ISimpleCommandCreator, CreateICommand)
        HRESULT ( STDMETHODCALLTYPE *CreateICommand )( 
            ISimpleCommandCreator * This,
            IUnknown **ppIUnknown,
            IUnknown *pOuterUnk);
        
        DECLSPEC_XFGVIRT(ISimpleCommandCreator, VerifyCatalog)
        HRESULT ( STDMETHODCALLTYPE *VerifyCatalog )( 
            ISimpleCommandCreator * This,
            const WCHAR *pwszMachine,
            const WCHAR *pwszCatalogName);
        
        DECLSPEC_XFGVIRT(ISimpleCommandCreator, GetDefaultCatalog)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultCatalog )( 
            ISimpleCommandCreator * This,
            WCHAR *pwszCatalogName,
            ULONG cwcIn,
            ULONG *pcwcOut);
        
        END_INTERFACE
    } ISimpleCommandCreatorVtbl;

    interface ISimpleCommandCreator
    {
        CONST_VTBL struct ISimpleCommandCreatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISimpleCommandCreator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISimpleCommandCreator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISimpleCommandCreator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISimpleCommandCreator_CreateICommand(This,ppIUnknown,pOuterUnk)	\
    ( (This)->lpVtbl -> CreateICommand(This,ppIUnknown,pOuterUnk) ) 

#define ISimpleCommandCreator_VerifyCatalog(This,pwszMachine,pwszCatalogName)	\
    ( (This)->lpVtbl -> VerifyCatalog(This,pwszMachine,pwszCatalogName) ) 

#define ISimpleCommandCreator_GetDefaultCatalog(This,pwszCatalogName,cwcIn,pcwcOut)	\
    ( (This)->lpVtbl -> GetDefaultCatalog(This,pwszCatalogName,cwcIn,pcwcOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISimpleCommandCreator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_indexsrv_0000_0006 */
/* [local] */ 

#define CLSID_CISimpleCommandCreator {0xc7b6c04a, 0xcbb5, 0x11d0, {0xbb, 0x4c, 0x0, 0xc0, 0x4f, 0xc2, 0xf4, 0x10 } }
typedef struct tagDBID DBID;

typedef WORD DBTYPE;



extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0006_v0_0_s_ifspec;

#ifndef __IColumnMapper_INTERFACE_DEFINED__
#define __IColumnMapper_INTERFACE_DEFINED__

/* interface IColumnMapper */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IColumnMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0b63e37a-9ccc-11d0-bcdb-00805fccce04")
    IColumnMapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropInfoFromName( 
            /* [string][in] */ const WCHAR *wcsPropName,
            /* [out] */ DBID **ppPropId,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropInfoFromId( 
            /* [in] */ const DBID *pPropId,
            /* [out] */ WCHAR **pwcsName,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumPropInfo( 
            /* [in] */ ULONG iEntry,
            /* [out] */ const WCHAR **pwcsName,
            /* [out] */ DBID **ppPropId,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsMapUpToDate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IColumnMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IColumnMapper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IColumnMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IColumnMapper * This);
        
        DECLSPEC_XFGVIRT(IColumnMapper, GetPropInfoFromName)
        HRESULT ( STDMETHODCALLTYPE *GetPropInfoFromName )( 
            IColumnMapper * This,
            /* [string][in] */ const WCHAR *wcsPropName,
            /* [out] */ DBID **ppPropId,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth);
        
        DECLSPEC_XFGVIRT(IColumnMapper, GetPropInfoFromId)
        HRESULT ( STDMETHODCALLTYPE *GetPropInfoFromId )( 
            IColumnMapper * This,
            /* [in] */ const DBID *pPropId,
            /* [out] */ WCHAR **pwcsName,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth);
        
        DECLSPEC_XFGVIRT(IColumnMapper, EnumPropInfo)
        HRESULT ( STDMETHODCALLTYPE *EnumPropInfo )( 
            IColumnMapper * This,
            /* [in] */ ULONG iEntry,
            /* [out] */ const WCHAR **pwcsName,
            /* [out] */ DBID **ppPropId,
            /* [out] */ DBTYPE *pPropType,
            /* [out] */ unsigned int *puiWidth);
        
        DECLSPEC_XFGVIRT(IColumnMapper, IsMapUpToDate)
        HRESULT ( STDMETHODCALLTYPE *IsMapUpToDate )( 
            IColumnMapper * This);
        
        END_INTERFACE
    } IColumnMapperVtbl;

    interface IColumnMapper
    {
        CONST_VTBL struct IColumnMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IColumnMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IColumnMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IColumnMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IColumnMapper_GetPropInfoFromName(This,wcsPropName,ppPropId,pPropType,puiWidth)	\
    ( (This)->lpVtbl -> GetPropInfoFromName(This,wcsPropName,ppPropId,pPropType,puiWidth) ) 

#define IColumnMapper_GetPropInfoFromId(This,pPropId,pwcsName,pPropType,puiWidth)	\
    ( (This)->lpVtbl -> GetPropInfoFromId(This,pPropId,pwcsName,pPropType,puiWidth) ) 

#define IColumnMapper_EnumPropInfo(This,iEntry,pwcsName,ppPropId,pPropType,puiWidth)	\
    ( (This)->lpVtbl -> EnumPropInfo(This,iEntry,pwcsName,ppPropId,pPropType,puiWidth) ) 

#define IColumnMapper_IsMapUpToDate(This)	\
    ( (This)->lpVtbl -> IsMapUpToDate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IColumnMapper_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_indexsrv_0000_0007 */
/* [local] */ 

#define	LOCAL_MACHINE	( L"." )

#define	SYSTEM_DEFAULT_CAT	( L"__SystemDefault__" )

#define	INDEX_SERVER_DEFAULT_CAT	( L"__IndexServerDefault__" )



extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0007_v0_0_s_ifspec;

#ifndef __IColumnMapperCreator_INTERFACE_DEFINED__
#define __IColumnMapperCreator_INTERFACE_DEFINED__

/* interface IColumnMapperCreator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IColumnMapperCreator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0b63e37b-9ccc-11d0-bcdb-00805fccce04")
    IColumnMapperCreator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetColumnMapper( 
            /* [string][in] */ const WCHAR *wcsMachineName,
            /* [string][in] */ const WCHAR *wcsCatalogName,
            /* [out] */ IColumnMapper **ppColumnMapper) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IColumnMapperCreatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IColumnMapperCreator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IColumnMapperCreator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IColumnMapperCreator * This);
        
        DECLSPEC_XFGVIRT(IColumnMapperCreator, GetColumnMapper)
        HRESULT ( STDMETHODCALLTYPE *GetColumnMapper )( 
            IColumnMapperCreator * This,
            /* [string][in] */ const WCHAR *wcsMachineName,
            /* [string][in] */ const WCHAR *wcsCatalogName,
            /* [out] */ IColumnMapper **ppColumnMapper);
        
        END_INTERFACE
    } IColumnMapperCreatorVtbl;

    interface IColumnMapperCreator
    {
        CONST_VTBL struct IColumnMapperCreatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IColumnMapperCreator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IColumnMapperCreator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IColumnMapperCreator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IColumnMapperCreator_GetColumnMapper(This,wcsMachineName,wcsCatalogName,ppColumnMapper)	\
    ( (This)->lpVtbl -> GetColumnMapper(This,wcsMachineName,wcsCatalogName,ppColumnMapper) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IColumnMapperCreator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_indexsrv_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_indexsrv_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


