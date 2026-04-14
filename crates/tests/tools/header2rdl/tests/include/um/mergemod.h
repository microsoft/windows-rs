/*****************************************************************************\
                                                                             
 mergemod.h - - Interface for MergeMod COM object                            
                                                                             
 Version 2.0                                                                 
                                                                             
 NOTES:  All strings are of type BSTR. For [in] strings, LPCWSTR may be      
 safely used instead, the object will not try to free this memory.           
 All [out] BSTR values must be released by the client.                       
                                                                             
 Copyright (c) Microsoft Corp. All rights reserved.          
                                                                             
\*****************************************************************************/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "rpc.h"
#include "rpcndr.h"
#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __mergemod_h__
#define __mergemod_h__

#ifndef _WIN32_MSM
#define _WIN32_MSM   100
#endif // !_WIN32_MSM

#ifdef __cplusplus
extern "C"{
#endif 

/* Forward Declarations */ 

#ifndef __IEnumMsmString_FWD_DEFINED__
#define __IEnumMsmString_FWD_DEFINED__
typedef interface IEnumMsmString IEnumMsmString;
#endif 	/* __IEnumMsmString_FWD_DEFINED__ */


#ifndef __IMsmStrings_FWD_DEFINED__
#define __IMsmStrings_FWD_DEFINED__
typedef interface IMsmStrings IMsmStrings;
#endif 	/* __IMsmStrings_FWD_DEFINED__ */


#ifndef __IMsmError_FWD_DEFINED__
#define __IMsmError_FWD_DEFINED__
typedef interface IMsmError IMsmError;
#endif 	/* __IMsmError_FWD_DEFINED__ */


#ifndef __IEnumMsmError_FWD_DEFINED__
#define __IEnumMsmError_FWD_DEFINED__
typedef interface IEnumMsmError IEnumMsmError;
#endif 	/* __IEnumMsmError_FWD_DEFINED__ */


#ifndef __IMsmErrors_FWD_DEFINED__
#define __IMsmErrors_FWD_DEFINED__
typedef interface IMsmErrors IMsmErrors;
#endif 	/* __IMsmErrors_FWD_DEFINED__ */


#ifndef __IMsmDependency_FWD_DEFINED__
#define __IMsmDependency_FWD_DEFINED__
typedef interface IMsmDependency IMsmDependency;
#endif 	/* __IMsmDependency_FWD_DEFINED__ */


#ifndef __IEnumMsmDependency_FWD_DEFINED__
#define __IEnumMsmDependency_FWD_DEFINED__
typedef interface IEnumMsmDependency IEnumMsmDependency;
#endif 	/* __IEnumMsmDependency_FWD_DEFINED__ */


#ifndef __IMsmDependencies_FWD_DEFINED__
#define __IMsmDependencies_FWD_DEFINED__
typedef interface IMsmDependencies IMsmDependencies;
#endif 	/* __IMsmDependencies_FWD_DEFINED__ */


#ifndef __IMsmMerge_FWD_DEFINED__
#define __IMsmMerge_FWD_DEFINED__
typedef interface IMsmMerge IMsmMerge;
#endif 	/* __IMsmMerge_FWD_DEFINED__ */


#ifndef __IMsmGetFiles_FWD_DEFINED__
#define __IMsmGetFiles_FWD_DEFINED__
typedef interface IMsmGetFiles IMsmGetFiles;
#endif 	/* __IMsmGetFiles_FWD_DEFINED__ */


#ifndef __IMsmStrings_FWD_DEFINED__
#define __IMsmStrings_FWD_DEFINED__
typedef interface IMsmStrings IMsmStrings;
#endif 	/* __IMsmStrings_FWD_DEFINED__ */


#ifndef __IMsmError_FWD_DEFINED__
#define __IMsmError_FWD_DEFINED__
typedef interface IMsmError IMsmError;
#endif 	/* __IMsmError_FWD_DEFINED__ */


#ifndef __IMsmErrors_FWD_DEFINED__
#define __IMsmErrors_FWD_DEFINED__
typedef interface IMsmErrors IMsmErrors;
#endif 	/* __IMsmErrors_FWD_DEFINED__ */


#ifndef __IMsmDependency_FWD_DEFINED__
#define __IMsmDependency_FWD_DEFINED__
typedef interface IMsmDependency IMsmDependency;
#endif 	/* __IMsmDependency_FWD_DEFINED__ */


#ifndef __IMsmDependencies_FWD_DEFINED__
#define __IMsmDependencies_FWD_DEFINED__
typedef interface IMsmDependencies IMsmDependencies;
#endif 	/* __IMsmDependencies_FWD_DEFINED__ */


#ifndef __IMsmGetFiles_FWD_DEFINED__
#define __IMsmGetFiles_FWD_DEFINED__
typedef interface IMsmGetFiles IMsmGetFiles;
#endif 	/* __IMsmGetFiles_FWD_DEFINED__ */

#if (_WIN32_MSM >= 150)

#ifndef __IMsmConfigurableItem_FWD_DEFINED__
#define __IMsmConfigurableItem_FWD_DEFINED__
typedef interface IMsmConfigurableItem IMsmConfigurableItem;
#endif 	/* __IMsmConfigurableItem_FWD_DEFINED__ */


#ifndef __IEnumMsmConfigurableItem_FWD_DEFINED__
#define __IEnumMsmConfigurableItem_FWD_DEFINED__
typedef interface IEnumMsmConfigurableItem IEnumMsmConfigurableItem;
#endif 	/* __IEnumMsmConfigurableItem_FWD_DEFINED__ */


#ifndef __IMsmConfigurableItems_FWD_DEFINED__
#define __IMsmConfigurableItems_FWD_DEFINED__
typedef interface IMsmConfigurableItems IMsmConfigurableItems;
#endif 	/* __IMsmConfigurableItems_FWD_DEFINED__ */

#ifndef __IMsmMerge2_FWD_DEFINED__
#define __IMsmMerge2_FWD_DEFINED__
typedef interface IMsmMerge2 IMsmMerge2;
#endif 	/* __IMsmMerge2_FWD_DEFINED__ */

#ifndef __IMsmConfigureModule_FWD_DEFINED__
#define __IMsmConfigureModule_FWD_DEFINED__
typedef interface IMsmConfigureModule IMsmConfigureModule;
#endif 	/* __IMsmConfigureModule_FWD_DEFINED__ */

#ifndef __MsmMerge2_FWD_DEFINED__
#define __MsmMerge2_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsmMerge2 MsmMerge2;
#else
typedef struct MsmMerge2 MsmMerge2;
#endif /* __cplusplus */

#endif 	/* __MsmMerge2_FWD_DEFINED__ */


#endif /* _WIN32_MSM */

#ifndef __MsmMerge_FWD_DEFINED__
#define __MsmMerge_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsmMerge MsmMerge;
#else
typedef struct MsmMerge MsmMerge;
#endif /* __cplusplus */

#endif 	/* __MsmMerge_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

void __RPC_FAR * __RPC_USER MIDL_user_allocate(size_t);
void __RPC_USER MIDL_user_free( void __RPC_FAR * ); 


#ifndef __FORWARD_IID_IMSMMERGETYPELIB
#define __FORWARD_IID_IMSMMERGETYPELIB

// --------------------------------------------------------------------------
// MergeMod error types, returned from IMsmError::get_Type
// --------------------------------------------------------------------------
typedef /* [helpstring][uuid] */ 
enum msmErrorType
    {	
	msmErrorLanguageUnsupported	= 1,
	msmErrorLanguageFailed          = 2,
	msmErrorExclusion               = 3,
	msmErrorTableMerge              = 4,
	msmErrorResequenceMerge	        = 5,
	msmErrorFileCreate              = 6,
	msmErrorDirCreate               = 7,
	msmErrorFeatureRequired	        = 8,

#if (_WIN32_MSM >= 150)
	msmErrorBadNullSubstitution     = 9,
	msmErrorBadSubstitutionType     = 10,
	msmErrorMissingConfigItem       = 11,
	msmErrorBadNullResponse         = 12,
	msmErrorDataRequestFailed       = 13,    
	msmErrorPlatformMismatch        = 14    
#endif
	}	
	msmErrorType;

#if (_WIN32_MSM >= 150)

// --------------------------------------------------------------------------
// MergeMod formats for ModuleConfiguration items.
// --------------------------------------------------------------------------
typedef /* [helpstring][uuid] */ 
enum msmConfigurableItemFormat
    {
	msmConfigurableItemText = 0,
	msmConfigurableItemKey = 1,
	msmConfigurableItemInteger = 2,
	msmConfigurableItemBitfield = 3
    }
    msmConfigurableItemFormat;


// --------------------------------------------------------------------------
// MergeMod options for ModuleConfiguration items.
// --------------------------------------------------------------------------
typedef /* [helpstring][uuid] */   
enum msmConfigurableItemOptions
    {	
	msmConfigurableOptionKeyNoOrphan	= 1,
	msmConfigurableOptionNonNullable	= 2
    } 	
    msmConfigurableItemOptions;


#endif


#endif // __FORWARD_IID_IMSMMERGETYPELIB

extern RPC_IF_HANDLE __MIDL_itf_mergemod_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mergemod_0000_v0_0_s_ifspec;


// --------------------------------------------------------------------------
// IEnumMsmString - enumeration of BSTR
// --------------------------------------------------------------------------

#ifndef __IEnumMsmString_INTERFACE_DEFINED__
#define __IEnumMsmString_INTERFACE_DEFINED__


#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA826-2C26-11D2-AD65-00A0C9AF11A6")
    IEnumMsmString : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ unsigned long cFetch,
            /* [out] */ BSTR __RPC_FAR *rgbstrStrings,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ unsigned long cSkip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumMsmString __RPC_FAR *__RPC_FAR *pemsmStrings) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IEnumMsmStringVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IEnumMsmString __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IEnumMsmString __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IEnumMsmString __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Next )( 
            IEnumMsmString __RPC_FAR * This,
            /* [in] */ unsigned long cFetch,
            /* [out] */ BSTR __RPC_FAR *rgbstrStrings,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Skip )( 
            IEnumMsmString __RPC_FAR * This,
            /* [in] */ unsigned long cSkip);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Reset )( 
            IEnumMsmString __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Clone )( 
            IEnumMsmString __RPC_FAR * This,
            /* [retval][out] */ IEnumMsmString __RPC_FAR *__RPC_FAR *pemsmStrings);
        
        END_INTERFACE
    } IEnumMsmStringVtbl;

    interface IEnumMsmString
    {
        CONST_VTBL struct IEnumMsmStringVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMsmString_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IEnumMsmString_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IEnumMsmString_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IEnumMsmString_Next(This,cFetch,rgbstrStrings,pcFetched)	\
    (This)->lpVtbl -> Next(This,cFetch,rgbstrStrings,pcFetched)

#define IEnumMsmString_Skip(This,cSkip)	\
    (This)->lpVtbl -> Skip(This,cSkip)

#define IEnumMsmString_Reset(This)	\
    (This)->lpVtbl -> Reset(This)

#define IEnumMsmString_Clone(This,pemsmStrings)	\
    (This)->lpVtbl -> Clone(This,pemsmStrings)

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IEnumMsmString_Next_Proxy( 
    IEnumMsmString __RPC_FAR * This,
    /* [in] */ unsigned long cFetch,
    /* [out] */ BSTR __RPC_FAR *rgbstrStrings,
    /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);


void __RPC_STUB IEnumMsmString_Next_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmString_Skip_Proxy( 
    IEnumMsmString __RPC_FAR * This,
    /* [in] */ unsigned long cSkip);


void __RPC_STUB IEnumMsmString_Skip_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmString_Reset_Proxy( 
    IEnumMsmString __RPC_FAR * This);


void __RPC_STUB IEnumMsmString_Reset_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmString_Clone_Proxy( 
    IEnumMsmString __RPC_FAR * This,
    /* [retval][out] */ IEnumMsmString __RPC_FAR *__RPC_FAR *pemsmStrings);


void __RPC_STUB IEnumMsmString_Clone_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumMsmString_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmStrings - a collection of MergeMod BSTR
// --------------------------------------------------------------------------

#ifndef __IMsmStrings_INTERFACE_DEFINED__
#define __IMsmStrings_INTERFACE_DEFINED__


#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA827-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmStrings : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Item,
            /* [retval][out] */ BSTR __RPC_FAR *Return) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long __RPC_FAR *Count) = 0;
        
        virtual /* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmStringsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmStrings __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmStrings __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmStrings __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmStrings __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmStrings __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmStrings __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmStrings __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Item )( 
            IMsmStrings __RPC_FAR * This,
            /* [in] */ long Item,
            /* [retval][out] */ BSTR __RPC_FAR *Return);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Count )( 
            IMsmStrings __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *Count);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get__NewEnum )( 
            IMsmStrings __RPC_FAR * This,
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);
        
        END_INTERFACE
    } IMsmStringsVtbl;

    interface IMsmStrings
    {
        CONST_VTBL struct IMsmStringsVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmStrings_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmStrings_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmStrings_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmStrings_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmStrings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmStrings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmStrings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmStrings_get_Item(This,Item,Return)	\
    (This)->lpVtbl -> get_Item(This,Item,Return)

#define IMsmStrings_get_Count(This,Count)	\
    (This)->lpVtbl -> get_Count(This,Count)

#define IMsmStrings_get__NewEnum(This,NewEnum)	\
    (This)->lpVtbl -> get__NewEnum(This,NewEnum)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmStrings_get_Item_Proxy( 
    IMsmStrings __RPC_FAR * This,
    /* [in] */ long Item,
    /* [retval][out] */ BSTR __RPC_FAR *Return);


void __RPC_STUB IMsmStrings_get_Item_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmStrings_get_Count_Proxy( 
    IMsmStrings __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *Count);


void __RPC_STUB IMsmStrings_get_Count_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmStrings_get__NewEnum_Proxy( 
    IMsmStrings __RPC_FAR * This,
    /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);


void __RPC_STUB IMsmStrings_get__NewEnum_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmStrings_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmError - interface for retrieving details on a single merge error
// --------------------------------------------------------------------------

#ifndef __IMsmError_INTERFACE_DEFINED__
#define __IMsmError_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA828-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmError : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ msmErrorType __RPC_FAR *ErrorType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ BSTR __RPC_FAR *ErrorPath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Language( 
            /* [retval][out] */ short __RPC_FAR *ErrorLanguage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DatabaseTable( 
            /* [retval][out] */ BSTR __RPC_FAR *ErrorTable) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DatabaseKeys( 
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModuleTable( 
            /* [retval][out] */ BSTR __RPC_FAR *ErrorTable) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModuleKeys( 
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmErrorVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmError __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmError __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmError __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmError __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmError __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmError __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmError __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Type )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ msmErrorType __RPC_FAR *ErrorType);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Path )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *ErrorPath);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Language )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ short __RPC_FAR *ErrorLanguage);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DatabaseTable )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *ErrorTable);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DatabaseKeys )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_ModuleTable )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *ErrorTable);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_ModuleKeys )( 
            IMsmError __RPC_FAR * This,
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys);
        
        END_INTERFACE
    } IMsmErrorVtbl;

    interface IMsmError
    {
        CONST_VTBL struct IMsmErrorVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmError_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmError_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmError_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmError_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmError_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmError_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmError_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmError_get_Type(This,ErrorType)	\
    (This)->lpVtbl -> get_Type(This,ErrorType)

#define IMsmError_get_Path(This,ErrorPath)	\
    (This)->lpVtbl -> get_Path(This,ErrorPath)

#define IMsmError_get_Language(This,ErrorLanguage)	\
    (This)->lpVtbl -> get_Language(This,ErrorLanguage)

#define IMsmError_get_DatabaseTable(This,ErrorTable)	\
    (This)->lpVtbl -> get_DatabaseTable(This,ErrorTable)

#define IMsmError_get_DatabaseKeys(This,ErrorKeys)	\
    (This)->lpVtbl -> get_DatabaseKeys(This,ErrorKeys)

#define IMsmError_get_ModuleTable(This,ErrorTable)	\
    (This)->lpVtbl -> get_ModuleTable(This,ErrorTable)

#define IMsmError_get_ModuleKeys(This,ErrorKeys)	\
    (This)->lpVtbl -> get_ModuleKeys(This,ErrorKeys)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_Type_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ msmErrorType __RPC_FAR *ErrorType);


