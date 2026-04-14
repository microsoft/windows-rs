

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

#ifndef __structuredquery_h__
#define __structuredquery_h__

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

#ifndef __IQueryParser_FWD_DEFINED__
#define __IQueryParser_FWD_DEFINED__
typedef interface IQueryParser IQueryParser;

#endif 	/* __IQueryParser_FWD_DEFINED__ */


#ifndef __IConditionFactory_FWD_DEFINED__
#define __IConditionFactory_FWD_DEFINED__
typedef interface IConditionFactory IConditionFactory;

#endif 	/* __IConditionFactory_FWD_DEFINED__ */


#ifndef __IQuerySolution_FWD_DEFINED__
#define __IQuerySolution_FWD_DEFINED__
typedef interface IQuerySolution IQuerySolution;

#endif 	/* __IQuerySolution_FWD_DEFINED__ */


#ifndef __IConditionFactory2_FWD_DEFINED__
#define __IConditionFactory2_FWD_DEFINED__
typedef interface IConditionFactory2 IConditionFactory2;

#endif 	/* __IConditionFactory2_FWD_DEFINED__ */


#ifndef __IConditionGenerator_FWD_DEFINED__
#define __IConditionGenerator_FWD_DEFINED__
typedef interface IConditionGenerator IConditionGenerator;

#endif 	/* __IConditionGenerator_FWD_DEFINED__ */


#ifndef __IInterval_FWD_DEFINED__
#define __IInterval_FWD_DEFINED__
typedef interface IInterval IInterval;

#endif 	/* __IInterval_FWD_DEFINED__ */


#ifndef __IMetaData_FWD_DEFINED__
#define __IMetaData_FWD_DEFINED__
typedef interface IMetaData IMetaData;

#endif 	/* __IMetaData_FWD_DEFINED__ */


#ifndef __IEntity_FWD_DEFINED__
#define __IEntity_FWD_DEFINED__
typedef interface IEntity IEntity;

#endif 	/* __IEntity_FWD_DEFINED__ */


#ifndef __IRelationship_FWD_DEFINED__
#define __IRelationship_FWD_DEFINED__
typedef interface IRelationship IRelationship;

#endif 	/* __IRelationship_FWD_DEFINED__ */


#ifndef __INamedEntity_FWD_DEFINED__
#define __INamedEntity_FWD_DEFINED__
typedef interface INamedEntity INamedEntity;

#endif 	/* __INamedEntity_FWD_DEFINED__ */


#ifndef __ISchemaProvider_FWD_DEFINED__
#define __ISchemaProvider_FWD_DEFINED__
typedef interface ISchemaProvider ISchemaProvider;

#endif 	/* __ISchemaProvider_FWD_DEFINED__ */


#ifndef __ITokenCollection_FWD_DEFINED__
#define __ITokenCollection_FWD_DEFINED__
typedef interface ITokenCollection ITokenCollection;

#endif 	/* __ITokenCollection_FWD_DEFINED__ */


#ifndef __INamedEntityCollector_FWD_DEFINED__
#define __INamedEntityCollector_FWD_DEFINED__
typedef interface INamedEntityCollector INamedEntityCollector;

#endif 	/* __INamedEntityCollector_FWD_DEFINED__ */


#ifndef __ISchemaLocalizerSupport_FWD_DEFINED__
#define __ISchemaLocalizerSupport_FWD_DEFINED__
typedef interface ISchemaLocalizerSupport ISchemaLocalizerSupport;

#endif 	/* __ISchemaLocalizerSupport_FWD_DEFINED__ */


#ifndef __IQueryParserManager_FWD_DEFINED__
#define __IQueryParserManager_FWD_DEFINED__
typedef interface IQueryParserManager IQueryParserManager;

#endif 	/* __IQueryParserManager_FWD_DEFINED__ */


#ifndef __QueryParser_FWD_DEFINED__
#define __QueryParser_FWD_DEFINED__

#ifdef __cplusplus
typedef class QueryParser QueryParser;
#else
typedef struct QueryParser QueryParser;
#endif /* __cplusplus */

#endif 	/* __QueryParser_FWD_DEFINED__ */


#ifndef __NegationCondition_FWD_DEFINED__
#define __NegationCondition_FWD_DEFINED__

#ifdef __cplusplus
typedef class NegationCondition NegationCondition;
#else
typedef struct NegationCondition NegationCondition;
#endif /* __cplusplus */

#endif 	/* __NegationCondition_FWD_DEFINED__ */


#ifndef __CompoundCondition_FWD_DEFINED__
#define __CompoundCondition_FWD_DEFINED__

#ifdef __cplusplus
typedef class CompoundCondition CompoundCondition;
#else
typedef struct CompoundCondition CompoundCondition;
#endif /* __cplusplus */

#endif 	/* __CompoundCondition_FWD_DEFINED__ */


#ifndef __LeafCondition_FWD_DEFINED__
#define __LeafCondition_FWD_DEFINED__

#ifdef __cplusplus
typedef class LeafCondition LeafCondition;
#else
typedef struct LeafCondition LeafCondition;
#endif /* __cplusplus */

#endif 	/* __LeafCondition_FWD_DEFINED__ */


#ifndef __ConditionFactory_FWD_DEFINED__
#define __ConditionFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class ConditionFactory ConditionFactory;
#else
typedef struct ConditionFactory ConditionFactory;
#endif /* __cplusplus */

#endif 	/* __ConditionFactory_FWD_DEFINED__ */


#ifndef __Interval_FWD_DEFINED__
#define __Interval_FWD_DEFINED__

#ifdef __cplusplus
typedef class Interval Interval;
#else
typedef struct Interval Interval;
#endif /* __cplusplus */

#endif 	/* __Interval_FWD_DEFINED__ */


#ifndef __QueryParserManager_FWD_DEFINED__
#define __QueryParserManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class QueryParserManager QueryParserManager;
#else
typedef struct QueryParserManager QueryParserManager;
#endif /* __cplusplus */

#endif 	/* __QueryParserManager_FWD_DEFINED__ */


/* header files for imported files */
#include "StructuredQueryCondition.h"
#include "ObjectArray.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_structuredquery_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if 0
typedef PROPERTYKEY *REFPROPERTYKEY;

#endif // 0
#include <propkeydef.h>









typedef /* [v1_enum] */ 
enum tagSTRUCTURED_QUERY_SYNTAX
    {
        SQS_NO_SYNTAX	= 0,
        SQS_ADVANCED_QUERY_SYNTAX	= ( SQS_NO_SYNTAX + 1 ) ,
        SQS_NATURAL_QUERY_SYNTAX	= ( SQS_ADVANCED_QUERY_SYNTAX + 1 ) 
    } 	STRUCTURED_QUERY_SYNTAX;

typedef /* [v1_enum] */ 
enum tagSTRUCTURED_QUERY_SINGLE_OPTION
    {
        SQSO_SCHEMA	= 0,
        SQSO_LOCALE_WORD_BREAKING	= ( SQSO_SCHEMA + 1 ) ,
        SQSO_WORD_BREAKER	= ( SQSO_LOCALE_WORD_BREAKING + 1 ) ,
        SQSO_NATURAL_SYNTAX	= ( SQSO_WORD_BREAKER + 1 ) ,
        SQSO_AUTOMATIC_WILDCARD	= ( SQSO_NATURAL_SYNTAX + 1 ) ,
        SQSO_TRACE_LEVEL	= ( SQSO_AUTOMATIC_WILDCARD + 1 ) ,
        SQSO_LANGUAGE_KEYWORDS	= ( SQSO_TRACE_LEVEL + 1 ) ,
        SQSO_SYNTAX	= ( SQSO_LANGUAGE_KEYWORDS + 1 ) ,
        SQSO_TIME_ZONE	= ( SQSO_SYNTAX + 1 ) ,
        SQSO_IMPLICIT_CONNECTOR	= ( SQSO_TIME_ZONE + 1 ) ,
        SQSO_CONNECTOR_CASE	= ( SQSO_IMPLICIT_CONNECTOR + 1 ) 
    } 	STRUCTURED_QUERY_SINGLE_OPTION;

