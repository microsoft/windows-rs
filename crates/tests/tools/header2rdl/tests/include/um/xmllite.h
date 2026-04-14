

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


#ifndef __xmllite_h__
#define __xmllite_h__

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

#ifndef __IXmlReader_FWD_DEFINED__
#define __IXmlReader_FWD_DEFINED__
typedef interface IXmlReader IXmlReader;

#endif 	/* __IXmlReader_FWD_DEFINED__ */


#ifndef __IXmlResolver_FWD_DEFINED__
#define __IXmlResolver_FWD_DEFINED__
typedef interface IXmlResolver IXmlResolver;

#endif 	/* __IXmlResolver_FWD_DEFINED__ */


#ifndef __IXmlWriter_FWD_DEFINED__
#define __IXmlWriter_FWD_DEFINED__
typedef interface IXmlWriter IXmlWriter;

#endif 	/* __IXmlWriter_FWD_DEFINED__ */


#ifndef __IXmlWriterLite_FWD_DEFINED__
#define __IXmlWriterLite_FWD_DEFINED__
typedef interface IXmlWriterLite IXmlWriterLite;

#endif 	/* __IXmlWriterLite_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xmllite_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft. All rights reserved.
//
//--------------------------------------------------------------------------
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)




typedef IUnknown IXmlReaderInput;
typedef IUnknown IXmlWriterOutput;
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// XmlReader Constructors
STDAPI CreateXmlReader(_In_ REFIID riid,
    _Outptr_ void ** ppvObject,
    _In_opt_ IMalloc * pMalloc);
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// XmlReaderInput Constructors
STDAPI CreateXmlReaderInputWithEncodingCodePage(
    _In_ IUnknown * pInputStream,
    _In_opt_ IMalloc * pMalloc,
    _In_ UINT nEncodingCodePage,
    _In_ BOOL fEncodingHint,
    _In_opt_ LPCWSTR pwszBaseUri,
    _Out_ IXmlReaderInput ** ppInput);
STDAPI CreateXmlReaderInputWithEncodingName(
    _In_ IUnknown * pInputStream,
    _In_opt_ IMalloc * pMalloc,
    _In_ LPCWSTR pwszEncodingName,
    _In_ BOOL fEncodingHint,
    _In_opt_ LPCWSTR pwszBaseUri,
    _Out_ IXmlReaderInput ** ppInput);
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// XmlWriter Constructors
STDAPI CreateXmlWriter(_In_ REFIID riid,
    _Out_ void ** ppvObject,
    _In_opt_ IMalloc * pMalloc);
////////////////////////////////////////////////////////////////////////////////////////////////////////////
// XmlWriterOutput Constructors
STDAPI CreateXmlWriterOutputWithEncodingCodePage(
    _In_ IUnknown *pOutputStream,
    _In_opt_ IMalloc *pMalloc,
    _In_ UINT nEncodingCodePage,
    _Out_ IXmlWriterOutput **ppOutput);
STDAPI CreateXmlWriterOutputWithEncodingName(
    _In_ IUnknown *pOutputStream,
    _In_opt_ IMalloc *pMalloc,
    _In_ LPCWSTR pwszEncodingName,
    _Out_ IXmlWriterOutput **ppOutput);


extern RPC_IF_HANDLE __MIDL_itf_xmllite_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xmllite_0000_0000_v0_0_s_ifspec;


#ifndef __XmlLite_LIBRARY_DEFINED__
#define __XmlLite_LIBRARY_DEFINED__

/* library XmlLite */
/* [version][lcid][helpstring][uuid] */ 

typedef 
enum XmlNodeType
    {
        XmlNodeType_None	= 0,
        XmlNodeType_Element	= 1,
        XmlNodeType_Attribute	= 2,
        XmlNodeType_Text	= 3,
        XmlNodeType_CDATA	= 4,
        XmlNodeType_ProcessingInstruction	= 7,
        XmlNodeType_Comment	= 8,
        XmlNodeType_DocumentType	= 10,
        XmlNodeType_Whitespace	= 13,
        XmlNodeType_EndElement	= 15,
        XmlNodeType_XmlDeclaration	= 17,
        _XmlNodeType_Last	= 17
    } 	XmlNodeType;

typedef 
enum XmlConformanceLevel
    {
        XmlConformanceLevel_Auto	= 0,
        XmlConformanceLevel_Fragment	= 1,
        XmlConformanceLevel_Document	= 2,
        _XmlConformanceLevel_Last	= 2
    } 	XmlConformanceLevel;

typedef 
enum DtdProcessing
    {
        DtdProcessing_Prohibit	= 0,
        DtdProcessing_Parse	= ( DtdProcessing_Prohibit + 1 ) ,
        _DtdProcessing_Last	= DtdProcessing_Parse
    } 	DtdProcessing;

typedef 
enum XmlReadState
    {
        XmlReadState_Initial	= 0,
        XmlReadState_Interactive	= 1,
        XmlReadState_Error	= 2,
        XmlReadState_EndOfFile	= 3,
        XmlReadState_Closed	= 4
    } 	XmlReadState;

typedef 
enum XmlReaderProperty
    {
        XmlReaderProperty_MultiLanguage	= 0,
        XmlReaderProperty_ConformanceLevel	= ( XmlReaderProperty_MultiLanguage + 1 ) ,
        XmlReaderProperty_RandomAccess	= ( XmlReaderProperty_ConformanceLevel + 1 ) ,
        XmlReaderProperty_XmlResolver	= ( XmlReaderProperty_RandomAccess + 1 ) ,
        XmlReaderProperty_DtdProcessing	= ( XmlReaderProperty_XmlResolver + 1 ) ,
        XmlReaderProperty_ReadState	= ( XmlReaderProperty_DtdProcessing + 1 ) ,
        XmlReaderProperty_MaxElementDepth	= ( XmlReaderProperty_ReadState + 1 ) ,
        XmlReaderProperty_MaxEntityExpansion	= ( XmlReaderProperty_MaxElementDepth + 1 ) ,
        _XmlReaderProperty_Last	= XmlReaderProperty_MaxEntityExpansion
    } 	XmlReaderProperty;