void __RPC_STUB IMsmError_get_Type_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_Path_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *ErrorPath);


void __RPC_STUB IMsmError_get_Path_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_Language_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ short __RPC_FAR *ErrorLanguage);


void __RPC_STUB IMsmError_get_Language_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_DatabaseTable_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *ErrorTable);


void __RPC_STUB IMsmError_get_DatabaseTable_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_DatabaseKeys_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys);


void __RPC_STUB IMsmError_get_DatabaseKeys_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_ModuleTable_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *ErrorTable);


void __RPC_STUB IMsmError_get_ModuleTable_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmError_get_ModuleKeys_Proxy( 
    IMsmError __RPC_FAR * This,
    /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *ErrorKeys);


void __RPC_STUB IMsmError_get_ModuleKeys_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmError_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IEnumMsmError - enumeration of IMsmError interfaces
// --------------------------------------------------------------------------

#ifndef __IEnumMsmError_INTERFACE_DEFINED__
#define __IEnumMsmError_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA829-2C26-11D2-AD65-00A0C9AF11A6")
    IEnumMsmError : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmError __RPC_FAR *__RPC_FAR *rgmsmErrors,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ unsigned long cSkip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumMsmError __RPC_FAR *__RPC_FAR *pemsmErrors) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IEnumMsmErrorVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IEnumMsmError __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IEnumMsmError __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IEnumMsmError __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Next )( 
            IEnumMsmError __RPC_FAR * This,
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmError __RPC_FAR *__RPC_FAR *rgmsmErrors,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Skip )( 
            IEnumMsmError __RPC_FAR * This,
            /* [in] */ unsigned long cSkip);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Reset )( 
            IEnumMsmError __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Clone )( 
            IEnumMsmError __RPC_FAR * This,
            /* [retval][out] */ IEnumMsmError __RPC_FAR *__RPC_FAR *pemsmErrors);
        
        END_INTERFACE
    } IEnumMsmErrorVtbl;

    interface IEnumMsmError
    {
        CONST_VTBL struct IEnumMsmErrorVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMsmError_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IEnumMsmError_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IEnumMsmError_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IEnumMsmError_Next(This,cFetch,rgmsmErrors,pcFetched)	\
    (This)->lpVtbl -> Next(This,cFetch,rgmsmErrors,pcFetched)

#define IEnumMsmError_Skip(This,cSkip)	\
    (This)->lpVtbl -> Skip(This,cSkip)

#define IEnumMsmError_Reset(This)	\
    (This)->lpVtbl -> Reset(This)

#define IEnumMsmError_Clone(This,pemsmErrors)	\
    (This)->lpVtbl -> Clone(This,pemsmErrors)

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IEnumMsmError_Next_Proxy( 
    IEnumMsmError __RPC_FAR * This,
    /* [in] */ unsigned long cFetch,
    /* [out] */ IMsmError __RPC_FAR *__RPC_FAR *rgmsmErrors,
    /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);


void __RPC_STUB IEnumMsmError_Next_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmError_Skip_Proxy( 
    IEnumMsmError __RPC_FAR * This,
    /* [in] */ unsigned long cSkip);


void __RPC_STUB IEnumMsmError_Skip_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmError_Reset_Proxy( 
    IEnumMsmError __RPC_FAR * This);


void __RPC_STUB IEnumMsmError_Reset_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmError_Clone_Proxy( 
    IEnumMsmError __RPC_FAR * This,
    /* [retval][out] */ IEnumMsmError __RPC_FAR *__RPC_FAR *pemsmErrors);


void __RPC_STUB IEnumMsmError_Clone_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IEnumMsmError_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmErrors - collection of IMsmError interfaces
// --------------------------------------------------------------------------

#ifndef __IMsmErrors_INTERFACE_DEFINED__
#define __IMsmErrors_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA82A-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmErrors : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Item,
            /* [retval][out] */ IMsmError __RPC_FAR *__RPC_FAR *Return) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long __RPC_FAR *Count) = 0;
        
        virtual /* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmErrorsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmErrors __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmErrors __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmErrors __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmErrors __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmErrors __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmErrors __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmErrors __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Item )( 
            IMsmErrors __RPC_FAR * This,
            /* [in] */ long Item,
            /* [retval][out] */ IMsmError __RPC_FAR *__RPC_FAR *Return);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Count )( 
            IMsmErrors __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *Count);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get__NewEnum )( 
            IMsmErrors __RPC_FAR * This,
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);
        
        END_INTERFACE
    } IMsmErrorsVtbl;

    interface IMsmErrors
    {
        CONST_VTBL struct IMsmErrorsVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmErrors_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmErrors_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmErrors_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmErrors_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmErrors_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmErrors_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmErrors_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmErrors_get_Item(This,Item,Return)	\
    (This)->lpVtbl -> get_Item(This,Item,Return)

#define IMsmErrors_get_Count(This,Count)	\
    (This)->lpVtbl -> get_Count(This,Count)

#define IMsmErrors_get__NewEnum(This,NewEnum)	\
    (This)->lpVtbl -> get__NewEnum(This,NewEnum)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmErrors_get_Item_Proxy( 
    IMsmErrors __RPC_FAR * This,
    /* [in] */ long Item,
    /* [retval][out] */ IMsmError __RPC_FAR *__RPC_FAR *Return);


void __RPC_STUB IMsmErrors_get_Item_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmErrors_get_Count_Proxy( 
    IMsmErrors __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *Count);


void __RPC_STUB IMsmErrors_get_Count_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmErrors_get__NewEnum_Proxy( 
    IMsmErrors __RPC_FAR * This,
    /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);


void __RPC_STUB IMsmErrors_get__NewEnum_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmErrors_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmDependency - interface for retrieving details on a single module 
//   dependency.
// --------------------------------------------------------------------------

#ifndef __IMsmDependency_INTERFACE_DEFINED__
#define __IMsmDependency_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA82B-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmDependency : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Module( 
            /* [retval][out] */ BSTR __RPC_FAR *Module) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Language( 
            /* [retval][out] */ short __RPC_FAR *Language) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ BSTR __RPC_FAR *Version) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmDependencyVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmDependency __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmDependency __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmDependency __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmDependency __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmDependency __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmDependency __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmDependency __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Module )( 
            IMsmDependency __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Module);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Language )( 
            IMsmDependency __RPC_FAR * This,
            /* [retval][out] */ short __RPC_FAR *Language);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Version )( 
            IMsmDependency __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Version);
        
        END_INTERFACE
    } IMsmDependencyVtbl;

    interface IMsmDependency
    {
        CONST_VTBL struct IMsmDependencyVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmDependency_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmDependency_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmDependency_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmDependency_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmDependency_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmDependency_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmDependency_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmDependency_get_Module(This,Module)	\
    (This)->lpVtbl -> get_Module(This,Module)

#define IMsmDependency_get_Language(This,Language)	\
    (This)->lpVtbl -> get_Language(This,Language)

#define IMsmDependency_get_Version(This,Version)	\
    (This)->lpVtbl -> get_Version(This,Version)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependency_get_Module_Proxy( 
    IMsmDependency __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Module);


void __RPC_STUB IMsmDependency_get_Module_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependency_get_Language_Proxy( 
    IMsmDependency __RPC_FAR * This,
    /* [retval][out] */ short __RPC_FAR *Language);


void __RPC_STUB IMsmDependency_get_Language_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependency_get_Version_Proxy( 
    IMsmDependency __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Version);


void __RPC_STUB IMsmDependency_get_Version_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmDependency_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IEnumMsmDependency - enumeration of IMsmDependency interfaces
// --------------------------------------------------------------------------

#ifndef __IEnumMsmDependency_INTERFACE_DEFINED__
#define __IEnumMsmDependency_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA82C-2C26-11D2-AD65-00A0C9AF11A6")
    IEnumMsmDependency : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmDependency __RPC_FAR *__RPC_FAR *rgmsmDependencies,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ unsigned long cSkip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumMsmDependency __RPC_FAR *__RPC_FAR *pemsmDependencies) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IEnumMsmDependencyVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IEnumMsmDependency __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IEnumMsmDependency __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IEnumMsmDependency __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Next )( 
            IEnumMsmDependency __RPC_FAR * This,
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmDependency __RPC_FAR *__RPC_FAR *rgmsmDependencies,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Skip )( 
            IEnumMsmDependency __RPC_FAR * This,
            /* [in] */ unsigned long cSkip);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Reset )( 
            IEnumMsmDependency __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Clone )( 
            IEnumMsmDependency __RPC_FAR * This,
            /* [retval][out] */ IEnumMsmDependency __RPC_FAR *__RPC_FAR *pemsmDependencies);
        
        END_INTERFACE
    } IEnumMsmDependencyVtbl;

    interface IEnumMsmDependency
    {
        CONST_VTBL struct IEnumMsmDependencyVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMsmDependency_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IEnumMsmDependency_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IEnumMsmDependency_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IEnumMsmDependency_Next(This,cFetch,rgmsmDependencies,pcFetched)	\
    (This)->lpVtbl -> Next(This,cFetch,rgmsmDependencies,pcFetched)

#define IEnumMsmDependency_Skip(This,cSkip)	\
    (This)->lpVtbl -> Skip(This,cSkip)

#define IEnumMsmDependency_Reset(This)	\
    (This)->lpVtbl -> Reset(This)

#define IEnumMsmDependency_Clone(This,pemsmDependencies)	\
    (This)->lpVtbl -> Clone(This,pemsmDependencies)

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IEnumMsmDependency_Next_Proxy( 
    IEnumMsmDependency __RPC_FAR * This,
    /* [in] */ unsigned long cFetch,
    /* [out] */ IMsmDependency __RPC_FAR *__RPC_FAR *rgmsmDependencies,
    /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);


void __RPC_STUB IEnumMsmDependency_Next_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmDependency_Skip_Proxy( 
    IEnumMsmDependency __RPC_FAR * This,
    /* [in] */ unsigned long cSkip);


void __RPC_STUB IEnumMsmDependency_Skip_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmDependency_Reset_Proxy( 
    IEnumMsmDependency __RPC_FAR * This);


void __RPC_STUB IEnumMsmDependency_Reset_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmDependency_Clone_Proxy( 
    IEnumMsmDependency __RPC_FAR * This,
    /* [retval][out] */ IEnumMsmDependency __RPC_FAR *__RPC_FAR *pemsmDependencies);


void __RPC_STUB IEnumMsmDependency_Clone_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IEnumMsmDependency_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmDependencies - collection of IMsmDependency interfaces
// --------------------------------------------------------------------------

#ifndef __IMsmDependencies_INTERFACE_DEFINED__
#define __IMsmDependencies_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA82D-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmDependencies : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Item,
            /* [retval][out] */ IMsmDependency __RPC_FAR *__RPC_FAR *Return) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long __RPC_FAR *Count) = 0;
        
        virtual /* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmDependenciesVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmDependencies __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmDependencies __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmDependencies __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmDependencies __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmDependencies __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmDependencies __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmDependencies __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Item )( 
            IMsmDependencies __RPC_FAR * This,
            /* [in] */ long Item,
            /* [retval][out] */ IMsmDependency __RPC_FAR *__RPC_FAR *Return);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Count )( 
            IMsmDependencies __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *Count);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get__NewEnum )( 
            IMsmDependencies __RPC_FAR * This,
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);
        
        END_INTERFACE
    } IMsmDependenciesVtbl;

    interface IMsmDependencies
    {
        CONST_VTBL struct IMsmDependenciesVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmDependencies_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmDependencies_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmDependencies_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmDependencies_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmDependencies_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmDependencies_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmDependencies_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmDependencies_get_Item(This,Item,Return)	\
    (This)->lpVtbl -> get_Item(This,Item,Return)

#define IMsmDependencies_get_Count(This,Count)	\
    (This)->lpVtbl -> get_Count(This,Count)

#define IMsmDependencies_get__NewEnum(This,NewEnum)	\
    (This)->lpVtbl -> get__NewEnum(This,NewEnum)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependencies_get_Item_Proxy( 
    IMsmDependencies __RPC_FAR * This,
    /* [in] */ long Item,
    /* [retval][out] */ IMsmDependency __RPC_FAR *__RPC_FAR *Return);


void __RPC_STUB IMsmDependencies_get_Item_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependencies_get_Count_Proxy( 
    IMsmDependencies __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *Count);


void __RPC_STUB IMsmDependencies_get_Count_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmDependencies_get__NewEnum_Proxy( 
    IMsmDependencies __RPC_FAR * This,
    /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);


void __RPC_STUB IMsmDependencies_get__NewEnum_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmDependencies_INTERFACE_DEFINED__ */


// the following interfaces are available only on MergeMod v1.5 or later
#if (_WIN32_MSM >= 150)

// --------------------------------------------------------------------------
// IMsmConfigurableItem - object describing the properties of a single
// configurable item.
// --------------------------------------------------------------------------


#ifndef __IMsmConfigurableItem_INTERFACE_DEFINED__
#define __IMsmConfigurableItem_INTERFACE_DEFINED__


#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D6E6284-D21D-401E-84F6-909E00B50F71")
    IMsmConfigurableItem : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR __RPC_FAR *Name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Format( 
            /* [retval][out] */ msmConfigurableItemFormat __RPC_FAR *Format) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ BSTR __RPC_FAR *Type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Context( 
            /* [retval][out] */ BSTR __RPC_FAR *Context) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultValue( 
            /* [retval][out] */ BSTR __RPC_FAR *DefaultValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Attributes( 
            /* [retval][out] */ long __RPC_FAR *Attributes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ BSTR __RPC_FAR *DisplayName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ BSTR __RPC_FAR *Description) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HelpLocation( 
            /* [retval][out] */ BSTR __RPC_FAR *HelpLocation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HelpKeyword( 
            /* [retval][out] */ BSTR __RPC_FAR *HelpKeyword) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmConfigurableItemVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmConfigurableItem __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmConfigurableItem __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Name )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Name);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Format )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ msmConfigurableItemFormat __RPC_FAR *Format);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Type )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Type);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Context )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Context);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DefaultValue )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *DefaultValue);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Attributes )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *Attributes);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_DisplayName )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *DisplayName);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Description )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *Description);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_HelpLocation )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *HelpLocation);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_HelpKeyword )( 
            IMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ BSTR __RPC_FAR *HelpKeyword);
        
        END_INTERFACE
    } IMsmConfigurableItemVtbl;

    interface IMsmConfigurableItem
    {
        CONST_VTBL struct IMsmConfigurableItemVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmConfigurableItem_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmConfigurableItem_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmConfigurableItem_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmConfigurableItem_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmConfigurableItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmConfigurableItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmConfigurableItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmConfigurableItem_get_Name(This,Name)	\
    (This)->lpVtbl -> get_Name(This,Name)

#define IMsmConfigurableItem_get_Format(This,Format)	\
    (This)->lpVtbl -> get_Format(This,Format)

#define IMsmConfigurableItem_get_Type(This,Type)	\
    (This)->lpVtbl -> get_Type(This,Type)

#define IMsmConfigurableItem_get_Context(This,Context)	\
    (This)->lpVtbl -> get_Context(This,Context)

#define IMsmConfigurableItem_get_DefaultValue(This,DefaultValue)	\
    (This)->lpVtbl -> get_DefaultValue(This,DefaultValue)

#define IMsmConfigurableItem_get_Attributes(This,Attributes)	\
    (This)->lpVtbl -> get_Attributes(This,Attributes)

#define IMsmConfigurableItem_get_DisplayName(This,DisplayName)	\
    (This)->lpVtbl -> get_DisplayName(This,DisplayName)

#define IMsmConfigurableItem_get_Description(This,Description)	\
    (This)->lpVtbl -> get_Description(This,Description)

#define IMsmConfigurableItem_get_HelpLocation(This,HelpLocation)	\
    (This)->lpVtbl -> get_HelpLocation(This,HelpLocation)

#define IMsmConfigurableItem_get_HelpKeyword(This,HelpKeyword)	\
    (This)->lpVtbl -> get_HelpKeyword(This,HelpKeyword)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Name_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Name);


