

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

#ifndef __wsdxml_h__
#define __wsdxml_h__

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

#ifndef __IWSDXMLContext_FWD_DEFINED__
#define __IWSDXMLContext_FWD_DEFINED__
typedef interface IWSDXMLContext IWSDXMLContext;

#endif 	/* __IWSDXMLContext_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wsdxml_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _WSDXML_NAMESPACE WSDXML_NAMESPACE;

typedef struct _WSDXML_NAME WSDXML_NAME;

typedef struct _WSDXML_TYPE WSDXML_TYPE;

typedef struct _WSDXML_PREFIX_MAPPING WSDXML_PREFIX_MAPPING;

typedef struct _WSDXML_ATTRIBUTE WSDXML_ATTRIBUTE;

typedef struct _WSDXML_NODE WSDXML_NODE;

typedef struct _WSDXML_ELEMENT WSDXML_ELEMENT;

typedef struct _WSDXML_TEXT WSDXML_TEXT;

typedef struct _WSDXML_ELEMENT_LIST WSDXML_ELEMENT_LIST;

typedef const WSDXML_NAMESPACE *PCWSDXML_NAMESPACE;

typedef const WSDXML_TYPE *PCWSDXML_TYPE;

typedef struct _WSD_DATETIME
    {
    BOOL isPositive;
    ULONG year;
    UCHAR month;
    UCHAR day;
    UCHAR hour;
    UCHAR minute;
    UCHAR second;
    UINT millisecond;
    BOOL TZIsLocal;
    BOOL TZIsPositive;
    UCHAR TZHour;
    UCHAR TZMinute;
    } 	WSD_DATETIME;

typedef struct _WSD_DURATION
    {
    BOOL isPositive;
    ULONG year;
    ULONG month;
    ULONG day;
    ULONG hour;
    ULONG minute;
    ULONG second;
    ULONG millisecond;
    } 	WSD_DURATION;


enum __MIDL___MIDL_itf_wsdxml_0000_0000_0001
    {
        OpNone	= 0,
        OpEndOfTable	= ( OpNone + 1 ) ,
        OpBeginElement_	= ( OpEndOfTable + 1 ) ,
        OpBeginAnyElement	= ( OpBeginElement_ + 1 ) ,
        OpEndElement	= ( OpBeginAnyElement + 1 ) ,
        OpElement_	= ( OpEndElement + 1 ) ,
        OpAnyElement	= ( OpElement_ + 1 ) ,
        OpAnyElements	= ( OpAnyElement + 1 ) ,
        OpAnyText	= ( OpAnyElements + 1 ) ,
        OpAttribute_	= ( OpAnyText + 1 ) ,
        OpBeginChoice	= ( OpAttribute_ + 1 ) ,
        OpEndChoice	= ( OpBeginChoice + 1 ) ,
        OpBeginSequence	= ( OpEndChoice + 1 ) ,
        OpEndSequence	= ( OpBeginSequence + 1 ) ,
        OpBeginAll	= ( OpEndSequence + 1 ) ,
        OpEndAll	= ( OpBeginAll + 1 ) ,
        OpAnything	= ( OpEndAll + 1 ) ,
        OpAnyNumber	= ( OpAnything + 1 ) ,
        OpOneOrMore	= ( OpAnyNumber + 1 ) ,
        OpOptional	= ( OpOneOrMore + 1 ) ,
        OpFormatBool_	= ( OpOptional + 1 ) ,
        OpFormatInt8_	= ( OpFormatBool_ + 1 ) ,
        OpFormatInt16_	= ( OpFormatInt8_ + 1 ) ,
        OpFormatInt32_	= ( OpFormatInt16_ + 1 ) ,
        OpFormatInt64_	= ( OpFormatInt32_ + 1 ) ,
        OpFormatUInt8_	= ( OpFormatInt64_ + 1 ) ,
        OpFormatUInt16_	= ( OpFormatUInt8_ + 1 ) ,
        OpFormatUInt32_	= ( OpFormatUInt16_ + 1 ) ,
        OpFormatUInt64_	= ( OpFormatUInt32_ + 1 ) ,
        OpFormatUnicodeString_	= ( OpFormatUInt64_ + 1 ) ,
        OpFormatDom_	= ( OpFormatUnicodeString_ + 1 ) ,
        OpFormatStruct_	= ( OpFormatDom_ + 1 ) ,
        OpFormatUri_	= ( OpFormatStruct_ + 1 ) ,
        OpFormatUuidUri_	= ( OpFormatUri_ + 1 ) ,
        OpFormatName_	= ( OpFormatUuidUri_ + 1 ) ,
        OpFormatListInsertTail_	= ( OpFormatName_ + 1 ) ,
        OpFormatType_	= ( OpFormatListInsertTail_ + 1 ) ,
        OpFormatDynamicType_	= ( OpFormatType_ + 1 ) ,
        OpFormatLookupType_	= ( OpFormatDynamicType_ + 1 ) ,
        OpFormatDuration_	= ( OpFormatLookupType_ + 1 ) ,
        OpFormatDateTime_	= ( OpFormatDuration_ + 1 ) ,
        OpFormatFloat_	= ( OpFormatDateTime_ + 1 ) ,
        OpFormatDouble_	= ( OpFormatFloat_ + 1 ) ,
        OpProcess_	= ( OpFormatDouble_ + 1 ) ,
        OpQualifiedAttribute_	= ( OpProcess_ + 1 ) ,
        OpFormatXMLDeclaration_	= ( OpQualifiedAttribute_ + 1 ) ,
        OpFormatMax	= ( OpFormatXMLDeclaration_ + 1 ) 
    } ;