typedef 
enum XmlError
    {
        MX_E_MX	= 0xc00cee00,
        MX_E_INPUTEND	= ( MX_E_MX + 1 ) ,
        MX_E_ENCODING	= ( MX_E_INPUTEND + 1 ) ,
        MX_E_ENCODINGSWITCH	= ( MX_E_ENCODING + 1 ) ,
        MX_E_ENCODINGSIGNATURE	= ( MX_E_ENCODINGSWITCH + 1 ) ,
        WC_E_WC	= 0xc00cee20,
        WC_E_WHITESPACE	= ( WC_E_WC + 1 ) ,
        WC_E_SEMICOLON	= ( WC_E_WHITESPACE + 1 ) ,
        WC_E_GREATERTHAN	= ( WC_E_SEMICOLON + 1 ) ,
        WC_E_QUOTE	= ( WC_E_GREATERTHAN + 1 ) ,
        WC_E_EQUAL	= ( WC_E_QUOTE + 1 ) ,
        WC_E_LESSTHAN	= ( WC_E_EQUAL + 1 ) ,
        WC_E_HEXDIGIT	= ( WC_E_LESSTHAN + 1 ) ,
        WC_E_DIGIT	= ( WC_E_HEXDIGIT + 1 ) ,
        WC_E_LEFTBRACKET	= ( WC_E_DIGIT + 1 ) ,
        WC_E_LEFTPAREN	= ( WC_E_LEFTBRACKET + 1 ) ,
        WC_E_XMLCHARACTER	= ( WC_E_LEFTPAREN + 1 ) ,
        WC_E_NAMECHARACTER	= ( WC_E_XMLCHARACTER + 1 ) ,
        WC_E_SYNTAX	= ( WC_E_NAMECHARACTER + 1 ) ,
        WC_E_CDSECT	= ( WC_E_SYNTAX + 1 ) ,
        WC_E_COMMENT	= ( WC_E_CDSECT + 1 ) ,
        WC_E_CONDSECT	= ( WC_E_COMMENT + 1 ) ,
        WC_E_DECLATTLIST	= ( WC_E_CONDSECT + 1 ) ,
        WC_E_DECLDOCTYPE	= ( WC_E_DECLATTLIST + 1 ) ,
        WC_E_DECLELEMENT	= ( WC_E_DECLDOCTYPE + 1 ) ,
        WC_E_DECLENTITY	= ( WC_E_DECLELEMENT + 1 ) ,
        WC_E_DECLNOTATION	= ( WC_E_DECLENTITY + 1 ) ,
        WC_E_NDATA	= ( WC_E_DECLNOTATION + 1 ) ,
        WC_E_PUBLIC	= ( WC_E_NDATA + 1 ) ,
        WC_E_SYSTEM	= ( WC_E_PUBLIC + 1 ) ,
        WC_E_NAME	= ( WC_E_SYSTEM + 1 ) ,
        WC_E_ROOTELEMENT	= ( WC_E_NAME + 1 ) ,
        WC_E_ELEMENTMATCH	= ( WC_E_ROOTELEMENT + 1 ) ,
        WC_E_UNIQUEATTRIBUTE	= ( WC_E_ELEMENTMATCH + 1 ) ,
        WC_E_TEXTXMLDECL	= ( WC_E_UNIQUEATTRIBUTE + 1 ) ,
        WC_E_LEADINGXML	= ( WC_E_TEXTXMLDECL + 1 ) ,
        WC_E_TEXTDECL	= ( WC_E_LEADINGXML + 1 ) ,
        WC_E_XMLDECL	= ( WC_E_TEXTDECL + 1 ) ,
        WC_E_ENCNAME	= ( WC_E_XMLDECL + 1 ) ,
        WC_E_PUBLICID	= ( WC_E_ENCNAME + 1 ) ,
        WC_E_PESINTERNALSUBSET	= ( WC_E_PUBLICID + 1 ) ,
        WC_E_PESBETWEENDECLS	= ( WC_E_PESINTERNALSUBSET + 1 ) ,
        WC_E_NORECURSION	= ( WC_E_PESBETWEENDECLS + 1 ) ,
        WC_E_ENTITYCONTENT	= ( WC_E_NORECURSION + 1 ) ,
        WC_E_UNDECLAREDENTITY	= ( WC_E_ENTITYCONTENT + 1 ) ,
        WC_E_PARSEDENTITY	= ( WC_E_UNDECLAREDENTITY + 1 ) ,
        WC_E_NOEXTERNALENTITYREF	= ( WC_E_PARSEDENTITY + 1 ) ,
        WC_E_PI	= ( WC_E_NOEXTERNALENTITYREF + 1 ) ,
        WC_E_SYSTEMID	= ( WC_E_PI + 1 ) ,
        WC_E_QUESTIONMARK	= ( WC_E_SYSTEMID + 1 ) ,
        WC_E_CDSECTEND	= ( WC_E_QUESTIONMARK + 1 ) ,
        WC_E_MOREDATA	= ( WC_E_CDSECTEND + 1 ) ,
        WC_E_DTDPROHIBITED	= ( WC_E_MOREDATA + 1 ) ,
        WC_E_INVALIDXMLSPACE	= ( WC_E_DTDPROHIBITED + 1 ) ,
        NC_E_NC	= 0xc00cee60,
        NC_E_QNAMECHARACTER	= ( NC_E_NC + 1 ) ,
        NC_E_QNAMECOLON	= ( NC_E_QNAMECHARACTER + 1 ) ,
        NC_E_NAMECOLON	= ( NC_E_QNAMECOLON + 1 ) ,
        NC_E_DECLAREDPREFIX	= ( NC_E_NAMECOLON + 1 ) ,
        NC_E_UNDECLAREDPREFIX	= ( NC_E_DECLAREDPREFIX + 1 ) ,
        NC_E_EMPTYURI	= ( NC_E_UNDECLAREDPREFIX + 1 ) ,
        NC_E_XMLPREFIXRESERVED	= ( NC_E_EMPTYURI + 1 ) ,
        NC_E_XMLNSPREFIXRESERVED	= ( NC_E_XMLPREFIXRESERVED + 1 ) ,
        NC_E_XMLURIRESERVED	= ( NC_E_XMLNSPREFIXRESERVED + 1 ) ,
        NC_E_XMLNSURIRESERVED	= ( NC_E_XMLURIRESERVED + 1 ) ,
        SC_E_SC	= 0xc00cee80,
        SC_E_MAXELEMENTDEPTH	= ( SC_E_SC + 1 ) ,
        SC_E_MAXENTITYEXPANSION	= ( SC_E_MAXELEMENTDEPTH + 1 ) ,
        WR_E_WR	= 0xc00cef00,
        WR_E_NONWHITESPACE	= ( WR_E_WR + 1 ) ,
        WR_E_NSPREFIXDECLARED	= ( WR_E_NONWHITESPACE + 1 ) ,
        WR_E_NSPREFIXWITHEMPTYNSURI	= ( WR_E_NSPREFIXDECLARED + 1 ) ,
        WR_E_DUPLICATEATTRIBUTE	= ( WR_E_NSPREFIXWITHEMPTYNSURI + 1 ) ,
        WR_E_XMLNSPREFIXDECLARATION	= ( WR_E_DUPLICATEATTRIBUTE + 1 ) ,
        WR_E_XMLPREFIXDECLARATION	= ( WR_E_XMLNSPREFIXDECLARATION + 1 ) ,
        WR_E_XMLURIDECLARATION	= ( WR_E_XMLPREFIXDECLARATION + 1 ) ,
        WR_E_XMLNSURIDECLARATION	= ( WR_E_XMLURIDECLARATION + 1 ) ,
        WR_E_NAMESPACEUNDECLARED	= ( WR_E_XMLNSURIDECLARATION + 1 ) ,
        WR_E_INVALIDXMLSPACE	= ( WR_E_NAMESPACEUNDECLARED + 1 ) ,
        WR_E_INVALIDACTION	= ( WR_E_INVALIDXMLSPACE + 1 ) ,
        WR_E_INVALIDSURROGATEPAIR	= ( WR_E_INVALIDACTION + 1 ) ,
        XML_E_INVALID_DECIMAL	= 0xc00ce01d,
        XML_E_INVALID_HEXIDECIMAL	= 0xc00ce01e,
        XML_E_INVALID_UNICODE	= 0xc00ce01f,
        XML_E_INVALIDENCODING	= 0xc00ce06e
    } 	XmlError;