void __RPC_STUB IMsmConfigurableItem_get_Name_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Format_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ msmConfigurableItemFormat __RPC_FAR *Format);


void __RPC_STUB IMsmConfigurableItem_get_Format_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Type_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Type);


void __RPC_STUB IMsmConfigurableItem_get_Type_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Context_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Context);


void __RPC_STUB IMsmConfigurableItem_get_Context_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_DefaultValue_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *DefaultValue);


void __RPC_STUB IMsmConfigurableItem_get_DefaultValue_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Attributes_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *Attributes);


void __RPC_STUB IMsmConfigurableItem_get_Attributes_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_DisplayName_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *DisplayName);


void __RPC_STUB IMsmConfigurableItem_get_DisplayName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_Description_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *Description);


void __RPC_STUB IMsmConfigurableItem_get_Description_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_HelpLocation_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *HelpLocation);


void __RPC_STUB IMsmConfigurableItem_get_HelpLocation_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItem_get_HelpKeyword_Proxy( 
    IMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ BSTR __RPC_FAR *HelpKeyword);


void __RPC_STUB IMsmConfigurableItem_get_HelpKeyword_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMsmConfigurableItem_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IEnumMsmConfigurableItem - enumerator for configurable items
// --------------------------------------------------------------------------

#ifndef __IEnumMsmConfigurableItem_INTERFACE_DEFINED__
#define __IEnumMsmConfigurableItem_INTERFACE_DEFINED__