#define OFFSET(type,field) ((DWORD_PTR)(&((type*)0)->field))
#define BYTE0(n) (BYTE)((((DWORD)n)>>0)&0xFF)
#define BYTE1(n) (BYTE)((((DWORD)n)>>8)&0xFF)
#define BYTE2(n) (BYTE)((((DWORD)n)>>16)&0xFF)
#define BYTE3(n) (BYTE)((((DWORD)n)>>24)&0xFF)
#define BYTES(n) BYTE0(n), BYTE1(n), BYTE2(n), BYTE3(n)
#define OpBeginElement(name)                    OpBeginElement_,        BYTES(name)
#define OpElement(name)                         OpElement_,             BYTES(name)
#define OpAttribute(name)                       OpAttribute_,           BYTES(name)
#define OpFormatBool(type,field,isptr)          OpFormatBool_,          BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatInt8(type,field,isptr)          OpFormatInt8_,          BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatInt16(type,field,isptr)         OpFormatInt16_,         BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatInt32(type,field,isptr)         OpFormatInt32_,         BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatInt64(type,field,isptr)         OpFormatInt64_,         BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatUInt8(type,field,isptr)         OpFormatUInt8_,         BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatUInt16(type,field,isptr)        OpFormatUInt16_,        BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatUInt32(type,field,isptr)        OpFormatUInt32_,        BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatUInt64(type,field,isptr)        OpFormatUInt64_,        BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatUnicodeString(type,field)       OpFormatUnicodeString_, BYTES(OFFSET(type,field))
#define OpFormatDom(type,field)                 OpFormatDom_,           BYTES(OFFSET(type,field))
#define OpFormatStruct(sType,type,field)        OpFormatStruct_,        BYTES(sizeof(sType)),BYTES(OFFSET(type,field))
#define OpFormatPointerToStruct(sType)          OpFormatStruct_,        BYTES(sizeof(sType)),BYTES(0)
#define OpFormatUri(type,field)                 OpFormatUri_,           BYTES(OFFSET(type,field))
#define OpFormatUuidUri(type,field,isptr)       OpFormatUuidUri_,       BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatName(type,field)                OpFormatName_,          BYTES(OFFSET(type,field))
#define OpFormatListInsertTail(s,type,field)    OpFormatListInsertTail_,BYTES(sizeof(s)),BYTES(OFFSET(type,field))
#define OpFormatType(table,type,field)          OpFormatType_,          BYTES(table),BYTES(OFFSET(type,field))
#define OpFormatDynamicType(name,type,field)    OpFormatDynamicType_,   BYTES(name),BYTES(OFFSET(type,field))
#define OpFormatLookupType(uriField,type,field) OpFormatLookupType_, BYTES(OFFSET(type,uriField)),BYTES(OFFSET(type,field))
#define OpFormatDuration(type,field)            OpFormatDuration_,      BYTES(OFFSET(type,field))
#define OpFormatDateTime(type,field)            OpFormatDateTime_,      BYTES(OFFSET(type,field))
#define OpFormatFloat(type,field,isptr)         OpFormatFloat_,         BYTES(isptr),BYTES(OFFSET(type,field))
#define OpFormatDouble(type,field,isptr)        OpFormatDouble_,        BYTES(isptr),BYTES(OFFSET(type,field))
#define OpProcess(type,field)                   OpProcess_,             BYTES(OFFSET(type,field))
#define OpQualifiedAttribute(name)              OpQualifiedAttribute_,  BYTES(name)
#define OpFormatXMLDeclaration(type,field)      OpFormatXMLDeclaration_,BYTES(OFFSET(type,field))
#define WSDXML_TYPE_ENCODING(typeIndex,layerNumber) ((((DWORD)layerNumber) << 28) | typeIndex)
#define WSDXML_NAMESPACE_ENCODING(namespaceIndex,layerNumber) ((((WORD)layerNumber) << 12) | namespaceIndex)
#define WSDXML_NAME_ENCODING(nameIndex,nameSpaceEncoding) ((((DWORD)nameSpaceEncoding) << 16) | nameIndex)
HRESULT WINAPI
WSDXMLGetNameFromBuiltinNamespace(
    _In_ LPCWSTR pszNamespace,
    _In_ LPCWSTR pszName,
    _Outptr_ WSDXML_NAME** ppName);