typedef 
enum XmlStandalone
    {
        XmlStandalone_Omit	= 0,
        XmlStandalone_Yes	= 1,
        XmlStandalone_No	= 2,
        _XmlStandalone_Last	= 2
    } 	XmlStandalone;

typedef 
enum XmlWriterProperty
    {
        XmlWriterProperty_MultiLanguage	= 0,
        XmlWriterProperty_Indent	= ( XmlWriterProperty_MultiLanguage + 1 ) ,
        XmlWriterProperty_ByteOrderMark	= ( XmlWriterProperty_Indent + 1 ) ,
        XmlWriterProperty_OmitXmlDeclaration	= ( XmlWriterProperty_ByteOrderMark + 1 ) ,
        XmlWriterProperty_ConformanceLevel	= ( XmlWriterProperty_OmitXmlDeclaration + 1 ) ,
        XmlWriterProperty_CompactEmptyElement	= ( XmlWriterProperty_ConformanceLevel + 1 ) ,
        _XmlWriterProperty_Last	= XmlWriterProperty_CompactEmptyElement
    } 	XmlWriterProperty;


EXTERN_C const IID LIBID_XmlLite;

#ifndef __IXmlReader_INTERFACE_DEFINED__
#define __IXmlReader_INTERFACE_DEFINED__