#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("832C6969-4826-4C24-A397-B7002D8196E6")
    IEnumMsmConfigurableItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *rgmsmItems,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ unsigned long cSkip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumMsmConfigurableItem __RPC_FAR *__RPC_FAR *pemsmConfigurableItem) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IEnumMsmConfigurableItemVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IEnumMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IEnumMsmConfigurableItem __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IEnumMsmConfigurableItem __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Next )( 
            IEnumMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ unsigned long cFetch,
            /* [out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *rgmsmItems,
            /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Skip )( 
            IEnumMsmConfigurableItem __RPC_FAR * This,
            /* [in] */ unsigned long cSkip);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Reset )( 
            IEnumMsmConfigurableItem __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Clone )( 
            IEnumMsmConfigurableItem __RPC_FAR * This,
            /* [retval][out] */ IEnumMsmConfigurableItem __RPC_FAR *__RPC_FAR *pemsmConfigurableItem);
        
        END_INTERFACE
    } IEnumMsmConfigurableItemVtbl;

    interface IEnumMsmConfigurableItem
    {
        CONST_VTBL struct IEnumMsmConfigurableItemVtbl __RPC_FAR *lpVtbl;
    };

    
#ifdef COBJMACROS


#define IEnumMsmConfigurableItem_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IEnumMsmConfigurableItem_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IEnumMsmConfigurableItem_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IEnumMsmConfigurableItem_Next(This,cFetch,rgmsmItems,pcFetched)	\
    (This)->lpVtbl -> Next(This,cFetch,rgmsmItems,pcFetched)