HRESULT WINAPI
WSDXMLCreateContext(
    _Outptr_ IWSDXMLContext** ppContext);


extern RPC_IF_HANDLE __MIDL_itf_wsdxml_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdxml_0000_0000_v0_0_s_ifspec;

#ifndef __IWSDXMLContext_INTERFACE_DEFINED__
#define __IWSDXMLContext_INTERFACE_DEFINED__

/* interface IWSDXMLContext */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDXMLContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75d8f3ee-3e5a-43b4-a15a-bcf6887460c0")
    IWSDXMLContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddNamespace( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUri,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSuggestedPrefix,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_NAMESPACE **ppNamespace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddNameToNamespace( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUri,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_NAME **ppName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNamespaces( 
            /* [annotation][size_is][in] */ 
            _In_reads_(wNamespacesCount)  const PCWSDXML_NAMESPACE *pNamespaces,
            /* [in] */ WORD wNamespacesCount,
            /* [in] */ BYTE bLayerNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTypes( 
            /* [annotation][size_is][in] */ 
            _In_reads_(dwTypesCount)  const PCWSDXML_TYPE *pTypes,
            /* [in] */ DWORD dwTypesCount,
            /* [in] */ BYTE bLayerNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDXMLContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDXMLContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDXMLContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDXMLContext * This);
        
        DECLSPEC_XFGVIRT(IWSDXMLContext, AddNamespace)
        HRESULT ( STDMETHODCALLTYPE *AddNamespace )( 
            IWSDXMLContext * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUri,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSuggestedPrefix,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_NAMESPACE **ppNamespace);
        
        DECLSPEC_XFGVIRT(IWSDXMLContext, AddNameToNamespace)
        HRESULT ( STDMETHODCALLTYPE *AddNameToNamespace )( 
            IWSDXMLContext * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUri,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszName,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_NAME **ppName);
        
        DECLSPEC_XFGVIRT(IWSDXMLContext, SetNamespaces)
        HRESULT ( STDMETHODCALLTYPE *SetNamespaces )( 
            IWSDXMLContext * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(wNamespacesCount)  const PCWSDXML_NAMESPACE *pNamespaces,
            /* [in] */ WORD wNamespacesCount,
            /* [in] */ BYTE bLayerNumber);
        
        DECLSPEC_XFGVIRT(IWSDXMLContext, SetTypes)
        HRESULT ( STDMETHODCALLTYPE *SetTypes )( 
            IWSDXMLContext * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(dwTypesCount)  const PCWSDXML_TYPE *pTypes,
            /* [in] */ DWORD dwTypesCount,
            /* [in] */ BYTE bLayerNumber);
        
        END_INTERFACE
    } IWSDXMLContextVtbl;

    interface IWSDXMLContext
    {
        CONST_VTBL struct IWSDXMLContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDXMLContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDXMLContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDXMLContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDXMLContext_AddNamespace(This,pszUri,pszSuggestedPrefix,ppNamespace)	\
    ( (This)->lpVtbl -> AddNamespace(This,pszUri,pszSuggestedPrefix,ppNamespace) ) 

#define IWSDXMLContext_AddNameToNamespace(This,pszUri,pszName,ppName)	\
    ( (This)->lpVtbl -> AddNameToNamespace(This,pszUri,pszName,ppName) ) 

#define IWSDXMLContext_SetNamespaces(This,pNamespaces,wNamespacesCount,bLayerNumber)	\
    ( (This)->lpVtbl -> SetNamespaces(This,pNamespaces,wNamespacesCount,bLayerNumber) ) 

#define IWSDXMLContext_SetTypes(This,pTypes,dwTypesCount,bLayerNumber)	\
    ( (This)->lpVtbl -> SetTypes(This,pTypes,dwTypesCount,bLayerNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDXMLContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdxml_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wsdxml_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdxml_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