/* interface IXmlReader */
/* [unique][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_IXmlReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7279FC81-709D-4095-B63D-69FE4B0D9030")
    IXmlReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInput( 
            /* [annotation] */ 
            _In_opt_  IUnknown *pInput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [annotation] */ 
            _Out_opt_  XmlNodeType *pNodeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeType( 
            /* [annotation] */ 
            _Out_  XmlNodeType *pNodeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveToFirstAttribute( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveToNextAttribute( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveToAttributeByName( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveToElement( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQualifiedName( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchQualifiedName+1)  LPCWSTR *ppwszQualifiedName,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchQualifiedName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamespaceUri( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchNamespaceUri+1)  LPCWSTR *ppwszNamespaceUri,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchNamespaceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocalName( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchLocalName+1)  LPCWSTR *ppwszLocalName,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchLocalName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrefix( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchPrefix+1)  LPCWSTR *ppwszPrefix,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchValue+1)  LPCWSTR *ppwszValue,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadValueChunk( 
            /* [annotation] */ 
            _Out_writes_to_(cwchChunkSize, *pcwchRead)  WCHAR *pwchBuffer,
            /* [annotation] */ 
            _In_  UINT cwchChunkSize,
            /* [annotation] */ 
            _Inout_  UINT *pcwchRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBaseUri( 
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchBaseUri+1)  LPCWSTR *ppwszBaseUri,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchBaseUri) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsDefault( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsEmptyElement( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLineNumber( 
            /* [annotation] */ 
            _Out_  UINT *pnLineNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLinePosition( 
            /* [annotation] */ 
            _Out_  UINT *pnLinePosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributeCount( 
            /* [annotation] */ 
            _Out_  UINT *pnAttributeCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDepth( 
            /* [annotation] */ 
            _Out_  UINT *pnDepth) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsEOF( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXmlReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXmlReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, SetInput)
        HRESULT ( STDMETHODCALLTYPE *SetInput )( 
            IXmlReader * This,
            /* [annotation] */ 
            _In_opt_  IUnknown *pInput);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IXmlReader * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue);
        
        DECLSPEC_XFGVIRT(IXmlReader, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IXmlReader * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue);
        
        DECLSPEC_XFGVIRT(IXmlReader, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_opt_  XmlNodeType *pNodeType);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetNodeType)
        HRESULT ( STDMETHODCALLTYPE *GetNodeType )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_  XmlNodeType *pNodeType);
        
        DECLSPEC_XFGVIRT(IXmlReader, MoveToFirstAttribute)
        HRESULT ( STDMETHODCALLTYPE *MoveToFirstAttribute )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, MoveToNextAttribute)
        HRESULT ( STDMETHODCALLTYPE *MoveToNextAttribute )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, MoveToAttributeByName)
        HRESULT ( STDMETHODCALLTYPE *MoveToAttributeByName )( 
            IXmlReader * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri);
        
        DECLSPEC_XFGVIRT(IXmlReader, MoveToElement)
        HRESULT ( STDMETHODCALLTYPE *MoveToElement )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetQualifiedName)
        HRESULT ( STDMETHODCALLTYPE *GetQualifiedName )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchQualifiedName+1)  LPCWSTR *ppwszQualifiedName,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchQualifiedName);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetNamespaceUri)
        HRESULT ( STDMETHODCALLTYPE *GetNamespaceUri )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchNamespaceUri+1)  LPCWSTR *ppwszNamespaceUri,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchNamespaceUri);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetLocalName)
        HRESULT ( STDMETHODCALLTYPE *GetLocalName )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchLocalName+1)  LPCWSTR *ppwszLocalName,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchLocalName);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetPrefix)
        HRESULT ( STDMETHODCALLTYPE *GetPrefix )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchPrefix+1)  LPCWSTR *ppwszPrefix,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchPrefix);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchValue+1)  LPCWSTR *ppwszValue,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchValue);
        
        DECLSPEC_XFGVIRT(IXmlReader, ReadValueChunk)
        HRESULT ( STDMETHODCALLTYPE *ReadValueChunk )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_writes_to_(cwchChunkSize, *pcwchRead)  WCHAR *pwchBuffer,
            /* [annotation] */ 
            _In_  UINT cwchChunkSize,
            /* [annotation] */ 
            _Inout_  UINT *pcwchRead);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetBaseUri)
        HRESULT ( STDMETHODCALLTYPE *GetBaseUri )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Outptr_result_buffer_maybenull_(*pcwchBaseUri+1)  LPCWSTR *ppwszBaseUri,
            /* [annotation] */ 
            _Out_opt_  UINT *pcwchBaseUri);
        
        DECLSPEC_XFGVIRT(IXmlReader, IsDefault)
        BOOL ( STDMETHODCALLTYPE *IsDefault )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, IsEmptyElement)
        BOOL ( STDMETHODCALLTYPE *IsEmptyElement )( 
            IXmlReader * This);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetLineNumber)
        HRESULT ( STDMETHODCALLTYPE *GetLineNumber )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_  UINT *pnLineNumber);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetLinePosition)
        HRESULT ( STDMETHODCALLTYPE *GetLinePosition )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_  UINT *pnLinePosition);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetAttributeCount)
        HRESULT ( STDMETHODCALLTYPE *GetAttributeCount )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_  UINT *pnAttributeCount);
        
        DECLSPEC_XFGVIRT(IXmlReader, GetDepth)
        HRESULT ( STDMETHODCALLTYPE *GetDepth )( 
            IXmlReader * This,
            /* [annotation] */ 
            _Out_  UINT *pnDepth);
        
        DECLSPEC_XFGVIRT(IXmlReader, IsEOF)
        BOOL ( STDMETHODCALLTYPE *IsEOF )( 
            IXmlReader * This);
        
        END_INTERFACE
    } IXmlReaderVtbl;

    interface IXmlReader
    {
        CONST_VTBL struct IXmlReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXmlReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXmlReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXmlReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXmlReader_SetInput(This,pInput)	\
    ( (This)->lpVtbl -> SetInput(This,pInput) ) 

#define IXmlReader_GetProperty(This,nProperty,ppValue)	\
    ( (This)->lpVtbl -> GetProperty(This,nProperty,ppValue) ) 

#define IXmlReader_SetProperty(This,nProperty,pValue)	\
    ( (This)->lpVtbl -> SetProperty(This,nProperty,pValue) ) 

#define IXmlReader_Read(This,pNodeType)	\
    ( (This)->lpVtbl -> Read(This,pNodeType) ) 

#define IXmlReader_GetNodeType(This,pNodeType)	\
    ( (This)->lpVtbl -> GetNodeType(This,pNodeType) ) 

#define IXmlReader_MoveToFirstAttribute(This)	\
    ( (This)->lpVtbl -> MoveToFirstAttribute(This) ) 

#define IXmlReader_MoveToNextAttribute(This)	\
    ( (This)->lpVtbl -> MoveToNextAttribute(This) ) 

#define IXmlReader_MoveToAttributeByName(This,pwszLocalName,pwszNamespaceUri)	\
    ( (This)->lpVtbl -> MoveToAttributeByName(This,pwszLocalName,pwszNamespaceUri) ) 

#define IXmlReader_MoveToElement(This)	\
    ( (This)->lpVtbl -> MoveToElement(This) ) 

#define IXmlReader_GetQualifiedName(This,ppwszQualifiedName,pcwchQualifiedName)	\
    ( (This)->lpVtbl -> GetQualifiedName(This,ppwszQualifiedName,pcwchQualifiedName) ) 

#define IXmlReader_GetNamespaceUri(This,ppwszNamespaceUri,pcwchNamespaceUri)	\
    ( (This)->lpVtbl -> GetNamespaceUri(This,ppwszNamespaceUri,pcwchNamespaceUri) ) 

#define IXmlReader_GetLocalName(This,ppwszLocalName,pcwchLocalName)	\
    ( (This)->lpVtbl -> GetLocalName(This,ppwszLocalName,pcwchLocalName) ) 

#define IXmlReader_GetPrefix(This,ppwszPrefix,pcwchPrefix)	\
    ( (This)->lpVtbl -> GetPrefix(This,ppwszPrefix,pcwchPrefix) ) 

#define IXmlReader_GetValue(This,ppwszValue,pcwchValue)	\
    ( (This)->lpVtbl -> GetValue(This,ppwszValue,pcwchValue) ) 

#define IXmlReader_ReadValueChunk(This,pwchBuffer,cwchChunkSize,pcwchRead)	\
    ( (This)->lpVtbl -> ReadValueChunk(This,pwchBuffer,cwchChunkSize,pcwchRead) ) 

#define IXmlReader_GetBaseUri(This,ppwszBaseUri,pcwchBaseUri)	\
    ( (This)->lpVtbl -> GetBaseUri(This,ppwszBaseUri,pcwchBaseUri) ) 

#define IXmlReader_IsDefault(This)	\
    ( (This)->lpVtbl -> IsDefault(This) ) 

#define IXmlReader_IsEmptyElement(This)	\
    ( (This)->lpVtbl -> IsEmptyElement(This) ) 

#define IXmlReader_GetLineNumber(This,pnLineNumber)	\
    ( (This)->lpVtbl -> GetLineNumber(This,pnLineNumber) ) 

#define IXmlReader_GetLinePosition(This,pnLinePosition)	\
    ( (This)->lpVtbl -> GetLinePosition(This,pnLinePosition) ) 

#define IXmlReader_GetAttributeCount(This,pnAttributeCount)	\
    ( (This)->lpVtbl -> GetAttributeCount(This,pnAttributeCount) ) 

#define IXmlReader_GetDepth(This,pnDepth)	\
    ( (This)->lpVtbl -> GetDepth(This,pnDepth) ) 

#define IXmlReader_IsEOF(This)	\
    ( (This)->lpVtbl -> IsEOF(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXmlReader_INTERFACE_DEFINED__ */


#ifndef __IXmlResolver_INTERFACE_DEFINED__
#define __IXmlResolver_INTERFACE_DEFINED__

/* interface IXmlResolver */
/* [unique][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_IXmlResolver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7279FC82-709D-4095-B63D-69FE4B0D9030")
    IXmlResolver : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ResolveUri( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszBaseUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicIdentifier,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemIdentifier,
            /* [annotation] */ 
            _Out_  IUnknown **ppResolvedInput) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXmlResolverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXmlResolver * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXmlResolver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXmlResolver * This);
        
        DECLSPEC_XFGVIRT(IXmlResolver, ResolveUri)
        HRESULT ( STDMETHODCALLTYPE *ResolveUri )( 
            IXmlResolver * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszBaseUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicIdentifier,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemIdentifier,
            /* [annotation] */ 
            _Out_  IUnknown **ppResolvedInput);
        
        END_INTERFACE
    } IXmlResolverVtbl;

    interface IXmlResolver
    {
        CONST_VTBL struct IXmlResolverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXmlResolver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXmlResolver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXmlResolver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXmlResolver_ResolveUri(This,pwszBaseUri,pwszPublicIdentifier,pwszSystemIdentifier,ppResolvedInput)	\
    ( (This)->lpVtbl -> ResolveUri(This,pwszBaseUri,pwszPublicIdentifier,pwszSystemIdentifier,ppResolvedInput) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXmlResolver_INTERFACE_DEFINED__ */


#ifndef __IXmlWriter_INTERFACE_DEFINED__
#define __IXmlWriter_INTERFACE_DEFINED__

/* interface IXmlWriter */
/* [unique][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_IXmlWriter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7279FC88-709D-4095-B63D-69FE4B0D9030")
    IXmlWriter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutput( 
            /* [annotation] */ 
            _In_opt_  IUnknown *pOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAttributes( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAttributeString( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteCData( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteCharEntity( 
            /* [annotation] */ 
            _In_  WCHAR wch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteChars( 
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteComment( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszComment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteDocType( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSubset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteElementString( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEndDocument( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEndElement( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEntityRef( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteFullEndElement( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteName( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNmToken( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszNmToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNode( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNodeShallow( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteProcessingInstruction( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteQualifiedName( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteRaw( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteRawChars( 
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteStartDocument( 
            /* [annotation] */ 
            _In_  XmlStandalone standalone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteStartElement( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteString( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteSurrogateCharEntity( 
            /* [annotation] */ 
            _In_  WCHAR wchLow,
            /* [annotation] */ 
            _In_  WCHAR wchHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteWhitespace( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszWhitespace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXmlWriterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXmlWriter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXmlWriter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXmlWriter * This);
        
        DECLSPEC_XFGVIRT(IXmlWriter, SetOutput)
        HRESULT ( STDMETHODCALLTYPE *SetOutput )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  IUnknown *pOutput);
        
        DECLSPEC_XFGVIRT(IXmlWriter, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue);
        
        DECLSPEC_XFGVIRT(IXmlWriter, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteAttributes)
        HRESULT ( STDMETHODCALLTYPE *WriteAttributes )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteAttributeString)
        HRESULT ( STDMETHODCALLTYPE *WriteAttributeString )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteCData)
        HRESULT ( STDMETHODCALLTYPE *WriteCData )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteCharEntity)
        HRESULT ( STDMETHODCALLTYPE *WriteCharEntity )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  WCHAR wch);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteChars)
        HRESULT ( STDMETHODCALLTYPE *WriteChars )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteComment)
        HRESULT ( STDMETHODCALLTYPE *WriteComment )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszComment);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteDocType)
        HRESULT ( STDMETHODCALLTYPE *WriteDocType )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSubset);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteElementString)
        HRESULT ( STDMETHODCALLTYPE *WriteElementString )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteEndDocument)
        HRESULT ( STDMETHODCALLTYPE *WriteEndDocument )( 
            IXmlWriter * This);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteEndElement)
        HRESULT ( STDMETHODCALLTYPE *WriteEndElement )( 
            IXmlWriter * This);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteEntityRef)
        HRESULT ( STDMETHODCALLTYPE *WriteEntityRef )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteFullEndElement)
        HRESULT ( STDMETHODCALLTYPE *WriteFullEndElement )( 
            IXmlWriter * This);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteName)
        HRESULT ( STDMETHODCALLTYPE *WriteName )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteNmToken)
        HRESULT ( STDMETHODCALLTYPE *WriteNmToken )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszNmToken);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteNode)
        HRESULT ( STDMETHODCALLTYPE *WriteNode )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteNodeShallow)
        HRESULT ( STDMETHODCALLTYPE *WriteNodeShallow )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteProcessingInstruction)
        HRESULT ( STDMETHODCALLTYPE *WriteProcessingInstruction )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteQualifiedName)
        HRESULT ( STDMETHODCALLTYPE *WriteQualifiedName )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteRaw)
        HRESULT ( STDMETHODCALLTYPE *WriteRaw )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszData);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteRawChars)
        HRESULT ( STDMETHODCALLTYPE *WriteRawChars )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteStartDocument)
        HRESULT ( STDMETHODCALLTYPE *WriteStartDocument )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  XmlStandalone standalone);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteStartElement)
        HRESULT ( STDMETHODCALLTYPE *WriteStartElement )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPrefix,
            /* [annotation] */ 
            _In_  LPCWSTR pwszLocalName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszNamespaceUri);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteString)
        HRESULT ( STDMETHODCALLTYPE *WriteString )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteSurrogateCharEntity)
        HRESULT ( STDMETHODCALLTYPE *WriteSurrogateCharEntity )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_  WCHAR wchLow,
            /* [annotation] */ 
            _In_  WCHAR wchHigh);
        
        DECLSPEC_XFGVIRT(IXmlWriter, WriteWhitespace)
        HRESULT ( STDMETHODCALLTYPE *WriteWhitespace )( 
            IXmlWriter * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszWhitespace);
        
        DECLSPEC_XFGVIRT(IXmlWriter, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IXmlWriter * This);
        
        END_INTERFACE
    } IXmlWriterVtbl;

    interface IXmlWriter
    {
        CONST_VTBL struct IXmlWriterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXmlWriter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXmlWriter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXmlWriter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXmlWriter_SetOutput(This,pOutput)	\
    ( (This)->lpVtbl -> SetOutput(This,pOutput) ) 

#define IXmlWriter_GetProperty(This,nProperty,ppValue)	\
    ( (This)->lpVtbl -> GetProperty(This,nProperty,ppValue) ) 

#define IXmlWriter_SetProperty(This,nProperty,pValue)	\
    ( (This)->lpVtbl -> SetProperty(This,nProperty,pValue) ) 

#define IXmlWriter_WriteAttributes(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteAttributes(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriter_WriteAttributeString(This,pwszPrefix,pwszLocalName,pwszNamespaceUri,pwszValue)	\
    ( (This)->lpVtbl -> WriteAttributeString(This,pwszPrefix,pwszLocalName,pwszNamespaceUri,pwszValue) ) 

#define IXmlWriter_WriteCData(This,pwszText)	\
    ( (This)->lpVtbl -> WriteCData(This,pwszText) ) 

#define IXmlWriter_WriteCharEntity(This,wch)	\
    ( (This)->lpVtbl -> WriteCharEntity(This,wch) ) 

#define IXmlWriter_WriteChars(This,pwch,cwch)	\
    ( (This)->lpVtbl -> WriteChars(This,pwch,cwch) ) 

#define IXmlWriter_WriteComment(This,pwszComment)	\
    ( (This)->lpVtbl -> WriteComment(This,pwszComment) ) 

#define IXmlWriter_WriteDocType(This,pwszName,pwszPublicId,pwszSystemId,pwszSubset)	\
    ( (This)->lpVtbl -> WriteDocType(This,pwszName,pwszPublicId,pwszSystemId,pwszSubset) ) 

#define IXmlWriter_WriteElementString(This,pwszPrefix,pwszLocalName,pwszNamespaceUri,pwszValue)	\
    ( (This)->lpVtbl -> WriteElementString(This,pwszPrefix,pwszLocalName,pwszNamespaceUri,pwszValue) ) 

#define IXmlWriter_WriteEndDocument(This)	\
    ( (This)->lpVtbl -> WriteEndDocument(This) ) 

#define IXmlWriter_WriteEndElement(This)	\
    ( (This)->lpVtbl -> WriteEndElement(This) ) 

#define IXmlWriter_WriteEntityRef(This,pwszName)	\
    ( (This)->lpVtbl -> WriteEntityRef(This,pwszName) ) 

#define IXmlWriter_WriteFullEndElement(This)	\
    ( (This)->lpVtbl -> WriteFullEndElement(This) ) 

#define IXmlWriter_WriteName(This,pwszName)	\
    ( (This)->lpVtbl -> WriteName(This,pwszName) ) 

#define IXmlWriter_WriteNmToken(This,pwszNmToken)	\
    ( (This)->lpVtbl -> WriteNmToken(This,pwszNmToken) ) 

#define IXmlWriter_WriteNode(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteNode(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriter_WriteNodeShallow(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteNodeShallow(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriter_WriteProcessingInstruction(This,pwszName,pwszText)	\
    ( (This)->lpVtbl -> WriteProcessingInstruction(This,pwszName,pwszText) ) 

#define IXmlWriter_WriteQualifiedName(This,pwszLocalName,pwszNamespaceUri)	\
    ( (This)->lpVtbl -> WriteQualifiedName(This,pwszLocalName,pwszNamespaceUri) ) 

#define IXmlWriter_WriteRaw(This,pwszData)	\
    ( (This)->lpVtbl -> WriteRaw(This,pwszData) ) 

#define IXmlWriter_WriteRawChars(This,pwch,cwch)	\
    ( (This)->lpVtbl -> WriteRawChars(This,pwch,cwch) ) 

#define IXmlWriter_WriteStartDocument(This,standalone)	\
    ( (This)->lpVtbl -> WriteStartDocument(This,standalone) ) 

#define IXmlWriter_WriteStartElement(This,pwszPrefix,pwszLocalName,pwszNamespaceUri)	\
    ( (This)->lpVtbl -> WriteStartElement(This,pwszPrefix,pwszLocalName,pwszNamespaceUri) ) 

#define IXmlWriter_WriteString(This,pwszText)	\
    ( (This)->lpVtbl -> WriteString(This,pwszText) ) 

#define IXmlWriter_WriteSurrogateCharEntity(This,wchLow,wchHigh)	\
    ( (This)->lpVtbl -> WriteSurrogateCharEntity(This,wchLow,wchHigh) ) 

#define IXmlWriter_WriteWhitespace(This,pwszWhitespace)	\
    ( (This)->lpVtbl -> WriteWhitespace(This,pwszWhitespace) ) 

#define IXmlWriter_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXmlWriter_INTERFACE_DEFINED__ */


#ifndef __IXmlWriterLite_INTERFACE_DEFINED__
#define __IXmlWriterLite_INTERFACE_DEFINED__

/* interface IXmlWriterLite */
/* [unique][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_IXmlWriterLite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("862494C6-1310-4AAD-B3CD-2DBEEBF670D3")
    IXmlWriterLite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutput( 
            /* [annotation] */ 
            _In_opt_  IUnknown *pOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAttributes( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAttributeString( 
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName,
            /* [annotation] */ 
            _In_reads_opt_(cwszValue)  const WCHAR *pwszValue,
            /* [annotation] */ 
            _In_  UINT cwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteCData( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteCharEntity( 
            /* [annotation] */ 
            _In_  WCHAR wch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteChars( 
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteComment( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszComment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteDocType( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSubset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteElementString( 
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEndDocument( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEndElement( 
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteEntityRef( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteFullEndElement( 
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteName( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNmToken( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszNmToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNode( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteNodeShallow( 
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteProcessingInstruction( 
            /* [annotation] */ 
            _In_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteRaw( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteRawChars( 
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteStartDocument( 
            /* [annotation] */ 
            _In_  XmlStandalone standalone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteStartElement( 
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteString( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteSurrogateCharEntity( 
            /* [annotation] */ 
            _In_  WCHAR wchLow,
            /* [annotation] */ 
            _In_  WCHAR wchHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteWhitespace( 
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszWhitespace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXmlWriterLiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXmlWriterLite * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXmlWriterLite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXmlWriterLite * This);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, SetOutput)
        HRESULT ( STDMETHODCALLTYPE *SetOutput )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  IUnknown *pOutput);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _Out_  LONG_PTR *ppValue);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  UINT nProperty,
            /* [annotation] */ 
            _In_opt_  LONG_PTR pValue);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteAttributes)
        HRESULT ( STDMETHODCALLTYPE *WriteAttributes )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteAttributeString)
        HRESULT ( STDMETHODCALLTYPE *WriteAttributeString )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName,
            /* [annotation] */ 
            _In_reads_opt_(cwszValue)  const WCHAR *pwszValue,
            /* [annotation] */ 
            _In_  UINT cwszValue);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteCData)
        HRESULT ( STDMETHODCALLTYPE *WriteCData )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteCharEntity)
        HRESULT ( STDMETHODCALLTYPE *WriteCharEntity )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  WCHAR wch);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteChars)
        HRESULT ( STDMETHODCALLTYPE *WriteChars )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteComment)
        HRESULT ( STDMETHODCALLTYPE *WriteComment )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszComment);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteDocType)
        HRESULT ( STDMETHODCALLTYPE *WriteDocType )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszPublicId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSystemId,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszSubset);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteElementString)
        HRESULT ( STDMETHODCALLTYPE *WriteElementString )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszValue);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteEndDocument)
        HRESULT ( STDMETHODCALLTYPE *WriteEndDocument )( 
            IXmlWriterLite * This);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteEndElement)
        HRESULT ( STDMETHODCALLTYPE *WriteEndElement )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteEntityRef)
        HRESULT ( STDMETHODCALLTYPE *WriteEntityRef )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteFullEndElement)
        HRESULT ( STDMETHODCALLTYPE *WriteFullEndElement )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteName)
        HRESULT ( STDMETHODCALLTYPE *WriteName )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteNmToken)
        HRESULT ( STDMETHODCALLTYPE *WriteNmToken )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszNmToken);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteNode)
        HRESULT ( STDMETHODCALLTYPE *WriteNode )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteNodeShallow)
        HRESULT ( STDMETHODCALLTYPE *WriteNodeShallow )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  IXmlReader *pReader,
            /* [annotation] */ 
            _In_  BOOL fWriteDefaultAttributes);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteProcessingInstruction)
        HRESULT ( STDMETHODCALLTYPE *WriteProcessingInstruction )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  LPCWSTR pwszName,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteRaw)
        HRESULT ( STDMETHODCALLTYPE *WriteRaw )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszData);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteRawChars)
        HRESULT ( STDMETHODCALLTYPE *WriteRawChars )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_opt_(cwch)  const WCHAR *pwch,
            /* [annotation] */ 
            _In_  UINT cwch);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteStartDocument)
        HRESULT ( STDMETHODCALLTYPE *WriteStartDocument )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  XmlStandalone standalone);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteStartElement)
        HRESULT ( STDMETHODCALLTYPE *WriteStartElement )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_reads_(cwszQName)  const WCHAR *pwszQName,
            /* [annotation] */ 
            _In_  UINT cwszQName);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteString)
        HRESULT ( STDMETHODCALLTYPE *WriteString )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszText);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteSurrogateCharEntity)
        HRESULT ( STDMETHODCALLTYPE *WriteSurrogateCharEntity )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_  WCHAR wchLow,
            /* [annotation] */ 
            _In_  WCHAR wchHigh);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, WriteWhitespace)
        HRESULT ( STDMETHODCALLTYPE *WriteWhitespace )( 
            IXmlWriterLite * This,
            /* [annotation] */ 
            _In_opt_  LPCWSTR pwszWhitespace);
        
        DECLSPEC_XFGVIRT(IXmlWriterLite, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IXmlWriterLite * This);
        
        END_INTERFACE
    } IXmlWriterLiteVtbl;

    interface IXmlWriterLite
    {
        CONST_VTBL struct IXmlWriterLiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXmlWriterLite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXmlWriterLite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXmlWriterLite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXmlWriterLite_SetOutput(This,pOutput)	\
    ( (This)->lpVtbl -> SetOutput(This,pOutput) ) 

#define IXmlWriterLite_GetProperty(This,nProperty,ppValue)	\
    ( (This)->lpVtbl -> GetProperty(This,nProperty,ppValue) ) 

#define IXmlWriterLite_SetProperty(This,nProperty,pValue)	\
    ( (This)->lpVtbl -> SetProperty(This,nProperty,pValue) ) 

#define IXmlWriterLite_WriteAttributes(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteAttributes(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriterLite_WriteAttributeString(This,pwszQName,cwszQName,pwszValue,cwszValue)	\
    ( (This)->lpVtbl -> WriteAttributeString(This,pwszQName,cwszQName,pwszValue,cwszValue) ) 

#define IXmlWriterLite_WriteCData(This,pwszText)	\
    ( (This)->lpVtbl -> WriteCData(This,pwszText) ) 

#define IXmlWriterLite_WriteCharEntity(This,wch)	\
    ( (This)->lpVtbl -> WriteCharEntity(This,wch) ) 

#define IXmlWriterLite_WriteChars(This,pwch,cwch)	\
    ( (This)->lpVtbl -> WriteChars(This,pwch,cwch) ) 

#define IXmlWriterLite_WriteComment(This,pwszComment)	\
    ( (This)->lpVtbl -> WriteComment(This,pwszComment) ) 

#define IXmlWriterLite_WriteDocType(This,pwszName,pwszPublicId,pwszSystemId,pwszSubset)	\
    ( (This)->lpVtbl -> WriteDocType(This,pwszName,pwszPublicId,pwszSystemId,pwszSubset) ) 

#define IXmlWriterLite_WriteElementString(This,pwszQName,cwszQName,pwszValue)	\
    ( (This)->lpVtbl -> WriteElementString(This,pwszQName,cwszQName,pwszValue) ) 

#define IXmlWriterLite_WriteEndDocument(This)	\
    ( (This)->lpVtbl -> WriteEndDocument(This) ) 

#define IXmlWriterLite_WriteEndElement(This,pwszQName,cwszQName)	\
    ( (This)->lpVtbl -> WriteEndElement(This,pwszQName,cwszQName) ) 

#define IXmlWriterLite_WriteEntityRef(This,pwszName)	\
    ( (This)->lpVtbl -> WriteEntityRef(This,pwszName) ) 

#define IXmlWriterLite_WriteFullEndElement(This,pwszQName,cwszQName)	\
    ( (This)->lpVtbl -> WriteFullEndElement(This,pwszQName,cwszQName) ) 

#define IXmlWriterLite_WriteName(This,pwszName)	\
    ( (This)->lpVtbl -> WriteName(This,pwszName) ) 

#define IXmlWriterLite_WriteNmToken(This,pwszNmToken)	\
    ( (This)->lpVtbl -> WriteNmToken(This,pwszNmToken) ) 

#define IXmlWriterLite_WriteNode(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteNode(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriterLite_WriteNodeShallow(This,pReader,fWriteDefaultAttributes)	\
    ( (This)->lpVtbl -> WriteNodeShallow(This,pReader,fWriteDefaultAttributes) ) 

#define IXmlWriterLite_WriteProcessingInstruction(This,pwszName,pwszText)	\
    ( (This)->lpVtbl -> WriteProcessingInstruction(This,pwszName,pwszText) ) 

#define IXmlWriterLite_WriteRaw(This,pwszData)	\
    ( (This)->lpVtbl -> WriteRaw(This,pwszData) ) 

#define IXmlWriterLite_WriteRawChars(This,pwch,cwch)	\
    ( (This)->lpVtbl -> WriteRawChars(This,pwch,cwch) ) 

#define IXmlWriterLite_WriteStartDocument(This,standalone)	\
    ( (This)->lpVtbl -> WriteStartDocument(This,standalone) ) 

#define IXmlWriterLite_WriteStartElement(This,pwszQName,cwszQName)	\
    ( (This)->lpVtbl -> WriteStartElement(This,pwszQName,cwszQName) ) 

#define IXmlWriterLite_WriteString(This,pwszText)	\
    ( (This)->lpVtbl -> WriteString(This,pwszText) ) 

#define IXmlWriterLite_WriteSurrogateCharEntity(This,wchLow,wchHigh)	\
    ( (This)->lpVtbl -> WriteSurrogateCharEntity(This,wchLow,wchHigh) ) 

#define IXmlWriterLite_WriteWhitespace(This,pwszWhitespace)	\
    ( (This)->lpVtbl -> WriteWhitespace(This,pwszWhitespace) ) 

#define IXmlWriterLite_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXmlWriterLite_INTERFACE_DEFINED__ */

#endif /* __XmlLite_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_xmllite_0000_0001 */
/* [local] */ 

////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IID helpers
static const IID _IID_IXmlReader = { 0x7279FC81, 0x709D, 0x4095, { 0xB6, 0x3D, 0x69, 0xFE, 0x4B, 0x0D, 0x90, 0x30 } };
static const IID _IID_IXmlWriter = { 0x7279FC88, 0x709D, 0x4095, { 0xB6, 0x3D, 0x69, 0xFE, 0x4B, 0x0D, 0x90, 0x30 } };
static const IID _IID_IXmlResolver = { 0x7279FC82, 0x709D, 0x4095, { 0xB6, 0x3D, 0x69, 0xFE, 0x4B, 0x0D, 0x90, 0x30 } };
#define IID_IXmlReader _IID_IXmlReader
#define IID_IXmlWriter _IID_IXmlWriter
#define IID_IXmlResolver _IID_IXmlResolver
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xmllite_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xmllite_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