#define IEnumMsmConfigurableItem_Skip(This,cSkip)	\
    (This)->lpVtbl -> Skip(This,cSkip)

#define IEnumMsmConfigurableItem_Reset(This)	\
    (This)->lpVtbl -> Reset(This)

#define IEnumMsmConfigurableItem_Clone(This,pemsmConfigurableItem)	\
    (This)->lpVtbl -> Clone(This,pemsmConfigurableItem)

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IEnumMsmConfigurableItem_Next_Proxy( 
    IEnumMsmConfigurableItem __RPC_FAR * This,
    /* [in] */ unsigned long cFetch,
    /* [out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *rgmsmItems,
    /* [retval][out] */ unsigned long __RPC_FAR *pcFetched);


void __RPC_STUB IEnumMsmConfigurableItem_Next_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmConfigurableItem_Skip_Proxy( 
    IEnumMsmConfigurableItem __RPC_FAR * This,
    /* [in] */ unsigned long cSkip);


void __RPC_STUB IEnumMsmConfigurableItem_Skip_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmConfigurableItem_Reset_Proxy( 
    IEnumMsmConfigurableItem __RPC_FAR * This);


void __RPC_STUB IEnumMsmConfigurableItem_Reset_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


HRESULT STDMETHODCALLTYPE IEnumMsmConfigurableItem_Clone_Proxy( 
    IEnumMsmConfigurableItem __RPC_FAR * This,
    /* [retval][out] */ IEnumMsmConfigurableItem __RPC_FAR *__RPC_FAR *pemsmConfigurableItem);


void __RPC_STUB IEnumMsmConfigurableItem_Clone_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumMsmConfigurableItem_INTERFACE_DEFINED__ */


// --------------------------------------------------------------------------
// IMsmConfigurableItems - collection of configurable items
// --------------------------------------------------------------------------


#ifndef __IMsmConfigurableItems_INTERFACE_DEFINED__
#define __IMsmConfigurableItems_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55BF723C-9A0D-463E-B42B-B4FBC7BE3C7C")
    IMsmConfigurableItems : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Item,
            /* [retval][out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *Return) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ long __RPC_FAR *Count) = 0;
        
        virtual /* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmConfigurableItemsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmConfigurableItems __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmConfigurableItems __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Item )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [in] */ long Item,
            /* [retval][out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *Return);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Count )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [retval][out] */ long __RPC_FAR *Count);
        
        /* [hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get__NewEnum )( 
            IMsmConfigurableItems __RPC_FAR * This,
            /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);
        
        END_INTERFACE
    } IMsmConfigurableItemsVtbl;

    interface IMsmConfigurableItems
    {
        CONST_VTBL struct IMsmConfigurableItemsVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmConfigurableItems_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmConfigurableItems_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmConfigurableItems_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmConfigurableItems_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmConfigurableItems_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmConfigurableItems_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmConfigurableItems_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmConfigurableItems_get_Item(This,Item,Return)	\
    (This)->lpVtbl -> get_Item(This,Item,Return)

#define IMsmConfigurableItems_get_Count(This,Count)	\
    (This)->lpVtbl -> get_Count(This,Count)

#define IMsmConfigurableItems_get__NewEnum(This,NewEnum)	\
    (This)->lpVtbl -> get__NewEnum(This,NewEnum)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItems_get_Item_Proxy( 
    IMsmConfigurableItems __RPC_FAR * This,
    /* [in] */ long Item,
    /* [retval][out] */ IMsmConfigurableItem __RPC_FAR *__RPC_FAR *Return);


void __RPC_STUB IMsmConfigurableItems_get_Item_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItems_get_Count_Proxy( 
    IMsmConfigurableItems __RPC_FAR * This,
    /* [retval][out] */ long __RPC_FAR *Count);