typedef /* [v1_enum] */ 
enum tagSTRUCTURED_QUERY_MULTIOPTION
    {
        SQMO_VIRTUAL_PROPERTY	= 0,
        SQMO_DEFAULT_PROPERTY	= ( SQMO_VIRTUAL_PROPERTY + 1 ) ,
        SQMO_GENERATOR_FOR_TYPE	= ( SQMO_DEFAULT_PROPERTY + 1 ) ,
        SQMO_MAP_PROPERTY	= ( SQMO_GENERATOR_FOR_TYPE + 1 ) 
    } 	STRUCTURED_QUERY_MULTIOPTION;

typedef /* [v1_enum] */ 
enum tagSTRUCTURED_QUERY_PARSE_ERROR
    {
        SQPE_NONE	= 0,
        SQPE_EXTRA_OPENING_PARENTHESIS	= ( SQPE_NONE + 1 ) ,
        SQPE_EXTRA_CLOSING_PARENTHESIS	= ( SQPE_EXTRA_OPENING_PARENTHESIS + 1 ) ,
        SQPE_IGNORED_MODIFIER	= ( SQPE_EXTRA_CLOSING_PARENTHESIS + 1 ) ,
        SQPE_IGNORED_CONNECTOR	= ( SQPE_IGNORED_MODIFIER + 1 ) ,
        SQPE_IGNORED_KEYWORD	= ( SQPE_IGNORED_CONNECTOR + 1 ) ,
        SQPE_UNHANDLED	= ( SQPE_IGNORED_KEYWORD + 1 ) 
    } 	STRUCTURED_QUERY_PARSE_ERROR;

typedef /* [v1_enum] */ 
enum STRUCTURED_QUERY_RESOLVE_OPTION
    {
        SQRO_DEFAULT	= 0,
        SQRO_DONT_RESOLVE_DATETIME	= 0x1,
        SQRO_ALWAYS_ONE_INTERVAL	= 0x2,
        SQRO_DONT_SIMPLIFY_CONDITION_TREES	= 0x4,
        SQRO_DONT_MAP_RELATIONS	= 0x8,
        SQRO_DONT_RESOLVE_RANGES	= 0x10,
        SQRO_DONT_REMOVE_UNRESTRICTED_KEYWORDS	= 0x20,
        SQRO_DONT_SPLIT_WORDS	= 0x40,
        SQRO_IGNORE_PHRASE_ORDER	= 0x80,
        SQRO_ADD_VALUE_TYPE_FOR_PLAIN_VALUES	= 0x100,
        SQRO_ADD_ROBUST_ITEM_NAME	= 0x200
    } 	STRUCTURED_QUERY_RESOLVE_OPTION;

DEFINE_ENUM_FLAG_OPERATORS(STRUCTURED_QUERY_RESOLVE_OPTION);
typedef /* [v1_enum] */ 
enum CASE_REQUIREMENT
    {
        CASE_REQUIREMENT_ANY	= 0,
        CASE_REQUIREMENT_UPPER_IF_AQS	= ( CASE_REQUIREMENT_ANY + 1 ) 
    } 	CASE_REQUIREMENT;

typedef /* [v1_enum] */ 
enum tagINTERVAL_LIMIT_KIND
    {
        ILK_EXPLICIT_INCLUDED	= 0,
        ILK_EXPLICIT_EXCLUDED	= ( ILK_EXPLICIT_INCLUDED + 1 ) ,
        ILK_NEGATIVE_INFINITY	= ( ILK_EXPLICIT_EXCLUDED + 1 ) ,
        ILK_POSITIVE_INFINITY	= ( ILK_NEGATIVE_INFINITY + 1 ) 
    } 	INTERVAL_LIMIT_KIND;

typedef /* [v1_enum] */ 
enum tagQUERY_PARSER_MANAGER_OPTION
    {
        QPMO_SCHEMA_BINARY_NAME	= 0,
        QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH	= ( QPMO_SCHEMA_BINARY_NAME + 1 ) ,
        QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH	= ( QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH + 1 ) ,
        QPMO_LOCALIZED_SCHEMA_BINARY_PATH	= ( QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH + 1 ) ,
        QPMO_APPEND_LCID_TO_LOCALIZED_PATH	= ( QPMO_LOCALIZED_SCHEMA_BINARY_PATH + 1 ) ,
        QPMO_LOCALIZER_SUPPORT	= ( QPMO_APPEND_LCID_TO_LOCALIZED_PATH + 1 ) 
    } 	QUERY_PARSER_MANAGER_OPTION;



extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0000_v0_0_s_ifspec;

#ifndef __IQueryParser_INTERFACE_DEFINED__
#define __IQueryParser_INTERFACE_DEFINED__