void __RPC_STUB IMsmConfigurableItems_get_Count_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [hidden][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmConfigurableItems_get__NewEnum_Proxy( 
    IMsmConfigurableItems __RPC_FAR * This,
    /* [retval][out] */ IUnknown __RPC_FAR *__RPC_FAR *NewEnum);


void __RPC_STUB IMsmConfigurableItems_get__NewEnum_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMsmConfigurableItems_INTERFACE_DEFINED__ */



// --------------------------------------------------------------------------
// IMsmConfigureModule - callback interface called by the MergeMod object. 
// Allows the client to provide merge configuration information during the
// merge process.
// --------------------------------------------------------------------------

#ifndef __IMsmConfigureModule_INTERFACE_DEFINED__
#define __IMsmConfigureModule_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC013209-18A7-4851-8A21-2353443D70A0")
    IMsmConfigureModule : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProvideTextData( 
            /* [in] */ const BSTR Name,
            /* [retval][out] */ BSTR __RPC_FAR *ConfigData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProvideIntegerData( 
            /* [in] */ const BSTR Name,
            /* [retval][out] */ long __RPC_FAR *ConfigData) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmConfigureModuleVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmConfigureModule __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmConfigureModule __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ProvideTextData )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ const BSTR Name,
            /* [retval][out] */ BSTR __RPC_FAR *ConfigData);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ProvideIntegerData )( 
            IMsmConfigureModule __RPC_FAR * This,
            /* [in] */ const BSTR Name,
            /* [retval][out] */ long __RPC_FAR *ConfigData);
        
        END_INTERFACE
    } IMsmConfigureModuleVtbl;

    interface IMsmConfigureModule
    {
        CONST_VTBL struct IMsmConfigureModuleVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmConfigureModule_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmConfigureModule_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmConfigureModule_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmConfigureModule_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmConfigureModule_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmConfigureModule_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmConfigureModule_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmConfigureModule_ProvideTextData(This,Name,ConfigData)	\
    (This)->lpVtbl -> ProvideTextData(This,Name,ConfigData)

#define IMsmConfigureModule_ProvideIntegerData(This,Name,ConfigData)	\
    (This)->lpVtbl -> ProvideIntegerData(This,Name,ConfigData)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmConfigureModule_ProvideTextData_Proxy( 
    IMsmConfigureModule __RPC_FAR * This,
    /* [in] */ const BSTR Name,
    /* [retval][out] */ BSTR __RPC_FAR *ConfigData);


void __RPC_STUB IMsmConfigureModule_ProvideTextData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmConfigureModule_ProvideIntegerData_Proxy( 
    IMsmConfigureModule __RPC_FAR * This,
    /* [in] */ const BSTR Name,
    /* [retval][out] */ long __RPC_FAR *ConfigData);


void __RPC_STUB IMsmConfigureModule_ProvideIntegerData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMsmConfigureModule_INTERFACE_DEFINED__ */


#endif // _MSM_WIN32 >= 150

// --------------------------------------------------------------------------
// IMsmMerge - primary interface to the MergeMod object. Allows the client
// to open and close databases, perform merges, retrieve the results
// of a merge, control the creation of a debug-level log, and extract the
// files from a module to disk.
// --------------------------------------------------------------------------

#ifndef __IMsmMerge_INTERFACE_DEFINED__
#define __IMsmMerge_INTERFACE_DEFINED__

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("0ADDA82E-2C26-11D2-AD65-00A0C9AF11A6")
    IMsmMerge : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenDatabase( 
            /* [in] */ const BSTR Path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenModule( 
            /* [in] */ const BSTR Path,
            /* [in] */ const short Language) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseDatabase( 
            /* [in] */ const VARIANT_BOOL Commit) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseModule( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenLog( 
            /* [in] */ const BSTR Path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseLog( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Log( 
            /* [in] */ const BSTR Message) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Errors( 
            /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dependencies( 
            /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Merge( 
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ const BSTR Feature) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractCAB( 
            /* [in] */ const BSTR FileName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractFiles( 
            /* [in] */ const BSTR Path) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmMergeVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmMerge __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmMerge __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmMerge __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenDatabase )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenModule )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Path,
            /* [in] */ const short Language);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseDatabase )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const VARIANT_BOOL Commit);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseModule )( 
            IMsmMerge __RPC_FAR * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenLog )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseLog )( 
            IMsmMerge __RPC_FAR * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Log )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Message);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Errors )( 
            IMsmMerge __RPC_FAR * This,
            /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Dependencies )( 
            IMsmMerge __RPC_FAR * This,
            /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Merge )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Connect )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Feature);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExtractCAB )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR FileName);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExtractFiles )( 
            IMsmMerge __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        END_INTERFACE
    } IMsmMergeVtbl;

    interface IMsmMerge
    {
        CONST_VTBL struct IMsmMergeVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmMerge_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmMerge_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmMerge_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmMerge_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmMerge_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmMerge_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmMerge_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmMerge_OpenDatabase(This,Path)	\
    (This)->lpVtbl -> OpenDatabase(This,Path)

#define IMsmMerge_OpenModule(This,Path,Language)	\
    (This)->lpVtbl -> OpenModule(This,Path,Language)

#define IMsmMerge_CloseDatabase(This,Commit)	\
    (This)->lpVtbl -> CloseDatabase(This,Commit)

#define IMsmMerge_CloseModule(This)	\
    (This)->lpVtbl -> CloseModule(This)

#define IMsmMerge_OpenLog(This,Path)	\
    (This)->lpVtbl -> OpenLog(This,Path)

#define IMsmMerge_CloseLog(This)	\
    (This)->lpVtbl -> CloseLog(This)

#define IMsmMerge_Log(This,Message)	\
    (This)->lpVtbl -> Log(This,Message)

#define IMsmMerge_get_Errors(This,Errors)	\
    (This)->lpVtbl -> get_Errors(This,Errors)

#define IMsmMerge_get_Dependencies(This,Dependencies)	\
    (This)->lpVtbl -> get_Dependencies(This,Dependencies)

#define IMsmMerge_Merge(This,Feature,RedirectDir)	\
    (This)->lpVtbl -> Merge(This,Feature,RedirectDir)

#define IMsmMerge_Connect(This,Feature)	\
    (This)->lpVtbl -> Connect(This,Feature)

#define IMsmMerge_ExtractCAB(This,FileName)	\
    (This)->lpVtbl -> ExtractCAB(This,FileName)

#define IMsmMerge_ExtractFiles(This,Path)	\
    (This)->lpVtbl -> ExtractFiles(This,Path)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_OpenDatabase_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge_OpenDatabase_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_OpenModule_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Path,
    /* [in] */ const short Language);


void __RPC_STUB IMsmMerge_OpenModule_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_CloseDatabase_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const VARIANT_BOOL Commit);


void __RPC_STUB IMsmMerge_CloseDatabase_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_CloseModule_Proxy( 
    IMsmMerge __RPC_FAR * This);


void __RPC_STUB IMsmMerge_CloseModule_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_OpenLog_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge_OpenLog_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_CloseLog_Proxy( 
    IMsmMerge __RPC_FAR * This);


void __RPC_STUB IMsmMerge_CloseLog_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_Log_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Message);


void __RPC_STUB IMsmMerge_Log_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge_get_Errors_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors);


void __RPC_STUB IMsmMerge_get_Errors_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge_get_Dependencies_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies);


void __RPC_STUB IMsmMerge_get_Dependencies_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_Merge_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Feature,
    /* [in] */ const BSTR RedirectDir);


void __RPC_STUB IMsmMerge_Merge_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_Connect_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Feature);


void __RPC_STUB IMsmMerge_Connect_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_ExtractCAB_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR FileName);


void __RPC_STUB IMsmMerge_ExtractCAB_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge_ExtractFiles_Proxy( 
    IMsmMerge __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge_ExtractFiles_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMsmMerge_INTERFACE_DEFINED__ */

// --------------------------------------------------------------------------
// IMsmGetFiles - secondary interface to the MergeMod object, allows
// the client to retrieve the files needed in a particular language of the
// module. Requires certain actions be performed via the IMsmMerge interface
// before some calls on this interface will return valid results.
// --------------------------------------------------------------------------


#ifndef __IMsmGetFiles_INTERFACE_DEFINED__
#define __IMsmGetFiles_INTERFACE_DEFINED__


#if defined(__cplusplus) && !defined(CINTERFACE)
    
    interface DECLSPEC_UUID("7041AE26-2D78-11d2-888A-00A0C981B015")
    IMsmGetFiles : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModuleFiles( 
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *Files) = 0;
        
    };
    
#else 	/* C style interface */

    typedef struct IMsmGetFilesVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmGetFiles __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmGetFiles __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_ModuleFiles )( 
            IMsmGetFiles __RPC_FAR * This,
            /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *Files);
        
        END_INTERFACE
    } IMsmGetFilesVtbl;

    interface IMsmGetFiles
    {
        CONST_VTBL struct IMsmGetFilesVtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmGetFiles_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmGetFiles_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmGetFiles_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmGetFiles_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmGetFiles_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmGetFiles_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmGetFiles_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmGetFiles_get_ModuleFiles(This,Files)	\
    (This)->lpVtbl -> get_ModuleFiles(This,Files)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmGetFiles_get_ModuleFiles_Proxy( 
    IMsmGetFiles __RPC_FAR * This,
    /* [retval][out] */ IMsmStrings __RPC_FAR *__RPC_FAR *Files);


void __RPC_STUB IMsmGetFiles_get_ModuleFiles_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmGetFiles_INTERFACE_DEFINED__ */



// The following interface is available only on MergeMod v1.5 and later
#if (_WIN32_MSM >= 150)

// --------------------------------------------------------------------------
// IMsmMerge2 - primary interface to the MsmMerge2 object. Allows the client
// to open and close databases, perform merges, retrieve the results
// of a merge, control the creation of a debug-level log, and extract the
// files from a module to disk. Extends the original object by adding
// LFN support and configurable module support.
// --------------------------------------------------------------------------

#ifndef __IMsmMerge2_INTERFACE_DEFINED__
#define __IMsmMerge2_INTERFACE_DEFINED__




#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("351A72AB-21CB-47AB-B7AA-C4D7B02EA305")
    IMsmMerge2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenDatabase( 
            /* [in] */ const BSTR Path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenModule( 
            /* [in] */ const BSTR Path,
            /* [in] */ const short Language) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseDatabase( 
            /* [in] */ const VARIANT_BOOL Commit) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseModule( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenLog( 
            /* [in] */ const BSTR Path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseLog( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Log( 
            /* [in] */ const BSTR Message) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Errors( 
            /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dependencies( 
            /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Merge( 
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ const BSTR Feature) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractCAB( 
            /* [in] */ const BSTR FileName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractFiles( 
            /* [in] */ const BSTR Path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MergeEx( 
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir,
            /* [in] */ IUnknown __RPC_FAR *pConfiguration) = 0;
        
      
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractFilesEx( 
            /* [in] */ const BSTR Path,
            /* [in] */ VARIANT_BOOL fLongFileNames,
            /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConfigurableItems( 
            /* [retval][out] */ IMsmConfigurableItems __RPC_FAR *__RPC_FAR *ConfigurableItems) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSourceImage( 
            /* [in] */ const BSTR Path,
            /* [in] */ VARIANT_BOOL fLongFileNames,
            /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths) = 0;

        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModuleFiles( 
            /* [retval][out] */ IMsmStrings **Files) = 0;        
    };
    
#else 	/* C style interface */

    typedef struct IMsmMerge2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *QueryInterface )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void __RPC_FAR *__RPC_FAR *ppvObject);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *AddRef )( 
            IMsmMerge2 __RPC_FAR * This);
        
        ULONG ( STDMETHODCALLTYPE __RPC_FAR *Release )( 
            IMsmMerge2 __RPC_FAR * This);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfoCount )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [out] */ UINT __RPC_FAR *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetTypeInfo )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo __RPC_FAR *__RPC_FAR *ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE __RPC_FAR *GetIDsOfNames )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR __RPC_FAR *rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID __RPC_FAR *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Invoke )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ DISPID dispIdMember,
            /* [in] */ REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS __RPC_FAR *pDispParams,
            /* [out] */ VARIANT __RPC_FAR *pVarResult,
            /* [out] */ EXCEPINFO __RPC_FAR *pExcepInfo,
            /* [out] */ UINT __RPC_FAR *puArgErr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenDatabase )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenModule )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path,
            /* [in] */ const short Language);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseDatabase )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const VARIANT_BOOL Commit);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseModule )( 
            IMsmMerge2 __RPC_FAR * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *OpenLog )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CloseLog )( 
            IMsmMerge2 __RPC_FAR * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Log )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Message);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Errors )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_Dependencies )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Merge )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *Connect )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Feature);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExtractCAB )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR FileName);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExtractFiles )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *MergeEx )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Feature,
            /* [in] */ const BSTR RedirectDir,
            /* [in] */ IMsmConfigureModule __RPC_FAR *pConfiguration);
   
      /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *ExtractFilesEx )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path,
            /* [in] */ VARIANT_BOOL fLongFileNames,
            /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths);
        
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *get_ConfigurableItems )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [retval][out] */ IMsmConfigurableItems __RPC_FAR *__RPC_FAR *ConfigurableItems);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE __RPC_FAR *CreateSourceImage )( 
            IMsmMerge2 __RPC_FAR * This,
            /* [in] */ const BSTR Path,
            /* [in] */ VARIANT_BOOL fLongFileNames,
            /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths);

        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModuleFiles )( 
            IMsmMerge2 * This,
            /* [retval][out] */ IMsmStrings **Files);
        
        END_INTERFACE
    } IMsmMerge2Vtbl;

    interface IMsmMerge2
    {
        CONST_VTBL struct IMsmMerge2Vtbl __RPC_FAR *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMsmMerge2_QueryInterface(This,riid,ppvObject)	\
    (This)->lpVtbl -> QueryInterface(This,riid,ppvObject)

#define IMsmMerge2_AddRef(This)	\
    (This)->lpVtbl -> AddRef(This)

#define IMsmMerge2_Release(This)	\
    (This)->lpVtbl -> Release(This)


#define IMsmMerge2_GetTypeInfoCount(This,pctinfo)	\
    (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo)

#define IMsmMerge2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo)

#define IMsmMerge2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)

#define IMsmMerge2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)


#define IMsmMerge2_OpenDatabase(This,Path)	\
    (This)->lpVtbl -> OpenDatabase(This,Path)

#define IMsmMerge2_OpenModule(This,Path,Language)	\
    (This)->lpVtbl -> OpenModule(This,Path,Language)

#define IMsmMerge2_CloseDatabase(This,Commit)	\
    (This)->lpVtbl -> CloseDatabase(This,Commit)

#define IMsmMerge2_CloseModule(This)	\
    (This)->lpVtbl -> CloseModule(This)

#define IMsmMerge2_OpenLog(This,Path)	\
    (This)->lpVtbl -> OpenLog(This,Path)

#define IMsmMerge2_CloseLog(This)	\
    (This)->lpVtbl -> CloseLog(This)

#define IMsmMerge2_Log(This,Message)	\
    (This)->lpVtbl -> Log(This,Message)

#define IMsmMerge2_get_Errors(This,Errors)	\
    (This)->lpVtbl -> get_Errors(This,Errors)

#define IMsmMerge2_get_Dependencies(This,Dependencies)	\
    (This)->lpVtbl -> get_Dependencies(This,Dependencies)

#define IMsmMerge2_Merge(This,Feature,RedirectDir)	\
    (This)->lpVtbl -> Merge(This,Feature,RedirectDir)

#define IMsmMerge2_Connect(This,Feature)	\
    (This)->lpVtbl -> Connect(This,Feature)

#define IMsmMerge2_ExtractCAB(This,FileName)	\
    (This)->lpVtbl -> ExtractCAB(This,FileName)

#define IMsmMerge2_ExtractFiles(This,Path)	\
    (This)->lpVtbl -> ExtractFiles(This,Path)

#define IMsmMerge2_MergeEx(This,Feature,RedirectDir,pConfiguration)	\
    (This)->lpVtbl -> MergeEx(This,Feature,RedirectDir,pConfiguration)

#define IMsmMerge2_ExtractFilesEx(This,Path,fLongFileNames,pFilePaths)	\
    (This)->lpVtbl -> ExtractFilesEx(This,Path,fLongFileNames,pFilePaths)

#define IMsmMerge2_get_ConfigurableItems(This,ConfigurableItems)	\
    (This)->lpVtbl -> get_ConfigurableItems(This,ConfigurableItems)

#define IMsmMerge2_CreateSourceImage(This,Path,fLongFileNames,pFilePaths)	\
    (This)->lpVtbl -> CreateSourceImage(This,Path,fLongFileNames,pFilePaths)

#define IMsmMerge2_get_ModuleFiles(This,Files)	\
    (This)->lpVtbl -> get_ModuleFiles(This,Files)

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_OpenDatabase_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge2_OpenDatabase_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_OpenModule_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path,
    /* [in] */ const short Language);