/* interface IQueryParser */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IQueryParser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2EBDEE67-3505-43f8-9946-EA44ABC8E5B0")
    IQueryParser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Parse( 
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ __RPC__in_opt IEnumUnknown *pCustomProperties,
            /* [retval][out] */ __RPC__deref_out_opt IQuerySolution **ppSolution) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOption( 
            /* [in] */ STRUCTURED_QUERY_SINGLE_OPTION option,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOption( 
            /* [in] */ STRUCTURED_QUERY_SINGLE_OPTION option,
            /* [retval][out] */ __RPC__out PROPVARIANT *pOptionValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMultiOption( 
            /* [in] */ STRUCTURED_QUERY_MULTIOPTION option,
            /* [in] */ __RPC__in LPCWSTR pszOptionKey,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSchemaProvider( 
            /* [retval][out] */ __RPC__deref_out_opt ISchemaProvider **ppSchemaProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestateToString( 
            /* [in] */ __RPC__in_opt ICondition *pCondition,
            /* [in] */ BOOL fUseEnglish,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszQueryString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParsePropertyValue( 
            /* [in] */ __RPC__in LPCWSTR pszPropertyName,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [retval][out] */ __RPC__deref_out_opt IQuerySolution **ppSolution) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestatePropertyValueToString( 
            /* [in] */ __RPC__in_opt ICondition *pCondition,
            /* [in] */ BOOL fUseEnglish,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPropertyName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszQueryString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryParserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryParser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryParser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryParser * This);
        
        DECLSPEC_XFGVIRT(IQueryParser, Parse)
        HRESULT ( STDMETHODCALLTYPE *Parse )( 
            __RPC__in IQueryParser * This,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ __RPC__in_opt IEnumUnknown *pCustomProperties,
            /* [retval][out] */ __RPC__deref_out_opt IQuerySolution **ppSolution);
        
        DECLSPEC_XFGVIRT(IQueryParser, SetOption)
        HRESULT ( STDMETHODCALLTYPE *SetOption )( 
            __RPC__in IQueryParser * This,
            /* [in] */ STRUCTURED_QUERY_SINGLE_OPTION option,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue);
        
        DECLSPEC_XFGVIRT(IQueryParser, GetOption)
        HRESULT ( STDMETHODCALLTYPE *GetOption )( 
            __RPC__in IQueryParser * This,
            /* [in] */ STRUCTURED_QUERY_SINGLE_OPTION option,
            /* [retval][out] */ __RPC__out PROPVARIANT *pOptionValue);
        
        DECLSPEC_XFGVIRT(IQueryParser, SetMultiOption)
        HRESULT ( STDMETHODCALLTYPE *SetMultiOption )( 
            __RPC__in IQueryParser * This,
            /* [in] */ STRUCTURED_QUERY_MULTIOPTION option,
            /* [in] */ __RPC__in LPCWSTR pszOptionKey,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue);
        
        DECLSPEC_XFGVIRT(IQueryParser, GetSchemaProvider)
        HRESULT ( STDMETHODCALLTYPE *GetSchemaProvider )( 
            __RPC__in IQueryParser * This,
            /* [retval][out] */ __RPC__deref_out_opt ISchemaProvider **ppSchemaProvider);
        
        DECLSPEC_XFGVIRT(IQueryParser, RestateToString)
        HRESULT ( STDMETHODCALLTYPE *RestateToString )( 
            __RPC__in IQueryParser * This,
            /* [in] */ __RPC__in_opt ICondition *pCondition,
            /* [in] */ BOOL fUseEnglish,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszQueryString);
        
        DECLSPEC_XFGVIRT(IQueryParser, ParsePropertyValue)
        HRESULT ( STDMETHODCALLTYPE *ParsePropertyValue )( 
            __RPC__in IQueryParser * This,
            /* [in] */ __RPC__in LPCWSTR pszPropertyName,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [retval][out] */ __RPC__deref_out_opt IQuerySolution **ppSolution);
        
        DECLSPEC_XFGVIRT(IQueryParser, RestatePropertyValueToString)
        HRESULT ( STDMETHODCALLTYPE *RestatePropertyValueToString )( 
            __RPC__in IQueryParser * This,
            /* [in] */ __RPC__in_opt ICondition *pCondition,
            /* [in] */ BOOL fUseEnglish,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPropertyName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszQueryString);
        
        END_INTERFACE
    } IQueryParserVtbl;

    interface IQueryParser
    {
        CONST_VTBL struct IQueryParserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryParser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryParser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryParser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryParser_Parse(This,pszInputString,pCustomProperties,ppSolution)	\
    ( (This)->lpVtbl -> Parse(This,pszInputString,pCustomProperties,ppSolution) ) 

#define IQueryParser_SetOption(This,option,pOptionValue)	\
    ( (This)->lpVtbl -> SetOption(This,option,pOptionValue) ) 

#define IQueryParser_GetOption(This,option,pOptionValue)	\
    ( (This)->lpVtbl -> GetOption(This,option,pOptionValue) ) 

#define IQueryParser_SetMultiOption(This,option,pszOptionKey,pOptionValue)	\
    ( (This)->lpVtbl -> SetMultiOption(This,option,pszOptionKey,pOptionValue) ) 

#define IQueryParser_GetSchemaProvider(This,ppSchemaProvider)	\
    ( (This)->lpVtbl -> GetSchemaProvider(This,ppSchemaProvider) ) 

#define IQueryParser_RestateToString(This,pCondition,fUseEnglish,ppszQueryString)	\
    ( (This)->lpVtbl -> RestateToString(This,pCondition,fUseEnglish,ppszQueryString) ) 

#define IQueryParser_ParsePropertyValue(This,pszPropertyName,pszInputString,ppSolution)	\
    ( (This)->lpVtbl -> ParsePropertyValue(This,pszPropertyName,pszInputString,ppSolution) ) 

#define IQueryParser_RestatePropertyValueToString(This,pCondition,fUseEnglish,ppszPropertyName,ppszQueryString)	\
    ( (This)->lpVtbl -> RestatePropertyValueToString(This,pCondition,fUseEnglish,ppszPropertyName,ppszQueryString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryParser_INTERFACE_DEFINED__ */


#ifndef __IConditionFactory_INTERFACE_DEFINED__
#define __IConditionFactory_INTERFACE_DEFINED__

/* interface IConditionFactory */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IConditionFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A5EFE073-B16F-474f-9F3E-9F8B497A3E08")
    IConditionFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MakeNot( 
            /* [in] */ __RPC__in_opt ICondition *pcSub,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeAndOr( 
            /* [in] */ CONDITION_TYPE ct,
            /* [in] */ __RPC__in_opt IEnumUnknown *peuSubs,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeLeaf( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValueType,
            /* [in] */ __RPC__in const PROPVARIANT *ppropvar,
            /* [in] */ __RPC__in_opt IRichChunk *pPropertyNameTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pOperationTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pValueTerm,
            /* [in] */ BOOL fExpand,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Resolve( 
            /* [annotation][in] */ 
            _In_  ICondition *pc,
            /* [annotation][in] */ 
            _In_  STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][ref][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [annotation][retval][out] */ 
            _Out_  ICondition **ppcResolved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConditionFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConditionFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConditionFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConditionFactory * This);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeNot)
        HRESULT ( STDMETHODCALLTYPE *MakeNot )( 
            __RPC__in IConditionFactory * This,
            /* [in] */ __RPC__in_opt ICondition *pcSub,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeAndOr)
        HRESULT ( STDMETHODCALLTYPE *MakeAndOr )( 
            __RPC__in IConditionFactory * This,
            /* [in] */ CONDITION_TYPE ct,
            /* [in] */ __RPC__in_opt IEnumUnknown *peuSubs,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeLeaf)
        HRESULT ( STDMETHODCALLTYPE *MakeLeaf )( 
            __RPC__in IConditionFactory * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValueType,
            /* [in] */ __RPC__in const PROPVARIANT *ppropvar,
            /* [in] */ __RPC__in_opt IRichChunk *pPropertyNameTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pOperationTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pValueTerm,
            /* [in] */ BOOL fExpand,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, Resolve)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            IConditionFactory * This,
            /* [annotation][in] */ 
            _In_  ICondition *pc,
            /* [annotation][in] */ 
            _In_  STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][ref][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [annotation][retval][out] */ 
            _Out_  ICondition **ppcResolved);
        
        END_INTERFACE
    } IConditionFactoryVtbl;

    interface IConditionFactory
    {
        CONST_VTBL struct IConditionFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConditionFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConditionFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConditionFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConditionFactory_MakeNot(This,pcSub,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeNot(This,pcSub,fSimplify,ppcResult) ) 

#define IConditionFactory_MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult) ) 

#define IConditionFactory_MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult)	\
    ( (This)->lpVtbl -> MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult) ) 

#define IConditionFactory_Resolve(This,pc,sqro,pstReferenceTime,ppcResolved)	\
    ( (This)->lpVtbl -> Resolve(This,pc,sqro,pstReferenceTime,ppcResolved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConditionFactory_INTERFACE_DEFINED__ */


#ifndef __IQuerySolution_INTERFACE_DEFINED__
#define __IQuerySolution_INTERFACE_DEFINED__

/* interface IQuerySolution */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IQuerySolution;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6EBC66B-8921-4193-AFDD-A1789FB7FF57")
    IQuerySolution : public IConditionFactory
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetQuery( 
            /* [annotation][out] */ 
            _Out_opt_  ICondition **ppQueryNode,
            /* [annotation][out] */ 
            _Out_opt_  IEntity **ppMainType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrors( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **ppParseErrors) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetLexicalData( 
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszInputString,
            /* [annotation][out] */ 
            _Out_opt_  ITokenCollection **ppTokens,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plcid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppWordBreaker) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQuerySolutionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQuerySolution * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQuerySolution * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQuerySolution * This);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeNot)
        HRESULT ( STDMETHODCALLTYPE *MakeNot )( 
            __RPC__in IQuerySolution * This,
            /* [in] */ __RPC__in_opt ICondition *pcSub,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeAndOr)
        HRESULT ( STDMETHODCALLTYPE *MakeAndOr )( 
            __RPC__in IQuerySolution * This,
            /* [in] */ CONDITION_TYPE ct,
            /* [in] */ __RPC__in_opt IEnumUnknown *peuSubs,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeLeaf)
        HRESULT ( STDMETHODCALLTYPE *MakeLeaf )( 
            __RPC__in IQuerySolution * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValueType,
            /* [in] */ __RPC__in const PROPVARIANT *ppropvar,
            /* [in] */ __RPC__in_opt IRichChunk *pPropertyNameTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pOperationTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pValueTerm,
            /* [in] */ BOOL fExpand,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, Resolve)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            IQuerySolution * This,
            /* [annotation][in] */ 
            _In_  ICondition *pc,
            /* [annotation][in] */ 
            _In_  STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][ref][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [annotation][retval][out] */ 
            _Out_  ICondition **ppcResolved);
        
        DECLSPEC_XFGVIRT(IQuerySolution, GetQuery)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetQuery )( 
            IQuerySolution * This,
            /* [annotation][out] */ 
            _Out_opt_  ICondition **ppQueryNode,
            /* [annotation][out] */ 
            _Out_opt_  IEntity **ppMainType);
        
        DECLSPEC_XFGVIRT(IQuerySolution, GetErrors)
        HRESULT ( STDMETHODCALLTYPE *GetErrors )( 
            __RPC__in IQuerySolution * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **ppParseErrors);
        
        DECLSPEC_XFGVIRT(IQuerySolution, GetLexicalData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetLexicalData )( 
            IQuerySolution * This,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszInputString,
            /* [annotation][out] */ 
            _Out_opt_  ITokenCollection **ppTokens,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plcid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppWordBreaker);
        
        END_INTERFACE
    } IQuerySolutionVtbl;

    interface IQuerySolution
    {
        CONST_VTBL struct IQuerySolutionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQuerySolution_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQuerySolution_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQuerySolution_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQuerySolution_MakeNot(This,pcSub,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeNot(This,pcSub,fSimplify,ppcResult) ) 

#define IQuerySolution_MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult) ) 

#define IQuerySolution_MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult)	\
    ( (This)->lpVtbl -> MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult) ) 

#define IQuerySolution_Resolve(This,pc,sqro,pstReferenceTime,ppcResolved)	\
    ( (This)->lpVtbl -> Resolve(This,pc,sqro,pstReferenceTime,ppcResolved) ) 


#define IQuerySolution_GetQuery(This,ppQueryNode,ppMainType)	\
    ( (This)->lpVtbl -> GetQuery(This,ppQueryNode,ppMainType) ) 

#define IQuerySolution_GetErrors(This,riid,ppParseErrors)	\
    ( (This)->lpVtbl -> GetErrors(This,riid,ppParseErrors) ) 

#define IQuerySolution_GetLexicalData(This,ppszInputString,ppTokens,plcid,ppWordBreaker)	\
    ( (This)->lpVtbl -> GetLexicalData(This,ppszInputString,ppTokens,plcid,ppWordBreaker) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQuerySolution_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_structuredquery_0000_0003 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum CONDITION_CREATION_OPTIONS
    {
        CONDITION_CREATION_DEFAULT	= 0,
        CONDITION_CREATION_NONE	= 0,
        CONDITION_CREATION_SIMPLIFY	= 0x1,
        CONDITION_CREATION_VECTOR_AND	= 0x2,
        CONDITION_CREATION_VECTOR_OR	= 0x4,
        CONDITION_CREATION_VECTOR_LEAF	= 0x8,
        CONDITION_CREATION_USE_CONTENT_LOCALE	= 0x10
    } 	CONDITION_CREATION_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(CONDITION_CREATION_OPTIONS);


extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0003_v0_0_s_ifspec;

#ifndef __IConditionFactory2_INTERFACE_DEFINED__
#define __IConditionFactory2_INTERFACE_DEFINED__

/* interface IConditionFactory2 */
/* [local][unique][object][uuid] */ 


EXTERN_C const IID IID_IConditionFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71D222E1-432F-429e-8C13-B6DAFDE5077A")
    IConditionFactory2 : public IConditionFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateTrueFalse( 
            /* [in] */ BOOL fVal,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateNegation( 
            /* [in] */ ICondition *pcSub,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCompoundFromObjectArray( 
            /* [in] */ CONDITION_TYPE ct,
            /* [annotation][in] */ 
            _In_opt_  IObjectArray *poaSubs,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCompoundFromArray( 
            /* [in] */ CONDITION_TYPE ct,
            /* [size_is][in] */ ICondition **ppcondSubs,
            /* [in] */ ULONG cSubs,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStringLeaf( 
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ LPCWSTR pszValue,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszLocaleName,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateIntegerLeaf( 
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ INT32 lValue,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBooleanLeaf( 
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ BOOL fValue,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateLeaf( 
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ REFPROPVARIANT propvar,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszSemanticType,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszLocaleName,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pPropertyNameTerm,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pOperationTerm,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pValueTerm,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolveCondition( 
            /* [in] */ ICondition *pc,
            /* [in] */ STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConditionFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IConditionFactory2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IConditionFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IConditionFactory2 * This);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeNot)
        HRESULT ( STDMETHODCALLTYPE *MakeNot )( 
            IConditionFactory2 * This,
            /* [in] */ ICondition *pcSub,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeAndOr)
        HRESULT ( STDMETHODCALLTYPE *MakeAndOr )( 
            IConditionFactory2 * This,
            /* [in] */ CONDITION_TYPE ct,
            /* [in] */ IEnumUnknown *peuSubs,
            /* [in] */ BOOL fSimplify,
            /* [retval][out] */ ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, MakeLeaf)
        HRESULT ( STDMETHODCALLTYPE *MakeLeaf )( 
            IConditionFactory2 * This,
            /* [unique][in] */ LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ LPCWSTR pszValueType,
            /* [in] */ const PROPVARIANT *ppropvar,
            /* [in] */ IRichChunk *pPropertyNameTerm,
            /* [in] */ IRichChunk *pOperationTerm,
            /* [in] */ IRichChunk *pValueTerm,
            /* [in] */ BOOL fExpand,
            /* [retval][out] */ ICondition **ppcResult);
        
        DECLSPEC_XFGVIRT(IConditionFactory, Resolve)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            IConditionFactory2 * This,
            /* [annotation][in] */ 
            _In_  ICondition *pc,
            /* [annotation][in] */ 
            _In_  STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][ref][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [annotation][retval][out] */ 
            _Out_  ICondition **ppcResolved);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateTrueFalse)
        HRESULT ( STDMETHODCALLTYPE *CreateTrueFalse )( 
            IConditionFactory2 * This,
            /* [in] */ BOOL fVal,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateNegation)
        HRESULT ( STDMETHODCALLTYPE *CreateNegation )( 
            IConditionFactory2 * This,
            /* [in] */ ICondition *pcSub,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateCompoundFromObjectArray)
        HRESULT ( STDMETHODCALLTYPE *CreateCompoundFromObjectArray )( 
            IConditionFactory2 * This,
            /* [in] */ CONDITION_TYPE ct,
            /* [annotation][in] */ 
            _In_opt_  IObjectArray *poaSubs,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateCompoundFromArray)
        HRESULT ( STDMETHODCALLTYPE *CreateCompoundFromArray )( 
            IConditionFactory2 * This,
            /* [in] */ CONDITION_TYPE ct,
            /* [size_is][in] */ ICondition **ppcondSubs,
            /* [in] */ ULONG cSubs,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateStringLeaf)
        HRESULT ( STDMETHODCALLTYPE *CreateStringLeaf )( 
            IConditionFactory2 * This,
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ LPCWSTR pszValue,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszLocaleName,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateIntegerLeaf)
        HRESULT ( STDMETHODCALLTYPE *CreateIntegerLeaf )( 
            IConditionFactory2 * This,
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ INT32 lValue,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateBooleanLeaf)
        HRESULT ( STDMETHODCALLTYPE *CreateBooleanLeaf )( 
            IConditionFactory2 * This,
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ BOOL fValue,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, CreateLeaf)
        HRESULT ( STDMETHODCALLTYPE *CreateLeaf )( 
            IConditionFactory2 * This,
            /* [in] */ REFPROPERTYKEY propkey,
            /* [in] */ CONDITION_OPERATION cop,
            /* [in] */ REFPROPVARIANT propvar,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszSemanticType,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszLocaleName,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pPropertyNameTerm,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pOperationTerm,
            /* [annotation][in] */ 
            _In_opt_  IRichChunk *pValueTerm,
            /* [in] */ CONDITION_CREATION_OPTIONS cco,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IConditionFactory2, ResolveCondition)
        HRESULT ( STDMETHODCALLTYPE *ResolveCondition )( 
            IConditionFactory2 * This,
            /* [in] */ ICondition *pc,
            /* [in] */ STRUCTURED_QUERY_RESOLVE_OPTION sqro,
            /* [annotation][in] */ 
            _In_opt_  const SYSTEMTIME *pstReferenceTime,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        END_INTERFACE
    } IConditionFactory2Vtbl;

    interface IConditionFactory2
    {
        CONST_VTBL struct IConditionFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConditionFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConditionFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConditionFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConditionFactory2_MakeNot(This,pcSub,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeNot(This,pcSub,fSimplify,ppcResult) ) 

#define IConditionFactory2_MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult)	\
    ( (This)->lpVtbl -> MakeAndOr(This,ct,peuSubs,fSimplify,ppcResult) ) 

#define IConditionFactory2_MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult)	\
    ( (This)->lpVtbl -> MakeLeaf(This,pszPropertyName,cop,pszValueType,ppropvar,pPropertyNameTerm,pOperationTerm,pValueTerm,fExpand,ppcResult) ) 

#define IConditionFactory2_Resolve(This,pc,sqro,pstReferenceTime,ppcResolved)	\
    ( (This)->lpVtbl -> Resolve(This,pc,sqro,pstReferenceTime,ppcResolved) ) 


#define IConditionFactory2_CreateTrueFalse(This,fVal,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateTrueFalse(This,fVal,cco,riid,ppv) ) 

#define IConditionFactory2_CreateNegation(This,pcSub,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateNegation(This,pcSub,cco,riid,ppv) ) 

#define IConditionFactory2_CreateCompoundFromObjectArray(This,ct,poaSubs,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateCompoundFromObjectArray(This,ct,poaSubs,cco,riid,ppv) ) 

#define IConditionFactory2_CreateCompoundFromArray(This,ct,ppcondSubs,cSubs,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateCompoundFromArray(This,ct,ppcondSubs,cSubs,cco,riid,ppv) ) 

#define IConditionFactory2_CreateStringLeaf(This,propkey,cop,pszValue,pszLocaleName,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateStringLeaf(This,propkey,cop,pszValue,pszLocaleName,cco,riid,ppv) ) 

#define IConditionFactory2_CreateIntegerLeaf(This,propkey,cop,lValue,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateIntegerLeaf(This,propkey,cop,lValue,cco,riid,ppv) ) 

#define IConditionFactory2_CreateBooleanLeaf(This,propkey,cop,fValue,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateBooleanLeaf(This,propkey,cop,fValue,cco,riid,ppv) ) 

#define IConditionFactory2_CreateLeaf(This,propkey,cop,propvar,pszSemanticType,pszLocaleName,pPropertyNameTerm,pOperationTerm,pValueTerm,cco,riid,ppv)	\
    ( (This)->lpVtbl -> CreateLeaf(This,propkey,cop,propvar,pszSemanticType,pszLocaleName,pPropertyNameTerm,pOperationTerm,pValueTerm,cco,riid,ppv) ) 

#define IConditionFactory2_ResolveCondition(This,pc,sqro,pstReferenceTime,riid,ppv)	\
    ( (This)->lpVtbl -> ResolveCondition(This,pc,sqro,pstReferenceTime,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConditionFactory2_INTERFACE_DEFINED__ */


#ifndef __IConditionGenerator_INTERFACE_DEFINED__
#define __IConditionGenerator_INTERFACE_DEFINED__

/* interface IConditionGenerator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IConditionGenerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92D2CC58-4386-45a3-B98C-7E0CE64A4117")
    IConditionGenerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISchemaProvider *pSchemaProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecognizeNamedEntities( 
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ LCID lcidUserLocale,
            /* [in] */ __RPC__in_opt ITokenCollection *pTokenCollection,
            /* [out][in] */ __RPC__inout_opt INamedEntityCollector *pNamedEntities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateForLeaf( 
            /* [in] */ __RPC__in_opt IConditionFactory *pConditionFactory,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValueType,
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValue2,
            /* [in] */ __RPC__in_opt IRichChunk *pPropertyNameTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pOperationTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pValueTerm,
            /* [in] */ BOOL automaticWildcard,
            /* [out] */ __RPC__out BOOL *pNoStringQuery,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppQueryExpression) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DefaultPhrase( 
            /* [unique][in] */ LPCWSTR pszValueType,
            /* [in] */ const PROPVARIANT *ppropvar,
            /* [in] */ BOOL fUseEnglish,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConditionGeneratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConditionGenerator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConditionGenerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConditionGenerator * This);
        
        DECLSPEC_XFGVIRT(IConditionGenerator, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IConditionGenerator * This,
            /* [in] */ __RPC__in_opt ISchemaProvider *pSchemaProvider);
        
        DECLSPEC_XFGVIRT(IConditionGenerator, RecognizeNamedEntities)
        HRESULT ( STDMETHODCALLTYPE *RecognizeNamedEntities )( 
            __RPC__in IConditionGenerator * This,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ LCID lcidUserLocale,
            /* [in] */ __RPC__in_opt ITokenCollection *pTokenCollection,
            /* [out][in] */ __RPC__inout_opt INamedEntityCollector *pNamedEntities);
        
        DECLSPEC_XFGVIRT(IConditionGenerator, GenerateForLeaf)
        HRESULT ( STDMETHODCALLTYPE *GenerateForLeaf )( 
            __RPC__in IConditionGenerator * This,
            /* [in] */ __RPC__in_opt IConditionFactory *pConditionFactory,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszPropertyName,
            /* [in] */ CONDITION_OPERATION cop,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValueType,
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszValue2,
            /* [in] */ __RPC__in_opt IRichChunk *pPropertyNameTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pOperationTerm,
            /* [in] */ __RPC__in_opt IRichChunk *pValueTerm,
            /* [in] */ BOOL automaticWildcard,
            /* [out] */ __RPC__out BOOL *pNoStringQuery,
            /* [retval][out] */ __RPC__deref_out_opt ICondition **ppQueryExpression);
        
        DECLSPEC_XFGVIRT(IConditionGenerator, DefaultPhrase)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DefaultPhrase )( 
            IConditionGenerator * This,
            /* [unique][in] */ LPCWSTR pszValueType,
            /* [in] */ const PROPVARIANT *ppropvar,
            /* [in] */ BOOL fUseEnglish,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase);
        
        END_INTERFACE
    } IConditionGeneratorVtbl;

    interface IConditionGenerator
    {
        CONST_VTBL struct IConditionGeneratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConditionGenerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConditionGenerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConditionGenerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConditionGenerator_Initialize(This,pSchemaProvider)	\
    ( (This)->lpVtbl -> Initialize(This,pSchemaProvider) ) 

#define IConditionGenerator_RecognizeNamedEntities(This,pszInputString,lcidUserLocale,pTokenCollection,pNamedEntities)	\
    ( (This)->lpVtbl -> RecognizeNamedEntities(This,pszInputString,lcidUserLocale,pTokenCollection,pNamedEntities) ) 

#define IConditionGenerator_GenerateForLeaf(This,pConditionFactory,pszPropertyName,cop,pszValueType,pszValue,pszValue2,pPropertyNameTerm,pOperationTerm,pValueTerm,automaticWildcard,pNoStringQuery,ppQueryExpression)	\
    ( (This)->lpVtbl -> GenerateForLeaf(This,pConditionFactory,pszPropertyName,cop,pszValueType,pszValue,pszValue2,pPropertyNameTerm,pOperationTerm,pValueTerm,automaticWildcard,pNoStringQuery,ppQueryExpression) ) 

#define IConditionGenerator_DefaultPhrase(This,pszValueType,ppropvar,fUseEnglish,ppszPhrase)	\
    ( (This)->lpVtbl -> DefaultPhrase(This,pszValueType,ppropvar,fUseEnglish,ppszPhrase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConditionGenerator_INTERFACE_DEFINED__ */


#ifndef __IInterval_INTERFACE_DEFINED__
#define __IInterval_INTERFACE_DEFINED__

/* interface IInterval */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IInterval;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6BF0A714-3C18-430b-8B5D-83B1C234D3DB")
    IInterval : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLimits( 
            /* [out] */ __RPC__out INTERVAL_LIMIT_KIND *pilkLower,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarLower,
            /* [out] */ __RPC__out INTERVAL_LIMIT_KIND *pilkUpper,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarUpper) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIntervalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInterval * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInterval * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInterval * This);
        
        DECLSPEC_XFGVIRT(IInterval, GetLimits)
        HRESULT ( STDMETHODCALLTYPE *GetLimits )( 
            __RPC__in IInterval * This,
            /* [out] */ __RPC__out INTERVAL_LIMIT_KIND *pilkLower,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarLower,
            /* [out] */ __RPC__out INTERVAL_LIMIT_KIND *pilkUpper,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarUpper);
        
        END_INTERFACE
    } IIntervalVtbl;

    interface IInterval
    {
        CONST_VTBL struct IIntervalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInterval_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInterval_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInterval_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInterval_GetLimits(This,pilkLower,ppropvarLower,pilkUpper,ppropvarUpper)	\
    ( (This)->lpVtbl -> GetLimits(This,pilkLower,ppropvarLower,pilkUpper,ppropvarUpper) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInterval_INTERFACE_DEFINED__ */


#ifndef __IMetaData_INTERFACE_DEFINED__
#define __IMetaData_INTERFACE_DEFINED__

/* interface IMetaData */
/* [unique][uuid][object][helpstring] */ 


EXTERN_C const IID IID_IMetaData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("780102B0-C43B-4876-BC7B-5E9BA5C88794")
    IMetaData : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszKey,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMetaData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMetaData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMetaData * This);
        
        DECLSPEC_XFGVIRT(IMetaData, GetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMetaData * This,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszKey,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszValue);
        
        END_INTERFACE
    } IMetaDataVtbl;

    interface IMetaData
    {
        CONST_VTBL struct IMetaDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaData_GetData(This,ppszKey,ppszValue)	\
    ( (This)->lpVtbl -> GetData(This,ppszKey,ppszValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaData_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_structuredquery_0000_0007 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0007_v0_0_s_ifspec;

#ifndef __IEntity_INTERFACE_DEFINED__
#define __IEntity_INTERFACE_DEFINED__

/* interface IEntity */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IEntity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24264891-E80B-4fd3-B7CE-4FF2FAE8931F")
    IEntity : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Name( 
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Base( 
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pBaseEntity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Relationships( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pRelationships) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelationship( 
            /* [in] */ __RPC__in LPCWSTR pszRelationName,
            /* [retval][out] */ __RPC__deref_out_opt IRelationship **pRelationship) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MetaData( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NamedEntities( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pNamedEntities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamedEntity( 
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [retval][out] */ __RPC__deref_out_opt INamedEntity **ppNamedEntity) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DefaultPhrase( 
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEntityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEntity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEntity * This);
        
        DECLSPEC_XFGVIRT(IEntity, Name)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Name )( 
            IEntity * This,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IEntity, Base)
        HRESULT ( STDMETHODCALLTYPE *Base )( 
            __RPC__in IEntity * This,
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pBaseEntity);
        
        DECLSPEC_XFGVIRT(IEntity, Relationships)
        HRESULT ( STDMETHODCALLTYPE *Relationships )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pRelationships);
        
        DECLSPEC_XFGVIRT(IEntity, GetRelationship)
        HRESULT ( STDMETHODCALLTYPE *GetRelationship )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in LPCWSTR pszRelationName,
            /* [retval][out] */ __RPC__deref_out_opt IRelationship **pRelationship);
        
        DECLSPEC_XFGVIRT(IEntity, MetaData)
        HRESULT ( STDMETHODCALLTYPE *MetaData )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData);
        
        DECLSPEC_XFGVIRT(IEntity, NamedEntities)
        HRESULT ( STDMETHODCALLTYPE *NamedEntities )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pNamedEntities);
        
        DECLSPEC_XFGVIRT(IEntity, GetNamedEntity)
        HRESULT ( STDMETHODCALLTYPE *GetNamedEntity )( 
            __RPC__in IEntity * This,
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [retval][out] */ __RPC__deref_out_opt INamedEntity **ppNamedEntity);
        
        DECLSPEC_XFGVIRT(IEntity, DefaultPhrase)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DefaultPhrase )( 
            IEntity * This,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase);
        
        END_INTERFACE
    } IEntityVtbl;

    interface IEntity
    {
        CONST_VTBL struct IEntityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEntity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEntity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEntity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEntity_Name(This,ppszName)	\
    ( (This)->lpVtbl -> Name(This,ppszName) ) 

#define IEntity_Base(This,pBaseEntity)	\
    ( (This)->lpVtbl -> Base(This,pBaseEntity) ) 

#define IEntity_Relationships(This,riid,pRelationships)	\
    ( (This)->lpVtbl -> Relationships(This,riid,pRelationships) ) 

#define IEntity_GetRelationship(This,pszRelationName,pRelationship)	\
    ( (This)->lpVtbl -> GetRelationship(This,pszRelationName,pRelationship) ) 

#define IEntity_MetaData(This,riid,pMetaData)	\
    ( (This)->lpVtbl -> MetaData(This,riid,pMetaData) ) 

#define IEntity_NamedEntities(This,riid,pNamedEntities)	\
    ( (This)->lpVtbl -> NamedEntities(This,riid,pNamedEntities) ) 

#define IEntity_GetNamedEntity(This,pszValue,ppNamedEntity)	\
    ( (This)->lpVtbl -> GetNamedEntity(This,pszValue,ppNamedEntity) ) 

#define IEntity_DefaultPhrase(This,ppszPhrase)	\
    ( (This)->lpVtbl -> DefaultPhrase(This,ppszPhrase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEntity_INTERFACE_DEFINED__ */


#ifndef __IRelationship_INTERFACE_DEFINED__
#define __IRelationship_INTERFACE_DEFINED__

/* interface IRelationship */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IRelationship;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2769280B-5108-498c-9C7F-A51239B63147")
    IRelationship : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Name( 
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsReal( 
            /* [retval][out] */ __RPC__out BOOL *pIsReal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Destination( 
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pDestinationEntity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MetaData( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DefaultPhrase( 
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRelationshipVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRelationship * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRelationship * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRelationship * This);
        
        DECLSPEC_XFGVIRT(IRelationship, Name)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Name )( 
            IRelationship * This,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IRelationship, IsReal)
        HRESULT ( STDMETHODCALLTYPE *IsReal )( 
            __RPC__in IRelationship * This,
            /* [retval][out] */ __RPC__out BOOL *pIsReal);
        
        DECLSPEC_XFGVIRT(IRelationship, Destination)
        HRESULT ( STDMETHODCALLTYPE *Destination )( 
            __RPC__in IRelationship * This,
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pDestinationEntity);
        
        DECLSPEC_XFGVIRT(IRelationship, MetaData)
        HRESULT ( STDMETHODCALLTYPE *MetaData )( 
            __RPC__in IRelationship * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData);
        
        DECLSPEC_XFGVIRT(IRelationship, DefaultPhrase)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DefaultPhrase )( 
            IRelationship * This,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase);
        
        END_INTERFACE
    } IRelationshipVtbl;

    interface IRelationship
    {
        CONST_VTBL struct IRelationshipVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRelationship_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRelationship_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRelationship_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRelationship_Name(This,ppszName)	\
    ( (This)->lpVtbl -> Name(This,ppszName) ) 

#define IRelationship_IsReal(This,pIsReal)	\
    ( (This)->lpVtbl -> IsReal(This,pIsReal) ) 

#define IRelationship_Destination(This,pDestinationEntity)	\
    ( (This)->lpVtbl -> Destination(This,pDestinationEntity) ) 

#define IRelationship_MetaData(This,riid,pMetaData)	\
    ( (This)->lpVtbl -> MetaData(This,riid,pMetaData) ) 

#define IRelationship_DefaultPhrase(This,ppszPhrase)	\
    ( (This)->lpVtbl -> DefaultPhrase(This,ppszPhrase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRelationship_INTERFACE_DEFINED__ */


#ifndef __INamedEntity_INTERFACE_DEFINED__
#define __INamedEntity_INTERFACE_DEFINED__

/* interface INamedEntity */
/* [unique][uuid][object][helpstring] */ 


EXTERN_C const IID IID_INamedEntity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ABDBD0B1-7D54-49fb-AB5C-BFF4130004CD")
    INamedEntity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppszValue) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DefaultPhrase( 
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INamedEntityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INamedEntity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INamedEntity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INamedEntity * This);
        
        DECLSPEC_XFGVIRT(INamedEntity, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in INamedEntity * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppszValue);
        
        DECLSPEC_XFGVIRT(INamedEntity, DefaultPhrase)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DefaultPhrase )( 
            INamedEntity * This,
            /* [retval][annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppszPhrase);
        
        END_INTERFACE
    } INamedEntityVtbl;

    interface INamedEntity
    {
        CONST_VTBL struct INamedEntityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INamedEntity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INamedEntity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INamedEntity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INamedEntity_GetValue(This,ppszValue)	\
    ( (This)->lpVtbl -> GetValue(This,ppszValue) ) 

#define INamedEntity_DefaultPhrase(This,ppszPhrase)	\
    ( (This)->lpVtbl -> DefaultPhrase(This,ppszPhrase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INamedEntity_INTERFACE_DEFINED__ */


#ifndef __ISchemaProvider_INTERFACE_DEFINED__
#define __ISchemaProvider_INTERFACE_DEFINED__

/* interface ISchemaProvider */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_ISchemaProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CF89BCB-394C-49b2-AE28-A59DD4ED7F68")
    ISchemaProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Entities( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pEntities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RootEntity( 
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pRootEntity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEntity( 
            /* [in] */ __RPC__in LPCWSTR pszEntityName,
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pEntity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MetaData( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Localize( 
            /* [in] */ LCID lcid,
            /* [in] */ __RPC__in_opt ISchemaLocalizerSupport *pSchemaLocalizerSupport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveBinary( 
            /* [in] */ __RPC__in LPCWSTR pszSchemaBinaryPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupAuthoredNamedEntity( 
            /* [in] */ __RPC__in_opt IEntity *pEntity,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ __RPC__in_opt ITokenCollection *pTokenCollection,
            /* [in] */ ULONG cTokensBegin,
            /* [out] */ __RPC__out ULONG *pcTokensLength,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISchemaProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISchemaProvider * This);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, Entities)
        HRESULT ( STDMETHODCALLTYPE *Entities )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pEntities);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, RootEntity)
        HRESULT ( STDMETHODCALLTYPE *RootEntity )( 
            __RPC__in ISchemaProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pRootEntity);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, GetEntity)
        HRESULT ( STDMETHODCALLTYPE *GetEntity )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in LPCWSTR pszEntityName,
            /* [retval][out] */ __RPC__deref_out_opt IEntity **pEntity);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, MetaData)
        HRESULT ( STDMETHODCALLTYPE *MetaData )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **pMetaData);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, Localize)
        HRESULT ( STDMETHODCALLTYPE *Localize )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ LCID lcid,
            /* [in] */ __RPC__in_opt ISchemaLocalizerSupport *pSchemaLocalizerSupport);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, SaveBinary)
        HRESULT ( STDMETHODCALLTYPE *SaveBinary )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in LPCWSTR pszSchemaBinaryPath);
        
        DECLSPEC_XFGVIRT(ISchemaProvider, LookupAuthoredNamedEntity)
        HRESULT ( STDMETHODCALLTYPE *LookupAuthoredNamedEntity )( 
            __RPC__in ISchemaProvider * This,
            /* [in] */ __RPC__in_opt IEntity *pEntity,
            /* [in] */ __RPC__in LPCWSTR pszInputString,
            /* [in] */ __RPC__in_opt ITokenCollection *pTokenCollection,
            /* [in] */ ULONG cTokensBegin,
            /* [out] */ __RPC__out ULONG *pcTokensLength,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszValue);
        
        END_INTERFACE
    } ISchemaProviderVtbl;

    interface ISchemaProvider
    {
        CONST_VTBL struct ISchemaProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaProvider_Entities(This,riid,pEntities)	\
    ( (This)->lpVtbl -> Entities(This,riid,pEntities) ) 

#define ISchemaProvider_RootEntity(This,pRootEntity)	\
    ( (This)->lpVtbl -> RootEntity(This,pRootEntity) ) 

#define ISchemaProvider_GetEntity(This,pszEntityName,pEntity)	\
    ( (This)->lpVtbl -> GetEntity(This,pszEntityName,pEntity) ) 

#define ISchemaProvider_MetaData(This,riid,pMetaData)	\
    ( (This)->lpVtbl -> MetaData(This,riid,pMetaData) ) 

#define ISchemaProvider_Localize(This,lcid,pSchemaLocalizerSupport)	\
    ( (This)->lpVtbl -> Localize(This,lcid,pSchemaLocalizerSupport) ) 

#define ISchemaProvider_SaveBinary(This,pszSchemaBinaryPath)	\
    ( (This)->lpVtbl -> SaveBinary(This,pszSchemaBinaryPath) ) 

#define ISchemaProvider_LookupAuthoredNamedEntity(This,pEntity,pszInputString,pTokenCollection,cTokensBegin,pcTokensLength,ppszValue)	\
    ( (This)->lpVtbl -> LookupAuthoredNamedEntity(This,pEntity,pszInputString,pTokenCollection,cTokensBegin,pcTokensLength,ppszValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaProvider_INTERFACE_DEFINED__ */


#ifndef __ITokenCollection_INTERFACE_DEFINED__
#define __ITokenCollection_INTERFACE_DEFINED__

/* interface ITokenCollection */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_ITokenCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22D8B4F2-F577-4adb-A335-C2AE88416FAB")
    ITokenCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NumberOfTokens( 
            __RPC__in ULONG *pCount) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetToken( 
            /* [in] */ ULONG i,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pBegin,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pLength,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppsz) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITokenCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITokenCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITokenCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITokenCollection * This);
        
        DECLSPEC_XFGVIRT(ITokenCollection, NumberOfTokens)
        HRESULT ( STDMETHODCALLTYPE *NumberOfTokens )( 
            __RPC__in ITokenCollection * This,
            __RPC__in ULONG *pCount);
        
        DECLSPEC_XFGVIRT(ITokenCollection, GetToken)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetToken )( 
            ITokenCollection * This,
            /* [in] */ ULONG i,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pBegin,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pLength,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppsz);
        
        END_INTERFACE
    } ITokenCollectionVtbl;

    interface ITokenCollection
    {
        CONST_VTBL struct ITokenCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITokenCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITokenCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITokenCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITokenCollection_NumberOfTokens(This,pCount)	\
    ( (This)->lpVtbl -> NumberOfTokens(This,pCount) ) 

#define ITokenCollection_GetToken(This,i,pBegin,pLength,ppsz)	\
    ( (This)->lpVtbl -> GetToken(This,i,pBegin,pLength,ppsz) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITokenCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_structuredquery_0000_0012 */
/* [local] */ 

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_structuredquery_0000_0012_0001
    {
        NEC_LOW	= 0,
        NEC_MEDIUM	= ( NEC_LOW + 1 ) ,
        NEC_HIGH	= ( NEC_MEDIUM + 1 ) 
    } 	NAMED_ENTITY_CERTAINTY;



extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0012_v0_0_s_ifspec;

#ifndef __INamedEntityCollector_INTERFACE_DEFINED__
#define __INamedEntityCollector_INTERFACE_DEFINED__

/* interface INamedEntityCollector */
/* [unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_INamedEntityCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AF2440F6-8AFC-47d0-9A7F-396A0ACFB43D")
    INamedEntityCollector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ ULONG beginSpan,
            /* [in] */ ULONG endSpan,
            /* [in] */ ULONG beginActual,
            /* [in] */ ULONG endActual,
            /* [in] */ __RPC__in_opt IEntity *pType,
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [in] */ NAMED_ENTITY_CERTAINTY certainty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INamedEntityCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INamedEntityCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INamedEntityCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INamedEntityCollector * This);
        
        DECLSPEC_XFGVIRT(INamedEntityCollector, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in INamedEntityCollector * This,
            /* [in] */ ULONG beginSpan,
            /* [in] */ ULONG endSpan,
            /* [in] */ ULONG beginActual,
            /* [in] */ ULONG endActual,
            /* [in] */ __RPC__in_opt IEntity *pType,
            /* [in] */ __RPC__in LPCWSTR pszValue,
            /* [in] */ NAMED_ENTITY_CERTAINTY certainty);
        
        END_INTERFACE
    } INamedEntityCollectorVtbl;

    interface INamedEntityCollector
    {
        CONST_VTBL struct INamedEntityCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INamedEntityCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INamedEntityCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INamedEntityCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INamedEntityCollector_Add(This,beginSpan,endSpan,beginActual,endActual,pType,pszValue,certainty)	\
    ( (This)->lpVtbl -> Add(This,beginSpan,endSpan,beginActual,endActual,pType,pszValue,certainty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INamedEntityCollector_INTERFACE_DEFINED__ */


#ifndef __ISchemaLocalizerSupport_INTERFACE_DEFINED__
#define __ISchemaLocalizerSupport_INTERFACE_DEFINED__

/* interface ISchemaLocalizerSupport */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_ISchemaLocalizerSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CA3FDCA2-BFBE-4eed-90D7-0CAEF0A1BDA1")
    ISchemaLocalizerSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Localize( 
            /* [in] */ __RPC__in LPCWSTR pszGlobalString,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppszLocalString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaLocalizerSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISchemaLocalizerSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISchemaLocalizerSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISchemaLocalizerSupport * This);
        
        DECLSPEC_XFGVIRT(ISchemaLocalizerSupport, Localize)
        HRESULT ( STDMETHODCALLTYPE *Localize )( 
            __RPC__in ISchemaLocalizerSupport * This,
            /* [in] */ __RPC__in LPCWSTR pszGlobalString,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppszLocalString);
        
        END_INTERFACE
    } ISchemaLocalizerSupportVtbl;

    interface ISchemaLocalizerSupport
    {
        CONST_VTBL struct ISchemaLocalizerSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaLocalizerSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaLocalizerSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaLocalizerSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaLocalizerSupport_Localize(This,pszGlobalString,ppszLocalString)	\
    ( (This)->lpVtbl -> Localize(This,pszGlobalString,ppszLocalString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaLocalizerSupport_INTERFACE_DEFINED__ */


#ifndef __IQueryParserManager_INTERFACE_DEFINED__
#define __IQueryParserManager_INTERFACE_DEFINED__

/* interface IQueryParserManager */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IQueryParserManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A879E3C4-AF77-44fb-8F37-EBD1487CF920")
    IQueryParserManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateLoadedParser( 
            /* [in] */ __RPC__in LPCWSTR pszCatalog,
            /* [in] */ LANGID langidForKeywords,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **ppQueryParser) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeOptions( 
            /* [in] */ BOOL fUnderstandNQS,
            /* [in] */ BOOL fAutoWildCard,
            /* [in] */ __RPC__in_opt IQueryParser *pQueryParser) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOption( 
            /* [in] */ QUERY_PARSER_MANAGER_OPTION option,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryParserManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IQueryParserManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IQueryParserManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IQueryParserManager * This);
        
        DECLSPEC_XFGVIRT(IQueryParserManager, CreateLoadedParser)
        HRESULT ( STDMETHODCALLTYPE *CreateLoadedParser )( 
            __RPC__in IQueryParserManager * This,
            /* [in] */ __RPC__in LPCWSTR pszCatalog,
            /* [in] */ LANGID langidForKeywords,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **ppQueryParser);
        
        DECLSPEC_XFGVIRT(IQueryParserManager, InitializeOptions)
        HRESULT ( STDMETHODCALLTYPE *InitializeOptions )( 
            __RPC__in IQueryParserManager * This,
            /* [in] */ BOOL fUnderstandNQS,
            /* [in] */ BOOL fAutoWildCard,
            /* [in] */ __RPC__in_opt IQueryParser *pQueryParser);
        
        DECLSPEC_XFGVIRT(IQueryParserManager, SetOption)
        HRESULT ( STDMETHODCALLTYPE *SetOption )( 
            __RPC__in IQueryParserManager * This,
            /* [in] */ QUERY_PARSER_MANAGER_OPTION option,
            /* [in] */ __RPC__in const PROPVARIANT *pOptionValue);
        
        END_INTERFACE
    } IQueryParserManagerVtbl;

    interface IQueryParserManager
    {
        CONST_VTBL struct IQueryParserManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQueryParserManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQueryParserManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQueryParserManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQueryParserManager_CreateLoadedParser(This,pszCatalog,langidForKeywords,riid,ppQueryParser)	\
    ( (This)->lpVtbl -> CreateLoadedParser(This,pszCatalog,langidForKeywords,riid,ppQueryParser) ) 

#define IQueryParserManager_InitializeOptions(This,fUnderstandNQS,fAutoWildCard,pQueryParser)	\
    ( (This)->lpVtbl -> InitializeOptions(This,fUnderstandNQS,fAutoWildCard,pQueryParser) ) 

#define IQueryParserManager_SetOption(This,option,pOptionValue)	\
    ( (This)->lpVtbl -> SetOption(This,option,pOptionValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQueryParserManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_structuredquery_0000_0015 */
/* [local] */ 

typedef struct tagHITRANGE
    {
    ULONG iPosition;
    ULONG cLength;
    } 	HITRANGE;



extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0015_v0_0_s_ifspec;


#ifndef __StructuredQuery1_LIBRARY_DEFINED__
#define __StructuredQuery1_LIBRARY_DEFINED__

/* library StructuredQuery1 */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_StructuredQuery1;

EXTERN_C const CLSID CLSID_QueryParser;

#ifdef __cplusplus

class DECLSPEC_UUID("B72F8FD8-0FAB-4dd9-BDBF-245A6CE1485B")
QueryParser;
#endif

EXTERN_C const CLSID CLSID_NegationCondition;

#ifdef __cplusplus

class DECLSPEC_UUID("8DE9C74C-605A-4acd-BEE3-2B222AA2D23D")
NegationCondition;
#endif

EXTERN_C const CLSID CLSID_CompoundCondition;

#ifdef __cplusplus

class DECLSPEC_UUID("116F8D13-101E-4fa5-84D4-FF8279381935")
CompoundCondition;
#endif

EXTERN_C const CLSID CLSID_LeafCondition;

#ifdef __cplusplus

class DECLSPEC_UUID("52F15C89-5A17-48e1-BBCD-46A3F89C7CC2")
LeafCondition;
#endif

EXTERN_C const CLSID CLSID_ConditionFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("E03E85B0-7BE3-4000-BA98-6C13DE9FA486")
ConditionFactory;
#endif

EXTERN_C const CLSID CLSID_Interval;

#ifdef __cplusplus

class DECLSPEC_UUID("D957171F-4BF9-4de2-BCD5-C70A7CA55836")
Interval;
#endif

EXTERN_C const CLSID CLSID_QueryParserManager;

#ifdef __cplusplus

class DECLSPEC_UUID("5088B39A-29B4-4d9d-8245-4EE289222F66")
QueryParserManager;
#endif
#endif /* __StructuredQuery1_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_structuredquery_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_structuredquery_0000_0016_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