void __RPC_STUB IMsmMerge2_OpenModule_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_CloseDatabase_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const VARIANT_BOOL Commit);


void __RPC_STUB IMsmMerge2_CloseDatabase_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_CloseModule_Proxy( 
    IMsmMerge2 __RPC_FAR * This);


void __RPC_STUB IMsmMerge2_CloseModule_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_OpenLog_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge2_OpenLog_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_CloseLog_Proxy( 
    IMsmMerge2 __RPC_FAR * This);


void __RPC_STUB IMsmMerge2_CloseLog_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_Log_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Message);


void __RPC_STUB IMsmMerge2_Log_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_get_Errors_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [retval][out] */ IMsmErrors __RPC_FAR *__RPC_FAR *Errors);


void __RPC_STUB IMsmMerge2_get_Errors_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_get_Dependencies_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [retval][out] */ IMsmDependencies __RPC_FAR *__RPC_FAR *Dependencies);


void __RPC_STUB IMsmMerge2_get_Dependencies_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_Merge_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Feature,
    /* [in] */ const BSTR RedirectDir);


void __RPC_STUB IMsmMerge2_Merge_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_Connect_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Feature);


void __RPC_STUB IMsmMerge2_Connect_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_ExtractCAB_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR FileName);


void __RPC_STUB IMsmMerge2_ExtractCAB_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_ExtractFiles_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path);


void __RPC_STUB IMsmMerge2_ExtractFiles_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_MergeEx_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Feature,
    /* [in] */ const BSTR RedirectDir,
    /* [in] */ IMsmConfigureModule __RPC_FAR *pConfiguration);


void __RPC_STUB IMsmMerge2_MergeEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_ExtractFilesEx_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path,
    /* [in] */ VARIANT_BOOL fLongFileNames,
    /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths);


void __RPC_STUB IMsmMerge2_ExtractFilesEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_get_ConfigurableItems_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [retval][out] */ IMsmConfigurableItems __RPC_FAR *__RPC_FAR *
ConfigurableItems);


void __RPC_STUB IMsmMerge2_get_ConfigurableItems_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id] */ HRESULT STDMETHODCALLTYPE 
IMsmMerge2_CreateSourceImage_Proxy( 
    IMsmMerge2 __RPC_FAR * This,
    /* [in] */ const BSTR Path,
    /* [in] */ VARIANT_BOOL fLongFileNames,
    /* [out] */ IMsmStrings __RPC_FAR *__RPC_FAR *pFilePaths);


void __RPC_STUB IMsmMerge2_CreateSourceImage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE IMsmMerge2_get_ModuleFiles_Proxy( 
    IMsmMerge2 * This,
    /* [retval][out] */ IMsmStrings **Files);


void __RPC_STUB IMsmMerge2_get_ModuleFiles_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);

#endif 	/* __IMsmMerge2_INTERFACE_DEFINED__ */


#endif // _WIN32_MSM >= 150


// --------------------------------------------------------------------------
// TypeLib definitions
// --------------------------------------------------------------------------

#ifndef __MsmMergeTypeLib_LIBRARY_DEFINED__
#define __MsmMergeTypeLib_LIBRARY_DEFINED__

#ifdef __cplusplus
class DECLSPEC_UUID("0ADDA830-2C26-11D2-AD65-00A0C9AF11A6")
MsmMerge;
#endif

// the following class is available only on MergeMod v1.5 or later
#if (_WIN32_MSM >= 150)

#ifdef __cplusplus

class DECLSPEC_UUID("F94985D5-29F9-4743-9805-99BC3F35B678")
MsmMerge2;
#endif
#endif // _WIN32_MSM >= 150

#endif /* __MsmMergeTypeLib_LIBRARY_DEFINED__ */


// --------------------------------------------------------------------------
// Additional Prototypes for ALL interfaces
// --------------------------------------------------------------------------

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long __RPC_FAR *, unsigned long            , BSTR __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  BSTR_UserMarshal(  unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, BSTR __RPC_FAR * ); 
unsigned char __RPC_FAR * __RPC_USER  BSTR_UserUnmarshal(unsigned long __RPC_FAR *, unsigned char __RPC_FAR *, BSTR __RPC_FAR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long __RPC_FAR *, BSTR __RPC_FAR * ); 


#ifdef __cplusplus
}
#endif

// --------------------------------------------------------------------------
// MergeMod Interface IDs
// --------------------------------------------------------------------------
/* [local] */ 


DEFINE_GUID(IID_IEnumMsmString, 0x0ADDA826,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmStrings, 0x0ADDA827,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmError, 0x0ADDA828,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IEnumMsmError, 0x0ADDA829,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmErrors, 0x0ADDA82A,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmDependency, 0x0ADDA82B,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IEnumMsmDependency, 0x0ADDA82C,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmDependencies, 0x0ADDA82D,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmMerge, 0x0ADDA82E,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(IID_IMsmGetFiles, 0x7041ae26, 0x2d78, 0x11d2, 0x88, 0x8a, 0x0, 0xa0, 0xc9, 0x81, 0xb0, 0x15);

DEFINE_GUID(LIBID_MsmMergeTypeLib, 0x0ADDA82F,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);
DEFINE_GUID(CLSID_MsmMerge, 0x0ADDA830,0x2C26,0x11D2, 0xAD,0x65,0x00,0xA0,0xC9,0xAF,0x11,0xA6);

#if (_WIN32_MSM >= 150)
DEFINE_GUID(IID_IMsmMerge2, 0x351A72AB, 0x21CB, 0x47AB, 0xB7, 0xAA, 0xC4, 0xD7, 0xB0, 0x2E, 0xA3, 0x05);
DEFINE_GUID(IID_IMsmConfigurableItem, 0x4D6E6284, 0xD21D, 0x401E, 0x84, 0xF6, 0x90, 0x9E, 0x00, 0xB5, 0x0F, 0x71);
DEFINE_GUID(IID_IEnumMsmConfigurableItem, 0x832C6969, 0x4826, 0x4C24, 0xA3, 0x97, 0xB7, 0x00, 0x2D, 0x81, 0x96, 0xE6);
DEFINE_GUID(IID_IMsmConfigurableItems, 0x55BF723C, 0x9A0D, 0x463E, 0xB4, 0x2B, 0xB4, 0xFB, 0xC7, 0xBE, 0x3C, 0x7C);
DEFINE_GUID(IID_IMsmConfigureModule, 0xAC013209, 0x18A7, 0x4851, 0x8A, 0x21, 0x23, 0x53, 0x44, 0x3D, 0x70, 0xA0);
DEFINE_GUID(CLSID_MsmMerge2, 0xF94985D5,0x29F9,0x4743, 0x98,0x05,0x99,0xBC,0x3F,0x35,0xB6,0x78);
#endif

#endif // __mergemod_h__


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

