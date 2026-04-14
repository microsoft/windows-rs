

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

#ifndef __msxml6_h__
#define __msxml6_h__

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

#ifndef __IXMLDOMNode_FWD_DEFINED__
#define __IXMLDOMNode_FWD_DEFINED__
typedef interface IXMLDOMNode IXMLDOMNode;

#endif 	/* __IXMLDOMNode_FWD_DEFINED__ */


#ifndef __IXMLDOMDocument_FWD_DEFINED__
#define __IXMLDOMDocument_FWD_DEFINED__
typedef interface IXMLDOMDocument IXMLDOMDocument;

#endif 	/* __IXMLDOMDocument_FWD_DEFINED__ */


#ifndef __IXMLDOMDocument2_FWD_DEFINED__
#define __IXMLDOMDocument2_FWD_DEFINED__
typedef interface IXMLDOMDocument2 IXMLDOMDocument2;

#endif 	/* __IXMLDOMDocument2_FWD_DEFINED__ */


#ifndef __IXMLDOMDocument3_FWD_DEFINED__
#define __IXMLDOMDocument3_FWD_DEFINED__
typedef interface IXMLDOMDocument3 IXMLDOMDocument3;

#endif 	/* __IXMLDOMDocument3_FWD_DEFINED__ */


#ifndef __IXMLDOMSchemaCollection_FWD_DEFINED__
#define __IXMLDOMSchemaCollection_FWD_DEFINED__
typedef interface IXMLDOMSchemaCollection IXMLDOMSchemaCollection;

#endif 	/* __IXMLDOMSchemaCollection_FWD_DEFINED__ */


#ifndef __IXMLDOMNodeList_FWD_DEFINED__
#define __IXMLDOMNodeList_FWD_DEFINED__
typedef interface IXMLDOMNodeList IXMLDOMNodeList;

#endif 	/* __IXMLDOMNodeList_FWD_DEFINED__ */


#ifndef __IXMLDOMSelection_FWD_DEFINED__
#define __IXMLDOMSelection_FWD_DEFINED__
typedef interface IXMLDOMSelection IXMLDOMSelection;

#endif 	/* __IXMLDOMSelection_FWD_DEFINED__ */


#ifndef __IXMLDOMNamedNodeMap_FWD_DEFINED__
#define __IXMLDOMNamedNodeMap_FWD_DEFINED__
typedef interface IXMLDOMNamedNodeMap IXMLDOMNamedNodeMap;

#endif 	/* __IXMLDOMNamedNodeMap_FWD_DEFINED__ */


#ifndef __IXMLDOMDocumentFragment_FWD_DEFINED__
#define __IXMLDOMDocumentFragment_FWD_DEFINED__
typedef interface IXMLDOMDocumentFragment IXMLDOMDocumentFragment;

#endif 	/* __IXMLDOMDocumentFragment_FWD_DEFINED__ */


#ifndef __IXMLDOMCharacterData_FWD_DEFINED__
#define __IXMLDOMCharacterData_FWD_DEFINED__
typedef interface IXMLDOMCharacterData IXMLDOMCharacterData;

#endif 	/* __IXMLDOMCharacterData_FWD_DEFINED__ */


#ifndef __IXMLDOMAttribute_FWD_DEFINED__
#define __IXMLDOMAttribute_FWD_DEFINED__
typedef interface IXMLDOMAttribute IXMLDOMAttribute;

#endif 	/* __IXMLDOMAttribute_FWD_DEFINED__ */


#ifndef __IXMLDOMElement_FWD_DEFINED__
#define __IXMLDOMElement_FWD_DEFINED__
typedef interface IXMLDOMElement IXMLDOMElement;

#endif 	/* __IXMLDOMElement_FWD_DEFINED__ */


#ifndef __IXMLDOMText_FWD_DEFINED__
#define __IXMLDOMText_FWD_DEFINED__
typedef interface IXMLDOMText IXMLDOMText;

#endif 	/* __IXMLDOMText_FWD_DEFINED__ */


#ifndef __IXMLDOMComment_FWD_DEFINED__
#define __IXMLDOMComment_FWD_DEFINED__
typedef interface IXMLDOMComment IXMLDOMComment;

#endif 	/* __IXMLDOMComment_FWD_DEFINED__ */


#ifndef __IXMLDOMProcessingInstruction_FWD_DEFINED__
#define __IXMLDOMProcessingInstruction_FWD_DEFINED__
typedef interface IXMLDOMProcessingInstruction IXMLDOMProcessingInstruction;

#endif 	/* __IXMLDOMProcessingInstruction_FWD_DEFINED__ */


#ifndef __IXMLDOMCDATASection_FWD_DEFINED__
#define __IXMLDOMCDATASection_FWD_DEFINED__
typedef interface IXMLDOMCDATASection IXMLDOMCDATASection;

#endif 	/* __IXMLDOMCDATASection_FWD_DEFINED__ */


#ifndef __IXMLDOMDocumentType_FWD_DEFINED__
#define __IXMLDOMDocumentType_FWD_DEFINED__
typedef interface IXMLDOMDocumentType IXMLDOMDocumentType;

#endif 	/* __IXMLDOMDocumentType_FWD_DEFINED__ */


#ifndef __IXMLDOMNotation_FWD_DEFINED__
#define __IXMLDOMNotation_FWD_DEFINED__
typedef interface IXMLDOMNotation IXMLDOMNotation;

#endif 	/* __IXMLDOMNotation_FWD_DEFINED__ */


#ifndef __IXMLDOMEntity_FWD_DEFINED__
#define __IXMLDOMEntity_FWD_DEFINED__
typedef interface IXMLDOMEntity IXMLDOMEntity;

#endif 	/* __IXMLDOMEntity_FWD_DEFINED__ */


#ifndef __IXMLDOMEntityReference_FWD_DEFINED__
#define __IXMLDOMEntityReference_FWD_DEFINED__
typedef interface IXMLDOMEntityReference IXMLDOMEntityReference;

#endif 	/* __IXMLDOMEntityReference_FWD_DEFINED__ */


#ifndef __IXMLDOMImplementation_FWD_DEFINED__
#define __IXMLDOMImplementation_FWD_DEFINED__
typedef interface IXMLDOMImplementation IXMLDOMImplementation;

#endif 	/* __IXMLDOMImplementation_FWD_DEFINED__ */


#ifndef __IXTLRuntime_FWD_DEFINED__
#define __IXTLRuntime_FWD_DEFINED__
typedef interface IXTLRuntime IXTLRuntime;

#endif 	/* __IXTLRuntime_FWD_DEFINED__ */


#ifndef __IXMLDOMParseError_FWD_DEFINED__
#define __IXMLDOMParseError_FWD_DEFINED__
typedef interface IXMLDOMParseError IXMLDOMParseError;

#endif 	/* __IXMLDOMParseError_FWD_DEFINED__ */


#ifndef __IXMLDOMParseError2_FWD_DEFINED__
#define __IXMLDOMParseError2_FWD_DEFINED__
typedef interface IXMLDOMParseError2 IXMLDOMParseError2;

#endif 	/* __IXMLDOMParseError2_FWD_DEFINED__ */


#ifndef __IXMLDOMParseErrorCollection_FWD_DEFINED__
#define __IXMLDOMParseErrorCollection_FWD_DEFINED__
typedef interface IXMLDOMParseErrorCollection IXMLDOMParseErrorCollection;

#endif 	/* __IXMLDOMParseErrorCollection_FWD_DEFINED__ */


#ifndef __XMLDOMDocumentEvents_FWD_DEFINED__
#define __XMLDOMDocumentEvents_FWD_DEFINED__
typedef interface XMLDOMDocumentEvents XMLDOMDocumentEvents;

#endif 	/* __XMLDOMDocumentEvents_FWD_DEFINED__ */


#ifndef __IXSLProcessor_FWD_DEFINED__
#define __IXSLProcessor_FWD_DEFINED__
typedef interface IXSLProcessor IXSLProcessor;

#endif 	/* __IXSLProcessor_FWD_DEFINED__ */


#ifndef __IXSLTemplate_FWD_DEFINED__
#define __IXSLTemplate_FWD_DEFINED__
typedef interface IXSLTemplate IXSLTemplate;

#endif 	/* __IXSLTemplate_FWD_DEFINED__ */


#ifndef __IXMLHTTPRequest_FWD_DEFINED__
#define __IXMLHTTPRequest_FWD_DEFINED__
typedef interface IXMLHTTPRequest IXMLHTTPRequest;

#endif 	/* __IXMLHTTPRequest_FWD_DEFINED__ */


#ifndef __IServerXMLHTTPRequest_FWD_DEFINED__
#define __IServerXMLHTTPRequest_FWD_DEFINED__
typedef interface IServerXMLHTTPRequest IServerXMLHTTPRequest;

#endif 	/* __IServerXMLHTTPRequest_FWD_DEFINED__ */


#ifndef __IServerXMLHTTPRequest2_FWD_DEFINED__
#define __IServerXMLHTTPRequest2_FWD_DEFINED__
typedef interface IServerXMLHTTPRequest2 IServerXMLHTTPRequest2;

#endif 	/* __IServerXMLHTTPRequest2_FWD_DEFINED__ */


#ifndef __ISAXXMLReader_FWD_DEFINED__
#define __ISAXXMLReader_FWD_DEFINED__
typedef interface ISAXXMLReader ISAXXMLReader;

#endif 	/* __ISAXXMLReader_FWD_DEFINED__ */


#ifndef __ISAXXMLFilter_FWD_DEFINED__
#define __ISAXXMLFilter_FWD_DEFINED__
typedef interface ISAXXMLFilter ISAXXMLFilter;

#endif 	/* __ISAXXMLFilter_FWD_DEFINED__ */


#ifndef __ISAXLocator_FWD_DEFINED__
#define __ISAXLocator_FWD_DEFINED__
typedef interface ISAXLocator ISAXLocator;

#endif 	/* __ISAXLocator_FWD_DEFINED__ */


#ifndef __ISAXEntityResolver_FWD_DEFINED__
#define __ISAXEntityResolver_FWD_DEFINED__
typedef interface ISAXEntityResolver ISAXEntityResolver;

#endif 	/* __ISAXEntityResolver_FWD_DEFINED__ */


#ifndef __ISAXContentHandler_FWD_DEFINED__
#define __ISAXContentHandler_FWD_DEFINED__
typedef interface ISAXContentHandler ISAXContentHandler;

#endif 	/* __ISAXContentHandler_FWD_DEFINED__ */


#ifndef __ISAXDTDHandler_FWD_DEFINED__
#define __ISAXDTDHandler_FWD_DEFINED__
typedef interface ISAXDTDHandler ISAXDTDHandler;

#endif 	/* __ISAXDTDHandler_FWD_DEFINED__ */


#ifndef __ISAXErrorHandler_FWD_DEFINED__
#define __ISAXErrorHandler_FWD_DEFINED__
typedef interface ISAXErrorHandler ISAXErrorHandler;

#endif 	/* __ISAXErrorHandler_FWD_DEFINED__ */


#ifndef __ISAXLexicalHandler_FWD_DEFINED__
#define __ISAXLexicalHandler_FWD_DEFINED__
typedef interface ISAXLexicalHandler ISAXLexicalHandler;

#endif 	/* __ISAXLexicalHandler_FWD_DEFINED__ */


#ifndef __ISAXDeclHandler_FWD_DEFINED__
#define __ISAXDeclHandler_FWD_DEFINED__
typedef interface ISAXDeclHandler ISAXDeclHandler;

#endif 	/* __ISAXDeclHandler_FWD_DEFINED__ */


#ifndef __ISAXAttributes_FWD_DEFINED__
#define __ISAXAttributes_FWD_DEFINED__
typedef interface ISAXAttributes ISAXAttributes;

#endif 	/* __ISAXAttributes_FWD_DEFINED__ */


#ifndef __IVBSAXXMLReader_FWD_DEFINED__
#define __IVBSAXXMLReader_FWD_DEFINED__
typedef interface IVBSAXXMLReader IVBSAXXMLReader;

#endif 	/* __IVBSAXXMLReader_FWD_DEFINED__ */


#ifndef __IVBSAXXMLFilter_FWD_DEFINED__
#define __IVBSAXXMLFilter_FWD_DEFINED__
typedef interface IVBSAXXMLFilter IVBSAXXMLFilter;

#endif 	/* __IVBSAXXMLFilter_FWD_DEFINED__ */


#ifndef __IVBSAXLocator_FWD_DEFINED__
#define __IVBSAXLocator_FWD_DEFINED__
typedef interface IVBSAXLocator IVBSAXLocator;

#endif 	/* __IVBSAXLocator_FWD_DEFINED__ */


#ifndef __IVBSAXEntityResolver_FWD_DEFINED__
#define __IVBSAXEntityResolver_FWD_DEFINED__
typedef interface IVBSAXEntityResolver IVBSAXEntityResolver;

#endif 	/* __IVBSAXEntityResolver_FWD_DEFINED__ */


#ifndef __IVBSAXContentHandler_FWD_DEFINED__
#define __IVBSAXContentHandler_FWD_DEFINED__
typedef interface IVBSAXContentHandler IVBSAXContentHandler;

#endif 	/* __IVBSAXContentHandler_FWD_DEFINED__ */


#ifndef __IVBSAXDTDHandler_FWD_DEFINED__
#define __IVBSAXDTDHandler_FWD_DEFINED__
typedef interface IVBSAXDTDHandler IVBSAXDTDHandler;

#endif 	/* __IVBSAXDTDHandler_FWD_DEFINED__ */


#ifndef __IVBSAXErrorHandler_FWD_DEFINED__
#define __IVBSAXErrorHandler_FWD_DEFINED__
typedef interface IVBSAXErrorHandler IVBSAXErrorHandler;

#endif 	/* __IVBSAXErrorHandler_FWD_DEFINED__ */


#ifndef __IVBSAXLexicalHandler_FWD_DEFINED__
#define __IVBSAXLexicalHandler_FWD_DEFINED__
typedef interface IVBSAXLexicalHandler IVBSAXLexicalHandler;

#endif 	/* __IVBSAXLexicalHandler_FWD_DEFINED__ */


#ifndef __IVBSAXDeclHandler_FWD_DEFINED__
#define __IVBSAXDeclHandler_FWD_DEFINED__
typedef interface IVBSAXDeclHandler IVBSAXDeclHandler;

#endif 	/* __IVBSAXDeclHandler_FWD_DEFINED__ */


#ifndef __IVBSAXAttributes_FWD_DEFINED__
#define __IVBSAXAttributes_FWD_DEFINED__
typedef interface IVBSAXAttributes IVBSAXAttributes;

#endif 	/* __IVBSAXAttributes_FWD_DEFINED__ */


#ifndef __IMXWriter_FWD_DEFINED__
#define __IMXWriter_FWD_DEFINED__
typedef interface IMXWriter IMXWriter;

#endif 	/* __IMXWriter_FWD_DEFINED__ */


#ifndef __IMXAttributes_FWD_DEFINED__
#define __IMXAttributes_FWD_DEFINED__
typedef interface IMXAttributes IMXAttributes;

#endif 	/* __IMXAttributes_FWD_DEFINED__ */


#ifndef __IMXReaderControl_FWD_DEFINED__
#define __IMXReaderControl_FWD_DEFINED__
typedef interface IMXReaderControl IMXReaderControl;

#endif 	/* __IMXReaderControl_FWD_DEFINED__ */


#ifndef __IMXSchemaDeclHandler_FWD_DEFINED__
#define __IMXSchemaDeclHandler_FWD_DEFINED__
typedef interface IMXSchemaDeclHandler IMXSchemaDeclHandler;

#endif 	/* __IMXSchemaDeclHandler_FWD_DEFINED__ */


#ifndef __IMXNamespacePrefixes_FWD_DEFINED__
#define __IMXNamespacePrefixes_FWD_DEFINED__
typedef interface IMXNamespacePrefixes IMXNamespacePrefixes;

#endif 	/* __IMXNamespacePrefixes_FWD_DEFINED__ */


#ifndef __IVBMXNamespaceManager_FWD_DEFINED__
#define __IVBMXNamespaceManager_FWD_DEFINED__
typedef interface IVBMXNamespaceManager IVBMXNamespaceManager;

#endif 	/* __IVBMXNamespaceManager_FWD_DEFINED__ */


#ifndef __IMXNamespaceManager_FWD_DEFINED__
#define __IMXNamespaceManager_FWD_DEFINED__
typedef interface IMXNamespaceManager IMXNamespaceManager;

#endif 	/* __IMXNamespaceManager_FWD_DEFINED__ */


#ifndef __IMXXMLFilter_FWD_DEFINED__
#define __IMXXMLFilter_FWD_DEFINED__
typedef interface IMXXMLFilter IMXXMLFilter;

#endif 	/* __IMXXMLFilter_FWD_DEFINED__ */


#ifndef __IXMLDOMSchemaCollection2_FWD_DEFINED__
#define __IXMLDOMSchemaCollection2_FWD_DEFINED__
typedef interface IXMLDOMSchemaCollection2 IXMLDOMSchemaCollection2;

#endif 	/* __IXMLDOMSchemaCollection2_FWD_DEFINED__ */


#ifndef __ISchemaStringCollection_FWD_DEFINED__
#define __ISchemaStringCollection_FWD_DEFINED__
typedef interface ISchemaStringCollection ISchemaStringCollection;

#endif 	/* __ISchemaStringCollection_FWD_DEFINED__ */


#ifndef __ISchemaItemCollection_FWD_DEFINED__
#define __ISchemaItemCollection_FWD_DEFINED__
typedef interface ISchemaItemCollection ISchemaItemCollection;

#endif 	/* __ISchemaItemCollection_FWD_DEFINED__ */


#ifndef __ISchemaItem_FWD_DEFINED__
#define __ISchemaItem_FWD_DEFINED__
typedef interface ISchemaItem ISchemaItem;

#endif 	/* __ISchemaItem_FWD_DEFINED__ */


#ifndef __ISchema_FWD_DEFINED__
#define __ISchema_FWD_DEFINED__
typedef interface ISchema ISchema;

#endif 	/* __ISchema_FWD_DEFINED__ */


#ifndef __ISchemaParticle_FWD_DEFINED__
#define __ISchemaParticle_FWD_DEFINED__
typedef interface ISchemaParticle ISchemaParticle;

#endif 	/* __ISchemaParticle_FWD_DEFINED__ */


#ifndef __ISchemaAttribute_FWD_DEFINED__
#define __ISchemaAttribute_FWD_DEFINED__
typedef interface ISchemaAttribute ISchemaAttribute;

#endif 	/* __ISchemaAttribute_FWD_DEFINED__ */


#ifndef __ISchemaElement_FWD_DEFINED__
#define __ISchemaElement_FWD_DEFINED__
typedef interface ISchemaElement ISchemaElement;

#endif 	/* __ISchemaElement_FWD_DEFINED__ */


#ifndef __ISchemaType_FWD_DEFINED__
#define __ISchemaType_FWD_DEFINED__
typedef interface ISchemaType ISchemaType;

#endif 	/* __ISchemaType_FWD_DEFINED__ */


#ifndef __ISchemaComplexType_FWD_DEFINED__
#define __ISchemaComplexType_FWD_DEFINED__
typedef interface ISchemaComplexType ISchemaComplexType;

#endif 	/* __ISchemaComplexType_FWD_DEFINED__ */


#ifndef __ISchemaAttributeGroup_FWD_DEFINED__
#define __ISchemaAttributeGroup_FWD_DEFINED__
typedef interface ISchemaAttributeGroup ISchemaAttributeGroup;

#endif 	/* __ISchemaAttributeGroup_FWD_DEFINED__ */


#ifndef __ISchemaModelGroup_FWD_DEFINED__
#define __ISchemaModelGroup_FWD_DEFINED__
typedef interface ISchemaModelGroup ISchemaModelGroup;

#endif 	/* __ISchemaModelGroup_FWD_DEFINED__ */


#ifndef __ISchemaAny_FWD_DEFINED__
#define __ISchemaAny_FWD_DEFINED__
typedef interface ISchemaAny ISchemaAny;

#endif 	/* __ISchemaAny_FWD_DEFINED__ */


#ifndef __ISchemaIdentityConstraint_FWD_DEFINED__
#define __ISchemaIdentityConstraint_FWD_DEFINED__
typedef interface ISchemaIdentityConstraint ISchemaIdentityConstraint;

#endif 	/* __ISchemaIdentityConstraint_FWD_DEFINED__ */


#ifndef __ISchemaNotation_FWD_DEFINED__
#define __ISchemaNotation_FWD_DEFINED__
typedef interface ISchemaNotation ISchemaNotation;

#endif 	/* __ISchemaNotation_FWD_DEFINED__ */


#ifndef __IXMLDOMNode_FWD_DEFINED__
#define __IXMLDOMNode_FWD_DEFINED__
typedef interface IXMLDOMNode IXMLDOMNode;

#endif 	/* __IXMLDOMNode_FWD_DEFINED__ */


#ifndef __IXMLDOMNotation_FWD_DEFINED__
#define __IXMLDOMNotation_FWD_DEFINED__
typedef interface IXMLDOMNotation IXMLDOMNotation;

#endif 	/* __IXMLDOMNotation_FWD_DEFINED__ */


#ifndef __IXMLDOMEntity_FWD_DEFINED__
#define __IXMLDOMEntity_FWD_DEFINED__
typedef interface IXMLDOMEntity IXMLDOMEntity;

#endif 	/* __IXMLDOMEntity_FWD_DEFINED__ */


#ifndef __IXMLDOMParseError_FWD_DEFINED__
#define __IXMLDOMParseError_FWD_DEFINED__
typedef interface IXMLDOMParseError IXMLDOMParseError;

#endif 	/* __IXMLDOMParseError_FWD_DEFINED__ */


#ifndef __IXMLDOMParseError2_FWD_DEFINED__
#define __IXMLDOMParseError2_FWD_DEFINED__
typedef interface IXMLDOMParseError2 IXMLDOMParseError2;

#endif 	/* __IXMLDOMParseError2_FWD_DEFINED__ */


#ifndef __IXMLDOMParseErrorCollection_FWD_DEFINED__
#define __IXMLDOMParseErrorCollection_FWD_DEFINED__
typedef interface IXMLDOMParseErrorCollection IXMLDOMParseErrorCollection;

#endif 	/* __IXMLDOMParseErrorCollection_FWD_DEFINED__ */


#ifndef __IXTLRuntime_FWD_DEFINED__
#define __IXTLRuntime_FWD_DEFINED__
typedef interface IXTLRuntime IXTLRuntime;

#endif 	/* __IXTLRuntime_FWD_DEFINED__ */


#ifndef __ISAXXMLReader_FWD_DEFINED__
#define __ISAXXMLReader_FWD_DEFINED__
typedef interface ISAXXMLReader ISAXXMLReader;

#endif 	/* __ISAXXMLReader_FWD_DEFINED__ */


#ifndef __ISAXXMLFilter_FWD_DEFINED__
#define __ISAXXMLFilter_FWD_DEFINED__
typedef interface ISAXXMLFilter ISAXXMLFilter;

#endif 	/* __ISAXXMLFilter_FWD_DEFINED__ */


#ifndef __IVBSAXXMLFilter_FWD_DEFINED__
#define __IVBSAXXMLFilter_FWD_DEFINED__
typedef interface IVBSAXXMLFilter IVBSAXXMLFilter;

#endif 	/* __IVBSAXXMLFilter_FWD_DEFINED__ */


#ifndef __IMXReaderControl_FWD_DEFINED__
#define __IMXReaderControl_FWD_DEFINED__
typedef interface IMXReaderControl IMXReaderControl;

#endif 	/* __IMXReaderControl_FWD_DEFINED__ */


#ifndef __IMXSchemaDeclHandler_FWD_DEFINED__
#define __IMXSchemaDeclHandler_FWD_DEFINED__
typedef interface IMXSchemaDeclHandler IMXSchemaDeclHandler;

#endif 	/* __IMXSchemaDeclHandler_FWD_DEFINED__ */


#ifndef __ISchemaItem_FWD_DEFINED__
#define __ISchemaItem_FWD_DEFINED__
typedef interface ISchemaItem ISchemaItem;

#endif 	/* __ISchemaItem_FWD_DEFINED__ */


#ifndef __ISchemaParticle_FWD_DEFINED__
#define __ISchemaParticle_FWD_DEFINED__
typedef interface ISchemaParticle ISchemaParticle;

#endif 	/* __ISchemaParticle_FWD_DEFINED__ */


#ifndef __ISchemaElement_FWD_DEFINED__
#define __ISchemaElement_FWD_DEFINED__
typedef interface ISchemaElement ISchemaElement;

#endif 	/* __ISchemaElement_FWD_DEFINED__ */


#ifndef __ISchemaType_FWD_DEFINED__
#define __ISchemaType_FWD_DEFINED__
typedef interface ISchemaType ISchemaType;

#endif 	/* __ISchemaType_FWD_DEFINED__ */


#ifndef __ISchemaComplexType_FWD_DEFINED__
#define __ISchemaComplexType_FWD_DEFINED__
typedef interface ISchemaComplexType ISchemaComplexType;

#endif 	/* __ISchemaComplexType_FWD_DEFINED__ */


#ifndef __ISchemaAny_FWD_DEFINED__
#define __ISchemaAny_FWD_DEFINED__
typedef interface ISchemaAny ISchemaAny;

#endif 	/* __ISchemaAny_FWD_DEFINED__ */


#ifndef __ISchemaModelGroup_FWD_DEFINED__
#define __ISchemaModelGroup_FWD_DEFINED__
typedef interface ISchemaModelGroup ISchemaModelGroup;

#endif 	/* __ISchemaModelGroup_FWD_DEFINED__ */


#ifndef __IMXXMLFilter_FWD_DEFINED__
#define __IMXXMLFilter_FWD_DEFINED__
typedef interface IMXXMLFilter IMXXMLFilter;

#endif 	/* __IMXXMLFilter_FWD_DEFINED__ */


#ifndef __ISchemaAttribute_FWD_DEFINED__
#define __ISchemaAttribute_FWD_DEFINED__
typedef interface ISchemaAttribute ISchemaAttribute;

#endif 	/* __ISchemaAttribute_FWD_DEFINED__ */


#ifndef __ISchemaAttributeGroup_FWD_DEFINED__
#define __ISchemaAttributeGroup_FWD_DEFINED__
typedef interface ISchemaAttributeGroup ISchemaAttributeGroup;

#endif 	/* __ISchemaAttributeGroup_FWD_DEFINED__ */


#ifndef __ISchemaIdentityConstraint_FWD_DEFINED__
#define __ISchemaIdentityConstraint_FWD_DEFINED__
typedef interface ISchemaIdentityConstraint ISchemaIdentityConstraint;

#endif 	/* __ISchemaIdentityConstraint_FWD_DEFINED__ */


#ifndef __ISchemaNotation_FWD_DEFINED__
#define __ISchemaNotation_FWD_DEFINED__
typedef interface ISchemaNotation ISchemaNotation;

#endif 	/* __ISchemaNotation_FWD_DEFINED__ */


#ifndef __IXMLDOMNodeList_FWD_DEFINED__
#define __IXMLDOMNodeList_FWD_DEFINED__
typedef interface IXMLDOMNodeList IXMLDOMNodeList;

#endif 	/* __IXMLDOMNodeList_FWD_DEFINED__ */


#ifndef __IXMLDOMSelection_FWD_DEFINED__
#define __IXMLDOMSelection_FWD_DEFINED__
typedef interface IXMLDOMSelection IXMLDOMSelection;

#endif 	/* __IXMLDOMSelection_FWD_DEFINED__ */


#ifndef __XMLDOMDocumentEvents_FWD_DEFINED__
#define __XMLDOMDocumentEvents_FWD_DEFINED__
typedef interface XMLDOMDocumentEvents XMLDOMDocumentEvents;

#endif 	/* __XMLDOMDocumentEvents_FWD_DEFINED__ */


#ifndef __DOMDocument60_FWD_DEFINED__
#define __DOMDocument60_FWD_DEFINED__

#ifdef __cplusplus
typedef class DOMDocument60 DOMDocument60;
#else
typedef struct DOMDocument60 DOMDocument60;
#endif /* __cplusplus */

#endif 	/* __DOMDocument60_FWD_DEFINED__ */


#ifndef __FreeThreadedDOMDocument60_FWD_DEFINED__
#define __FreeThreadedDOMDocument60_FWD_DEFINED__

#ifdef __cplusplus
typedef class FreeThreadedDOMDocument60 FreeThreadedDOMDocument60;
#else
typedef struct FreeThreadedDOMDocument60 FreeThreadedDOMDocument60;
#endif /* __cplusplus */

#endif 	/* __FreeThreadedDOMDocument60_FWD_DEFINED__ */


#ifndef __XMLSchemaCache60_FWD_DEFINED__
#define __XMLSchemaCache60_FWD_DEFINED__

#ifdef __cplusplus
typedef class XMLSchemaCache60 XMLSchemaCache60;
#else
typedef struct XMLSchemaCache60 XMLSchemaCache60;
#endif /* __cplusplus */

#endif 	/* __XMLSchemaCache60_FWD_DEFINED__ */


#ifndef __XSLTemplate60_FWD_DEFINED__
#define __XSLTemplate60_FWD_DEFINED__

#ifdef __cplusplus
typedef class XSLTemplate60 XSLTemplate60;
#else
typedef struct XSLTemplate60 XSLTemplate60;
#endif /* __cplusplus */

#endif 	/* __XSLTemplate60_FWD_DEFINED__ */


#ifndef __XMLHTTP60_FWD_DEFINED__
#define __XMLHTTP60_FWD_DEFINED__

#ifdef __cplusplus
typedef class XMLHTTP60 XMLHTTP60;
#else
typedef struct XMLHTTP60 XMLHTTP60;
#endif /* __cplusplus */

#endif 	/* __XMLHTTP60_FWD_DEFINED__ */


#ifndef __FreeThreadedXMLHTTP60_FWD_DEFINED__
#define __FreeThreadedXMLHTTP60_FWD_DEFINED__

#ifdef __cplusplus
typedef class FreeThreadedXMLHTTP60 FreeThreadedXMLHTTP60;
#else
typedef struct FreeThreadedXMLHTTP60 FreeThreadedXMLHTTP60;
#endif /* __cplusplus */

#endif 	/* __FreeThreadedXMLHTTP60_FWD_DEFINED__ */


#ifndef __ServerXMLHTTP60_FWD_DEFINED__
#define __ServerXMLHTTP60_FWD_DEFINED__

#ifdef __cplusplus
typedef class ServerXMLHTTP60 ServerXMLHTTP60;
#else
typedef struct ServerXMLHTTP60 ServerXMLHTTP60;
#endif /* __cplusplus */

#endif 	/* __ServerXMLHTTP60_FWD_DEFINED__ */


#ifndef __SAXXMLReader60_FWD_DEFINED__
#define __SAXXMLReader60_FWD_DEFINED__

#ifdef __cplusplus
typedef class SAXXMLReader60 SAXXMLReader60;
#else
typedef struct SAXXMLReader60 SAXXMLReader60;
#endif /* __cplusplus */

#endif 	/* __SAXXMLReader60_FWD_DEFINED__ */


#ifndef __MXXMLWriter60_FWD_DEFINED__
#define __MXXMLWriter60_FWD_DEFINED__

#ifdef __cplusplus
typedef class MXXMLWriter60 MXXMLWriter60;
#else
typedef struct MXXMLWriter60 MXXMLWriter60;
#endif /* __cplusplus */

#endif 	/* __MXXMLWriter60_FWD_DEFINED__ */


#ifndef __MXHTMLWriter60_FWD_DEFINED__
#define __MXHTMLWriter60_FWD_DEFINED__

#ifdef __cplusplus
typedef class MXHTMLWriter60 MXHTMLWriter60;
#else
typedef struct MXHTMLWriter60 MXHTMLWriter60;
#endif /* __cplusplus */

#endif 	/* __MXHTMLWriter60_FWD_DEFINED__ */


#ifndef __SAXAttributes60_FWD_DEFINED__
#define __SAXAttributes60_FWD_DEFINED__

#ifdef __cplusplus
typedef class SAXAttributes60 SAXAttributes60;
#else
typedef struct SAXAttributes60 SAXAttributes60;
#endif /* __cplusplus */

#endif 	/* __SAXAttributes60_FWD_DEFINED__ */


#ifndef __MXNamespaceManager60_FWD_DEFINED__
#define __MXNamespaceManager60_FWD_DEFINED__

#ifdef __cplusplus
typedef class MXNamespaceManager60 MXNamespaceManager60;
#else
typedef struct MXNamespaceManager60 MXNamespaceManager60;
#endif /* __cplusplus */

#endif 	/* __MXNamespaceManager60_FWD_DEFINED__ */


#ifndef __IXMLHTTPRequest2Callback_FWD_DEFINED__
#define __IXMLHTTPRequest2Callback_FWD_DEFINED__
typedef interface IXMLHTTPRequest2Callback IXMLHTTPRequest2Callback;

#endif 	/* __IXMLHTTPRequest2Callback_FWD_DEFINED__ */


#ifndef __IXMLHTTPRequest2_FWD_DEFINED__
#define __IXMLHTTPRequest2_FWD_DEFINED__
typedef interface IXMLHTTPRequest2 IXMLHTTPRequest2;

#endif 	/* __IXMLHTTPRequest2_FWD_DEFINED__ */


#ifndef __IXMLHTTPRequest3Callback_FWD_DEFINED__
#define __IXMLHTTPRequest3Callback_FWD_DEFINED__
typedef interface IXMLHTTPRequest3Callback IXMLHTTPRequest3Callback;

#endif 	/* __IXMLHTTPRequest3Callback_FWD_DEFINED__ */


#ifndef __IXMLHTTPRequest3_FWD_DEFINED__
#define __IXMLHTTPRequest3_FWD_DEFINED__
typedef interface IXMLHTTPRequest3 IXMLHTTPRequest3;

#endif 	/* __IXMLHTTPRequest3_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msxml6_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1997-2006.
//
//--------------------------------------------------------------------------
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion
#pragma region Desktop Family
#pragma endregion
#if !defined(_W64)
#if !defined(__midl) && (defined(_X86_) || defined(_M_IX86)) && _MSC_VER >= 1300
#define _W64 __w64
#else
#define _W64
#endif
#endif
#if defined(_WIN64)
    typedef unsigned __int64 ULONG_PTR, *PULONG_PTR;
#else
    typedef _W64 unsigned long ULONG_PTR, *PULONG_PTR;
#endif
#ifdef __ISAXXMLReader_INTERFACE_DEFINED__
#undef __MSXML2_LIBRARY_DEFINED__
#endif
#ifdef __USE_MSXML6_NAMESPACE__
namespace MSXML6 {
#endif


































































#if !defined(__msxml_h__) || defined(__IXMLElementNotificationSink_INTERFACE_DEFINED__) || (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP))
typedef /* [helpstring] */ 
enum tagDOMNodeType
    {
        NODE_INVALID	= 0,
        NODE_ELEMENT	= ( NODE_INVALID + 1 ) ,
        NODE_ATTRIBUTE	= ( NODE_ELEMENT + 1 ) ,
        NODE_TEXT	= ( NODE_ATTRIBUTE + 1 ) ,
        NODE_CDATA_SECTION	= ( NODE_TEXT + 1 ) ,
        NODE_ENTITY_REFERENCE	= ( NODE_CDATA_SECTION + 1 ) ,
        NODE_ENTITY	= ( NODE_ENTITY_REFERENCE + 1 ) ,
        NODE_PROCESSING_INSTRUCTION	= ( NODE_ENTITY + 1 ) ,
        NODE_COMMENT	= ( NODE_PROCESSING_INSTRUCTION + 1 ) ,
        NODE_DOCUMENT	= ( NODE_COMMENT + 1 ) ,
        NODE_DOCUMENT_TYPE	= ( NODE_DOCUMENT + 1 ) ,
        NODE_DOCUMENT_FRAGMENT	= ( NODE_DOCUMENT_TYPE + 1 ) ,
        NODE_NOTATION	= ( NODE_DOCUMENT_FRAGMENT + 1 ) 
    } 	DOMNodeType;

#endif /* !defined(__msxml_h__) || defined(__IXMLElementNotificationSink_INTERFACE_DEFINED__) || (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)) */
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#undef ParseURL


extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0000_v0_0_s_ifspec;

#ifndef __IXMLDOMNode_INTERFACE_DEFINED__
#define __IXMLDOMNode_INTERFACE_DEFINED__

/* interface IXMLDOMNode */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF80-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMNode : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeName( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeValue( 
            /* [retval][out] */ VARIANT *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_nodeValue( 
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeType( 
            /* [retval][out] */ DOMNodeType *type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_parentNode( 
            /* [retval][out] */ IXMLDOMNode **parent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_childNodes( 
            /* [retval][out] */ IXMLDOMNodeList **childList) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_firstChild( 
            /* [retval][out] */ IXMLDOMNode **firstChild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lastChild( 
            /* [retval][out] */ IXMLDOMNode **lastChild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_previousSibling( 
            /* [retval][out] */ IXMLDOMNode **previousSibling) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nextSibling( 
            /* [retval][out] */ IXMLDOMNode **nextSibling) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE insertBefore( 
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE replaceChild( 
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeChild( 
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE appendChild( 
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE hasChildNodes( 
            /* [retval][out] */ VARIANT_BOOL *hasChild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ownerDocument( 
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE cloneNode( 
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeTypeString( 
            /* [out][retval] */ BSTR *nodeType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_text( 
            /* [out][retval] */ BSTR *text) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_text( 
            /* [in] */ BSTR text) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_specified( 
            /* [retval][out] */ VARIANT_BOOL *isSpecified) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_definition( 
            /* [out][retval] */ IXMLDOMNode **definitionNode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_nodeTypedValue( 
            /* [out][retval] */ VARIANT *typedValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_nodeTypedValue( 
            /* [in] */ VARIANT typedValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_dataType( 
            /* [out][retval] */ VARIANT *dataTypeName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_dataType( 
            /* [in] */ BSTR dataTypeName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_xml( 
            /* [out][retval] */ BSTR *xmlString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE transformNode( 
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE selectNodes( 
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE selectSingleNode( 
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_parsed( 
            /* [out][retval] */ VARIANT_BOOL *isParsed) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_namespaceURI( 
            /* [out][retval] */ BSTR *namespaceURI) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_prefix( 
            /* [out][retval] */ BSTR *prefixString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_baseName( 
            /* [out][retval] */ BSTR *nameString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE transformNodeToObject( 
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMNode * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMNode * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMNode * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMNode * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMNode * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMNode * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMNode * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMNode * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMNode * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMNode * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMNode * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMNode * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMNode * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMNode * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMNode * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMNode * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMNode * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMNode * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMNode * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMNode * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMNode * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMNode * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMNode * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMNode * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMNode * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        END_INTERFACE
    } IXMLDOMNodeVtbl;

    interface IXMLDOMNode
    {
        CONST_VTBL struct IXMLDOMNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMNode_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMNode_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMNode_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMNode_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMNode_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMNode_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMNode_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMNode_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMNode_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMNode_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMNode_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMNode_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMNode_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMNode_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMNode_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMNode_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMNode_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMNode_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMNode_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMNode_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMNode_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMNode_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMNode_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMNode_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMNode_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMNode_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMNode_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMNode_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMNode_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMNode_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMNode_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMNode_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMNode_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMNode_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMNode_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMNode_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMNode_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMNode_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMNode_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMNode_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMNode_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMDocument_INTERFACE_DEFINED__
#define __IXMLDOMDocument_INTERFACE_DEFINED__

/* interface IXMLDOMDocument */
/* [hidden][unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF81-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMDocument : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_doctype( 
            /* [retval][out] */ IXMLDOMDocumentType **documentType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_implementation( 
            /* [retval][out] */ IXMLDOMImplementation **impl) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_documentElement( 
            /* [retval][out] */ IXMLDOMElement **DOMElement) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_documentElement( 
            /* [in] */ IXMLDOMElement *DOMElement) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createElement( 
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMElement **element) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createDocumentFragment( 
            /* [retval][out] */ IXMLDOMDocumentFragment **docFrag) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createTextNode( 
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMText **text) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createComment( 
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMComment **comment) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createCDATASection( 
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMCDATASection **cdata) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createProcessingInstruction( 
            /* [in] */ BSTR target,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMProcessingInstruction **pi) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createAttribute( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attribute) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createEntityReference( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMEntityReference **entityRef) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getElementsByTagName( 
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createNode( 
            /* [in] */ VARIANT Type,
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [out][retval] */ IXMLDOMNode **node) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE nodeFromID( 
            /* [in] */ BSTR idString,
            /* [out][retval] */ IXMLDOMNode **node) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE load( 
            /* [in] */ VARIANT xmlSource,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_readyState( 
            /* [out][retval] */ long *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_parseError( 
            /* [out][retval] */ IXMLDOMParseError **errorObj) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_url( 
            /* [out][retval] */ BSTR *urlString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_async( 
            /* [out][retval] */ VARIANT_BOOL *isAsync) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_async( 
            /* [in] */ VARIANT_BOOL isAsync) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE abort( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE loadXML( 
            /* [in] */ BSTR bstrXML,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE save( 
            /* [in] */ VARIANT destination) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_validateOnParse( 
            /* [out][retval] */ VARIANT_BOOL *isValidating) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_validateOnParse( 
            /* [in] */ VARIANT_BOOL isValidating) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_resolveExternals( 
            /* [out][retval] */ VARIANT_BOOL *isResolving) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_resolveExternals( 
            /* [in] */ VARIANT_BOOL isResolving) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_preserveWhiteSpace( 
            /* [out][retval] */ VARIANT_BOOL *isPreserving) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_preserveWhiteSpace( 
            /* [in] */ VARIANT_BOOL isPreserving) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_onreadystatechange( 
            /* [in] */ VARIANT readystatechangeSink) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ondataavailable( 
            /* [in] */ VARIANT ondataavailableSink) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ontransformnode( 
            /* [in] */ VARIANT ontransformnodeSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMDocument * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMDocument * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMDocument * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMDocument * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMDocument * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMDocument * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_doctype)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_doctype )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMDocumentType **documentType);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_implementation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_implementation )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMImplementation **impl);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_documentElement)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_documentElement )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMElement **DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, putref_documentElement)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_documentElement )( 
            IXMLDOMDocument * This,
            /* [in] */ IXMLDOMElement *DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createElement)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createElement )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMElement **element);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createDocumentFragment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createDocumentFragment )( 
            IXMLDOMDocument * This,
            /* [retval][out] */ IXMLDOMDocumentFragment **docFrag);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createTextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createTextNode )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMText **text);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createComment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createComment )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMComment **comment);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createCDATASection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createCDATASection )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMCDATASection **cdata);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createProcessingInstruction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createProcessingInstruction )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR target,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMProcessingInstruction **pi);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createAttribute )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attribute);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createEntityReference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createEntityReference )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMEntityReference **entityRef);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, getElementsByTagName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getElementsByTagName )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createNode )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT Type,
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, nodeFromID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nodeFromID )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR idString,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, load)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *load )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT xmlSource,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ long *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_parseError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parseError )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_url)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_url )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ BSTR *urlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_async)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_async )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT_BOOL *isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_async)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_async )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT_BOOL isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            IXMLDOMDocument * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, loadXML)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *loadXML )( 
            IXMLDOMDocument * This,
            /* [in] */ BSTR bstrXML,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *save )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT destination);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_validateOnParse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_validateOnParse )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT_BOOL *isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_validateOnParse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_validateOnParse )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT_BOOL isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_resolveExternals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_resolveExternals )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT_BOOL *isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_resolveExternals)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_resolveExternals )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT_BOOL isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_preserveWhiteSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_preserveWhiteSpace )( 
            IXMLDOMDocument * This,
            /* [out][retval] */ VARIANT_BOOL *isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_preserveWhiteSpace)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_preserveWhiteSpace )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT_BOOL isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT readystatechangeSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ondataavailable)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ondataavailable )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT ondataavailableSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ontransformnode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ontransformnode )( 
            IXMLDOMDocument * This,
            /* [in] */ VARIANT ontransformnodeSink);
        
        END_INTERFACE
    } IXMLDOMDocumentVtbl;

    interface IXMLDOMDocument
    {
        CONST_VTBL struct IXMLDOMDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMDocument_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMDocument_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMDocument_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMDocument_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMDocument_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMDocument_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMDocument_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMDocument_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMDocument_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMDocument_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMDocument_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMDocument_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMDocument_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMDocument_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMDocument_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMDocument_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMDocument_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMDocument_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMDocument_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMDocument_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMDocument_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMDocument_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMDocument_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMDocument_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMDocument_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMDocument_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMDocument_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMDocument_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMDocument_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMDocument_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMDocument_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMDocument_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMDocument_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMDocument_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMDocument_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMDocument_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMDocument_get_doctype(This,documentType)	\
    ( (This)->lpVtbl -> get_doctype(This,documentType) ) 

#define IXMLDOMDocument_get_implementation(This,impl)	\
    ( (This)->lpVtbl -> get_implementation(This,impl) ) 

#define IXMLDOMDocument_get_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> get_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument_putref_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> putref_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument_createElement(This,tagName,element)	\
    ( (This)->lpVtbl -> createElement(This,tagName,element) ) 

#define IXMLDOMDocument_createDocumentFragment(This,docFrag)	\
    ( (This)->lpVtbl -> createDocumentFragment(This,docFrag) ) 

#define IXMLDOMDocument_createTextNode(This,data,text)	\
    ( (This)->lpVtbl -> createTextNode(This,data,text) ) 

#define IXMLDOMDocument_createComment(This,data,comment)	\
    ( (This)->lpVtbl -> createComment(This,data,comment) ) 

#define IXMLDOMDocument_createCDATASection(This,data,cdata)	\
    ( (This)->lpVtbl -> createCDATASection(This,data,cdata) ) 

#define IXMLDOMDocument_createProcessingInstruction(This,target,data,pi)	\
    ( (This)->lpVtbl -> createProcessingInstruction(This,target,data,pi) ) 

#define IXMLDOMDocument_createAttribute(This,name,attribute)	\
    ( (This)->lpVtbl -> createAttribute(This,name,attribute) ) 

#define IXMLDOMDocument_createEntityReference(This,name,entityRef)	\
    ( (This)->lpVtbl -> createEntityReference(This,name,entityRef) ) 

#define IXMLDOMDocument_getElementsByTagName(This,tagName,resultList)	\
    ( (This)->lpVtbl -> getElementsByTagName(This,tagName,resultList) ) 

#define IXMLDOMDocument_createNode(This,Type,name,namespaceURI,node)	\
    ( (This)->lpVtbl -> createNode(This,Type,name,namespaceURI,node) ) 

#define IXMLDOMDocument_nodeFromID(This,idString,node)	\
    ( (This)->lpVtbl -> nodeFromID(This,idString,node) ) 

#define IXMLDOMDocument_load(This,xmlSource,isSuccessful)	\
    ( (This)->lpVtbl -> load(This,xmlSource,isSuccessful) ) 

#define IXMLDOMDocument_get_readyState(This,value)	\
    ( (This)->lpVtbl -> get_readyState(This,value) ) 

#define IXMLDOMDocument_get_parseError(This,errorObj)	\
    ( (This)->lpVtbl -> get_parseError(This,errorObj) ) 

#define IXMLDOMDocument_get_url(This,urlString)	\
    ( (This)->lpVtbl -> get_url(This,urlString) ) 

#define IXMLDOMDocument_get_async(This,isAsync)	\
    ( (This)->lpVtbl -> get_async(This,isAsync) ) 

#define IXMLDOMDocument_put_async(This,isAsync)	\
    ( (This)->lpVtbl -> put_async(This,isAsync) ) 

#define IXMLDOMDocument_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IXMLDOMDocument_loadXML(This,bstrXML,isSuccessful)	\
    ( (This)->lpVtbl -> loadXML(This,bstrXML,isSuccessful) ) 

#define IXMLDOMDocument_save(This,destination)	\
    ( (This)->lpVtbl -> save(This,destination) ) 

#define IXMLDOMDocument_get_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> get_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument_put_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> put_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument_get_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> get_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument_put_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> put_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument_get_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> get_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument_put_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> put_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument_put_onreadystatechange(This,readystatechangeSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,readystatechangeSink) ) 

#define IXMLDOMDocument_put_ondataavailable(This,ondataavailableSink)	\
    ( (This)->lpVtbl -> put_ondataavailable(This,ondataavailableSink) ) 

#define IXMLDOMDocument_put_ontransformnode(This,ontransformnodeSink)	\
    ( (This)->lpVtbl -> put_ontransformnode(This,ontransformnodeSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMDocument_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMDocument2_INTERFACE_DEFINED__
#define __IXMLDOMDocument2_INTERFACE_DEFINED__

/* interface IXMLDOMDocument2 */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMDocument2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF95-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMDocument2 : public IXMLDOMDocument
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_namespaces( 
            /* [retval][out] */ IXMLDOMSchemaCollection **namespaceCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_schemas( 
            /* [retval][out] */ VARIANT *otherCollection) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_schemas( 
            /* [in] */ VARIANT otherCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE validate( 
            /* [out][retval] */ IXMLDOMParseError **errorObj) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setProperty( 
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getProperty( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMDocument2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMDocument2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMDocument2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMDocument2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMDocument2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMDocument2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMDocument2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMDocument2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_doctype)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_doctype )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMDocumentType **documentType);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_implementation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_implementation )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMImplementation **impl);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_documentElement)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_documentElement )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMElement **DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, putref_documentElement)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_documentElement )( 
            IXMLDOMDocument2 * This,
            /* [in] */ IXMLDOMElement *DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createElement)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createElement )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMElement **element);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createDocumentFragment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createDocumentFragment )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMDocumentFragment **docFrag);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createTextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createTextNode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMText **text);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createComment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createComment )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMComment **comment);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createCDATASection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createCDATASection )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMCDATASection **cdata);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createProcessingInstruction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createProcessingInstruction )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR target,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMProcessingInstruction **pi);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createAttribute )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attribute);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createEntityReference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createEntityReference )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMEntityReference **entityRef);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, getElementsByTagName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getElementsByTagName )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createNode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT Type,
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, nodeFromID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nodeFromID )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR idString,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, load)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *load )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT xmlSource,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ long *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_parseError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parseError )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_url)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_url )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ BSTR *urlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_async)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_async )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT_BOOL *isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_async)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_async )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT_BOOL isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            IXMLDOMDocument2 * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, loadXML)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *loadXML )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR bstrXML,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *save )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT destination);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_validateOnParse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_validateOnParse )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT_BOOL *isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_validateOnParse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_validateOnParse )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT_BOOL isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_resolveExternals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_resolveExternals )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT_BOOL *isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_resolveExternals)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_resolveExternals )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT_BOOL isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_preserveWhiteSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_preserveWhiteSpace )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ VARIANT_BOOL *isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_preserveWhiteSpace)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_preserveWhiteSpace )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT_BOOL isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT readystatechangeSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ondataavailable)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ondataavailable )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT ondataavailableSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ontransformnode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ontransformnode )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT ontransformnodeSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, get_namespaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaces )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ IXMLDOMSchemaCollection **namespaceCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, get_schemas)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_schemas )( 
            IXMLDOMDocument2 * This,
            /* [retval][out] */ VARIANT *otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, putref_schemas)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_schemas )( 
            IXMLDOMDocument2 * This,
            /* [in] */ VARIANT otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, validate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *validate )( 
            IXMLDOMDocument2 * This,
            /* [out][retval] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, setProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setProperty )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, getProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            IXMLDOMDocument2 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value);
        
        END_INTERFACE
    } IXMLDOMDocument2Vtbl;

    interface IXMLDOMDocument2
    {
        CONST_VTBL struct IXMLDOMDocument2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMDocument2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMDocument2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMDocument2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMDocument2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMDocument2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMDocument2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMDocument2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMDocument2_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMDocument2_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMDocument2_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMDocument2_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMDocument2_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMDocument2_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMDocument2_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMDocument2_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMDocument2_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMDocument2_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMDocument2_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMDocument2_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMDocument2_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMDocument2_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMDocument2_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMDocument2_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMDocument2_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMDocument2_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMDocument2_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMDocument2_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMDocument2_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMDocument2_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMDocument2_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMDocument2_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument2_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument2_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument2_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument2_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMDocument2_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMDocument2_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMDocument2_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMDocument2_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMDocument2_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMDocument2_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMDocument2_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMDocument2_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMDocument2_get_doctype(This,documentType)	\
    ( (This)->lpVtbl -> get_doctype(This,documentType) ) 

#define IXMLDOMDocument2_get_implementation(This,impl)	\
    ( (This)->lpVtbl -> get_implementation(This,impl) ) 

#define IXMLDOMDocument2_get_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> get_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument2_putref_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> putref_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument2_createElement(This,tagName,element)	\
    ( (This)->lpVtbl -> createElement(This,tagName,element) ) 

#define IXMLDOMDocument2_createDocumentFragment(This,docFrag)	\
    ( (This)->lpVtbl -> createDocumentFragment(This,docFrag) ) 

#define IXMLDOMDocument2_createTextNode(This,data,text)	\
    ( (This)->lpVtbl -> createTextNode(This,data,text) ) 

#define IXMLDOMDocument2_createComment(This,data,comment)	\
    ( (This)->lpVtbl -> createComment(This,data,comment) ) 

#define IXMLDOMDocument2_createCDATASection(This,data,cdata)	\
    ( (This)->lpVtbl -> createCDATASection(This,data,cdata) ) 

#define IXMLDOMDocument2_createProcessingInstruction(This,target,data,pi)	\
    ( (This)->lpVtbl -> createProcessingInstruction(This,target,data,pi) ) 

#define IXMLDOMDocument2_createAttribute(This,name,attribute)	\
    ( (This)->lpVtbl -> createAttribute(This,name,attribute) ) 

#define IXMLDOMDocument2_createEntityReference(This,name,entityRef)	\
    ( (This)->lpVtbl -> createEntityReference(This,name,entityRef) ) 

#define IXMLDOMDocument2_getElementsByTagName(This,tagName,resultList)	\
    ( (This)->lpVtbl -> getElementsByTagName(This,tagName,resultList) ) 

#define IXMLDOMDocument2_createNode(This,Type,name,namespaceURI,node)	\
    ( (This)->lpVtbl -> createNode(This,Type,name,namespaceURI,node) ) 

#define IXMLDOMDocument2_nodeFromID(This,idString,node)	\
    ( (This)->lpVtbl -> nodeFromID(This,idString,node) ) 

#define IXMLDOMDocument2_load(This,xmlSource,isSuccessful)	\
    ( (This)->lpVtbl -> load(This,xmlSource,isSuccessful) ) 

#define IXMLDOMDocument2_get_readyState(This,value)	\
    ( (This)->lpVtbl -> get_readyState(This,value) ) 

#define IXMLDOMDocument2_get_parseError(This,errorObj)	\
    ( (This)->lpVtbl -> get_parseError(This,errorObj) ) 

#define IXMLDOMDocument2_get_url(This,urlString)	\
    ( (This)->lpVtbl -> get_url(This,urlString) ) 

#define IXMLDOMDocument2_get_async(This,isAsync)	\
    ( (This)->lpVtbl -> get_async(This,isAsync) ) 

#define IXMLDOMDocument2_put_async(This,isAsync)	\
    ( (This)->lpVtbl -> put_async(This,isAsync) ) 

#define IXMLDOMDocument2_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IXMLDOMDocument2_loadXML(This,bstrXML,isSuccessful)	\
    ( (This)->lpVtbl -> loadXML(This,bstrXML,isSuccessful) ) 

#define IXMLDOMDocument2_save(This,destination)	\
    ( (This)->lpVtbl -> save(This,destination) ) 

#define IXMLDOMDocument2_get_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> get_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument2_put_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> put_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument2_get_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> get_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument2_put_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> put_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument2_get_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> get_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument2_put_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> put_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument2_put_onreadystatechange(This,readystatechangeSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,readystatechangeSink) ) 

#define IXMLDOMDocument2_put_ondataavailable(This,ondataavailableSink)	\
    ( (This)->lpVtbl -> put_ondataavailable(This,ondataavailableSink) ) 

#define IXMLDOMDocument2_put_ontransformnode(This,ontransformnodeSink)	\
    ( (This)->lpVtbl -> put_ontransformnode(This,ontransformnodeSink) ) 


#define IXMLDOMDocument2_get_namespaces(This,namespaceCollection)	\
    ( (This)->lpVtbl -> get_namespaces(This,namespaceCollection) ) 

#define IXMLDOMDocument2_get_schemas(This,otherCollection)	\
    ( (This)->lpVtbl -> get_schemas(This,otherCollection) ) 

#define IXMLDOMDocument2_putref_schemas(This,otherCollection)	\
    ( (This)->lpVtbl -> putref_schemas(This,otherCollection) ) 

#define IXMLDOMDocument2_validate(This,errorObj)	\
    ( (This)->lpVtbl -> validate(This,errorObj) ) 

#define IXMLDOMDocument2_setProperty(This,name,value)	\
    ( (This)->lpVtbl -> setProperty(This,name,value) ) 

#define IXMLDOMDocument2_getProperty(This,name,value)	\
    ( (This)->lpVtbl -> getProperty(This,name,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMDocument2_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMDocument3_INTERFACE_DEFINED__
#define __IXMLDOMDocument3_INTERFACE_DEFINED__

/* interface IXMLDOMDocument3 */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMDocument3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF96-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMDocument3 : public IXMLDOMDocument2
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE validateNode( 
            /* [in] */ IXMLDOMNode *node,
            /* [retval][out] */ IXMLDOMParseError **errorObj) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE importNode( 
            /* [in] */ IXMLDOMNode *node,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **clone) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMDocument3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMDocument3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMDocument3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMDocument3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMDocument3 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMDocument3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMDocument3 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMDocument3 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_doctype)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_doctype )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMDocumentType **documentType);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_implementation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_implementation )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMImplementation **impl);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_documentElement)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_documentElement )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMElement **DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, putref_documentElement)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_documentElement )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMElement *DOMElement);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createElement)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createElement )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMElement **element);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createDocumentFragment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createDocumentFragment )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMDocumentFragment **docFrag);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createTextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createTextNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMText **text);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createComment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createComment )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMComment **comment);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createCDATASection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createCDATASection )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMCDATASection **cdata);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createProcessingInstruction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createProcessingInstruction )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR target,
            /* [in] */ BSTR data,
            /* [retval][out] */ IXMLDOMProcessingInstruction **pi);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createAttribute )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attribute);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createEntityReference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createEntityReference )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMEntityReference **entityRef);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, getElementsByTagName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getElementsByTagName )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, createNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT Type,
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, nodeFromID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nodeFromID )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR idString,
            /* [out][retval] */ IXMLDOMNode **node);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, load)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *load )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT xmlSource,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ long *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_parseError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parseError )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_url)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_url )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ BSTR *urlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_async)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_async )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT_BOOL *isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_async)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_async )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT_BOOL isAsync);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            IXMLDOMDocument3 * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, loadXML)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *loadXML )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR bstrXML,
            /* [retval][out] */ VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *save )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT destination);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_validateOnParse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_validateOnParse )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT_BOOL *isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_validateOnParse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_validateOnParse )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT_BOOL isValidating);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_resolveExternals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_resolveExternals )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT_BOOL *isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_resolveExternals)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_resolveExternals )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT_BOOL isResolving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, get_preserveWhiteSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_preserveWhiteSpace )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ VARIANT_BOOL *isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_preserveWhiteSpace)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_preserveWhiteSpace )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT_BOOL isPreserving);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT readystatechangeSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ondataavailable)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ondataavailable )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT ondataavailableSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument, put_ontransformnode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ontransformnode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT ontransformnodeSink);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, get_namespaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaces )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ IXMLDOMSchemaCollection **namespaceCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, get_schemas)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_schemas )( 
            IXMLDOMDocument3 * This,
            /* [retval][out] */ VARIANT *otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, putref_schemas)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_schemas )( 
            IXMLDOMDocument3 * This,
            /* [in] */ VARIANT otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, validate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *validate )( 
            IXMLDOMDocument3 * This,
            /* [out][retval] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, setProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setProperty )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument2, getProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            IXMLDOMDocument3 * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument3, validateNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *validateNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *node,
            /* [retval][out] */ IXMLDOMParseError **errorObj);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocument3, importNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *importNode )( 
            IXMLDOMDocument3 * This,
            /* [in] */ IXMLDOMNode *node,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **clone);
        
        END_INTERFACE
    } IXMLDOMDocument3Vtbl;

    interface IXMLDOMDocument3
    {
        CONST_VTBL struct IXMLDOMDocument3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMDocument3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMDocument3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMDocument3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMDocument3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMDocument3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMDocument3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMDocument3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMDocument3_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMDocument3_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMDocument3_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMDocument3_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMDocument3_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMDocument3_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMDocument3_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMDocument3_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMDocument3_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMDocument3_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMDocument3_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMDocument3_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMDocument3_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMDocument3_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMDocument3_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMDocument3_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMDocument3_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMDocument3_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMDocument3_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMDocument3_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMDocument3_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMDocument3_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMDocument3_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMDocument3_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument3_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocument3_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument3_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMDocument3_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMDocument3_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMDocument3_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMDocument3_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMDocument3_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMDocument3_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMDocument3_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMDocument3_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMDocument3_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMDocument3_get_doctype(This,documentType)	\
    ( (This)->lpVtbl -> get_doctype(This,documentType) ) 

#define IXMLDOMDocument3_get_implementation(This,impl)	\
    ( (This)->lpVtbl -> get_implementation(This,impl) ) 

#define IXMLDOMDocument3_get_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> get_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument3_putref_documentElement(This,DOMElement)	\
    ( (This)->lpVtbl -> putref_documentElement(This,DOMElement) ) 

#define IXMLDOMDocument3_createElement(This,tagName,element)	\
    ( (This)->lpVtbl -> createElement(This,tagName,element) ) 

#define IXMLDOMDocument3_createDocumentFragment(This,docFrag)	\
    ( (This)->lpVtbl -> createDocumentFragment(This,docFrag) ) 

#define IXMLDOMDocument3_createTextNode(This,data,text)	\
    ( (This)->lpVtbl -> createTextNode(This,data,text) ) 

#define IXMLDOMDocument3_createComment(This,data,comment)	\
    ( (This)->lpVtbl -> createComment(This,data,comment) ) 

#define IXMLDOMDocument3_createCDATASection(This,data,cdata)	\
    ( (This)->lpVtbl -> createCDATASection(This,data,cdata) ) 

#define IXMLDOMDocument3_createProcessingInstruction(This,target,data,pi)	\
    ( (This)->lpVtbl -> createProcessingInstruction(This,target,data,pi) ) 

#define IXMLDOMDocument3_createAttribute(This,name,attribute)	\
    ( (This)->lpVtbl -> createAttribute(This,name,attribute) ) 

#define IXMLDOMDocument3_createEntityReference(This,name,entityRef)	\
    ( (This)->lpVtbl -> createEntityReference(This,name,entityRef) ) 

#define IXMLDOMDocument3_getElementsByTagName(This,tagName,resultList)	\
    ( (This)->lpVtbl -> getElementsByTagName(This,tagName,resultList) ) 

#define IXMLDOMDocument3_createNode(This,Type,name,namespaceURI,node)	\
    ( (This)->lpVtbl -> createNode(This,Type,name,namespaceURI,node) ) 

#define IXMLDOMDocument3_nodeFromID(This,idString,node)	\
    ( (This)->lpVtbl -> nodeFromID(This,idString,node) ) 

#define IXMLDOMDocument3_load(This,xmlSource,isSuccessful)	\
    ( (This)->lpVtbl -> load(This,xmlSource,isSuccessful) ) 

#define IXMLDOMDocument3_get_readyState(This,value)	\
    ( (This)->lpVtbl -> get_readyState(This,value) ) 

#define IXMLDOMDocument3_get_parseError(This,errorObj)	\
    ( (This)->lpVtbl -> get_parseError(This,errorObj) ) 

#define IXMLDOMDocument3_get_url(This,urlString)	\
    ( (This)->lpVtbl -> get_url(This,urlString) ) 

#define IXMLDOMDocument3_get_async(This,isAsync)	\
    ( (This)->lpVtbl -> get_async(This,isAsync) ) 

#define IXMLDOMDocument3_put_async(This,isAsync)	\
    ( (This)->lpVtbl -> put_async(This,isAsync) ) 

#define IXMLDOMDocument3_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IXMLDOMDocument3_loadXML(This,bstrXML,isSuccessful)	\
    ( (This)->lpVtbl -> loadXML(This,bstrXML,isSuccessful) ) 

#define IXMLDOMDocument3_save(This,destination)	\
    ( (This)->lpVtbl -> save(This,destination) ) 

#define IXMLDOMDocument3_get_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> get_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument3_put_validateOnParse(This,isValidating)	\
    ( (This)->lpVtbl -> put_validateOnParse(This,isValidating) ) 

#define IXMLDOMDocument3_get_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> get_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument3_put_resolveExternals(This,isResolving)	\
    ( (This)->lpVtbl -> put_resolveExternals(This,isResolving) ) 

#define IXMLDOMDocument3_get_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> get_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument3_put_preserveWhiteSpace(This,isPreserving)	\
    ( (This)->lpVtbl -> put_preserveWhiteSpace(This,isPreserving) ) 

#define IXMLDOMDocument3_put_onreadystatechange(This,readystatechangeSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,readystatechangeSink) ) 

#define IXMLDOMDocument3_put_ondataavailable(This,ondataavailableSink)	\
    ( (This)->lpVtbl -> put_ondataavailable(This,ondataavailableSink) ) 

#define IXMLDOMDocument3_put_ontransformnode(This,ontransformnodeSink)	\
    ( (This)->lpVtbl -> put_ontransformnode(This,ontransformnodeSink) ) 


#define IXMLDOMDocument3_get_namespaces(This,namespaceCollection)	\
    ( (This)->lpVtbl -> get_namespaces(This,namespaceCollection) ) 

#define IXMLDOMDocument3_get_schemas(This,otherCollection)	\
    ( (This)->lpVtbl -> get_schemas(This,otherCollection) ) 

#define IXMLDOMDocument3_putref_schemas(This,otherCollection)	\
    ( (This)->lpVtbl -> putref_schemas(This,otherCollection) ) 

#define IXMLDOMDocument3_validate(This,errorObj)	\
    ( (This)->lpVtbl -> validate(This,errorObj) ) 

#define IXMLDOMDocument3_setProperty(This,name,value)	\
    ( (This)->lpVtbl -> setProperty(This,name,value) ) 

#define IXMLDOMDocument3_getProperty(This,name,value)	\
    ( (This)->lpVtbl -> getProperty(This,name,value) ) 


#define IXMLDOMDocument3_validateNode(This,node,errorObj)	\
    ( (This)->lpVtbl -> validateNode(This,node,errorObj) ) 

#define IXMLDOMDocument3_importNode(This,node,deep,clone)	\
    ( (This)->lpVtbl -> importNode(This,node,deep,clone) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMDocument3_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMSchemaCollection_INTERFACE_DEFINED__
#define __IXMLDOMSchemaCollection_INTERFACE_DEFINED__

/* interface IXMLDOMSchemaCollection */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMSchemaCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("373984c8-b845-449b-91e7-45ac83036ade")
    IXMLDOMSchemaCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE add( 
            /* [in] */ BSTR namespaceURI,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE get( 
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **schemaNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE remove( 
            /* [in] */ BSTR namespaceURI) = 0;
        
        virtual /* [propget][helpstring][id] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *length) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_namespaceURI( 
            /* [in] */ long index,
            /* [retval][out] */ BSTR *length) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE addCollection( 
            /* [in] */ IXMLDOMSchemaCollection *otherCollection) = 0;
        
        virtual /* [id][hidden][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [out][retval] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMSchemaCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMSchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMSchemaCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMSchemaCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMSchemaCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *add )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ BSTR namespaceURI,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *get )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **schemaNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *remove )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get_length)
        /* [propget][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMSchemaCollection * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *length);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, addCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addCollection )( 
            IXMLDOMSchemaCollection * This,
            /* [in] */ IXMLDOMSchemaCollection *otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMSchemaCollection * This,
            /* [out][retval] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } IXMLDOMSchemaCollectionVtbl;

    interface IXMLDOMSchemaCollection
    {
        CONST_VTBL struct IXMLDOMSchemaCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMSchemaCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMSchemaCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMSchemaCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMSchemaCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMSchemaCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMSchemaCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMSchemaCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMSchemaCollection_add(This,namespaceURI,var)	\
    ( (This)->lpVtbl -> add(This,namespaceURI,var) ) 

#define IXMLDOMSchemaCollection_get(This,namespaceURI,schemaNode)	\
    ( (This)->lpVtbl -> get(This,namespaceURI,schemaNode) ) 

#define IXMLDOMSchemaCollection_remove(This,namespaceURI)	\
    ( (This)->lpVtbl -> remove(This,namespaceURI) ) 

#define IXMLDOMSchemaCollection_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define IXMLDOMSchemaCollection_get_namespaceURI(This,index,length)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,index,length) ) 

#define IXMLDOMSchemaCollection_addCollection(This,otherCollection)	\
    ( (This)->lpVtbl -> addCollection(This,otherCollection) ) 

#define IXMLDOMSchemaCollection_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMSchemaCollection_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMNodeList_INTERFACE_DEFINED__
#define __IXMLDOMNodeList_INTERFACE_DEFINED__

/* interface IXMLDOMNodeList */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMNodeList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF82-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMNodeList : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMNode **listItem) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *listLength) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE nextNode( 
            /* [retval][out] */ IXMLDOMNode **nextItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual /* [id][hidden][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [out][retval] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMNodeListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMNodeList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMNodeList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMNodeList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMNodeList * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMNodeList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMNodeList * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMNodeList * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get_item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            IXMLDOMNodeList * This,
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMNode **listItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMNodeList * This,
            /* [retval][out] */ long *listLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, nextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nextNode )( 
            IXMLDOMNodeList * This,
            /* [retval][out] */ IXMLDOMNode **nextItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IXMLDOMNodeList * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMNodeList * This,
            /* [out][retval] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } IXMLDOMNodeListVtbl;

    interface IXMLDOMNodeList
    {
        CONST_VTBL struct IXMLDOMNodeListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMNodeList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMNodeList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMNodeList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMNodeList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMNodeList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMNodeList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMNodeList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMNodeList_get_item(This,index,listItem)	\
    ( (This)->lpVtbl -> get_item(This,index,listItem) ) 

#define IXMLDOMNodeList_get_length(This,listLength)	\
    ( (This)->lpVtbl -> get_length(This,listLength) ) 

#define IXMLDOMNodeList_nextNode(This,nextItem)	\
    ( (This)->lpVtbl -> nextNode(This,nextItem) ) 

#define IXMLDOMNodeList_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IXMLDOMNodeList_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMNodeList_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMSelection_INTERFACE_DEFINED__
#define __IXMLDOMSelection_INTERFACE_DEFINED__

/* interface IXMLDOMSelection */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMSelection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA634FC7-5888-44a7-A257-3A47150D3A0E")
    IXMLDOMSelection : public IXMLDOMNodeList
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_expr( 
            /* [retval][out] */ BSTR *expression) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_expr( 
            /* [in] */ BSTR expression) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_context( 
            /* [retval][out] */ IXMLDOMNode **ppNode) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_context( 
            /* [in] */ IXMLDOMNode *pNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE peekNode( 
            /* [retval][out] */ IXMLDOMNode **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE matches( 
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ IXMLDOMNode **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeNext( 
            /* [retval][out] */ IXMLDOMNode **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeAll( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE clone( 
            /* [retval][out] */ IXMLDOMSelection **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getProperty( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setProperty( 
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMSelectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMSelection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMSelection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMSelection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMSelection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMSelection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMSelection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMSelection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get_item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            IXMLDOMSelection * This,
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMNode **listItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ long *listLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, nextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nextNode )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ IXMLDOMNode **nextItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IXMLDOMSelection * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMNodeList, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMSelection * This,
            /* [out][retval] */ IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, get_expr)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_expr )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ BSTR *expression);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, put_expr)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_expr )( 
            IXMLDOMSelection * This,
            /* [in] */ BSTR expression);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, get_context)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_context )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ IXMLDOMNode **ppNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, putref_context)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_context )( 
            IXMLDOMSelection * This,
            /* [in] */ IXMLDOMNode *pNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, peekNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *peekNode )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ IXMLDOMNode **ppNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, matches)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *matches )( 
            IXMLDOMSelection * This,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ IXMLDOMNode **ppNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, removeNext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeNext )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ IXMLDOMNode **ppNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, removeAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeAll )( 
            IXMLDOMSelection * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *clone )( 
            IXMLDOMSelection * This,
            /* [retval][out] */ IXMLDOMSelection **ppNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, getProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            IXMLDOMSelection * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMSelection, setProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setProperty )( 
            IXMLDOMSelection * This,
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value);
        
        END_INTERFACE
    } IXMLDOMSelectionVtbl;

    interface IXMLDOMSelection
    {
        CONST_VTBL struct IXMLDOMSelectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMSelection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMSelection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMSelection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMSelection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMSelection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMSelection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMSelection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMSelection_get_item(This,index,listItem)	\
    ( (This)->lpVtbl -> get_item(This,index,listItem) ) 

#define IXMLDOMSelection_get_length(This,listLength)	\
    ( (This)->lpVtbl -> get_length(This,listLength) ) 

#define IXMLDOMSelection_nextNode(This,nextItem)	\
    ( (This)->lpVtbl -> nextNode(This,nextItem) ) 

#define IXMLDOMSelection_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IXMLDOMSelection_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 


#define IXMLDOMSelection_get_expr(This,expression)	\
    ( (This)->lpVtbl -> get_expr(This,expression) ) 

#define IXMLDOMSelection_put_expr(This,expression)	\
    ( (This)->lpVtbl -> put_expr(This,expression) ) 

#define IXMLDOMSelection_get_context(This,ppNode)	\
    ( (This)->lpVtbl -> get_context(This,ppNode) ) 

#define IXMLDOMSelection_putref_context(This,pNode)	\
    ( (This)->lpVtbl -> putref_context(This,pNode) ) 

#define IXMLDOMSelection_peekNode(This,ppNode)	\
    ( (This)->lpVtbl -> peekNode(This,ppNode) ) 

#define IXMLDOMSelection_matches(This,pNode,ppNode)	\
    ( (This)->lpVtbl -> matches(This,pNode,ppNode) ) 

#define IXMLDOMSelection_removeNext(This,ppNode)	\
    ( (This)->lpVtbl -> removeNext(This,ppNode) ) 

#define IXMLDOMSelection_removeAll(This)	\
    ( (This)->lpVtbl -> removeAll(This) ) 

#define IXMLDOMSelection_clone(This,ppNode)	\
    ( (This)->lpVtbl -> clone(This,ppNode) ) 

#define IXMLDOMSelection_getProperty(This,name,value)	\
    ( (This)->lpVtbl -> getProperty(This,name,value) ) 

#define IXMLDOMSelection_setProperty(This,name,value)	\
    ( (This)->lpVtbl -> setProperty(This,name,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMSelection_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMNamedNodeMap_INTERFACE_DEFINED__
#define __IXMLDOMNamedNodeMap_INTERFACE_DEFINED__

/* interface IXMLDOMNamedNodeMap */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMNamedNodeMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF83-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMNamedNodeMap : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getNamedItem( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMNode **namedItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setNamedItem( 
            /* [in] */ IXMLDOMNode *newItem,
            /* [retval][out] */ IXMLDOMNode **nameItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeNamedItem( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMNode **namedItem) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMNode **listItem) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *listLength) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getQualifiedItem( 
            /* [in] */ BSTR baseName,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **qualifiedItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeQualifiedItem( 
            /* [in] */ BSTR baseName,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **qualifiedItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE nextNode( 
            /* [retval][out] */ IXMLDOMNode **nextItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual /* [id][hidden][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [out][retval] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMNamedNodeMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMNamedNodeMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMNamedNodeMap * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMNamedNodeMap * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMNamedNodeMap * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, getNamedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getNamedItem )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMNode **namedItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, setNamedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setNamedItem )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ IXMLDOMNode *newItem,
            /* [retval][out] */ IXMLDOMNode **nameItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, removeNamedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeNamedItem )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMNode **namedItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, get_item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMNode **listItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMNamedNodeMap * This,
            /* [retval][out] */ long *listLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, getQualifiedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getQualifiedItem )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ BSTR baseName,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **qualifiedItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, removeQualifiedItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeQualifiedItem )( 
            IXMLDOMNamedNodeMap * This,
            /* [in] */ BSTR baseName,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **qualifiedItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, nextNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *nextNode )( 
            IXMLDOMNamedNodeMap * This,
            /* [retval][out] */ IXMLDOMNode **nextItem);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IXMLDOMNamedNodeMap * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMNamedNodeMap, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMNamedNodeMap * This,
            /* [out][retval] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } IXMLDOMNamedNodeMapVtbl;

    interface IXMLDOMNamedNodeMap
    {
        CONST_VTBL struct IXMLDOMNamedNodeMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMNamedNodeMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMNamedNodeMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMNamedNodeMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMNamedNodeMap_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMNamedNodeMap_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMNamedNodeMap_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMNamedNodeMap_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMNamedNodeMap_getNamedItem(This,name,namedItem)	\
    ( (This)->lpVtbl -> getNamedItem(This,name,namedItem) ) 

#define IXMLDOMNamedNodeMap_setNamedItem(This,newItem,nameItem)	\
    ( (This)->lpVtbl -> setNamedItem(This,newItem,nameItem) ) 

#define IXMLDOMNamedNodeMap_removeNamedItem(This,name,namedItem)	\
    ( (This)->lpVtbl -> removeNamedItem(This,name,namedItem) ) 

#define IXMLDOMNamedNodeMap_get_item(This,index,listItem)	\
    ( (This)->lpVtbl -> get_item(This,index,listItem) ) 

#define IXMLDOMNamedNodeMap_get_length(This,listLength)	\
    ( (This)->lpVtbl -> get_length(This,listLength) ) 

#define IXMLDOMNamedNodeMap_getQualifiedItem(This,baseName,namespaceURI,qualifiedItem)	\
    ( (This)->lpVtbl -> getQualifiedItem(This,baseName,namespaceURI,qualifiedItem) ) 

#define IXMLDOMNamedNodeMap_removeQualifiedItem(This,baseName,namespaceURI,qualifiedItem)	\
    ( (This)->lpVtbl -> removeQualifiedItem(This,baseName,namespaceURI,qualifiedItem) ) 

#define IXMLDOMNamedNodeMap_nextNode(This,nextItem)	\
    ( (This)->lpVtbl -> nextNode(This,nextItem) ) 

#define IXMLDOMNamedNodeMap_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IXMLDOMNamedNodeMap_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMNamedNodeMap_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMDocumentFragment_INTERFACE_DEFINED__
#define __IXMLDOMDocumentFragment_INTERFACE_DEFINED__

/* interface IXMLDOMDocumentFragment */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMDocumentFragment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3efaa413-272f-11d2-836f-0000f87a7782")
    IXMLDOMDocumentFragment : public IXMLDOMNode
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMDocumentFragmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMDocumentFragment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMDocumentFragment * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMDocumentFragment * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMDocumentFragment * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMDocumentFragment * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMDocumentFragment * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMDocumentFragment * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        END_INTERFACE
    } IXMLDOMDocumentFragmentVtbl;

    interface IXMLDOMDocumentFragment
    {
        CONST_VTBL struct IXMLDOMDocumentFragmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMDocumentFragment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMDocumentFragment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMDocumentFragment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMDocumentFragment_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMDocumentFragment_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMDocumentFragment_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMDocumentFragment_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMDocumentFragment_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMDocumentFragment_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMDocumentFragment_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMDocumentFragment_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMDocumentFragment_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMDocumentFragment_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMDocumentFragment_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMDocumentFragment_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMDocumentFragment_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMDocumentFragment_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMDocumentFragment_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMDocumentFragment_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMDocumentFragment_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMDocumentFragment_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMDocumentFragment_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMDocumentFragment_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMDocumentFragment_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMDocumentFragment_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMDocumentFragment_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMDocumentFragment_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMDocumentFragment_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMDocumentFragment_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMDocumentFragment_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMDocumentFragment_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocumentFragment_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocumentFragment_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMDocumentFragment_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMDocumentFragment_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMDocumentFragment_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMDocumentFragment_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMDocumentFragment_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMDocumentFragment_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMDocumentFragment_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMDocumentFragment_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMDocumentFragment_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMDocumentFragment_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMDocumentFragment_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMCharacterData_INTERFACE_DEFINED__
#define __IXMLDOMCharacterData_INTERFACE_DEFINED__

/* interface IXMLDOMCharacterData */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMCharacterData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF84-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMCharacterData : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_data( 
            /* [retval][out] */ BSTR *data) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_data( 
            /* [in] */ BSTR data) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *dataLength) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE substringData( 
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [retval][out] */ BSTR *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE appendData( 
            /* [in] */ BSTR data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE insertData( 
            /* [in] */ long offset,
            /* [in] */ BSTR data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE deleteData( 
            /* [in] */ long offset,
            /* [in] */ long count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE replaceData( 
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [in] */ BSTR data) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMCharacterDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMCharacterData * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMCharacterData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMCharacterData * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMCharacterData * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMCharacterData * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMCharacterData * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMCharacterData * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMCharacterData * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMCharacterData * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMCharacterData * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMCharacterData * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMCharacterData * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_data )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, put_data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_data )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMCharacterData * This,
            /* [retval][out] */ long *dataLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, substringData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *substringData )( 
            IXMLDOMCharacterData * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, appendData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendData )( 
            IXMLDOMCharacterData * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, insertData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertData )( 
            IXMLDOMCharacterData * This,
            /* [in] */ long offset,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, deleteData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *deleteData )( 
            IXMLDOMCharacterData * This,
            /* [in] */ long offset,
            /* [in] */ long count);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, replaceData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceData )( 
            IXMLDOMCharacterData * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [in] */ BSTR data);
        
        END_INTERFACE
    } IXMLDOMCharacterDataVtbl;

    interface IXMLDOMCharacterData
    {
        CONST_VTBL struct IXMLDOMCharacterDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMCharacterData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMCharacterData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMCharacterData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMCharacterData_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMCharacterData_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMCharacterData_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMCharacterData_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMCharacterData_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMCharacterData_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMCharacterData_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMCharacterData_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMCharacterData_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMCharacterData_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMCharacterData_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMCharacterData_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMCharacterData_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMCharacterData_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMCharacterData_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMCharacterData_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMCharacterData_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMCharacterData_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMCharacterData_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMCharacterData_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMCharacterData_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMCharacterData_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMCharacterData_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMCharacterData_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMCharacterData_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMCharacterData_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMCharacterData_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMCharacterData_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMCharacterData_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMCharacterData_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMCharacterData_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMCharacterData_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMCharacterData_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMCharacterData_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMCharacterData_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMCharacterData_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMCharacterData_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMCharacterData_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMCharacterData_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMCharacterData_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMCharacterData_get_data(This,data)	\
    ( (This)->lpVtbl -> get_data(This,data) ) 

#define IXMLDOMCharacterData_put_data(This,data)	\
    ( (This)->lpVtbl -> put_data(This,data) ) 

#define IXMLDOMCharacterData_get_length(This,dataLength)	\
    ( (This)->lpVtbl -> get_length(This,dataLength) ) 

#define IXMLDOMCharacterData_substringData(This,offset,count,data)	\
    ( (This)->lpVtbl -> substringData(This,offset,count,data) ) 

#define IXMLDOMCharacterData_appendData(This,data)	\
    ( (This)->lpVtbl -> appendData(This,data) ) 

#define IXMLDOMCharacterData_insertData(This,offset,data)	\
    ( (This)->lpVtbl -> insertData(This,offset,data) ) 

#define IXMLDOMCharacterData_deleteData(This,offset,count)	\
    ( (This)->lpVtbl -> deleteData(This,offset,count) ) 

#define IXMLDOMCharacterData_replaceData(This,offset,count,data)	\
    ( (This)->lpVtbl -> replaceData(This,offset,count,data) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMCharacterData_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMAttribute_INTERFACE_DEFINED__
#define __IXMLDOMAttribute_INTERFACE_DEFINED__

/* interface IXMLDOMAttribute */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMAttribute;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF85-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMAttribute : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_name( 
            /* [retval][out] */ BSTR *attributeName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_value( 
            /* [retval][out] */ VARIANT *attributeValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_value( 
            /* [in] */ VARIANT attributeValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMAttributeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMAttribute * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMAttribute * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMAttribute * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMAttribute * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMAttribute * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMAttribute * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMAttribute * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMAttribute * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMAttribute * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMAttribute * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMAttribute * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMAttribute * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMAttribute * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMAttribute * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMAttribute * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMAttribute * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMAttribute, get_name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ BSTR *attributeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMAttribute, get_value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_value )( 
            IXMLDOMAttribute * This,
            /* [retval][out] */ VARIANT *attributeValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMAttribute, put_value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_value )( 
            IXMLDOMAttribute * This,
            /* [in] */ VARIANT attributeValue);
        
        END_INTERFACE
    } IXMLDOMAttributeVtbl;

    interface IXMLDOMAttribute
    {
        CONST_VTBL struct IXMLDOMAttributeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMAttribute_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMAttribute_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMAttribute_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMAttribute_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMAttribute_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMAttribute_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMAttribute_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMAttribute_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMAttribute_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMAttribute_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMAttribute_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMAttribute_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMAttribute_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMAttribute_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMAttribute_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMAttribute_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMAttribute_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMAttribute_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMAttribute_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMAttribute_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMAttribute_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMAttribute_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMAttribute_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMAttribute_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMAttribute_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMAttribute_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMAttribute_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMAttribute_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMAttribute_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMAttribute_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMAttribute_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMAttribute_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMAttribute_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMAttribute_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMAttribute_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMAttribute_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMAttribute_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMAttribute_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMAttribute_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMAttribute_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMAttribute_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMAttribute_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMAttribute_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMAttribute_get_name(This,attributeName)	\
    ( (This)->lpVtbl -> get_name(This,attributeName) ) 

#define IXMLDOMAttribute_get_value(This,attributeValue)	\
    ( (This)->lpVtbl -> get_value(This,attributeValue) ) 

#define IXMLDOMAttribute_put_value(This,attributeValue)	\
    ( (This)->lpVtbl -> put_value(This,attributeValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMAttribute_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMElement_INTERFACE_DEFINED__
#define __IXMLDOMElement_INTERFACE_DEFINED__

/* interface IXMLDOMElement */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF86-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMElement : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_tagName( 
            /* [retval][out] */ BSTR *tagName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getAttribute( 
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setAttribute( 
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeAttribute( 
            /* [in] */ BSTR name) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getAttributeNode( 
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setAttributeNode( 
            /* [in] */ IXMLDOMAttribute *DOMAttribute,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeAttributeNode( 
            /* [in] */ IXMLDOMAttribute *DOMAttribute,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getElementsByTagName( 
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE normalize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMElement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMElement * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMElement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMElement * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMElement * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMElement * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMElement * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMElement * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMElement * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMElement * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMElement * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMElement * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMElement * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMElement * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMElement * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMElement * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMElement * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMElement * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMElement * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, get_tagName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_tagName )( 
            IXMLDOMElement * This,
            /* [retval][out] */ BSTR *tagName);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, getAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getAttribute )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, setAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setAttribute )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR name,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, removeAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeAttribute )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR name);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, getAttributeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getAttributeNode )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, setAttributeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setAttributeNode )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMAttribute *DOMAttribute,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, removeAttributeNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeAttributeNode )( 
            IXMLDOMElement * This,
            /* [in] */ IXMLDOMAttribute *DOMAttribute,
            /* [retval][out] */ IXMLDOMAttribute **attributeNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, getElementsByTagName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getElementsByTagName )( 
            IXMLDOMElement * This,
            /* [in] */ BSTR tagName,
            /* [retval][out] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMElement, normalize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *normalize )( 
            IXMLDOMElement * This);
        
        END_INTERFACE
    } IXMLDOMElementVtbl;

    interface IXMLDOMElement
    {
        CONST_VTBL struct IXMLDOMElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMElement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMElement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMElement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMElement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMElement_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMElement_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMElement_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMElement_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMElement_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMElement_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMElement_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMElement_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMElement_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMElement_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMElement_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMElement_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMElement_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMElement_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMElement_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMElement_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMElement_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMElement_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMElement_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMElement_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMElement_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMElement_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMElement_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMElement_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMElement_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMElement_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMElement_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMElement_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMElement_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMElement_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMElement_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMElement_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMElement_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMElement_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMElement_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMElement_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMElement_get_tagName(This,tagName)	\
    ( (This)->lpVtbl -> get_tagName(This,tagName) ) 

#define IXMLDOMElement_getAttribute(This,name,value)	\
    ( (This)->lpVtbl -> getAttribute(This,name,value) ) 

#define IXMLDOMElement_setAttribute(This,name,value)	\
    ( (This)->lpVtbl -> setAttribute(This,name,value) ) 

#define IXMLDOMElement_removeAttribute(This,name)	\
    ( (This)->lpVtbl -> removeAttribute(This,name) ) 

#define IXMLDOMElement_getAttributeNode(This,name,attributeNode)	\
    ( (This)->lpVtbl -> getAttributeNode(This,name,attributeNode) ) 

#define IXMLDOMElement_setAttributeNode(This,DOMAttribute,attributeNode)	\
    ( (This)->lpVtbl -> setAttributeNode(This,DOMAttribute,attributeNode) ) 

#define IXMLDOMElement_removeAttributeNode(This,DOMAttribute,attributeNode)	\
    ( (This)->lpVtbl -> removeAttributeNode(This,DOMAttribute,attributeNode) ) 

#define IXMLDOMElement_getElementsByTagName(This,tagName,resultList)	\
    ( (This)->lpVtbl -> getElementsByTagName(This,tagName,resultList) ) 

#define IXMLDOMElement_normalize(This)	\
    ( (This)->lpVtbl -> normalize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMElement_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMText_INTERFACE_DEFINED__
#define __IXMLDOMText_INTERFACE_DEFINED__

/* interface IXMLDOMText */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF87-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMText : public IXMLDOMCharacterData
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE splitText( 
            /* [in] */ long offset,
            /* [retval][out] */ IXMLDOMText **rightHandTextNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMText * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMText * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMText * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMText * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMText * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMText * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMText * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMText * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMText * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMText * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMText * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMText * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMText * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMText * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMText * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMText * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMText * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMText * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMText * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMText * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMText * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMText * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMText * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMText * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_data )( 
            IXMLDOMText * This,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, put_data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_data )( 
            IXMLDOMText * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMText * This,
            /* [retval][out] */ long *dataLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, substringData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *substringData )( 
            IXMLDOMText * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, appendData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendData )( 
            IXMLDOMText * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, insertData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertData )( 
            IXMLDOMText * This,
            /* [in] */ long offset,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, deleteData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *deleteData )( 
            IXMLDOMText * This,
            /* [in] */ long offset,
            /* [in] */ long count);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, replaceData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceData )( 
            IXMLDOMText * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMText, splitText)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *splitText )( 
            IXMLDOMText * This,
            /* [in] */ long offset,
            /* [retval][out] */ IXMLDOMText **rightHandTextNode);
        
        END_INTERFACE
    } IXMLDOMTextVtbl;

    interface IXMLDOMText
    {
        CONST_VTBL struct IXMLDOMTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMText_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMText_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMText_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMText_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMText_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMText_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMText_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMText_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMText_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMText_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMText_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMText_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMText_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMText_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMText_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMText_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMText_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMText_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMText_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMText_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMText_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMText_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMText_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMText_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMText_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMText_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMText_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMText_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMText_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMText_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMText_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMText_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMText_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMText_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMText_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMText_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMText_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMText_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMText_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMText_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMText_get_data(This,data)	\
    ( (This)->lpVtbl -> get_data(This,data) ) 

#define IXMLDOMText_put_data(This,data)	\
    ( (This)->lpVtbl -> put_data(This,data) ) 

#define IXMLDOMText_get_length(This,dataLength)	\
    ( (This)->lpVtbl -> get_length(This,dataLength) ) 

#define IXMLDOMText_substringData(This,offset,count,data)	\
    ( (This)->lpVtbl -> substringData(This,offset,count,data) ) 

#define IXMLDOMText_appendData(This,data)	\
    ( (This)->lpVtbl -> appendData(This,data) ) 

#define IXMLDOMText_insertData(This,offset,data)	\
    ( (This)->lpVtbl -> insertData(This,offset,data) ) 

#define IXMLDOMText_deleteData(This,offset,count)	\
    ( (This)->lpVtbl -> deleteData(This,offset,count) ) 

#define IXMLDOMText_replaceData(This,offset,count,data)	\
    ( (This)->lpVtbl -> replaceData(This,offset,count,data) ) 


#define IXMLDOMText_splitText(This,offset,rightHandTextNode)	\
    ( (This)->lpVtbl -> splitText(This,offset,rightHandTextNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMText_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMComment_INTERFACE_DEFINED__
#define __IXMLDOMComment_INTERFACE_DEFINED__

/* interface IXMLDOMComment */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMComment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF88-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMComment : public IXMLDOMCharacterData
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMCommentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMComment * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMComment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMComment * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMComment * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMComment * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMComment * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMComment * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMComment * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMComment * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMComment * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMComment * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMComment * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMComment * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMComment * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMComment * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMComment * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMComment * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMComment * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMComment * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMComment * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMComment * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMComment * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_data )( 
            IXMLDOMComment * This,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, put_data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_data )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMComment * This,
            /* [retval][out] */ long *dataLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, substringData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *substringData )( 
            IXMLDOMComment * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, appendData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendData )( 
            IXMLDOMComment * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, insertData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertData )( 
            IXMLDOMComment * This,
            /* [in] */ long offset,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, deleteData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *deleteData )( 
            IXMLDOMComment * This,
            /* [in] */ long offset,
            /* [in] */ long count);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, replaceData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceData )( 
            IXMLDOMComment * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [in] */ BSTR data);
        
        END_INTERFACE
    } IXMLDOMCommentVtbl;

    interface IXMLDOMComment
    {
        CONST_VTBL struct IXMLDOMCommentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMComment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMComment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMComment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMComment_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMComment_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMComment_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMComment_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMComment_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMComment_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMComment_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMComment_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMComment_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMComment_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMComment_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMComment_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMComment_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMComment_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMComment_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMComment_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMComment_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMComment_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMComment_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMComment_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMComment_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMComment_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMComment_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMComment_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMComment_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMComment_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMComment_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMComment_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMComment_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMComment_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMComment_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMComment_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMComment_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMComment_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMComment_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMComment_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMComment_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMComment_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMComment_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMComment_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMComment_get_data(This,data)	\
    ( (This)->lpVtbl -> get_data(This,data) ) 

#define IXMLDOMComment_put_data(This,data)	\
    ( (This)->lpVtbl -> put_data(This,data) ) 

#define IXMLDOMComment_get_length(This,dataLength)	\
    ( (This)->lpVtbl -> get_length(This,dataLength) ) 

#define IXMLDOMComment_substringData(This,offset,count,data)	\
    ( (This)->lpVtbl -> substringData(This,offset,count,data) ) 

#define IXMLDOMComment_appendData(This,data)	\
    ( (This)->lpVtbl -> appendData(This,data) ) 

#define IXMLDOMComment_insertData(This,offset,data)	\
    ( (This)->lpVtbl -> insertData(This,offset,data) ) 

#define IXMLDOMComment_deleteData(This,offset,count)	\
    ( (This)->lpVtbl -> deleteData(This,offset,count) ) 

#define IXMLDOMComment_replaceData(This,offset,count,data)	\
    ( (This)->lpVtbl -> replaceData(This,offset,count,data) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMComment_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMProcessingInstruction_INTERFACE_DEFINED__
#define __IXMLDOMProcessingInstruction_INTERFACE_DEFINED__

/* interface IXMLDOMProcessingInstruction */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMProcessingInstruction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF89-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMProcessingInstruction : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_target( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_data( 
            /* [retval][out] */ BSTR *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_data( 
            /* [in] */ BSTR value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMProcessingInstructionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMProcessingInstruction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMProcessingInstruction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMProcessingInstruction * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMProcessingInstruction * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMProcessingInstruction * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMProcessingInstruction, get_target)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_target )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMProcessingInstruction, get_data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_data )( 
            IXMLDOMProcessingInstruction * This,
            /* [retval][out] */ BSTR *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMProcessingInstruction, put_data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_data )( 
            IXMLDOMProcessingInstruction * This,
            /* [in] */ BSTR value);
        
        END_INTERFACE
    } IXMLDOMProcessingInstructionVtbl;

    interface IXMLDOMProcessingInstruction
    {
        CONST_VTBL struct IXMLDOMProcessingInstructionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMProcessingInstruction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMProcessingInstruction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMProcessingInstruction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMProcessingInstruction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMProcessingInstruction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMProcessingInstruction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMProcessingInstruction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMProcessingInstruction_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMProcessingInstruction_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMProcessingInstruction_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMProcessingInstruction_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMProcessingInstruction_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMProcessingInstruction_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMProcessingInstruction_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMProcessingInstruction_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMProcessingInstruction_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMProcessingInstruction_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMProcessingInstruction_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMProcessingInstruction_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMProcessingInstruction_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMProcessingInstruction_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMProcessingInstruction_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMProcessingInstruction_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMProcessingInstruction_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMProcessingInstruction_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMProcessingInstruction_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMProcessingInstruction_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMProcessingInstruction_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMProcessingInstruction_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMProcessingInstruction_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMProcessingInstruction_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMProcessingInstruction_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMProcessingInstruction_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMProcessingInstruction_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMProcessingInstruction_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMProcessingInstruction_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMProcessingInstruction_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMProcessingInstruction_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMProcessingInstruction_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMProcessingInstruction_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMProcessingInstruction_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMProcessingInstruction_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMProcessingInstruction_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMProcessingInstruction_get_target(This,name)	\
    ( (This)->lpVtbl -> get_target(This,name) ) 

#define IXMLDOMProcessingInstruction_get_data(This,value)	\
    ( (This)->lpVtbl -> get_data(This,value) ) 

#define IXMLDOMProcessingInstruction_put_data(This,value)	\
    ( (This)->lpVtbl -> put_data(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMProcessingInstruction_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMCDATASection_INTERFACE_DEFINED__
#define __IXMLDOMCDATASection_INTERFACE_DEFINED__

/* interface IXMLDOMCDATASection */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMCDATASection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8A-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMCDATASection : public IXMLDOMText
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMCDATASectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMCDATASection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMCDATASection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMCDATASection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMCDATASection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMCDATASection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMCDATASection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMCDATASection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMCDATASection * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMCDATASection * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMCDATASection * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMCDATASection * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMCDATASection * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_data )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, put_data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_data )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMCDATASection * This,
            /* [retval][out] */ long *dataLength);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, substringData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *substringData )( 
            IXMLDOMCDATASection * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [retval][out] */ BSTR *data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, appendData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendData )( 
            IXMLDOMCDATASection * This,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, insertData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertData )( 
            IXMLDOMCDATASection * This,
            /* [in] */ long offset,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, deleteData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *deleteData )( 
            IXMLDOMCDATASection * This,
            /* [in] */ long offset,
            /* [in] */ long count);
        
        DECLSPEC_XFGVIRT(IXMLDOMCharacterData, replaceData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceData )( 
            IXMLDOMCDATASection * This,
            /* [in] */ long offset,
            /* [in] */ long count,
            /* [in] */ BSTR data);
        
        DECLSPEC_XFGVIRT(IXMLDOMText, splitText)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *splitText )( 
            IXMLDOMCDATASection * This,
            /* [in] */ long offset,
            /* [retval][out] */ IXMLDOMText **rightHandTextNode);
        
        END_INTERFACE
    } IXMLDOMCDATASectionVtbl;

    interface IXMLDOMCDATASection
    {
        CONST_VTBL struct IXMLDOMCDATASectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMCDATASection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMCDATASection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMCDATASection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMCDATASection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMCDATASection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMCDATASection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMCDATASection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMCDATASection_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMCDATASection_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMCDATASection_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMCDATASection_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMCDATASection_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMCDATASection_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMCDATASection_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMCDATASection_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMCDATASection_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMCDATASection_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMCDATASection_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMCDATASection_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMCDATASection_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMCDATASection_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMCDATASection_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMCDATASection_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMCDATASection_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMCDATASection_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMCDATASection_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMCDATASection_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMCDATASection_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMCDATASection_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMCDATASection_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMCDATASection_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMCDATASection_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMCDATASection_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMCDATASection_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMCDATASection_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMCDATASection_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMCDATASection_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMCDATASection_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMCDATASection_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMCDATASection_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMCDATASection_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMCDATASection_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMCDATASection_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMCDATASection_get_data(This,data)	\
    ( (This)->lpVtbl -> get_data(This,data) ) 

#define IXMLDOMCDATASection_put_data(This,data)	\
    ( (This)->lpVtbl -> put_data(This,data) ) 

#define IXMLDOMCDATASection_get_length(This,dataLength)	\
    ( (This)->lpVtbl -> get_length(This,dataLength) ) 

#define IXMLDOMCDATASection_substringData(This,offset,count,data)	\
    ( (This)->lpVtbl -> substringData(This,offset,count,data) ) 

#define IXMLDOMCDATASection_appendData(This,data)	\
    ( (This)->lpVtbl -> appendData(This,data) ) 

#define IXMLDOMCDATASection_insertData(This,offset,data)	\
    ( (This)->lpVtbl -> insertData(This,offset,data) ) 

#define IXMLDOMCDATASection_deleteData(This,offset,count)	\
    ( (This)->lpVtbl -> deleteData(This,offset,count) ) 

#define IXMLDOMCDATASection_replaceData(This,offset,count,data)	\
    ( (This)->lpVtbl -> replaceData(This,offset,count,data) ) 


#define IXMLDOMCDATASection_splitText(This,offset,rightHandTextNode)	\
    ( (This)->lpVtbl -> splitText(This,offset,rightHandTextNode) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMCDATASection_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMDocumentType_INTERFACE_DEFINED__
#define __IXMLDOMDocumentType_INTERFACE_DEFINED__

/* interface IXMLDOMDocumentType */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMDocumentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8B-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMDocumentType : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_name( 
            /* [retval][out] */ BSTR *rootName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_entities( 
            /* [retval][out] */ IXMLDOMNamedNodeMap **entityMap) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_notations( 
            /* [retval][out] */ IXMLDOMNamedNodeMap **notationMap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMDocumentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMDocumentType * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMDocumentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMDocumentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMDocumentType * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMDocumentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMDocumentType * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMDocumentType * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMDocumentType * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMDocumentType * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMDocumentType * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMDocumentType * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMDocumentType * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMDocumentType * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMDocumentType * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMDocumentType * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMDocumentType * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocumentType, get_name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ BSTR *rootName);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocumentType, get_entities)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_entities )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **entityMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMDocumentType, get_notations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_notations )( 
            IXMLDOMDocumentType * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **notationMap);
        
        END_INTERFACE
    } IXMLDOMDocumentTypeVtbl;

    interface IXMLDOMDocumentType
    {
        CONST_VTBL struct IXMLDOMDocumentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMDocumentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMDocumentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMDocumentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMDocumentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMDocumentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMDocumentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMDocumentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMDocumentType_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMDocumentType_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMDocumentType_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMDocumentType_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMDocumentType_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMDocumentType_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMDocumentType_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMDocumentType_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMDocumentType_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMDocumentType_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMDocumentType_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMDocumentType_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMDocumentType_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMDocumentType_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMDocumentType_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMDocumentType_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMDocumentType_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMDocumentType_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMDocumentType_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMDocumentType_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMDocumentType_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMDocumentType_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMDocumentType_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMDocumentType_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocumentType_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMDocumentType_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMDocumentType_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMDocumentType_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMDocumentType_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMDocumentType_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMDocumentType_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMDocumentType_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMDocumentType_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMDocumentType_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMDocumentType_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMDocumentType_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMDocumentType_get_name(This,rootName)	\
    ( (This)->lpVtbl -> get_name(This,rootName) ) 

#define IXMLDOMDocumentType_get_entities(This,entityMap)	\
    ( (This)->lpVtbl -> get_entities(This,entityMap) ) 

#define IXMLDOMDocumentType_get_notations(This,notationMap)	\
    ( (This)->lpVtbl -> get_notations(This,notationMap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMDocumentType_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMNotation_INTERFACE_DEFINED__
#define __IXMLDOMNotation_INTERFACE_DEFINED__

/* interface IXMLDOMNotation */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMNotation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8C-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMNotation : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_publicId( 
            /* [retval][out] */ VARIANT *publicID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_systemId( 
            /* [retval][out] */ VARIANT *systemID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMNotationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMNotation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMNotation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMNotation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMNotation * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMNotation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMNotation * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMNotation * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMNotation * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMNotation * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMNotation * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMNotation * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMNotation * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMNotation * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMNotation * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMNotation * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMNotation * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMNotation, get_publicId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_publicId )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ VARIANT *publicID);
        
        DECLSPEC_XFGVIRT(IXMLDOMNotation, get_systemId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_systemId )( 
            IXMLDOMNotation * This,
            /* [retval][out] */ VARIANT *systemID);
        
        END_INTERFACE
    } IXMLDOMNotationVtbl;

    interface IXMLDOMNotation
    {
        CONST_VTBL struct IXMLDOMNotationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMNotation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMNotation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMNotation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMNotation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMNotation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMNotation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMNotation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMNotation_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMNotation_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMNotation_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMNotation_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMNotation_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMNotation_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMNotation_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMNotation_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMNotation_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMNotation_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMNotation_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMNotation_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMNotation_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMNotation_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMNotation_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMNotation_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMNotation_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMNotation_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMNotation_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMNotation_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMNotation_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMNotation_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMNotation_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMNotation_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMNotation_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMNotation_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMNotation_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMNotation_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMNotation_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMNotation_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMNotation_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMNotation_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMNotation_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMNotation_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMNotation_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMNotation_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMNotation_get_publicId(This,publicID)	\
    ( (This)->lpVtbl -> get_publicId(This,publicID) ) 

#define IXMLDOMNotation_get_systemId(This,systemID)	\
    ( (This)->lpVtbl -> get_systemId(This,systemID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMNotation_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMEntity_INTERFACE_DEFINED__
#define __IXMLDOMEntity_INTERFACE_DEFINED__

/* interface IXMLDOMEntity */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMEntity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8D-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMEntity : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_publicId( 
            /* [retval][out] */ VARIANT *publicID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_systemId( 
            /* [retval][out] */ VARIANT *systemID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_notationName( 
            /* [retval][out] */ BSTR *name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMEntityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMEntity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMEntity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMEntity * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMEntity * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMEntity * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMEntity * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMEntity * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMEntity * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMEntity * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMEntity * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMEntity * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMEntity * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMEntity * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMEntity * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMEntity * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMEntity * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXMLDOMEntity, get_publicId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_publicId )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ VARIANT *publicID);
        
        DECLSPEC_XFGVIRT(IXMLDOMEntity, get_systemId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_systemId )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ VARIANT *systemID);
        
        DECLSPEC_XFGVIRT(IXMLDOMEntity, get_notationName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_notationName )( 
            IXMLDOMEntity * This,
            /* [retval][out] */ BSTR *name);
        
        END_INTERFACE
    } IXMLDOMEntityVtbl;

    interface IXMLDOMEntity
    {
        CONST_VTBL struct IXMLDOMEntityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMEntity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMEntity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMEntity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMEntity_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMEntity_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMEntity_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMEntity_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMEntity_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMEntity_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMEntity_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMEntity_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMEntity_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMEntity_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMEntity_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMEntity_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMEntity_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMEntity_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMEntity_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMEntity_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMEntity_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMEntity_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMEntity_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMEntity_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMEntity_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMEntity_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMEntity_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMEntity_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMEntity_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMEntity_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMEntity_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMEntity_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMEntity_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMEntity_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMEntity_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMEntity_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMEntity_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMEntity_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMEntity_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMEntity_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMEntity_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMEntity_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMEntity_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMEntity_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXMLDOMEntity_get_publicId(This,publicID)	\
    ( (This)->lpVtbl -> get_publicId(This,publicID) ) 

#define IXMLDOMEntity_get_systemId(This,systemID)	\
    ( (This)->lpVtbl -> get_systemId(This,systemID) ) 

#define IXMLDOMEntity_get_notationName(This,name)	\
    ( (This)->lpVtbl -> get_notationName(This,name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMEntity_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMEntityReference_INTERFACE_DEFINED__
#define __IXMLDOMEntityReference_INTERFACE_DEFINED__

/* interface IXMLDOMEntityReference */
/* [unique][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMEntityReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8E-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMEntityReference : public IXMLDOMNode
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMEntityReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMEntityReference * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMEntityReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMEntityReference * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMEntityReference * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMEntityReference * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMEntityReference * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMEntityReference * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXMLDOMEntityReference * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXMLDOMEntityReference * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXMLDOMEntityReference * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXMLDOMEntityReference * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXMLDOMEntityReference * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXMLDOMEntityReference * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXMLDOMEntityReference * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXMLDOMEntityReference * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXMLDOMEntityReference * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXMLDOMEntityReference * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        END_INTERFACE
    } IXMLDOMEntityReferenceVtbl;

    interface IXMLDOMEntityReference
    {
        CONST_VTBL struct IXMLDOMEntityReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMEntityReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMEntityReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMEntityReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMEntityReference_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMEntityReference_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMEntityReference_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMEntityReference_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMEntityReference_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXMLDOMEntityReference_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXMLDOMEntityReference_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXMLDOMEntityReference_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXMLDOMEntityReference_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXMLDOMEntityReference_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXMLDOMEntityReference_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXMLDOMEntityReference_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXMLDOMEntityReference_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXMLDOMEntityReference_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXMLDOMEntityReference_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXMLDOMEntityReference_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXMLDOMEntityReference_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXMLDOMEntityReference_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXMLDOMEntityReference_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXMLDOMEntityReference_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXMLDOMEntityReference_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXMLDOMEntityReference_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXMLDOMEntityReference_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXMLDOMEntityReference_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXMLDOMEntityReference_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXMLDOMEntityReference_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXMLDOMEntityReference_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXMLDOMEntityReference_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMEntityReference_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXMLDOMEntityReference_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXMLDOMEntityReference_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXMLDOMEntityReference_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXMLDOMEntityReference_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXMLDOMEntityReference_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXMLDOMEntityReference_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXMLDOMEntityReference_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXMLDOMEntityReference_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXMLDOMEntityReference_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXMLDOMEntityReference_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXMLDOMEntityReference_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMEntityReference_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMImplementation_INTERFACE_DEFINED__
#define __IXMLDOMImplementation_INTERFACE_DEFINED__

/* interface IXMLDOMImplementation */
/* [uuid][dual][oleautomation][unique][nonextensible][object][local] */ 


EXTERN_C const IID IID_IXMLDOMImplementation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF8F-7B36-11d2-B20E-00C04F983E60")
    IXMLDOMImplementation : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE hasFeature( 
            /* [in] */ BSTR feature,
            /* [in] */ BSTR version,
            /* [retval][out] */ VARIANT_BOOL *hasFeature) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMImplementationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMImplementation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMImplementation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMImplementation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMImplementation * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMImplementation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMImplementation * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMImplementation * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMImplementation, hasFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *hasFeature )( 
            IXMLDOMImplementation * This,
            /* [in] */ BSTR feature,
            /* [in] */ BSTR version,
            /* [retval][out] */ VARIANT_BOOL *hasFeature);
        
        END_INTERFACE
    } IXMLDOMImplementationVtbl;

    interface IXMLDOMImplementation
    {
        CONST_VTBL struct IXMLDOMImplementationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMImplementation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMImplementation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMImplementation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMImplementation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMImplementation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMImplementation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMImplementation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMImplementation_hasFeature(This,feature,version,hasFeature)	\
    ( (This)->lpVtbl -> hasFeature(This,feature,version,hasFeature) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMImplementation_INTERFACE_DEFINED__ */


#ifndef __IXTLRuntime_INTERFACE_DEFINED__
#define __IXTLRuntime_INTERFACE_DEFINED__

/* interface IXTLRuntime */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXTLRuntime;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3efaa425-272f-11d2-836f-0000f87a7782")
    IXTLRuntime : public IXMLDOMNode
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE uniqueID( 
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE depth( 
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pDepth) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE childNumber( 
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ancestorChildNumber( 
            /* [in] */ BSTR bstrNodeName,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE absoluteChildNumber( 
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE formatIndex( 
            /* [in] */ long lIndex,
            /* [in] */ BSTR bstrFormat,
            /* [retval][out] */ BSTR *pbstrFormattedString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE formatNumber( 
            /* [in] */ double dblNumber,
            /* [in] */ BSTR bstrFormat,
            /* [retval][out] */ BSTR *pbstrFormattedString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE formatDate( 
            /* [in] */ VARIANT varDate,
            /* [in] */ BSTR bstrFormat,
            /* [optional][in] */ VARIANT varDestLocale,
            /* [retval][out] */ BSTR *pbstrFormattedString) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE formatTime( 
            /* [in] */ VARIANT varTime,
            /* [in] */ BSTR bstrFormat,
            /* [optional][in] */ VARIANT varDestLocale,
            /* [retval][out] */ BSTR *pbstrFormattedString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXTLRuntimeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXTLRuntime * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXTLRuntime * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXTLRuntime * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXTLRuntime * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXTLRuntime * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXTLRuntime * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXTLRuntime * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeName )( 
            IXTLRuntime * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeValue )( 
            IXTLRuntime * This,
            /* [retval][out] */ VARIANT *value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeValue )( 
            IXTLRuntime * This,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeType )( 
            IXTLRuntime * This,
            /* [retval][out] */ DOMNodeType *type);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parentNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parentNode )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNode **parent);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_childNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_childNodes )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNodeList **childList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_firstChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_firstChild )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNode **firstChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_lastChild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lastChild )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNode **lastChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_previousSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_previousSibling )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNode **previousSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nextSibling)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextSibling )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNode **nextSibling);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_attributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMNamedNodeMap **attributeMap);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, insertBefore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *insertBefore )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ VARIANT refChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, replaceChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *replaceChild )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [in] */ IXMLDOMNode *oldChild,
            /* [retval][out] */ IXMLDOMNode **outOldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, removeChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeChild )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *childNode,
            /* [retval][out] */ IXMLDOMNode **oldChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, appendChild)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *appendChild )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *newChild,
            /* [retval][out] */ IXMLDOMNode **outNewChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, hasChildNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *hasChildNodes )( 
            IXTLRuntime * This,
            /* [retval][out] */ VARIANT_BOOL *hasChild);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_ownerDocument)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerDocument )( 
            IXTLRuntime * This,
            /* [retval][out] */ IXMLDOMDocument **XMLDOMDocument);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, cloneNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cloneNode )( 
            IXTLRuntime * This,
            /* [in] */ VARIANT_BOOL deep,
            /* [retval][out] */ IXMLDOMNode **cloneRoot);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypeString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypeString )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *nodeType);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_text)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_text )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_text)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_text )( 
            IXTLRuntime * This,
            /* [in] */ BSTR text);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_specified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_specified )( 
            IXTLRuntime * This,
            /* [retval][out] */ VARIANT_BOOL *isSpecified);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_definition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_definition )( 
            IXTLRuntime * This,
            /* [out][retval] */ IXMLDOMNode **definitionNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_nodeTypedValue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nodeTypedValue )( 
            IXTLRuntime * This,
            /* [out][retval] */ VARIANT *typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_nodeTypedValue)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nodeTypedValue )( 
            IXTLRuntime * This,
            /* [in] */ VARIANT typedValue);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_dataType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dataType )( 
            IXTLRuntime * This,
            /* [out][retval] */ VARIANT *dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, put_dataType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dataType )( 
            IXTLRuntime * This,
            /* [in] */ BSTR dataTypeName);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_xml)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_xml )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNode )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [out][retval] */ BSTR *xmlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectNodes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectNodes )( 
            IXTLRuntime * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNodeList **resultList);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, selectSingleNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *selectSingleNode )( 
            IXTLRuntime * This,
            /* [in] */ BSTR queryString,
            /* [out][retval] */ IXMLDOMNode **resultNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_parsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parsed )( 
            IXTLRuntime * This,
            /* [out][retval] */ VARIANT_BOOL *isParsed);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_prefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_prefix )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *prefixString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, get_baseName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseName )( 
            IXTLRuntime * This,
            /* [out][retval] */ BSTR *nameString);
        
        DECLSPEC_XFGVIRT(IXMLDOMNode, transformNodeToObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transformNodeToObject )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *stylesheet,
            /* [in] */ VARIANT outputObject);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, uniqueID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *uniqueID )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pID);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, depth)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *depth )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pDepth);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, childNumber)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *childNumber )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, ancestorChildNumber)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ancestorChildNumber )( 
            IXTLRuntime * This,
            /* [in] */ BSTR bstrNodeName,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, absoluteChildNumber)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *absoluteChildNumber )( 
            IXTLRuntime * This,
            /* [in] */ IXMLDOMNode *pNode,
            /* [retval][out] */ long *pNumber);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, formatIndex)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *formatIndex )( 
            IXTLRuntime * This,
            /* [in] */ long lIndex,
            /* [in] */ BSTR bstrFormat,
            /* [retval][out] */ BSTR *pbstrFormattedString);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, formatNumber)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *formatNumber )( 
            IXTLRuntime * This,
            /* [in] */ double dblNumber,
            /* [in] */ BSTR bstrFormat,
            /* [retval][out] */ BSTR *pbstrFormattedString);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, formatDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *formatDate )( 
            IXTLRuntime * This,
            /* [in] */ VARIANT varDate,
            /* [in] */ BSTR bstrFormat,
            /* [optional][in] */ VARIANT varDestLocale,
            /* [retval][out] */ BSTR *pbstrFormattedString);
        
        DECLSPEC_XFGVIRT(IXTLRuntime, formatTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *formatTime )( 
            IXTLRuntime * This,
            /* [in] */ VARIANT varTime,
            /* [in] */ BSTR bstrFormat,
            /* [optional][in] */ VARIANT varDestLocale,
            /* [retval][out] */ BSTR *pbstrFormattedString);
        
        END_INTERFACE
    } IXTLRuntimeVtbl;

    interface IXTLRuntime
    {
        CONST_VTBL struct IXTLRuntimeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXTLRuntime_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXTLRuntime_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXTLRuntime_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXTLRuntime_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXTLRuntime_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXTLRuntime_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXTLRuntime_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXTLRuntime_get_nodeName(This,name)	\
    ( (This)->lpVtbl -> get_nodeName(This,name) ) 

#define IXTLRuntime_get_nodeValue(This,value)	\
    ( (This)->lpVtbl -> get_nodeValue(This,value) ) 

#define IXTLRuntime_put_nodeValue(This,value)	\
    ( (This)->lpVtbl -> put_nodeValue(This,value) ) 

#define IXTLRuntime_get_nodeType(This,type)	\
    ( (This)->lpVtbl -> get_nodeType(This,type) ) 

#define IXTLRuntime_get_parentNode(This,parent)	\
    ( (This)->lpVtbl -> get_parentNode(This,parent) ) 

#define IXTLRuntime_get_childNodes(This,childList)	\
    ( (This)->lpVtbl -> get_childNodes(This,childList) ) 

#define IXTLRuntime_get_firstChild(This,firstChild)	\
    ( (This)->lpVtbl -> get_firstChild(This,firstChild) ) 

#define IXTLRuntime_get_lastChild(This,lastChild)	\
    ( (This)->lpVtbl -> get_lastChild(This,lastChild) ) 

#define IXTLRuntime_get_previousSibling(This,previousSibling)	\
    ( (This)->lpVtbl -> get_previousSibling(This,previousSibling) ) 

#define IXTLRuntime_get_nextSibling(This,nextSibling)	\
    ( (This)->lpVtbl -> get_nextSibling(This,nextSibling) ) 

#define IXTLRuntime_get_attributes(This,attributeMap)	\
    ( (This)->lpVtbl -> get_attributes(This,attributeMap) ) 

#define IXTLRuntime_insertBefore(This,newChild,refChild,outNewChild)	\
    ( (This)->lpVtbl -> insertBefore(This,newChild,refChild,outNewChild) ) 

#define IXTLRuntime_replaceChild(This,newChild,oldChild,outOldChild)	\
    ( (This)->lpVtbl -> replaceChild(This,newChild,oldChild,outOldChild) ) 

#define IXTLRuntime_removeChild(This,childNode,oldChild)	\
    ( (This)->lpVtbl -> removeChild(This,childNode,oldChild) ) 

#define IXTLRuntime_appendChild(This,newChild,outNewChild)	\
    ( (This)->lpVtbl -> appendChild(This,newChild,outNewChild) ) 

#define IXTLRuntime_hasChildNodes(This,hasChild)	\
    ( (This)->lpVtbl -> hasChildNodes(This,hasChild) ) 

#define IXTLRuntime_get_ownerDocument(This,XMLDOMDocument)	\
    ( (This)->lpVtbl -> get_ownerDocument(This,XMLDOMDocument) ) 

#define IXTLRuntime_cloneNode(This,deep,cloneRoot)	\
    ( (This)->lpVtbl -> cloneNode(This,deep,cloneRoot) ) 

#define IXTLRuntime_get_nodeTypeString(This,nodeType)	\
    ( (This)->lpVtbl -> get_nodeTypeString(This,nodeType) ) 

#define IXTLRuntime_get_text(This,text)	\
    ( (This)->lpVtbl -> get_text(This,text) ) 

#define IXTLRuntime_put_text(This,text)	\
    ( (This)->lpVtbl -> put_text(This,text) ) 

#define IXTLRuntime_get_specified(This,isSpecified)	\
    ( (This)->lpVtbl -> get_specified(This,isSpecified) ) 

#define IXTLRuntime_get_definition(This,definitionNode)	\
    ( (This)->lpVtbl -> get_definition(This,definitionNode) ) 

#define IXTLRuntime_get_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> get_nodeTypedValue(This,typedValue) ) 

#define IXTLRuntime_put_nodeTypedValue(This,typedValue)	\
    ( (This)->lpVtbl -> put_nodeTypedValue(This,typedValue) ) 

#define IXTLRuntime_get_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> get_dataType(This,dataTypeName) ) 

#define IXTLRuntime_put_dataType(This,dataTypeName)	\
    ( (This)->lpVtbl -> put_dataType(This,dataTypeName) ) 

#define IXTLRuntime_get_xml(This,xmlString)	\
    ( (This)->lpVtbl -> get_xml(This,xmlString) ) 

#define IXTLRuntime_transformNode(This,stylesheet,xmlString)	\
    ( (This)->lpVtbl -> transformNode(This,stylesheet,xmlString) ) 

#define IXTLRuntime_selectNodes(This,queryString,resultList)	\
    ( (This)->lpVtbl -> selectNodes(This,queryString,resultList) ) 

#define IXTLRuntime_selectSingleNode(This,queryString,resultNode)	\
    ( (This)->lpVtbl -> selectSingleNode(This,queryString,resultNode) ) 

#define IXTLRuntime_get_parsed(This,isParsed)	\
    ( (This)->lpVtbl -> get_parsed(This,isParsed) ) 

#define IXTLRuntime_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define IXTLRuntime_get_prefix(This,prefixString)	\
    ( (This)->lpVtbl -> get_prefix(This,prefixString) ) 

#define IXTLRuntime_get_baseName(This,nameString)	\
    ( (This)->lpVtbl -> get_baseName(This,nameString) ) 

#define IXTLRuntime_transformNodeToObject(This,stylesheet,outputObject)	\
    ( (This)->lpVtbl -> transformNodeToObject(This,stylesheet,outputObject) ) 


#define IXTLRuntime_uniqueID(This,pNode,pID)	\
    ( (This)->lpVtbl -> uniqueID(This,pNode,pID) ) 

#define IXTLRuntime_depth(This,pNode,pDepth)	\
    ( (This)->lpVtbl -> depth(This,pNode,pDepth) ) 

#define IXTLRuntime_childNumber(This,pNode,pNumber)	\
    ( (This)->lpVtbl -> childNumber(This,pNode,pNumber) ) 

#define IXTLRuntime_ancestorChildNumber(This,bstrNodeName,pNode,pNumber)	\
    ( (This)->lpVtbl -> ancestorChildNumber(This,bstrNodeName,pNode,pNumber) ) 

#define IXTLRuntime_absoluteChildNumber(This,pNode,pNumber)	\
    ( (This)->lpVtbl -> absoluteChildNumber(This,pNode,pNumber) ) 

#define IXTLRuntime_formatIndex(This,lIndex,bstrFormat,pbstrFormattedString)	\
    ( (This)->lpVtbl -> formatIndex(This,lIndex,bstrFormat,pbstrFormattedString) ) 

#define IXTLRuntime_formatNumber(This,dblNumber,bstrFormat,pbstrFormattedString)	\
    ( (This)->lpVtbl -> formatNumber(This,dblNumber,bstrFormat,pbstrFormattedString) ) 

#define IXTLRuntime_formatDate(This,varDate,bstrFormat,varDestLocale,pbstrFormattedString)	\
    ( (This)->lpVtbl -> formatDate(This,varDate,bstrFormat,varDestLocale,pbstrFormattedString) ) 

#define IXTLRuntime_formatTime(This,varTime,bstrFormat,varDestLocale,pbstrFormattedString)	\
    ( (This)->lpVtbl -> formatTime(This,varTime,bstrFormat,varDestLocale,pbstrFormattedString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXTLRuntime_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMParseError_INTERFACE_DEFINED__
#define __IXMLDOMParseError_INTERFACE_DEFINED__

/* interface IXMLDOMParseError */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMParseError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3efaa426-272f-11d2-836f-0000f87a7782")
    IXMLDOMParseError : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_errorCode( 
            /* [out][retval] */ long *errorCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_url( 
            /* [out][retval] */ BSTR *urlString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_reason( 
            /* [out][retval] */ BSTR *reasonString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_srcText( 
            /* [out][retval] */ BSTR *sourceString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_line( 
            /* [out][retval] */ long *lineNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_linepos( 
            /* [out][retval] */ long *linePosition) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_filepos( 
            /* [out][retval] */ long *filePosition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMParseErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMParseError * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMParseError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMParseError * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMParseError * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMParseError * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMParseError * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMParseError * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_errorCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ long *errorCode);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_url)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_url )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ BSTR *urlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_reason)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_reason )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ BSTR *reasonString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_srcText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_srcText )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ BSTR *sourceString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_line)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_line )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ long *lineNumber);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_linepos)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_linepos )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ long *linePosition);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_filepos)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_filepos )( 
            IXMLDOMParseError * This,
            /* [out][retval] */ long *filePosition);
        
        END_INTERFACE
    } IXMLDOMParseErrorVtbl;

    interface IXMLDOMParseError
    {
        CONST_VTBL struct IXMLDOMParseErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMParseError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMParseError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMParseError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMParseError_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMParseError_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMParseError_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMParseError_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMParseError_get_errorCode(This,errorCode)	\
    ( (This)->lpVtbl -> get_errorCode(This,errorCode) ) 

#define IXMLDOMParseError_get_url(This,urlString)	\
    ( (This)->lpVtbl -> get_url(This,urlString) ) 

#define IXMLDOMParseError_get_reason(This,reasonString)	\
    ( (This)->lpVtbl -> get_reason(This,reasonString) ) 

#define IXMLDOMParseError_get_srcText(This,sourceString)	\
    ( (This)->lpVtbl -> get_srcText(This,sourceString) ) 

#define IXMLDOMParseError_get_line(This,lineNumber)	\
    ( (This)->lpVtbl -> get_line(This,lineNumber) ) 

#define IXMLDOMParseError_get_linepos(This,linePosition)	\
    ( (This)->lpVtbl -> get_linepos(This,linePosition) ) 

#define IXMLDOMParseError_get_filepos(This,filePosition)	\
    ( (This)->lpVtbl -> get_filepos(This,filePosition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMParseError_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMParseError2_INTERFACE_DEFINED__
#define __IXMLDOMParseError2_INTERFACE_DEFINED__

/* interface IXMLDOMParseError2 */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMParseError2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3efaa428-272f-11d2-836f-0000f87a7782")
    IXMLDOMParseError2 : public IXMLDOMParseError
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_errorXPath( 
            /* [retval][out] */ BSTR *xpathexpr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_allErrors( 
            /* [retval][out] */ IXMLDOMParseErrorCollection **allErrors) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE errorParameters( 
            /* [in] */ long index,
            /* [retval][out] */ BSTR *param) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_errorParametersCount( 
            /* [retval][out] */ long *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMParseError2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMParseError2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMParseError2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMParseError2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMParseError2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMParseError2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMParseError2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMParseError2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_errorCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorCode )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ long *errorCode);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_url)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_url )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ BSTR *urlString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_reason)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_reason )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ BSTR *reasonString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_srcText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_srcText )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ BSTR *sourceString);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_line)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_line )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ long *lineNumber);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_linepos)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_linepos )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ long *linePosition);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError, get_filepos)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_filepos )( 
            IXMLDOMParseError2 * This,
            /* [out][retval] */ long *filePosition);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError2, get_errorXPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorXPath )( 
            IXMLDOMParseError2 * This,
            /* [retval][out] */ BSTR *xpathexpr);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError2, get_allErrors)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_allErrors )( 
            IXMLDOMParseError2 * This,
            /* [retval][out] */ IXMLDOMParseErrorCollection **allErrors);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError2, errorParameters)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *errorParameters )( 
            IXMLDOMParseError2 * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *param);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseError2, get_errorParametersCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorParametersCount )( 
            IXMLDOMParseError2 * This,
            /* [retval][out] */ long *count);
        
        END_INTERFACE
    } IXMLDOMParseError2Vtbl;

    interface IXMLDOMParseError2
    {
        CONST_VTBL struct IXMLDOMParseError2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMParseError2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMParseError2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMParseError2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMParseError2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMParseError2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMParseError2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMParseError2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMParseError2_get_errorCode(This,errorCode)	\
    ( (This)->lpVtbl -> get_errorCode(This,errorCode) ) 

#define IXMLDOMParseError2_get_url(This,urlString)	\
    ( (This)->lpVtbl -> get_url(This,urlString) ) 

#define IXMLDOMParseError2_get_reason(This,reasonString)	\
    ( (This)->lpVtbl -> get_reason(This,reasonString) ) 

#define IXMLDOMParseError2_get_srcText(This,sourceString)	\
    ( (This)->lpVtbl -> get_srcText(This,sourceString) ) 

#define IXMLDOMParseError2_get_line(This,lineNumber)	\
    ( (This)->lpVtbl -> get_line(This,lineNumber) ) 

#define IXMLDOMParseError2_get_linepos(This,linePosition)	\
    ( (This)->lpVtbl -> get_linepos(This,linePosition) ) 

#define IXMLDOMParseError2_get_filepos(This,filePosition)	\
    ( (This)->lpVtbl -> get_filepos(This,filePosition) ) 


#define IXMLDOMParseError2_get_errorXPath(This,xpathexpr)	\
    ( (This)->lpVtbl -> get_errorXPath(This,xpathexpr) ) 

#define IXMLDOMParseError2_get_allErrors(This,allErrors)	\
    ( (This)->lpVtbl -> get_allErrors(This,allErrors) ) 

#define IXMLDOMParseError2_errorParameters(This,index,param)	\
    ( (This)->lpVtbl -> errorParameters(This,index,param) ) 

#define IXMLDOMParseError2_get_errorParametersCount(This,count)	\
    ( (This)->lpVtbl -> get_errorParametersCount(This,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMParseError2_INTERFACE_DEFINED__ */


#ifndef __IXMLDOMParseErrorCollection_INTERFACE_DEFINED__
#define __IXMLDOMParseErrorCollection_INTERFACE_DEFINED__

/* interface IXMLDOMParseErrorCollection */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMParseErrorCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3efaa429-272f-11d2-836f-0000f87a7782")
    IXMLDOMParseErrorCollection : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMParseError2 **error) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *length) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_next( 
            /* [retval][out] */ IXMLDOMParseError2 **error) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual /* [propget][restricted][hidden][id] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [retval][out] */ IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMParseErrorCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMParseErrorCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMParseErrorCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMParseErrorCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMParseErrorCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMParseErrorCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMParseErrorCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMParseErrorCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseErrorCollection, get_item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            IXMLDOMParseErrorCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ IXMLDOMParseError2 **error);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseErrorCollection, get_length)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMParseErrorCollection * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseErrorCollection, get_next)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_next )( 
            IXMLDOMParseErrorCollection * This,
            /* [retval][out] */ IXMLDOMParseError2 **error);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseErrorCollection, reset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IXMLDOMParseErrorCollection * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMParseErrorCollection, get__newEnum)
        /* [propget][restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMParseErrorCollection * This,
            /* [retval][out] */ IUnknown **ppunk);
        
        END_INTERFACE
    } IXMLDOMParseErrorCollectionVtbl;

    interface IXMLDOMParseErrorCollection
    {
        CONST_VTBL struct IXMLDOMParseErrorCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMParseErrorCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMParseErrorCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMParseErrorCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMParseErrorCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMParseErrorCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMParseErrorCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMParseErrorCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMParseErrorCollection_get_item(This,index,error)	\
    ( (This)->lpVtbl -> get_item(This,index,error) ) 

#define IXMLDOMParseErrorCollection_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define IXMLDOMParseErrorCollection_get_next(This,error)	\
    ( (This)->lpVtbl -> get_next(This,error) ) 

#define IXMLDOMParseErrorCollection_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IXMLDOMParseErrorCollection_get__newEnum(This,ppunk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMParseErrorCollection_INTERFACE_DEFINED__ */


#ifndef __IXSLProcessor_INTERFACE_DEFINED__
#define __IXSLProcessor_INTERFACE_DEFINED__

/* interface IXSLProcessor */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXSLProcessor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF92-7B36-11d2-B20E-00C04F983E60")
    IXSLProcessor : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_input( 
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_input( 
            /* [retval][out] */ VARIANT *pVar) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ownerTemplate( 
            /* [retval][out] */ IXSLTemplate **ppTemplate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setStartMode( 
            /* [in] */ BSTR mode,
            /* [defaultvalue][in] */ BSTR namespaceURI = 0) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_startMode( 
            /* [retval][out] */ BSTR *mode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_startModeURI( 
            /* [retval][out] */ BSTR *namespaceURI) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_output( 
            /* [in] */ VARIANT output) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_output( 
            /* [retval][out] */ VARIANT *pOutput) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE transform( 
            /* [retval][out] */ VARIANT_BOOL *pDone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_readyState( 
            /* [retval][out] */ long *pReadyState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE addParameter( 
            /* [in] */ BSTR baseName,
            /* [in] */ VARIANT parameter,
            /* [defaultvalue][in] */ BSTR namespaceURI = 0) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE addObject( 
            /* [in] */ IDispatch *obj,
            /* [in] */ BSTR namespaceURI) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_stylesheet( 
            /* [retval][out] */ IXMLDOMNode **stylesheet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXSLProcessorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXSLProcessor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXSLProcessor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXSLProcessor * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXSLProcessor * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXSLProcessor * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXSLProcessor * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXSLProcessor * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, put_input)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_input )( 
            IXSLProcessor * This,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_input)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_input )( 
            IXSLProcessor * This,
            /* [retval][out] */ VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_ownerTemplate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ownerTemplate )( 
            IXSLProcessor * This,
            /* [retval][out] */ IXSLTemplate **ppTemplate);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, setStartMode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setStartMode )( 
            IXSLProcessor * This,
            /* [in] */ BSTR mode,
            /* [defaultvalue][in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_startMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_startMode )( 
            IXSLProcessor * This,
            /* [retval][out] */ BSTR *mode);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_startModeURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_startModeURI )( 
            IXSLProcessor * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, put_output)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_output )( 
            IXSLProcessor * This,
            /* [in] */ VARIANT output);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_output)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_output )( 
            IXSLProcessor * This,
            /* [retval][out] */ VARIANT *pOutput);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, transform)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *transform )( 
            IXSLProcessor * This,
            /* [retval][out] */ VARIANT_BOOL *pDone);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IXSLProcessor * This);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            IXSLProcessor * This,
            /* [retval][out] */ long *pReadyState);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, addParameter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addParameter )( 
            IXSLProcessor * This,
            /* [in] */ BSTR baseName,
            /* [in] */ VARIANT parameter,
            /* [defaultvalue][in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, addObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addObject )( 
            IXSLProcessor * This,
            /* [in] */ IDispatch *obj,
            /* [in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IXSLProcessor, get_stylesheet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_stylesheet )( 
            IXSLProcessor * This,
            /* [retval][out] */ IXMLDOMNode **stylesheet);
        
        END_INTERFACE
    } IXSLProcessorVtbl;

    interface IXSLProcessor
    {
        CONST_VTBL struct IXSLProcessorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXSLProcessor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXSLProcessor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXSLProcessor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXSLProcessor_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXSLProcessor_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXSLProcessor_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXSLProcessor_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXSLProcessor_put_input(This,var)	\
    ( (This)->lpVtbl -> put_input(This,var) ) 

#define IXSLProcessor_get_input(This,pVar)	\
    ( (This)->lpVtbl -> get_input(This,pVar) ) 

#define IXSLProcessor_get_ownerTemplate(This,ppTemplate)	\
    ( (This)->lpVtbl -> get_ownerTemplate(This,ppTemplate) ) 

#define IXSLProcessor_setStartMode(This,mode,namespaceURI)	\
    ( (This)->lpVtbl -> setStartMode(This,mode,namespaceURI) ) 

#define IXSLProcessor_get_startMode(This,mode)	\
    ( (This)->lpVtbl -> get_startMode(This,mode) ) 

#define IXSLProcessor_get_startModeURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_startModeURI(This,namespaceURI) ) 

#define IXSLProcessor_put_output(This,output)	\
    ( (This)->lpVtbl -> put_output(This,output) ) 

#define IXSLProcessor_get_output(This,pOutput)	\
    ( (This)->lpVtbl -> get_output(This,pOutput) ) 

#define IXSLProcessor_transform(This,pDone)	\
    ( (This)->lpVtbl -> transform(This,pDone) ) 

#define IXSLProcessor_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IXSLProcessor_get_readyState(This,pReadyState)	\
    ( (This)->lpVtbl -> get_readyState(This,pReadyState) ) 

#define IXSLProcessor_addParameter(This,baseName,parameter,namespaceURI)	\
    ( (This)->lpVtbl -> addParameter(This,baseName,parameter,namespaceURI) ) 

#define IXSLProcessor_addObject(This,obj,namespaceURI)	\
    ( (This)->lpVtbl -> addObject(This,obj,namespaceURI) ) 

#define IXSLProcessor_get_stylesheet(This,stylesheet)	\
    ( (This)->lpVtbl -> get_stylesheet(This,stylesheet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXSLProcessor_INTERFACE_DEFINED__ */


#ifndef __IXSLTemplate_INTERFACE_DEFINED__
#define __IXSLTemplate_INTERFACE_DEFINED__

/* interface IXSLTemplate */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXSLTemplate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2933BF93-7B36-11d2-B20E-00C04F983E60")
    IXSLTemplate : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_stylesheet( 
            /* [in] */ IXMLDOMNode *stylesheet) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_stylesheet( 
            /* [retval][out] */ IXMLDOMNode **stylesheet) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createProcessor( 
            /* [retval][out] */ IXSLProcessor **ppProcessor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXSLTemplateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXSLTemplate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXSLTemplate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXSLTemplate * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXSLTemplate * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXSLTemplate * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXSLTemplate * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXSLTemplate * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXSLTemplate, putref_stylesheet)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_stylesheet )( 
            IXSLTemplate * This,
            /* [in] */ IXMLDOMNode *stylesheet);
        
        DECLSPEC_XFGVIRT(IXSLTemplate, get_stylesheet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_stylesheet )( 
            IXSLTemplate * This,
            /* [retval][out] */ IXMLDOMNode **stylesheet);
        
        DECLSPEC_XFGVIRT(IXSLTemplate, createProcessor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createProcessor )( 
            IXSLTemplate * This,
            /* [retval][out] */ IXSLProcessor **ppProcessor);
        
        END_INTERFACE
    } IXSLTemplateVtbl;

    interface IXSLTemplate
    {
        CONST_VTBL struct IXSLTemplateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXSLTemplate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXSLTemplate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXSLTemplate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXSLTemplate_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXSLTemplate_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXSLTemplate_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXSLTemplate_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXSLTemplate_putref_stylesheet(This,stylesheet)	\
    ( (This)->lpVtbl -> putref_stylesheet(This,stylesheet) ) 

#define IXSLTemplate_get_stylesheet(This,stylesheet)	\
    ( (This)->lpVtbl -> get_stylesheet(This,stylesheet) ) 

#define IXSLTemplate_createProcessor(This,ppProcessor)	\
    ( (This)->lpVtbl -> createProcessor(This,ppProcessor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXSLTemplate_INTERFACE_DEFINED__ */


#ifndef __IXMLHTTPRequest_INTERFACE_DEFINED__
#define __IXMLHTTPRequest_INTERFACE_DEFINED__

/* interface IXMLHTTPRequest */
/* [unique][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IXMLHTTPRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED8C108D-4349-11D2-91A4-00C04F7969E8")
    IXMLHTTPRequest : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE open( 
            /* [in] */ __RPC__in BSTR bstrMethod,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [optional][in] */ VARIANT varAsync,
            /* [optional][in] */ VARIANT bstrUser,
            /* [optional][in] */ VARIANT bstrPassword) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setRequestHeader( 
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [in] */ __RPC__in BSTR bstrValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getResponseHeader( 
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getAllResponseHeaders( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHeaders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE send( 
            /* [optional][in] */ VARIANT varBody) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE abort( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_status( 
            /* [retval][out] */ __RPC__out long *plStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_statusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_responseXML( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppBody) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_responseText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_responseBody( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_responseStream( 
            /* [retval][out] */ __RPC__out VARIANT *pvarBody) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_readyState( 
            /* [retval][out] */ __RPC__out long *plState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_onreadystatechange( 
            /* [in] */ __RPC__in_opt IDispatch *pReadyStateSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLHTTPRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLHTTPRequest * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, open)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *open )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrMethod,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [optional][in] */ VARIANT varAsync,
            /* [optional][in] */ VARIANT bstrUser,
            /* [optional][in] */ VARIANT bstrPassword);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, setRequestHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setRequestHeader )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getResponseHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getResponseHeader )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getAllResponseHeaders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getAllResponseHeaders )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHeaders);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, send)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *send )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [optional][in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            __RPC__in IXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_status )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out long *plStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_statusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusText )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseXML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseXML )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseText )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseBody)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseBody )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseStream )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out long *plState);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            __RPC__in IXMLHTTPRequest * This,
            /* [in] */ __RPC__in_opt IDispatch *pReadyStateSink);
        
        END_INTERFACE
    } IXMLHTTPRequestVtbl;

    interface IXMLHTTPRequest
    {
        CONST_VTBL struct IXMLHTTPRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLHTTPRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLHTTPRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLHTTPRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLHTTPRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLHTTPRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLHTTPRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLHTTPRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLHTTPRequest_open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword)	\
    ( (This)->lpVtbl -> open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword) ) 

#define IXMLHTTPRequest_setRequestHeader(This,bstrHeader,bstrValue)	\
    ( (This)->lpVtbl -> setRequestHeader(This,bstrHeader,bstrValue) ) 

#define IXMLHTTPRequest_getResponseHeader(This,bstrHeader,pbstrValue)	\
    ( (This)->lpVtbl -> getResponseHeader(This,bstrHeader,pbstrValue) ) 

#define IXMLHTTPRequest_getAllResponseHeaders(This,pbstrHeaders)	\
    ( (This)->lpVtbl -> getAllResponseHeaders(This,pbstrHeaders) ) 

#define IXMLHTTPRequest_send(This,varBody)	\
    ( (This)->lpVtbl -> send(This,varBody) ) 

#define IXMLHTTPRequest_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IXMLHTTPRequest_get_status(This,plStatus)	\
    ( (This)->lpVtbl -> get_status(This,plStatus) ) 

#define IXMLHTTPRequest_get_statusText(This,pbstrStatus)	\
    ( (This)->lpVtbl -> get_statusText(This,pbstrStatus) ) 

#define IXMLHTTPRequest_get_responseXML(This,ppBody)	\
    ( (This)->lpVtbl -> get_responseXML(This,ppBody) ) 

#define IXMLHTTPRequest_get_responseText(This,pbstrBody)	\
    ( (This)->lpVtbl -> get_responseText(This,pbstrBody) ) 

#define IXMLHTTPRequest_get_responseBody(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseBody(This,pvarBody) ) 

#define IXMLHTTPRequest_get_responseStream(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseStream(This,pvarBody) ) 

#define IXMLHTTPRequest_get_readyState(This,plState)	\
    ( (This)->lpVtbl -> get_readyState(This,plState) ) 

#define IXMLHTTPRequest_put_onreadystatechange(This,pReadyStateSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,pReadyStateSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLHTTPRequest_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msxml6_0000_0029 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if !defined(__msxml_sxh_enums__)
#define __msxml_sxh_enums__
typedef /* [helpstring] */ 
enum _SERVERXMLHTTP_OPTION
    {
        SXH_OPTION_URL	= -1,
        SXH_OPTION_URL_CODEPAGE	= ( SXH_OPTION_URL + 1 ) ,
        SXH_OPTION_ESCAPE_PERCENT_IN_URL	= ( SXH_OPTION_URL_CODEPAGE + 1 ) ,
        SXH_OPTION_IGNORE_SERVER_SSL_CERT_ERROR_FLAGS	= ( SXH_OPTION_ESCAPE_PERCENT_IN_URL + 1 ) ,
        SXH_OPTION_SELECT_CLIENT_SSL_CERT	= ( SXH_OPTION_IGNORE_SERVER_SSL_CERT_ERROR_FLAGS + 1 ) 
    } 	SERVERXMLHTTP_OPTION;

typedef /* [helpstring] */ 
enum _SXH_SERVER_CERT_OPTION
    {
        SXH_SERVER_CERT_IGNORE_UNKNOWN_CA	= 0x100,
        SXH_SERVER_CERT_IGNORE_WRONG_USAGE	= 0x200,
        SXH_SERVER_CERT_IGNORE_CERT_CN_INVALID	= 0x1000,
        SXH_SERVER_CERT_IGNORE_CERT_DATE_INVALID	= 0x2000,
        SXH_SERVER_CERT_IGNORE_ALL_SERVER_ERRORS	= ( ( ( SXH_SERVER_CERT_IGNORE_UNKNOWN_CA | SXH_SERVER_CERT_IGNORE_WRONG_USAGE )  | SXH_SERVER_CERT_IGNORE_CERT_CN_INVALID )  | SXH_SERVER_CERT_IGNORE_CERT_DATE_INVALID ) 
    } 	SXH_SERVER_CERT_OPTION;

typedef /* [helpstring] */ 
enum _SXH_PROXY_SETTING
    {
        SXH_PROXY_SET_DEFAULT	= 0,
        SXH_PROXY_SET_PRECONFIG	= 0,
        SXH_PROXY_SET_DIRECT	= 0x1,
        SXH_PROXY_SET_PROXY	= 0x2
    } 	SXH_PROXY_SETTING;

#endif // !defined(__msxml_sxh_enums__)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0029_v0_0_s_ifspec;

#ifndef __IServerXMLHTTPRequest_INTERFACE_DEFINED__
#define __IServerXMLHTTPRequest_INTERFACE_DEFINED__

/* interface IServerXMLHTTPRequest */
/* [unique][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IServerXMLHTTPRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2e9196bf-13ba-4dd4-91ca-6c571f281495")
    IServerXMLHTTPRequest : public IXMLHTTPRequest
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setTimeouts( 
            /* [in] */ long resolveTimeout,
            /* [in] */ long connectTimeout,
            /* [in] */ long sendTimeout,
            /* [in] */ long receiveTimeout) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE waitForResponse( 
            /* [optional][in] */ VARIANT timeoutInSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isSuccessful) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getOption( 
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [retval][out] */ __RPC__out VARIANT *value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setOption( 
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [in] */ VARIANT value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServerXMLHTTPRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServerXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServerXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IServerXMLHTTPRequest * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, open)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *open )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrMethod,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [optional][in] */ VARIANT varAsync,
            /* [optional][in] */ VARIANT bstrUser,
            /* [optional][in] */ VARIANT bstrPassword);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, setRequestHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setRequestHeader )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getResponseHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getResponseHeader )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getAllResponseHeaders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getAllResponseHeaders )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHeaders);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, send)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *send )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [optional][in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            __RPC__in IServerXMLHTTPRequest * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_status )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out long *plStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_statusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusText )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseXML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseXML )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseText )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseBody)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseBody )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseStream )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [retval][out] */ __RPC__out long *plState);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ __RPC__in_opt IDispatch *pReadyStateSink);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, setTimeouts)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setTimeouts )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ long resolveTimeout,
            /* [in] */ long connectTimeout,
            /* [in] */ long sendTimeout,
            /* [in] */ long receiveTimeout);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, waitForResponse)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *waitForResponse )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [optional][in] */ VARIANT timeoutInSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, getOption)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getOption )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [retval][out] */ __RPC__out VARIANT *value);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, setOption)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setOption )( 
            __RPC__in IServerXMLHTTPRequest * This,
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [in] */ VARIANT value);
        
        END_INTERFACE
    } IServerXMLHTTPRequestVtbl;

    interface IServerXMLHTTPRequest
    {
        CONST_VTBL struct IServerXMLHTTPRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServerXMLHTTPRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServerXMLHTTPRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServerXMLHTTPRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServerXMLHTTPRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IServerXMLHTTPRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IServerXMLHTTPRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IServerXMLHTTPRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IServerXMLHTTPRequest_open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword)	\
    ( (This)->lpVtbl -> open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword) ) 

#define IServerXMLHTTPRequest_setRequestHeader(This,bstrHeader,bstrValue)	\
    ( (This)->lpVtbl -> setRequestHeader(This,bstrHeader,bstrValue) ) 

#define IServerXMLHTTPRequest_getResponseHeader(This,bstrHeader,pbstrValue)	\
    ( (This)->lpVtbl -> getResponseHeader(This,bstrHeader,pbstrValue) ) 

#define IServerXMLHTTPRequest_getAllResponseHeaders(This,pbstrHeaders)	\
    ( (This)->lpVtbl -> getAllResponseHeaders(This,pbstrHeaders) ) 

#define IServerXMLHTTPRequest_send(This,varBody)	\
    ( (This)->lpVtbl -> send(This,varBody) ) 

#define IServerXMLHTTPRequest_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IServerXMLHTTPRequest_get_status(This,plStatus)	\
    ( (This)->lpVtbl -> get_status(This,plStatus) ) 

#define IServerXMLHTTPRequest_get_statusText(This,pbstrStatus)	\
    ( (This)->lpVtbl -> get_statusText(This,pbstrStatus) ) 

#define IServerXMLHTTPRequest_get_responseXML(This,ppBody)	\
    ( (This)->lpVtbl -> get_responseXML(This,ppBody) ) 

#define IServerXMLHTTPRequest_get_responseText(This,pbstrBody)	\
    ( (This)->lpVtbl -> get_responseText(This,pbstrBody) ) 

#define IServerXMLHTTPRequest_get_responseBody(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseBody(This,pvarBody) ) 

#define IServerXMLHTTPRequest_get_responseStream(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseStream(This,pvarBody) ) 

#define IServerXMLHTTPRequest_get_readyState(This,plState)	\
    ( (This)->lpVtbl -> get_readyState(This,plState) ) 

#define IServerXMLHTTPRequest_put_onreadystatechange(This,pReadyStateSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,pReadyStateSink) ) 


#define IServerXMLHTTPRequest_setTimeouts(This,resolveTimeout,connectTimeout,sendTimeout,receiveTimeout)	\
    ( (This)->lpVtbl -> setTimeouts(This,resolveTimeout,connectTimeout,sendTimeout,receiveTimeout) ) 

#define IServerXMLHTTPRequest_waitForResponse(This,timeoutInSeconds,isSuccessful)	\
    ( (This)->lpVtbl -> waitForResponse(This,timeoutInSeconds,isSuccessful) ) 

#define IServerXMLHTTPRequest_getOption(This,option,value)	\
    ( (This)->lpVtbl -> getOption(This,option,value) ) 

#define IServerXMLHTTPRequest_setOption(This,option,value)	\
    ( (This)->lpVtbl -> setOption(This,option,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServerXMLHTTPRequest_INTERFACE_DEFINED__ */


#ifndef __IServerXMLHTTPRequest2_INTERFACE_DEFINED__
#define __IServerXMLHTTPRequest2_INTERFACE_DEFINED__

/* interface IServerXMLHTTPRequest2 */
/* [unique][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IServerXMLHTTPRequest2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2e01311b-c322-4b0a-bd77-b90cfdc8dce7")
    IServerXMLHTTPRequest2 : public IServerXMLHTTPRequest
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setProxy( 
            /* [in] */ SXH_PROXY_SETTING proxySetting,
            /* [optional][in] */ VARIANT varProxyServer,
            /* [optional][in] */ VARIANT varBypassList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setProxyCredentials( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrPassword) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServerXMLHTTPRequest2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServerXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServerXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IServerXMLHTTPRequest2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, open)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *open )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in BSTR bstrMethod,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [optional][in] */ VARIANT varAsync,
            /* [optional][in] */ VARIANT bstrUser,
            /* [optional][in] */ VARIANT bstrPassword);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, setRequestHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setRequestHeader )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getResponseHeader)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getResponseHeader )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in BSTR bstrHeader,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, getAllResponseHeaders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getAllResponseHeaders )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHeaders);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, send)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *send )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [optional][in] */ VARIANT varBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            __RPC__in IServerXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_status )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__out long *plStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_statusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_statusText )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseXML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseXML )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseText )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseBody)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseBody )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_responseStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_responseStream )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, get_readyState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_readyState )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [retval][out] */ __RPC__out long *plState);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest, put_onreadystatechange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_onreadystatechange )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in_opt IDispatch *pReadyStateSink);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, setTimeouts)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setTimeouts )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ long resolveTimeout,
            /* [in] */ long connectTimeout,
            /* [in] */ long sendTimeout,
            /* [in] */ long receiveTimeout);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, waitForResponse)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *waitForResponse )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [optional][in] */ VARIANT timeoutInSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *isSuccessful);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, getOption)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getOption )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [retval][out] */ __RPC__out VARIANT *value);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest, setOption)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setOption )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ SERVERXMLHTTP_OPTION option,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest2, setProxy)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setProxy )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ SXH_PROXY_SETTING proxySetting,
            /* [optional][in] */ VARIANT varProxyServer,
            /* [optional][in] */ VARIANT varBypassList);
        
        DECLSPEC_XFGVIRT(IServerXMLHTTPRequest2, setProxyCredentials)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setProxyCredentials )( 
            __RPC__in IServerXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrPassword);
        
        END_INTERFACE
    } IServerXMLHTTPRequest2Vtbl;

    interface IServerXMLHTTPRequest2
    {
        CONST_VTBL struct IServerXMLHTTPRequest2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServerXMLHTTPRequest2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServerXMLHTTPRequest2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServerXMLHTTPRequest2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServerXMLHTTPRequest2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IServerXMLHTTPRequest2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IServerXMLHTTPRequest2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IServerXMLHTTPRequest2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IServerXMLHTTPRequest2_open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword)	\
    ( (This)->lpVtbl -> open(This,bstrMethod,bstrUrl,varAsync,bstrUser,bstrPassword) ) 

#define IServerXMLHTTPRequest2_setRequestHeader(This,bstrHeader,bstrValue)	\
    ( (This)->lpVtbl -> setRequestHeader(This,bstrHeader,bstrValue) ) 

#define IServerXMLHTTPRequest2_getResponseHeader(This,bstrHeader,pbstrValue)	\
    ( (This)->lpVtbl -> getResponseHeader(This,bstrHeader,pbstrValue) ) 

#define IServerXMLHTTPRequest2_getAllResponseHeaders(This,pbstrHeaders)	\
    ( (This)->lpVtbl -> getAllResponseHeaders(This,pbstrHeaders) ) 

#define IServerXMLHTTPRequest2_send(This,varBody)	\
    ( (This)->lpVtbl -> send(This,varBody) ) 

#define IServerXMLHTTPRequest2_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IServerXMLHTTPRequest2_get_status(This,plStatus)	\
    ( (This)->lpVtbl -> get_status(This,plStatus) ) 

#define IServerXMLHTTPRequest2_get_statusText(This,pbstrStatus)	\
    ( (This)->lpVtbl -> get_statusText(This,pbstrStatus) ) 

#define IServerXMLHTTPRequest2_get_responseXML(This,ppBody)	\
    ( (This)->lpVtbl -> get_responseXML(This,ppBody) ) 

#define IServerXMLHTTPRequest2_get_responseText(This,pbstrBody)	\
    ( (This)->lpVtbl -> get_responseText(This,pbstrBody) ) 

#define IServerXMLHTTPRequest2_get_responseBody(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseBody(This,pvarBody) ) 

#define IServerXMLHTTPRequest2_get_responseStream(This,pvarBody)	\
    ( (This)->lpVtbl -> get_responseStream(This,pvarBody) ) 

#define IServerXMLHTTPRequest2_get_readyState(This,plState)	\
    ( (This)->lpVtbl -> get_readyState(This,plState) ) 

#define IServerXMLHTTPRequest2_put_onreadystatechange(This,pReadyStateSink)	\
    ( (This)->lpVtbl -> put_onreadystatechange(This,pReadyStateSink) ) 


#define IServerXMLHTTPRequest2_setTimeouts(This,resolveTimeout,connectTimeout,sendTimeout,receiveTimeout)	\
    ( (This)->lpVtbl -> setTimeouts(This,resolveTimeout,connectTimeout,sendTimeout,receiveTimeout) ) 

#define IServerXMLHTTPRequest2_waitForResponse(This,timeoutInSeconds,isSuccessful)	\
    ( (This)->lpVtbl -> waitForResponse(This,timeoutInSeconds,isSuccessful) ) 

#define IServerXMLHTTPRequest2_getOption(This,option,value)	\
    ( (This)->lpVtbl -> getOption(This,option,value) ) 

#define IServerXMLHTTPRequest2_setOption(This,option,value)	\
    ( (This)->lpVtbl -> setOption(This,option,value) ) 


#define IServerXMLHTTPRequest2_setProxy(This,proxySetting,varProxyServer,varBypassList)	\
    ( (This)->lpVtbl -> setProxy(This,proxySetting,varProxyServer,varBypassList) ) 

#define IServerXMLHTTPRequest2_setProxyCredentials(This,bstrUserName,bstrPassword)	\
    ( (This)->lpVtbl -> setProxyCredentials(This,bstrUserName,bstrPassword) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServerXMLHTTPRequest2_INTERFACE_DEFINED__ */


#ifndef __ISAXXMLReader_INTERFACE_DEFINED__
#define __ISAXXMLReader_INTERFACE_DEFINED__

/* interface ISAXXMLReader */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXXMLReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a4f96ed0-f829-476e-81c0-cdc7bd2a0802")
    ISAXXMLReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE getFeature( 
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT_BOOL *pvfValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putFeature( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT_BOOL vfValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getProperty( 
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putProperty( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT varValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getEntityResolver( 
            /* [retval][out] */ ISAXEntityResolver **ppResolver) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putEntityResolver( 
            /* [in] */ ISAXEntityResolver *pResolver) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getContentHandler( 
            /* [retval][out] */ ISAXContentHandler **ppHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putContentHandler( 
            /* [in] */ ISAXContentHandler *pHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getDTDHandler( 
            /* [retval][out] */ ISAXDTDHandler **ppHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putDTDHandler( 
            /* [in] */ ISAXDTDHandler *pHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getErrorHandler( 
            /* [retval][out] */ ISAXErrorHandler **ppHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putErrorHandler( 
            /* [in] */ ISAXErrorHandler *pHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getBaseURL( 
            /* [retval][out] */ const wchar_t **ppwchBaseUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putBaseURL( 
            /* [in] */ const wchar_t *pwchBaseUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getSecureBaseURL( 
            /* [retval][out] */ const wchar_t **ppwchSecureBaseUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putSecureBaseURL( 
            /* [in] */ const wchar_t *pwchSecureBaseUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE parse( 
            /* [optional][in] */ VARIANT varInput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE parseURL( 
            /* [in] */ const wchar_t *pwchUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXXMLReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXXMLReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXXMLReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXXMLReader * This);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getFeature)
        HRESULT ( STDMETHODCALLTYPE *getFeature )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT_BOOL *pvfValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putFeature)
        HRESULT ( STDMETHODCALLTYPE *putFeature )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT_BOOL vfValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getProperty)
        HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putProperty)
        HRESULT ( STDMETHODCALLTYPE *putProperty )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getEntityResolver)
        HRESULT ( STDMETHODCALLTYPE *getEntityResolver )( 
            ISAXXMLReader * This,
            /* [retval][out] */ ISAXEntityResolver **ppResolver);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putEntityResolver)
        HRESULT ( STDMETHODCALLTYPE *putEntityResolver )( 
            ISAXXMLReader * This,
            /* [in] */ ISAXEntityResolver *pResolver);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getContentHandler)
        HRESULT ( STDMETHODCALLTYPE *getContentHandler )( 
            ISAXXMLReader * This,
            /* [retval][out] */ ISAXContentHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putContentHandler)
        HRESULT ( STDMETHODCALLTYPE *putContentHandler )( 
            ISAXXMLReader * This,
            /* [in] */ ISAXContentHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getDTDHandler)
        HRESULT ( STDMETHODCALLTYPE *getDTDHandler )( 
            ISAXXMLReader * This,
            /* [retval][out] */ ISAXDTDHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putDTDHandler)
        HRESULT ( STDMETHODCALLTYPE *putDTDHandler )( 
            ISAXXMLReader * This,
            /* [in] */ ISAXDTDHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getErrorHandler)
        HRESULT ( STDMETHODCALLTYPE *getErrorHandler )( 
            ISAXXMLReader * This,
            /* [retval][out] */ ISAXErrorHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putErrorHandler)
        HRESULT ( STDMETHODCALLTYPE *putErrorHandler )( 
            ISAXXMLReader * This,
            /* [in] */ ISAXErrorHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getBaseURL)
        HRESULT ( STDMETHODCALLTYPE *getBaseURL )( 
            ISAXXMLReader * This,
            /* [retval][out] */ const wchar_t **ppwchBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putBaseURL)
        HRESULT ( STDMETHODCALLTYPE *putBaseURL )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getSecureBaseURL)
        HRESULT ( STDMETHODCALLTYPE *getSecureBaseURL )( 
            ISAXXMLReader * This,
            /* [retval][out] */ const wchar_t **ppwchSecureBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putSecureBaseURL)
        HRESULT ( STDMETHODCALLTYPE *putSecureBaseURL )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchSecureBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, parse)
        HRESULT ( STDMETHODCALLTYPE *parse )( 
            ISAXXMLReader * This,
            /* [optional][in] */ VARIANT varInput);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, parseURL)
        HRESULT ( STDMETHODCALLTYPE *parseURL )( 
            ISAXXMLReader * This,
            /* [in] */ const wchar_t *pwchUrl);
        
        END_INTERFACE
    } ISAXXMLReaderVtbl;

    interface ISAXXMLReader
    {
        CONST_VTBL struct ISAXXMLReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXXMLReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXXMLReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXXMLReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXXMLReader_getFeature(This,pwchName,pvfValue)	\
    ( (This)->lpVtbl -> getFeature(This,pwchName,pvfValue) ) 

#define ISAXXMLReader_putFeature(This,pwchName,vfValue)	\
    ( (This)->lpVtbl -> putFeature(This,pwchName,vfValue) ) 

#define ISAXXMLReader_getProperty(This,pwchName,pvarValue)	\
    ( (This)->lpVtbl -> getProperty(This,pwchName,pvarValue) ) 

#define ISAXXMLReader_putProperty(This,pwchName,varValue)	\
    ( (This)->lpVtbl -> putProperty(This,pwchName,varValue) ) 

#define ISAXXMLReader_getEntityResolver(This,ppResolver)	\
    ( (This)->lpVtbl -> getEntityResolver(This,ppResolver) ) 

#define ISAXXMLReader_putEntityResolver(This,pResolver)	\
    ( (This)->lpVtbl -> putEntityResolver(This,pResolver) ) 

#define ISAXXMLReader_getContentHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getContentHandler(This,ppHandler) ) 

#define ISAXXMLReader_putContentHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putContentHandler(This,pHandler) ) 

#define ISAXXMLReader_getDTDHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getDTDHandler(This,ppHandler) ) 

#define ISAXXMLReader_putDTDHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putDTDHandler(This,pHandler) ) 

#define ISAXXMLReader_getErrorHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getErrorHandler(This,ppHandler) ) 

#define ISAXXMLReader_putErrorHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putErrorHandler(This,pHandler) ) 

#define ISAXXMLReader_getBaseURL(This,ppwchBaseUrl)	\
    ( (This)->lpVtbl -> getBaseURL(This,ppwchBaseUrl) ) 

#define ISAXXMLReader_putBaseURL(This,pwchBaseUrl)	\
    ( (This)->lpVtbl -> putBaseURL(This,pwchBaseUrl) ) 

#define ISAXXMLReader_getSecureBaseURL(This,ppwchSecureBaseUrl)	\
    ( (This)->lpVtbl -> getSecureBaseURL(This,ppwchSecureBaseUrl) ) 

#define ISAXXMLReader_putSecureBaseURL(This,pwchSecureBaseUrl)	\
    ( (This)->lpVtbl -> putSecureBaseURL(This,pwchSecureBaseUrl) ) 

#define ISAXXMLReader_parse(This,varInput)	\
    ( (This)->lpVtbl -> parse(This,varInput) ) 

#define ISAXXMLReader_parseURL(This,pwchUrl)	\
    ( (This)->lpVtbl -> parseURL(This,pwchUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXXMLReader_INTERFACE_DEFINED__ */


#ifndef __ISAXXMLFilter_INTERFACE_DEFINED__
#define __ISAXXMLFilter_INTERFACE_DEFINED__

/* interface ISAXXMLFilter */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXXMLFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70409222-ca09-4475-acb8-40312fe8d145")
    ISAXXMLFilter : public ISAXXMLReader
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE getParent( 
            /* [retval][out] */ ISAXXMLReader **ppReader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE putParent( 
            /* [in] */ ISAXXMLReader *pReader) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXXMLFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXXMLFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getFeature)
        HRESULT ( STDMETHODCALLTYPE *getFeature )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT_BOOL *pvfValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putFeature)
        HRESULT ( STDMETHODCALLTYPE *putFeature )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT_BOOL vfValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getProperty)
        HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchName,
            /* [retval][out] */ VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putProperty)
        HRESULT ( STDMETHODCALLTYPE *putProperty )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getEntityResolver)
        HRESULT ( STDMETHODCALLTYPE *getEntityResolver )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ ISAXEntityResolver **ppResolver);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putEntityResolver)
        HRESULT ( STDMETHODCALLTYPE *putEntityResolver )( 
            ISAXXMLFilter * This,
            /* [in] */ ISAXEntityResolver *pResolver);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getContentHandler)
        HRESULT ( STDMETHODCALLTYPE *getContentHandler )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ ISAXContentHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putContentHandler)
        HRESULT ( STDMETHODCALLTYPE *putContentHandler )( 
            ISAXXMLFilter * This,
            /* [in] */ ISAXContentHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getDTDHandler)
        HRESULT ( STDMETHODCALLTYPE *getDTDHandler )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ ISAXDTDHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putDTDHandler)
        HRESULT ( STDMETHODCALLTYPE *putDTDHandler )( 
            ISAXXMLFilter * This,
            /* [in] */ ISAXDTDHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getErrorHandler)
        HRESULT ( STDMETHODCALLTYPE *getErrorHandler )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ ISAXErrorHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putErrorHandler)
        HRESULT ( STDMETHODCALLTYPE *putErrorHandler )( 
            ISAXXMLFilter * This,
            /* [in] */ ISAXErrorHandler *pHandler);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getBaseURL)
        HRESULT ( STDMETHODCALLTYPE *getBaseURL )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ const wchar_t **ppwchBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putBaseURL)
        HRESULT ( STDMETHODCALLTYPE *putBaseURL )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, getSecureBaseURL)
        HRESULT ( STDMETHODCALLTYPE *getSecureBaseURL )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ const wchar_t **ppwchSecureBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, putSecureBaseURL)
        HRESULT ( STDMETHODCALLTYPE *putSecureBaseURL )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchSecureBaseUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, parse)
        HRESULT ( STDMETHODCALLTYPE *parse )( 
            ISAXXMLFilter * This,
            /* [optional][in] */ VARIANT varInput);
        
        DECLSPEC_XFGVIRT(ISAXXMLReader, parseURL)
        HRESULT ( STDMETHODCALLTYPE *parseURL )( 
            ISAXXMLFilter * This,
            /* [in] */ const wchar_t *pwchUrl);
        
        DECLSPEC_XFGVIRT(ISAXXMLFilter, getParent)
        HRESULT ( STDMETHODCALLTYPE *getParent )( 
            ISAXXMLFilter * This,
            /* [retval][out] */ ISAXXMLReader **ppReader);
        
        DECLSPEC_XFGVIRT(ISAXXMLFilter, putParent)
        HRESULT ( STDMETHODCALLTYPE *putParent )( 
            ISAXXMLFilter * This,
            /* [in] */ ISAXXMLReader *pReader);
        
        END_INTERFACE
    } ISAXXMLFilterVtbl;

    interface ISAXXMLFilter
    {
        CONST_VTBL struct ISAXXMLFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXXMLFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXXMLFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXXMLFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXXMLFilter_getFeature(This,pwchName,pvfValue)	\
    ( (This)->lpVtbl -> getFeature(This,pwchName,pvfValue) ) 

#define ISAXXMLFilter_putFeature(This,pwchName,vfValue)	\
    ( (This)->lpVtbl -> putFeature(This,pwchName,vfValue) ) 

#define ISAXXMLFilter_getProperty(This,pwchName,pvarValue)	\
    ( (This)->lpVtbl -> getProperty(This,pwchName,pvarValue) ) 

#define ISAXXMLFilter_putProperty(This,pwchName,varValue)	\
    ( (This)->lpVtbl -> putProperty(This,pwchName,varValue) ) 

#define ISAXXMLFilter_getEntityResolver(This,ppResolver)	\
    ( (This)->lpVtbl -> getEntityResolver(This,ppResolver) ) 

#define ISAXXMLFilter_putEntityResolver(This,pResolver)	\
    ( (This)->lpVtbl -> putEntityResolver(This,pResolver) ) 

#define ISAXXMLFilter_getContentHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getContentHandler(This,ppHandler) ) 

#define ISAXXMLFilter_putContentHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putContentHandler(This,pHandler) ) 

#define ISAXXMLFilter_getDTDHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getDTDHandler(This,ppHandler) ) 

#define ISAXXMLFilter_putDTDHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putDTDHandler(This,pHandler) ) 

#define ISAXXMLFilter_getErrorHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> getErrorHandler(This,ppHandler) ) 

#define ISAXXMLFilter_putErrorHandler(This,pHandler)	\
    ( (This)->lpVtbl -> putErrorHandler(This,pHandler) ) 

#define ISAXXMLFilter_getBaseURL(This,ppwchBaseUrl)	\
    ( (This)->lpVtbl -> getBaseURL(This,ppwchBaseUrl) ) 

#define ISAXXMLFilter_putBaseURL(This,pwchBaseUrl)	\
    ( (This)->lpVtbl -> putBaseURL(This,pwchBaseUrl) ) 

#define ISAXXMLFilter_getSecureBaseURL(This,ppwchSecureBaseUrl)	\
    ( (This)->lpVtbl -> getSecureBaseURL(This,ppwchSecureBaseUrl) ) 

#define ISAXXMLFilter_putSecureBaseURL(This,pwchSecureBaseUrl)	\
    ( (This)->lpVtbl -> putSecureBaseURL(This,pwchSecureBaseUrl) ) 

#define ISAXXMLFilter_parse(This,varInput)	\
    ( (This)->lpVtbl -> parse(This,varInput) ) 

#define ISAXXMLFilter_parseURL(This,pwchUrl)	\
    ( (This)->lpVtbl -> parseURL(This,pwchUrl) ) 


#define ISAXXMLFilter_getParent(This,ppReader)	\
    ( (This)->lpVtbl -> getParent(This,ppReader) ) 

#define ISAXXMLFilter_putParent(This,pReader)	\
    ( (This)->lpVtbl -> putParent(This,pReader) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXXMLFilter_INTERFACE_DEFINED__ */


#ifndef __ISAXLocator_INTERFACE_DEFINED__
#define __ISAXLocator_INTERFACE_DEFINED__

/* interface ISAXLocator */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9b7e472a-0de4-4640-bff3-84d38a051c31")
    ISAXLocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE getColumnNumber( 
            /* [retval][out] */ int *pnColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getLineNumber( 
            /* [retval][out] */ int *pnLine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getPublicId( 
            /* [retval][out] */ const wchar_t **ppwchPublicId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getSystemId( 
            /* [retval][out] */ const wchar_t **ppwchSystemId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXLocator * This);
        
        DECLSPEC_XFGVIRT(ISAXLocator, getColumnNumber)
        HRESULT ( STDMETHODCALLTYPE *getColumnNumber )( 
            ISAXLocator * This,
            /* [retval][out] */ int *pnColumn);
        
        DECLSPEC_XFGVIRT(ISAXLocator, getLineNumber)
        HRESULT ( STDMETHODCALLTYPE *getLineNumber )( 
            ISAXLocator * This,
            /* [retval][out] */ int *pnLine);
        
        DECLSPEC_XFGVIRT(ISAXLocator, getPublicId)
        HRESULT ( STDMETHODCALLTYPE *getPublicId )( 
            ISAXLocator * This,
            /* [retval][out] */ const wchar_t **ppwchPublicId);
        
        DECLSPEC_XFGVIRT(ISAXLocator, getSystemId)
        HRESULT ( STDMETHODCALLTYPE *getSystemId )( 
            ISAXLocator * This,
            /* [retval][out] */ const wchar_t **ppwchSystemId);
        
        END_INTERFACE
    } ISAXLocatorVtbl;

    interface ISAXLocator
    {
        CONST_VTBL struct ISAXLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXLocator_getColumnNumber(This,pnColumn)	\
    ( (This)->lpVtbl -> getColumnNumber(This,pnColumn) ) 

#define ISAXLocator_getLineNumber(This,pnLine)	\
    ( (This)->lpVtbl -> getLineNumber(This,pnLine) ) 

#define ISAXLocator_getPublicId(This,ppwchPublicId)	\
    ( (This)->lpVtbl -> getPublicId(This,ppwchPublicId) ) 

#define ISAXLocator_getSystemId(This,ppwchSystemId)	\
    ( (This)->lpVtbl -> getSystemId(This,ppwchSystemId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXLocator_INTERFACE_DEFINED__ */


#ifndef __ISAXEntityResolver_INTERFACE_DEFINED__
#define __ISAXEntityResolver_INTERFACE_DEFINED__

/* interface ISAXEntityResolver */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXEntityResolver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("99bca7bd-e8c4-4d5f-a0cf-6d907901ff07")
    ISAXEntityResolver : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE resolveEntity( 
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [retval][out] */ VARIANT *pvarInput) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXEntityResolverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXEntityResolver * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXEntityResolver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXEntityResolver * This);
        
        DECLSPEC_XFGVIRT(ISAXEntityResolver, resolveEntity)
        HRESULT ( STDMETHODCALLTYPE *resolveEntity )( 
            ISAXEntityResolver * This,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [retval][out] */ VARIANT *pvarInput);
        
        END_INTERFACE
    } ISAXEntityResolverVtbl;

    interface ISAXEntityResolver
    {
        CONST_VTBL struct ISAXEntityResolverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXEntityResolver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXEntityResolver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXEntityResolver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXEntityResolver_resolveEntity(This,pwchPublicId,pwchSystemId,pvarInput)	\
    ( (This)->lpVtbl -> resolveEntity(This,pwchPublicId,pwchSystemId,pvarInput) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXEntityResolver_INTERFACE_DEFINED__ */


#ifndef __ISAXContentHandler_INTERFACE_DEFINED__
#define __ISAXContentHandler_INTERFACE_DEFINED__

/* interface ISAXContentHandler */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXContentHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1545cdfa-9e4e-4497-a8a4-2bf7d0112c44")
    ISAXContentHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE putDocumentLocator( 
            /* [in] */ ISAXLocator *pLocator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startDocument( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endDocument( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startPrefixMapping( 
            /* [in] */ const wchar_t *pwchPrefix,
            /* [in] */ int cchPrefix,
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endPrefixMapping( 
            /* [in] */ const wchar_t *pwchPrefix,
            /* [in] */ int cchPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startElement( 
            /* [in] */ const wchar_t *pwchNamespaceUri,
            /* [in] */ int cchNamespaceUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [in] */ ISAXAttributes *pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endElement( 
            /* [in] */ const wchar_t *pwchNamespaceUri,
            /* [in] */ int cchNamespaceUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE characters( 
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ignorableWhitespace( 
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE processingInstruction( 
            /* [in] */ const wchar_t *pwchTarget,
            /* [in] */ int cchTarget,
            /* [in] */ const wchar_t *pwchData,
            /* [in] */ int cchData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE skippedEntity( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXContentHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXContentHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, putDocumentLocator)
        HRESULT ( STDMETHODCALLTYPE *putDocumentLocator )( 
            ISAXContentHandler * This,
            /* [in] */ ISAXLocator *pLocator);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, startDocument)
        HRESULT ( STDMETHODCALLTYPE *startDocument )( 
            ISAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, endDocument)
        HRESULT ( STDMETHODCALLTYPE *endDocument )( 
            ISAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, startPrefixMapping)
        HRESULT ( STDMETHODCALLTYPE *startPrefixMapping )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchPrefix,
            /* [in] */ int cchPrefix,
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, endPrefixMapping)
        HRESULT ( STDMETHODCALLTYPE *endPrefixMapping )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchPrefix,
            /* [in] */ int cchPrefix);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, startElement)
        HRESULT ( STDMETHODCALLTYPE *startElement )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchNamespaceUri,
            /* [in] */ int cchNamespaceUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [in] */ ISAXAttributes *pAttributes);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, endElement)
        HRESULT ( STDMETHODCALLTYPE *endElement )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchNamespaceUri,
            /* [in] */ int cchNamespaceUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, characters)
        HRESULT ( STDMETHODCALLTYPE *characters )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, ignorableWhitespace)
        HRESULT ( STDMETHODCALLTYPE *ignorableWhitespace )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, processingInstruction)
        HRESULT ( STDMETHODCALLTYPE *processingInstruction )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchTarget,
            /* [in] */ int cchTarget,
            /* [in] */ const wchar_t *pwchData,
            /* [in] */ int cchData);
        
        DECLSPEC_XFGVIRT(ISAXContentHandler, skippedEntity)
        HRESULT ( STDMETHODCALLTYPE *skippedEntity )( 
            ISAXContentHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName);
        
        END_INTERFACE
    } ISAXContentHandlerVtbl;

    interface ISAXContentHandler
    {
        CONST_VTBL struct ISAXContentHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXContentHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXContentHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXContentHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXContentHandler_putDocumentLocator(This,pLocator)	\
    ( (This)->lpVtbl -> putDocumentLocator(This,pLocator) ) 

#define ISAXContentHandler_startDocument(This)	\
    ( (This)->lpVtbl -> startDocument(This) ) 

#define ISAXContentHandler_endDocument(This)	\
    ( (This)->lpVtbl -> endDocument(This) ) 

#define ISAXContentHandler_startPrefixMapping(This,pwchPrefix,cchPrefix,pwchUri,cchUri)	\
    ( (This)->lpVtbl -> startPrefixMapping(This,pwchPrefix,cchPrefix,pwchUri,cchUri) ) 

#define ISAXContentHandler_endPrefixMapping(This,pwchPrefix,cchPrefix)	\
    ( (This)->lpVtbl -> endPrefixMapping(This,pwchPrefix,cchPrefix) ) 

#define ISAXContentHandler_startElement(This,pwchNamespaceUri,cchNamespaceUri,pwchLocalName,cchLocalName,pwchQName,cchQName,pAttributes)	\
    ( (This)->lpVtbl -> startElement(This,pwchNamespaceUri,cchNamespaceUri,pwchLocalName,cchLocalName,pwchQName,cchQName,pAttributes) ) 

#define ISAXContentHandler_endElement(This,pwchNamespaceUri,cchNamespaceUri,pwchLocalName,cchLocalName,pwchQName,cchQName)	\
    ( (This)->lpVtbl -> endElement(This,pwchNamespaceUri,cchNamespaceUri,pwchLocalName,cchLocalName,pwchQName,cchQName) ) 

#define ISAXContentHandler_characters(This,pwchChars,cchChars)	\
    ( (This)->lpVtbl -> characters(This,pwchChars,cchChars) ) 

#define ISAXContentHandler_ignorableWhitespace(This,pwchChars,cchChars)	\
    ( (This)->lpVtbl -> ignorableWhitespace(This,pwchChars,cchChars) ) 

#define ISAXContentHandler_processingInstruction(This,pwchTarget,cchTarget,pwchData,cchData)	\
    ( (This)->lpVtbl -> processingInstruction(This,pwchTarget,cchTarget,pwchData,cchData) ) 

#define ISAXContentHandler_skippedEntity(This,pwchName,cchName)	\
    ( (This)->lpVtbl -> skippedEntity(This,pwchName,cchName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXContentHandler_INTERFACE_DEFINED__ */


#ifndef __ISAXDTDHandler_INTERFACE_DEFINED__
#define __ISAXDTDHandler_INTERFACE_DEFINED__

/* interface ISAXDTDHandler */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXDTDHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e15c1baf-afb3-4d60-8c36-19a8c45defed")
    ISAXDTDHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE notationDecl( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE unparsedEntityDecl( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId,
            /* [in] */ const wchar_t *pwchNotationName,
            /* [in] */ int cchNotationName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXDTDHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXDTDHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXDTDHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXDTDHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXDTDHandler, notationDecl)
        HRESULT ( STDMETHODCALLTYPE *notationDecl )( 
            ISAXDTDHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId);
        
        DECLSPEC_XFGVIRT(ISAXDTDHandler, unparsedEntityDecl)
        HRESULT ( STDMETHODCALLTYPE *unparsedEntityDecl )( 
            ISAXDTDHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId,
            /* [in] */ const wchar_t *pwchNotationName,
            /* [in] */ int cchNotationName);
        
        END_INTERFACE
    } ISAXDTDHandlerVtbl;

    interface ISAXDTDHandler
    {
        CONST_VTBL struct ISAXDTDHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXDTDHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXDTDHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXDTDHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXDTDHandler_notationDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId)	\
    ( (This)->lpVtbl -> notationDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId) ) 

#define ISAXDTDHandler_unparsedEntityDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId,pwchNotationName,cchNotationName)	\
    ( (This)->lpVtbl -> unparsedEntityDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId,pwchNotationName,cchNotationName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXDTDHandler_INTERFACE_DEFINED__ */


#ifndef __ISAXErrorHandler_INTERFACE_DEFINED__
#define __ISAXErrorHandler_INTERFACE_DEFINED__

/* interface ISAXErrorHandler */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXErrorHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a60511c4-ccf5-479e-98a3-dc8dc545b7d0")
    ISAXErrorHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE error( 
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE fatalError( 
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ignorableWarning( 
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXErrorHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXErrorHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXErrorHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXErrorHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXErrorHandler, error)
        HRESULT ( STDMETHODCALLTYPE *error )( 
            ISAXErrorHandler * This,
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode);
        
        DECLSPEC_XFGVIRT(ISAXErrorHandler, fatalError)
        HRESULT ( STDMETHODCALLTYPE *fatalError )( 
            ISAXErrorHandler * This,
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode);
        
        DECLSPEC_XFGVIRT(ISAXErrorHandler, ignorableWarning)
        HRESULT ( STDMETHODCALLTYPE *ignorableWarning )( 
            ISAXErrorHandler * This,
            /* [in] */ ISAXLocator *pLocator,
            /* [in] */ const wchar_t *pwchErrorMessage,
            /* [in] */ HRESULT hrErrorCode);
        
        END_INTERFACE
    } ISAXErrorHandlerVtbl;

    interface ISAXErrorHandler
    {
        CONST_VTBL struct ISAXErrorHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXErrorHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXErrorHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXErrorHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXErrorHandler_error(This,pLocator,pwchErrorMessage,hrErrorCode)	\
    ( (This)->lpVtbl -> error(This,pLocator,pwchErrorMessage,hrErrorCode) ) 

#define ISAXErrorHandler_fatalError(This,pLocator,pwchErrorMessage,hrErrorCode)	\
    ( (This)->lpVtbl -> fatalError(This,pLocator,pwchErrorMessage,hrErrorCode) ) 

#define ISAXErrorHandler_ignorableWarning(This,pLocator,pwchErrorMessage,hrErrorCode)	\
    ( (This)->lpVtbl -> ignorableWarning(This,pLocator,pwchErrorMessage,hrErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXErrorHandler_INTERFACE_DEFINED__ */


#ifndef __ISAXLexicalHandler_INTERFACE_DEFINED__
#define __ISAXLexicalHandler_INTERFACE_DEFINED__

/* interface ISAXLexicalHandler */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXLexicalHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7f85d5f5-47a8-4497-bda5-84ba04819ea6")
    ISAXLexicalHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE startDTD( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endDTD( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startEntity( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endEntity( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startCDATA( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE endCDATA( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE comment( 
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXLexicalHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXLexicalHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, startDTD)
        HRESULT ( STDMETHODCALLTYPE *startDTD )( 
            ISAXLexicalHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, endDTD)
        HRESULT ( STDMETHODCALLTYPE *endDTD )( 
            ISAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, startEntity)
        HRESULT ( STDMETHODCALLTYPE *startEntity )( 
            ISAXLexicalHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, endEntity)
        HRESULT ( STDMETHODCALLTYPE *endEntity )( 
            ISAXLexicalHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, startCDATA)
        HRESULT ( STDMETHODCALLTYPE *startCDATA )( 
            ISAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, endCDATA)
        HRESULT ( STDMETHODCALLTYPE *endCDATA )( 
            ISAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXLexicalHandler, comment)
        HRESULT ( STDMETHODCALLTYPE *comment )( 
            ISAXLexicalHandler * This,
            /* [in] */ const wchar_t *pwchChars,
            /* [in] */ int cchChars);
        
        END_INTERFACE
    } ISAXLexicalHandlerVtbl;

    interface ISAXLexicalHandler
    {
        CONST_VTBL struct ISAXLexicalHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXLexicalHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXLexicalHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXLexicalHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXLexicalHandler_startDTD(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId)	\
    ( (This)->lpVtbl -> startDTD(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId) ) 

#define ISAXLexicalHandler_endDTD(This)	\
    ( (This)->lpVtbl -> endDTD(This) ) 

#define ISAXLexicalHandler_startEntity(This,pwchName,cchName)	\
    ( (This)->lpVtbl -> startEntity(This,pwchName,cchName) ) 

#define ISAXLexicalHandler_endEntity(This,pwchName,cchName)	\
    ( (This)->lpVtbl -> endEntity(This,pwchName,cchName) ) 

#define ISAXLexicalHandler_startCDATA(This)	\
    ( (This)->lpVtbl -> startCDATA(This) ) 

#define ISAXLexicalHandler_endCDATA(This)	\
    ( (This)->lpVtbl -> endCDATA(This) ) 

#define ISAXLexicalHandler_comment(This,pwchChars,cchChars)	\
    ( (This)->lpVtbl -> comment(This,pwchChars,cchChars) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXLexicalHandler_INTERFACE_DEFINED__ */


#ifndef __ISAXDeclHandler_INTERFACE_DEFINED__
#define __ISAXDeclHandler_INTERFACE_DEFINED__

/* interface ISAXDeclHandler */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXDeclHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("862629ac-771a-47b2-8337-4e6843c1be90")
    ISAXDeclHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE elementDecl( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchModel,
            /* [in] */ int cchModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE attributeDecl( 
            /* [in] */ const wchar_t *pwchElementName,
            /* [in] */ int cchElementName,
            /* [in] */ const wchar_t *pwchAttributeName,
            /* [in] */ int cchAttributeName,
            /* [in] */ const wchar_t *pwchType,
            /* [in] */ int cchType,
            /* [in] */ const wchar_t *pwchValueDefault,
            /* [in] */ int cchValueDefault,
            /* [in] */ const wchar_t *pwchValue,
            /* [in] */ int cchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE internalEntityDecl( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchValue,
            /* [in] */ int cchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE externalEntityDecl( 
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXDeclHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXDeclHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXDeclHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXDeclHandler * This);
        
        DECLSPEC_XFGVIRT(ISAXDeclHandler, elementDecl)
        HRESULT ( STDMETHODCALLTYPE *elementDecl )( 
            ISAXDeclHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchModel,
            /* [in] */ int cchModel);
        
        DECLSPEC_XFGVIRT(ISAXDeclHandler, attributeDecl)
        HRESULT ( STDMETHODCALLTYPE *attributeDecl )( 
            ISAXDeclHandler * This,
            /* [in] */ const wchar_t *pwchElementName,
            /* [in] */ int cchElementName,
            /* [in] */ const wchar_t *pwchAttributeName,
            /* [in] */ int cchAttributeName,
            /* [in] */ const wchar_t *pwchType,
            /* [in] */ int cchType,
            /* [in] */ const wchar_t *pwchValueDefault,
            /* [in] */ int cchValueDefault,
            /* [in] */ const wchar_t *pwchValue,
            /* [in] */ int cchValue);
        
        DECLSPEC_XFGVIRT(ISAXDeclHandler, internalEntityDecl)
        HRESULT ( STDMETHODCALLTYPE *internalEntityDecl )( 
            ISAXDeclHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchValue,
            /* [in] */ int cchValue);
        
        DECLSPEC_XFGVIRT(ISAXDeclHandler, externalEntityDecl)
        HRESULT ( STDMETHODCALLTYPE *externalEntityDecl )( 
            ISAXDeclHandler * This,
            /* [in] */ const wchar_t *pwchName,
            /* [in] */ int cchName,
            /* [in] */ const wchar_t *pwchPublicId,
            /* [in] */ int cchPublicId,
            /* [in] */ const wchar_t *pwchSystemId,
            /* [in] */ int cchSystemId);
        
        END_INTERFACE
    } ISAXDeclHandlerVtbl;

    interface ISAXDeclHandler
    {
        CONST_VTBL struct ISAXDeclHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXDeclHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXDeclHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXDeclHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXDeclHandler_elementDecl(This,pwchName,cchName,pwchModel,cchModel)	\
    ( (This)->lpVtbl -> elementDecl(This,pwchName,cchName,pwchModel,cchModel) ) 

#define ISAXDeclHandler_attributeDecl(This,pwchElementName,cchElementName,pwchAttributeName,cchAttributeName,pwchType,cchType,pwchValueDefault,cchValueDefault,pwchValue,cchValue)	\
    ( (This)->lpVtbl -> attributeDecl(This,pwchElementName,cchElementName,pwchAttributeName,cchAttributeName,pwchType,cchType,pwchValueDefault,cchValueDefault,pwchValue,cchValue) ) 

#define ISAXDeclHandler_internalEntityDecl(This,pwchName,cchName,pwchValue,cchValue)	\
    ( (This)->lpVtbl -> internalEntityDecl(This,pwchName,cchName,pwchValue,cchValue) ) 

#define ISAXDeclHandler_externalEntityDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId)	\
    ( (This)->lpVtbl -> externalEntityDecl(This,pwchName,cchName,pwchPublicId,cchPublicId,pwchSystemId,cchSystemId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXDeclHandler_INTERFACE_DEFINED__ */


#ifndef __ISAXAttributes_INTERFACE_DEFINED__
#define __ISAXAttributes_INTERFACE_DEFINED__

/* interface ISAXAttributes */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_ISAXAttributes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f078abe1-45d2-4832-91ea-4466ce2f25c9")
    ISAXAttributes : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE getLength( 
            /* [retval][out] */ int *pnLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getURI( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchUri,
            /* [out] */ int *pcchUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getLocalName( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchLocalName,
            /* [out] */ int *pcchLocalName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getQName( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchQName,
            /* [out] */ int *pcchQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getName( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchUri,
            /* [out] */ int *pcchUri,
            /* [out] */ const wchar_t **ppwchLocalName,
            /* [out] */ int *pcchLocalName,
            /* [out] */ const wchar_t **ppwchQName,
            /* [out] */ int *pcchQName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getIndexFromName( 
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [retval][out] */ int *pnIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getIndexFromQName( 
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [retval][out] */ int *pnIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getType( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getTypeFromName( 
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getTypeFromQName( 
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getValue( 
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getValueFromName( 
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getValueFromQName( 
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISAXAttributesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISAXAttributes * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISAXAttributes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISAXAttributes * This);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getLength)
        HRESULT ( STDMETHODCALLTYPE *getLength )( 
            ISAXAttributes * This,
            /* [retval][out] */ int *pnLength);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getURI)
        HRESULT ( STDMETHODCALLTYPE *getURI )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchUri,
            /* [out] */ int *pcchUri);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getLocalName)
        HRESULT ( STDMETHODCALLTYPE *getLocalName )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchLocalName,
            /* [out] */ int *pcchLocalName);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getQName)
        HRESULT ( STDMETHODCALLTYPE *getQName )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchQName,
            /* [out] */ int *pcchQName);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getName)
        HRESULT ( STDMETHODCALLTYPE *getName )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchUri,
            /* [out] */ int *pcchUri,
            /* [out] */ const wchar_t **ppwchLocalName,
            /* [out] */ int *pcchLocalName,
            /* [out] */ const wchar_t **ppwchQName,
            /* [out] */ int *pcchQName);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getIndexFromName)
        HRESULT ( STDMETHODCALLTYPE *getIndexFromName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [retval][out] */ int *pnIndex);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getIndexFromQName)
        HRESULT ( STDMETHODCALLTYPE *getIndexFromQName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [retval][out] */ int *pnIndex);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getType)
        HRESULT ( STDMETHODCALLTYPE *getType )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getTypeFromName)
        HRESULT ( STDMETHODCALLTYPE *getTypeFromName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getTypeFromQName)
        HRESULT ( STDMETHODCALLTYPE *getTypeFromQName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [out] */ const wchar_t **ppwchType,
            /* [out] */ int *pcchType);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getValue)
        HRESULT ( STDMETHODCALLTYPE *getValue )( 
            ISAXAttributes * This,
            /* [in] */ int nIndex,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getValueFromName)
        HRESULT ( STDMETHODCALLTYPE *getValueFromName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchUri,
            /* [in] */ int cchUri,
            /* [in] */ const wchar_t *pwchLocalName,
            /* [in] */ int cchLocalName,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue);
        
        DECLSPEC_XFGVIRT(ISAXAttributes, getValueFromQName)
        HRESULT ( STDMETHODCALLTYPE *getValueFromQName )( 
            ISAXAttributes * This,
            /* [in] */ const wchar_t *pwchQName,
            /* [in] */ int cchQName,
            /* [out] */ const wchar_t **ppwchValue,
            /* [out] */ int *pcchValue);
        
        END_INTERFACE
    } ISAXAttributesVtbl;

    interface ISAXAttributes
    {
        CONST_VTBL struct ISAXAttributesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISAXAttributes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISAXAttributes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISAXAttributes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISAXAttributes_getLength(This,pnLength)	\
    ( (This)->lpVtbl -> getLength(This,pnLength) ) 

#define ISAXAttributes_getURI(This,nIndex,ppwchUri,pcchUri)	\
    ( (This)->lpVtbl -> getURI(This,nIndex,ppwchUri,pcchUri) ) 

#define ISAXAttributes_getLocalName(This,nIndex,ppwchLocalName,pcchLocalName)	\
    ( (This)->lpVtbl -> getLocalName(This,nIndex,ppwchLocalName,pcchLocalName) ) 

#define ISAXAttributes_getQName(This,nIndex,ppwchQName,pcchQName)	\
    ( (This)->lpVtbl -> getQName(This,nIndex,ppwchQName,pcchQName) ) 

#define ISAXAttributes_getName(This,nIndex,ppwchUri,pcchUri,ppwchLocalName,pcchLocalName,ppwchQName,pcchQName)	\
    ( (This)->lpVtbl -> getName(This,nIndex,ppwchUri,pcchUri,ppwchLocalName,pcchLocalName,ppwchQName,pcchQName) ) 

#define ISAXAttributes_getIndexFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,pnIndex)	\
    ( (This)->lpVtbl -> getIndexFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,pnIndex) ) 

#define ISAXAttributes_getIndexFromQName(This,pwchQName,cchQName,pnIndex)	\
    ( (This)->lpVtbl -> getIndexFromQName(This,pwchQName,cchQName,pnIndex) ) 

#define ISAXAttributes_getType(This,nIndex,ppwchType,pcchType)	\
    ( (This)->lpVtbl -> getType(This,nIndex,ppwchType,pcchType) ) 

#define ISAXAttributes_getTypeFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,ppwchType,pcchType)	\
    ( (This)->lpVtbl -> getTypeFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,ppwchType,pcchType) ) 

#define ISAXAttributes_getTypeFromQName(This,pwchQName,cchQName,ppwchType,pcchType)	\
    ( (This)->lpVtbl -> getTypeFromQName(This,pwchQName,cchQName,ppwchType,pcchType) ) 

#define ISAXAttributes_getValue(This,nIndex,ppwchValue,pcchValue)	\
    ( (This)->lpVtbl -> getValue(This,nIndex,ppwchValue,pcchValue) ) 

#define ISAXAttributes_getValueFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,ppwchValue,pcchValue)	\
    ( (This)->lpVtbl -> getValueFromName(This,pwchUri,cchUri,pwchLocalName,cchLocalName,ppwchValue,pcchValue) ) 

#define ISAXAttributes_getValueFromQName(This,pwchQName,cchQName,ppwchValue,pcchValue)	\
    ( (This)->lpVtbl -> getValueFromQName(This,pwchQName,cchQName,ppwchValue,pcchValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISAXAttributes_INTERFACE_DEFINED__ */


#ifndef __IVBSAXXMLReader_INTERFACE_DEFINED__
#define __IVBSAXXMLReader_INTERFACE_DEFINED__

/* interface IVBSAXXMLReader */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXXMLReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8c033caa-6cd6-4f73-b728-4531af74945f")
    IVBSAXXMLReader : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getFeature( 
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT_BOOL *fValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE putFeature( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT_BOOL fValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getProperty( 
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT *varValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE putProperty( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT varValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_entityResolver( 
            /* [retval][out] */ IVBSAXEntityResolver **oResolver) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_entityResolver( 
            /* [in] */ IVBSAXEntityResolver *oResolver) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_contentHandler( 
            /* [retval][out] */ IVBSAXContentHandler **oHandler) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_contentHandler( 
            /* [in] */ IVBSAXContentHandler *oHandler) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_dtdHandler( 
            /* [retval][out] */ IVBSAXDTDHandler **oHandler) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_dtdHandler( 
            /* [in] */ IVBSAXDTDHandler *oHandler) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_errorHandler( 
            /* [retval][out] */ IVBSAXErrorHandler **oHandler) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_errorHandler( 
            /* [in] */ IVBSAXErrorHandler *oHandler) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_baseURL( 
            /* [retval][out] */ BSTR *strBaseURL) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_baseURL( 
            /* [in] */ BSTR strBaseURL) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_secureBaseURL( 
            /* [retval][out] */ BSTR *strSecureBaseURL) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_secureBaseURL( 
            /* [in] */ BSTR strSecureBaseURL) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE parse( 
            /* [optional][in] */ VARIANT varInput) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE parseURL( 
            /* [in] */ BSTR strURL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXXMLReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXXMLReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXXMLReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXXMLReader * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXXMLReader * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXXMLReader * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXXMLReader * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXXMLReader * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, getFeature)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getFeature )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT_BOOL *fValue);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putFeature)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *putFeature )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT_BOOL fValue);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, getProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *putProperty )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_entityResolver)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_entityResolver )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ IVBSAXEntityResolver **oResolver);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putref_entityResolver)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_entityResolver )( 
            IVBSAXXMLReader * This,
            /* [in] */ IVBSAXEntityResolver *oResolver);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_contentHandler)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_contentHandler )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ IVBSAXContentHandler **oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putref_contentHandler)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_contentHandler )( 
            IVBSAXXMLReader * This,
            /* [in] */ IVBSAXContentHandler *oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_dtdHandler)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dtdHandler )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ IVBSAXDTDHandler **oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putref_dtdHandler)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_dtdHandler )( 
            IVBSAXXMLReader * This,
            /* [in] */ IVBSAXDTDHandler *oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_errorHandler)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_errorHandler )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ IVBSAXErrorHandler **oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, putref_errorHandler)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_errorHandler )( 
            IVBSAXXMLReader * This,
            /* [in] */ IVBSAXErrorHandler *oHandler);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_baseURL)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_baseURL )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ BSTR *strBaseURL);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, put_baseURL)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_baseURL )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strBaseURL);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, get_secureBaseURL)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_secureBaseURL )( 
            IVBSAXXMLReader * This,
            /* [retval][out] */ BSTR *strSecureBaseURL);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, put_secureBaseURL)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_secureBaseURL )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strSecureBaseURL);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, parse)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *parse )( 
            IVBSAXXMLReader * This,
            /* [optional][in] */ VARIANT varInput);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLReader, parseURL)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *parseURL )( 
            IVBSAXXMLReader * This,
            /* [in] */ BSTR strURL);
        
        END_INTERFACE
    } IVBSAXXMLReaderVtbl;

    interface IVBSAXXMLReader
    {
        CONST_VTBL struct IVBSAXXMLReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXXMLReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXXMLReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXXMLReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXXMLReader_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXXMLReader_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXXMLReader_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXXMLReader_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXXMLReader_getFeature(This,strName,fValue)	\
    ( (This)->lpVtbl -> getFeature(This,strName,fValue) ) 

#define IVBSAXXMLReader_putFeature(This,strName,fValue)	\
    ( (This)->lpVtbl -> putFeature(This,strName,fValue) ) 

#define IVBSAXXMLReader_getProperty(This,strName,varValue)	\
    ( (This)->lpVtbl -> getProperty(This,strName,varValue) ) 

#define IVBSAXXMLReader_putProperty(This,strName,varValue)	\
    ( (This)->lpVtbl -> putProperty(This,strName,varValue) ) 

#define IVBSAXXMLReader_get_entityResolver(This,oResolver)	\
    ( (This)->lpVtbl -> get_entityResolver(This,oResolver) ) 

#define IVBSAXXMLReader_putref_entityResolver(This,oResolver)	\
    ( (This)->lpVtbl -> putref_entityResolver(This,oResolver) ) 

#define IVBSAXXMLReader_get_contentHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_contentHandler(This,oHandler) ) 

#define IVBSAXXMLReader_putref_contentHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_contentHandler(This,oHandler) ) 

#define IVBSAXXMLReader_get_dtdHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_dtdHandler(This,oHandler) ) 

#define IVBSAXXMLReader_putref_dtdHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_dtdHandler(This,oHandler) ) 

#define IVBSAXXMLReader_get_errorHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_errorHandler(This,oHandler) ) 

#define IVBSAXXMLReader_putref_errorHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_errorHandler(This,oHandler) ) 

#define IVBSAXXMLReader_get_baseURL(This,strBaseURL)	\
    ( (This)->lpVtbl -> get_baseURL(This,strBaseURL) ) 

#define IVBSAXXMLReader_put_baseURL(This,strBaseURL)	\
    ( (This)->lpVtbl -> put_baseURL(This,strBaseURL) ) 

#define IVBSAXXMLReader_get_secureBaseURL(This,strSecureBaseURL)	\
    ( (This)->lpVtbl -> get_secureBaseURL(This,strSecureBaseURL) ) 

#define IVBSAXXMLReader_put_secureBaseURL(This,strSecureBaseURL)	\
    ( (This)->lpVtbl -> put_secureBaseURL(This,strSecureBaseURL) ) 

#define IVBSAXXMLReader_parse(This,varInput)	\
    ( (This)->lpVtbl -> parse(This,varInput) ) 

#define IVBSAXXMLReader_parseURL(This,strURL)	\
    ( (This)->lpVtbl -> parseURL(This,strURL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXXMLReader_INTERFACE_DEFINED__ */


#ifndef __IVBSAXXMLFilter_INTERFACE_DEFINED__
#define __IVBSAXXMLFilter_INTERFACE_DEFINED__

/* interface IVBSAXXMLFilter */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXXMLFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1299eb1b-5b88-433e-82de-82ca75ad4e04")
    IVBSAXXMLFilter : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_parent( 
            /* [retval][out] */ IVBSAXXMLReader **oReader) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_parent( 
            /* [in] */ IVBSAXXMLReader *oReader) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXXMLFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXXMLFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXXMLFilter * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXXMLFilter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXXMLFilter * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXXMLFilter * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLFilter, get_parent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_parent )( 
            IVBSAXXMLFilter * This,
            /* [retval][out] */ IVBSAXXMLReader **oReader);
        
        DECLSPEC_XFGVIRT(IVBSAXXMLFilter, putref_parent)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_parent )( 
            IVBSAXXMLFilter * This,
            /* [in] */ IVBSAXXMLReader *oReader);
        
        END_INTERFACE
    } IVBSAXXMLFilterVtbl;

    interface IVBSAXXMLFilter
    {
        CONST_VTBL struct IVBSAXXMLFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXXMLFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXXMLFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXXMLFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXXMLFilter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXXMLFilter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXXMLFilter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXXMLFilter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXXMLFilter_get_parent(This,oReader)	\
    ( (This)->lpVtbl -> get_parent(This,oReader) ) 

#define IVBSAXXMLFilter_putref_parent(This,oReader)	\
    ( (This)->lpVtbl -> putref_parent(This,oReader) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXXMLFilter_INTERFACE_DEFINED__ */


#ifndef __IVBSAXLocator_INTERFACE_DEFINED__
#define __IVBSAXLocator_INTERFACE_DEFINED__

/* interface IVBSAXLocator */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("796e7ac5-5aa2-4eff-acad-3faaf01a3288")
    IVBSAXLocator : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_columnNumber( 
            /* [retval][out] */ int *nColumn) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lineNumber( 
            /* [retval][out] */ int *nLine) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_publicId( 
            /* [retval][out] */ BSTR *strPublicId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_systemId( 
            /* [retval][out] */ BSTR *strSystemId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXLocator * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXLocator * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXLocator * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXLocator, get_columnNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_columnNumber )( 
            IVBSAXLocator * This,
            /* [retval][out] */ int *nColumn);
        
        DECLSPEC_XFGVIRT(IVBSAXLocator, get_lineNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lineNumber )( 
            IVBSAXLocator * This,
            /* [retval][out] */ int *nLine);
        
        DECLSPEC_XFGVIRT(IVBSAXLocator, get_publicId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_publicId )( 
            IVBSAXLocator * This,
            /* [retval][out] */ BSTR *strPublicId);
        
        DECLSPEC_XFGVIRT(IVBSAXLocator, get_systemId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_systemId )( 
            IVBSAXLocator * This,
            /* [retval][out] */ BSTR *strSystemId);
        
        END_INTERFACE
    } IVBSAXLocatorVtbl;

    interface IVBSAXLocator
    {
        CONST_VTBL struct IVBSAXLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXLocator_get_columnNumber(This,nColumn)	\
    ( (This)->lpVtbl -> get_columnNumber(This,nColumn) ) 

#define IVBSAXLocator_get_lineNumber(This,nLine)	\
    ( (This)->lpVtbl -> get_lineNumber(This,nLine) ) 

#define IVBSAXLocator_get_publicId(This,strPublicId)	\
    ( (This)->lpVtbl -> get_publicId(This,strPublicId) ) 

#define IVBSAXLocator_get_systemId(This,strSystemId)	\
    ( (This)->lpVtbl -> get_systemId(This,strSystemId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXLocator_INTERFACE_DEFINED__ */


#ifndef __IVBSAXEntityResolver_INTERFACE_DEFINED__
#define __IVBSAXEntityResolver_INTERFACE_DEFINED__

/* interface IVBSAXEntityResolver */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXEntityResolver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c05d096-f45b-4aca-ad1a-aa0bc25518dc")
    IVBSAXEntityResolver : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE resolveEntity( 
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId,
            /* [retval][out] */ VARIANT *varInput) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXEntityResolverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXEntityResolver * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXEntityResolver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXEntityResolver * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXEntityResolver * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXEntityResolver * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXEntityResolver * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXEntityResolver * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXEntityResolver, resolveEntity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *resolveEntity )( 
            IVBSAXEntityResolver * This,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId,
            /* [retval][out] */ VARIANT *varInput);
        
        END_INTERFACE
    } IVBSAXEntityResolverVtbl;

    interface IVBSAXEntityResolver
    {
        CONST_VTBL struct IVBSAXEntityResolverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXEntityResolver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXEntityResolver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXEntityResolver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXEntityResolver_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXEntityResolver_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXEntityResolver_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXEntityResolver_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXEntityResolver_resolveEntity(This,strPublicId,strSystemId,varInput)	\
    ( (This)->lpVtbl -> resolveEntity(This,strPublicId,strSystemId,varInput) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXEntityResolver_INTERFACE_DEFINED__ */


#ifndef __IVBSAXContentHandler_INTERFACE_DEFINED__
#define __IVBSAXContentHandler_INTERFACE_DEFINED__

/* interface IVBSAXContentHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXContentHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2ed7290a-4dd5-4b46-bb26-4e4155e77faa")
    IVBSAXContentHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_documentLocator( 
            /* [in] */ IVBSAXLocator *oLocator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startDocument( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endDocument( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startPrefixMapping( 
            /* [out][in] */ BSTR *strPrefix,
            /* [out][in] */ BSTR *strURI) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endPrefixMapping( 
            /* [out][in] */ BSTR *strPrefix) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startElement( 
            /* [out][in] */ BSTR *strNamespaceURI,
            /* [out][in] */ BSTR *strLocalName,
            /* [out][in] */ BSTR *strQName,
            /* [in] */ IVBSAXAttributes *oAttributes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endElement( 
            /* [out][in] */ BSTR *strNamespaceURI,
            /* [out][in] */ BSTR *strLocalName,
            /* [out][in] */ BSTR *strQName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE characters( 
            /* [out][in] */ BSTR *strChars) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ignorableWhitespace( 
            /* [out][in] */ BSTR *strChars) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE processingInstruction( 
            /* [out][in] */ BSTR *strTarget,
            /* [out][in] */ BSTR *strData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE skippedEntity( 
            /* [out][in] */ BSTR *strName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXContentHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXContentHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXContentHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXContentHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXContentHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXContentHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, putref_documentLocator)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_documentLocator )( 
            IVBSAXContentHandler * This,
            /* [in] */ IVBSAXLocator *oLocator);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, startDocument)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startDocument )( 
            IVBSAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, endDocument)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endDocument )( 
            IVBSAXContentHandler * This);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, startPrefixMapping)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startPrefixMapping )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strPrefix,
            /* [out][in] */ BSTR *strURI);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, endPrefixMapping)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endPrefixMapping )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strPrefix);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, startElement)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startElement )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strNamespaceURI,
            /* [out][in] */ BSTR *strLocalName,
            /* [out][in] */ BSTR *strQName,
            /* [in] */ IVBSAXAttributes *oAttributes);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, endElement)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endElement )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strNamespaceURI,
            /* [out][in] */ BSTR *strLocalName,
            /* [out][in] */ BSTR *strQName);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, characters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *characters )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strChars);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, ignorableWhitespace)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ignorableWhitespace )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strChars);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, processingInstruction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *processingInstruction )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strTarget,
            /* [out][in] */ BSTR *strData);
        
        DECLSPEC_XFGVIRT(IVBSAXContentHandler, skippedEntity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *skippedEntity )( 
            IVBSAXContentHandler * This,
            /* [out][in] */ BSTR *strName);
        
        END_INTERFACE
    } IVBSAXContentHandlerVtbl;

    interface IVBSAXContentHandler
    {
        CONST_VTBL struct IVBSAXContentHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXContentHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXContentHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXContentHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXContentHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXContentHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXContentHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXContentHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXContentHandler_putref_documentLocator(This,oLocator)	\
    ( (This)->lpVtbl -> putref_documentLocator(This,oLocator) ) 

#define IVBSAXContentHandler_startDocument(This)	\
    ( (This)->lpVtbl -> startDocument(This) ) 

#define IVBSAXContentHandler_endDocument(This)	\
    ( (This)->lpVtbl -> endDocument(This) ) 

#define IVBSAXContentHandler_startPrefixMapping(This,strPrefix,strURI)	\
    ( (This)->lpVtbl -> startPrefixMapping(This,strPrefix,strURI) ) 

#define IVBSAXContentHandler_endPrefixMapping(This,strPrefix)	\
    ( (This)->lpVtbl -> endPrefixMapping(This,strPrefix) ) 

#define IVBSAXContentHandler_startElement(This,strNamespaceURI,strLocalName,strQName,oAttributes)	\
    ( (This)->lpVtbl -> startElement(This,strNamespaceURI,strLocalName,strQName,oAttributes) ) 

#define IVBSAXContentHandler_endElement(This,strNamespaceURI,strLocalName,strQName)	\
    ( (This)->lpVtbl -> endElement(This,strNamespaceURI,strLocalName,strQName) ) 

#define IVBSAXContentHandler_characters(This,strChars)	\
    ( (This)->lpVtbl -> characters(This,strChars) ) 

#define IVBSAXContentHandler_ignorableWhitespace(This,strChars)	\
    ( (This)->lpVtbl -> ignorableWhitespace(This,strChars) ) 

#define IVBSAXContentHandler_processingInstruction(This,strTarget,strData)	\
    ( (This)->lpVtbl -> processingInstruction(This,strTarget,strData) ) 

#define IVBSAXContentHandler_skippedEntity(This,strName)	\
    ( (This)->lpVtbl -> skippedEntity(This,strName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXContentHandler_INTERFACE_DEFINED__ */


#ifndef __IVBSAXDTDHandler_INTERFACE_DEFINED__
#define __IVBSAXDTDHandler_INTERFACE_DEFINED__

/* interface IVBSAXDTDHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXDTDHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24fb3297-302d-4620-ba39-3a732d850558")
    IVBSAXDTDHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE notationDecl( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE unparsedEntityDecl( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId,
            /* [out][in] */ BSTR *strNotationName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXDTDHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXDTDHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXDTDHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXDTDHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXDTDHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXDTDHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXDTDHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXDTDHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXDTDHandler, notationDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *notationDecl )( 
            IVBSAXDTDHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId);
        
        DECLSPEC_XFGVIRT(IVBSAXDTDHandler, unparsedEntityDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *unparsedEntityDecl )( 
            IVBSAXDTDHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId,
            /* [out][in] */ BSTR *strNotationName);
        
        END_INTERFACE
    } IVBSAXDTDHandlerVtbl;

    interface IVBSAXDTDHandler
    {
        CONST_VTBL struct IVBSAXDTDHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXDTDHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXDTDHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXDTDHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXDTDHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXDTDHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXDTDHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXDTDHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXDTDHandler_notationDecl(This,strName,strPublicId,strSystemId)	\
    ( (This)->lpVtbl -> notationDecl(This,strName,strPublicId,strSystemId) ) 

#define IVBSAXDTDHandler_unparsedEntityDecl(This,strName,strPublicId,strSystemId,strNotationName)	\
    ( (This)->lpVtbl -> unparsedEntityDecl(This,strName,strPublicId,strSystemId,strNotationName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXDTDHandler_INTERFACE_DEFINED__ */


#ifndef __IVBSAXErrorHandler_INTERFACE_DEFINED__
#define __IVBSAXErrorHandler_INTERFACE_DEFINED__

/* interface IVBSAXErrorHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXErrorHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d963d3fe-173c-4862-9095-b92f66995f52")
    IVBSAXErrorHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE error( 
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE fatalError( 
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ignorableWarning( 
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXErrorHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXErrorHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXErrorHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXErrorHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXErrorHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXErrorHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXErrorHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXErrorHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXErrorHandler, error)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *error )( 
            IVBSAXErrorHandler * This,
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode);
        
        DECLSPEC_XFGVIRT(IVBSAXErrorHandler, fatalError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *fatalError )( 
            IVBSAXErrorHandler * This,
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode);
        
        DECLSPEC_XFGVIRT(IVBSAXErrorHandler, ignorableWarning)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ignorableWarning )( 
            IVBSAXErrorHandler * This,
            /* [in] */ IVBSAXLocator *oLocator,
            /* [out][in] */ BSTR *strErrorMessage,
            /* [in] */ long nErrorCode);
        
        END_INTERFACE
    } IVBSAXErrorHandlerVtbl;

    interface IVBSAXErrorHandler
    {
        CONST_VTBL struct IVBSAXErrorHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXErrorHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXErrorHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXErrorHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXErrorHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXErrorHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXErrorHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXErrorHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXErrorHandler_error(This,oLocator,strErrorMessage,nErrorCode)	\
    ( (This)->lpVtbl -> error(This,oLocator,strErrorMessage,nErrorCode) ) 

#define IVBSAXErrorHandler_fatalError(This,oLocator,strErrorMessage,nErrorCode)	\
    ( (This)->lpVtbl -> fatalError(This,oLocator,strErrorMessage,nErrorCode) ) 

#define IVBSAXErrorHandler_ignorableWarning(This,oLocator,strErrorMessage,nErrorCode)	\
    ( (This)->lpVtbl -> ignorableWarning(This,oLocator,strErrorMessage,nErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXErrorHandler_INTERFACE_DEFINED__ */


#ifndef __IVBSAXLexicalHandler_INTERFACE_DEFINED__
#define __IVBSAXLexicalHandler_INTERFACE_DEFINED__

/* interface IVBSAXLexicalHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXLexicalHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("032aac35-8c0e-4d9d-979f-e3b702935576")
    IVBSAXLexicalHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startDTD( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endDTD( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startEntity( 
            /* [out][in] */ BSTR *strName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endEntity( 
            /* [out][in] */ BSTR *strName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startCDATA( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE endCDATA( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE comment( 
            /* [out][in] */ BSTR *strChars) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXLexicalHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXLexicalHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXLexicalHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXLexicalHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXLexicalHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXLexicalHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, startDTD)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startDTD )( 
            IVBSAXLexicalHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, endDTD)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endDTD )( 
            IVBSAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, startEntity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startEntity )( 
            IVBSAXLexicalHandler * This,
            /* [out][in] */ BSTR *strName);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, endEntity)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endEntity )( 
            IVBSAXLexicalHandler * This,
            /* [out][in] */ BSTR *strName);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, startCDATA)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startCDATA )( 
            IVBSAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, endCDATA)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *endCDATA )( 
            IVBSAXLexicalHandler * This);
        
        DECLSPEC_XFGVIRT(IVBSAXLexicalHandler, comment)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *comment )( 
            IVBSAXLexicalHandler * This,
            /* [out][in] */ BSTR *strChars);
        
        END_INTERFACE
    } IVBSAXLexicalHandlerVtbl;

    interface IVBSAXLexicalHandler
    {
        CONST_VTBL struct IVBSAXLexicalHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXLexicalHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXLexicalHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXLexicalHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXLexicalHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXLexicalHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXLexicalHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXLexicalHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXLexicalHandler_startDTD(This,strName,strPublicId,strSystemId)	\
    ( (This)->lpVtbl -> startDTD(This,strName,strPublicId,strSystemId) ) 

#define IVBSAXLexicalHandler_endDTD(This)	\
    ( (This)->lpVtbl -> endDTD(This) ) 

#define IVBSAXLexicalHandler_startEntity(This,strName)	\
    ( (This)->lpVtbl -> startEntity(This,strName) ) 

#define IVBSAXLexicalHandler_endEntity(This,strName)	\
    ( (This)->lpVtbl -> endEntity(This,strName) ) 

#define IVBSAXLexicalHandler_startCDATA(This)	\
    ( (This)->lpVtbl -> startCDATA(This) ) 

#define IVBSAXLexicalHandler_endCDATA(This)	\
    ( (This)->lpVtbl -> endCDATA(This) ) 

#define IVBSAXLexicalHandler_comment(This,strChars)	\
    ( (This)->lpVtbl -> comment(This,strChars) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXLexicalHandler_INTERFACE_DEFINED__ */


#ifndef __IVBSAXDeclHandler_INTERFACE_DEFINED__
#define __IVBSAXDeclHandler_INTERFACE_DEFINED__

/* interface IVBSAXDeclHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXDeclHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e8917260-7579-4be1-b5dd-7afbfa6f077b")
    IVBSAXDeclHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE elementDecl( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strModel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE attributeDecl( 
            /* [out][in] */ BSTR *strElementName,
            /* [out][in] */ BSTR *strAttributeName,
            /* [out][in] */ BSTR *strType,
            /* [out][in] */ BSTR *strValueDefault,
            /* [out][in] */ BSTR *strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE internalEntityDecl( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE externalEntityDecl( 
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXDeclHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXDeclHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXDeclHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXDeclHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXDeclHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXDeclHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXDeclHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXDeclHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXDeclHandler, elementDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *elementDecl )( 
            IVBSAXDeclHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strModel);
        
        DECLSPEC_XFGVIRT(IVBSAXDeclHandler, attributeDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *attributeDecl )( 
            IVBSAXDeclHandler * This,
            /* [out][in] */ BSTR *strElementName,
            /* [out][in] */ BSTR *strAttributeName,
            /* [out][in] */ BSTR *strType,
            /* [out][in] */ BSTR *strValueDefault,
            /* [out][in] */ BSTR *strValue);
        
        DECLSPEC_XFGVIRT(IVBSAXDeclHandler, internalEntityDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *internalEntityDecl )( 
            IVBSAXDeclHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strValue);
        
        DECLSPEC_XFGVIRT(IVBSAXDeclHandler, externalEntityDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *externalEntityDecl )( 
            IVBSAXDeclHandler * This,
            /* [out][in] */ BSTR *strName,
            /* [out][in] */ BSTR *strPublicId,
            /* [out][in] */ BSTR *strSystemId);
        
        END_INTERFACE
    } IVBSAXDeclHandlerVtbl;

    interface IVBSAXDeclHandler
    {
        CONST_VTBL struct IVBSAXDeclHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXDeclHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXDeclHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXDeclHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXDeclHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXDeclHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXDeclHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXDeclHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXDeclHandler_elementDecl(This,strName,strModel)	\
    ( (This)->lpVtbl -> elementDecl(This,strName,strModel) ) 

#define IVBSAXDeclHandler_attributeDecl(This,strElementName,strAttributeName,strType,strValueDefault,strValue)	\
    ( (This)->lpVtbl -> attributeDecl(This,strElementName,strAttributeName,strType,strValueDefault,strValue) ) 

#define IVBSAXDeclHandler_internalEntityDecl(This,strName,strValue)	\
    ( (This)->lpVtbl -> internalEntityDecl(This,strName,strValue) ) 

#define IVBSAXDeclHandler_externalEntityDecl(This,strName,strPublicId,strSystemId)	\
    ( (This)->lpVtbl -> externalEntityDecl(This,strName,strPublicId,strSystemId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXDeclHandler_INTERFACE_DEFINED__ */


#ifndef __IVBSAXAttributes_INTERFACE_DEFINED__
#define __IVBSAXAttributes_INTERFACE_DEFINED__

/* interface IVBSAXAttributes */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IVBSAXAttributes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10dc0586-132b-4cac-8bb3-db00ac8b7ee0")
    IVBSAXAttributes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ int *nLength) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getURI( 
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strURI) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getLocalName( 
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strLocalName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getQName( 
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strQName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getIndexFromName( 
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ int *nIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getIndexFromQName( 
            /* [in] */ BSTR strQName,
            /* [retval][out] */ int *nIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getType( 
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getTypeFromName( 
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ BSTR *strType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getTypeFromQName( 
            /* [in] */ BSTR strQName,
            /* [retval][out] */ BSTR *strType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getValue( 
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getValueFromName( 
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ BSTR *strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getValueFromQName( 
            /* [in] */ BSTR strQName,
            /* [retval][out] */ BSTR *strValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBSAXAttributesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBSAXAttributes * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBSAXAttributes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBSAXAttributes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBSAXAttributes * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBSAXAttributes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBSAXAttributes * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBSAXAttributes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, get_length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IVBSAXAttributes * This,
            /* [retval][out] */ int *nLength);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getURI)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getURI )( 
            IVBSAXAttributes * This,
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strURI);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getLocalName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getLocalName )( 
            IVBSAXAttributes * This,
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strLocalName);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getQName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getQName )( 
            IVBSAXAttributes * This,
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strQName);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getIndexFromName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getIndexFromName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ int *nIndex);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getIndexFromQName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getIndexFromQName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strQName,
            /* [retval][out] */ int *nIndex);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getType )( 
            IVBSAXAttributes * This,
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strType);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getTypeFromName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getTypeFromName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ BSTR *strType);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getTypeFromQName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getTypeFromQName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strQName,
            /* [retval][out] */ BSTR *strType);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getValue)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getValue )( 
            IVBSAXAttributes * This,
            /* [in] */ int nIndex,
            /* [retval][out] */ BSTR *strValue);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getValueFromName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getValueFromName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [retval][out] */ BSTR *strValue);
        
        DECLSPEC_XFGVIRT(IVBSAXAttributes, getValueFromQName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getValueFromQName )( 
            IVBSAXAttributes * This,
            /* [in] */ BSTR strQName,
            /* [retval][out] */ BSTR *strValue);
        
        END_INTERFACE
    } IVBSAXAttributesVtbl;

    interface IVBSAXAttributes
    {
        CONST_VTBL struct IVBSAXAttributesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBSAXAttributes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBSAXAttributes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBSAXAttributes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBSAXAttributes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBSAXAttributes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBSAXAttributes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBSAXAttributes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBSAXAttributes_get_length(This,nLength)	\
    ( (This)->lpVtbl -> get_length(This,nLength) ) 

#define IVBSAXAttributes_getURI(This,nIndex,strURI)	\
    ( (This)->lpVtbl -> getURI(This,nIndex,strURI) ) 

#define IVBSAXAttributes_getLocalName(This,nIndex,strLocalName)	\
    ( (This)->lpVtbl -> getLocalName(This,nIndex,strLocalName) ) 

#define IVBSAXAttributes_getQName(This,nIndex,strQName)	\
    ( (This)->lpVtbl -> getQName(This,nIndex,strQName) ) 

#define IVBSAXAttributes_getIndexFromName(This,strURI,strLocalName,nIndex)	\
    ( (This)->lpVtbl -> getIndexFromName(This,strURI,strLocalName,nIndex) ) 

#define IVBSAXAttributes_getIndexFromQName(This,strQName,nIndex)	\
    ( (This)->lpVtbl -> getIndexFromQName(This,strQName,nIndex) ) 

#define IVBSAXAttributes_getType(This,nIndex,strType)	\
    ( (This)->lpVtbl -> getType(This,nIndex,strType) ) 

#define IVBSAXAttributes_getTypeFromName(This,strURI,strLocalName,strType)	\
    ( (This)->lpVtbl -> getTypeFromName(This,strURI,strLocalName,strType) ) 

#define IVBSAXAttributes_getTypeFromQName(This,strQName,strType)	\
    ( (This)->lpVtbl -> getTypeFromQName(This,strQName,strType) ) 

#define IVBSAXAttributes_getValue(This,nIndex,strValue)	\
    ( (This)->lpVtbl -> getValue(This,nIndex,strValue) ) 

#define IVBSAXAttributes_getValueFromName(This,strURI,strLocalName,strValue)	\
    ( (This)->lpVtbl -> getValueFromName(This,strURI,strLocalName,strValue) ) 

#define IVBSAXAttributes_getValueFromQName(This,strQName,strValue)	\
    ( (This)->lpVtbl -> getValueFromQName(This,strQName,strValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBSAXAttributes_INTERFACE_DEFINED__ */


#ifndef __IMXWriter_INTERFACE_DEFINED__
#define __IMXWriter_INTERFACE_DEFINED__

/* interface IMXWriter */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IMXWriter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4d7ff4ba-1565-4ea8-94e1-6e724a46f98d")
    IMXWriter : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_output( 
            /* [in] */ VARIANT varDestination) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_output( 
            /* [retval][out] */ VARIANT *varDestination) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_encoding( 
            /* [in] */ BSTR strEncoding) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_encoding( 
            /* [retval][out] */ BSTR *strEncoding) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_byteOrderMark( 
            /* [in] */ VARIANT_BOOL fWriteByteOrderMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_byteOrderMark( 
            /* [retval][out] */ VARIANT_BOOL *fWriteByteOrderMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_indent( 
            /* [in] */ VARIANT_BOOL fIndentMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_indent( 
            /* [retval][out] */ VARIANT_BOOL *fIndentMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_standalone( 
            /* [in] */ VARIANT_BOOL fValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_standalone( 
            /* [retval][out] */ VARIANT_BOOL *fValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_omitXMLDeclaration( 
            /* [in] */ VARIANT_BOOL fValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_omitXMLDeclaration( 
            /* [retval][out] */ VARIANT_BOOL *fValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_version( 
            /* [in] */ BSTR strVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *strVersion) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_disableOutputEscaping( 
            /* [in] */ VARIANT_BOOL fValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_disableOutputEscaping( 
            /* [retval][out] */ VARIANT_BOOL *fValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXWriterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXWriter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXWriter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXWriter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXWriter * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXWriter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXWriter * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXWriter * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_output)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_output )( 
            IMXWriter * This,
            /* [in] */ VARIANT varDestination);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_output)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_output )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT *varDestination);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_encoding)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_encoding )( 
            IMXWriter * This,
            /* [in] */ BSTR strEncoding);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_encoding)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_encoding )( 
            IMXWriter * This,
            /* [retval][out] */ BSTR *strEncoding);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_byteOrderMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_byteOrderMark )( 
            IMXWriter * This,
            /* [in] */ VARIANT_BOOL fWriteByteOrderMark);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_byteOrderMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_byteOrderMark )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT_BOOL *fWriteByteOrderMark);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_indent)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_indent )( 
            IMXWriter * This,
            /* [in] */ VARIANT_BOOL fIndentMode);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_indent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_indent )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT_BOOL *fIndentMode);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_standalone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_standalone )( 
            IMXWriter * This,
            /* [in] */ VARIANT_BOOL fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_standalone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_standalone )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT_BOOL *fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_omitXMLDeclaration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_omitXMLDeclaration )( 
            IMXWriter * This,
            /* [in] */ VARIANT_BOOL fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_omitXMLDeclaration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_omitXMLDeclaration )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT_BOOL *fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_version)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_version )( 
            IMXWriter * This,
            /* [in] */ BSTR strVersion);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_version)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            IMXWriter * This,
            /* [retval][out] */ BSTR *strVersion);
        
        DECLSPEC_XFGVIRT(IMXWriter, put_disableOutputEscaping)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_disableOutputEscaping )( 
            IMXWriter * This,
            /* [in] */ VARIANT_BOOL fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, get_disableOutputEscaping)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_disableOutputEscaping )( 
            IMXWriter * This,
            /* [retval][out] */ VARIANT_BOOL *fValue);
        
        DECLSPEC_XFGVIRT(IMXWriter, flush)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *flush )( 
            IMXWriter * This);
        
        END_INTERFACE
    } IMXWriterVtbl;

    interface IMXWriter
    {
        CONST_VTBL struct IMXWriterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXWriter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXWriter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXWriter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXWriter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXWriter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXWriter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXWriter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXWriter_put_output(This,varDestination)	\
    ( (This)->lpVtbl -> put_output(This,varDestination) ) 

#define IMXWriter_get_output(This,varDestination)	\
    ( (This)->lpVtbl -> get_output(This,varDestination) ) 

#define IMXWriter_put_encoding(This,strEncoding)	\
    ( (This)->lpVtbl -> put_encoding(This,strEncoding) ) 

#define IMXWriter_get_encoding(This,strEncoding)	\
    ( (This)->lpVtbl -> get_encoding(This,strEncoding) ) 

#define IMXWriter_put_byteOrderMark(This,fWriteByteOrderMark)	\
    ( (This)->lpVtbl -> put_byteOrderMark(This,fWriteByteOrderMark) ) 

#define IMXWriter_get_byteOrderMark(This,fWriteByteOrderMark)	\
    ( (This)->lpVtbl -> get_byteOrderMark(This,fWriteByteOrderMark) ) 

#define IMXWriter_put_indent(This,fIndentMode)	\
    ( (This)->lpVtbl -> put_indent(This,fIndentMode) ) 

#define IMXWriter_get_indent(This,fIndentMode)	\
    ( (This)->lpVtbl -> get_indent(This,fIndentMode) ) 

#define IMXWriter_put_standalone(This,fValue)	\
    ( (This)->lpVtbl -> put_standalone(This,fValue) ) 

#define IMXWriter_get_standalone(This,fValue)	\
    ( (This)->lpVtbl -> get_standalone(This,fValue) ) 

#define IMXWriter_put_omitXMLDeclaration(This,fValue)	\
    ( (This)->lpVtbl -> put_omitXMLDeclaration(This,fValue) ) 

#define IMXWriter_get_omitXMLDeclaration(This,fValue)	\
    ( (This)->lpVtbl -> get_omitXMLDeclaration(This,fValue) ) 

#define IMXWriter_put_version(This,strVersion)	\
    ( (This)->lpVtbl -> put_version(This,strVersion) ) 

#define IMXWriter_get_version(This,strVersion)	\
    ( (This)->lpVtbl -> get_version(This,strVersion) ) 

#define IMXWriter_put_disableOutputEscaping(This,fValue)	\
    ( (This)->lpVtbl -> put_disableOutputEscaping(This,fValue) ) 

#define IMXWriter_get_disableOutputEscaping(This,fValue)	\
    ( (This)->lpVtbl -> get_disableOutputEscaping(This,fValue) ) 

#define IMXWriter_flush(This)	\
    ( (This)->lpVtbl -> flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXWriter_INTERFACE_DEFINED__ */


#ifndef __IMXAttributes_INTERFACE_DEFINED__
#define __IMXAttributes_INTERFACE_DEFINED__

/* interface IMXAttributes */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IMXAttributes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f10d27cc-3ec0-415c-8ed8-77ab1c5e7262")
    IMXAttributes : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE addAttribute( 
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [in] */ BSTR strQName,
            /* [in] */ BSTR strType,
            /* [in] */ BSTR strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE addAttributeFromIndex( 
            /* [in] */ VARIANT varAtts,
            /* [in] */ int nIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE clear( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeAttribute( 
            /* [in] */ int nIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setAttribute( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [in] */ BSTR strQName,
            /* [in] */ BSTR strType,
            /* [in] */ BSTR strValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setAttributes( 
            /* [in] */ VARIANT varAtts) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setLocalName( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strLocalName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setQName( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strQName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setType( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setURI( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strURI) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE setValue( 
            /* [in] */ int nIndex,
            /* [in] */ BSTR strValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXAttributesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXAttributes * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXAttributes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXAttributes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXAttributes * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXAttributes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXAttributes * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXAttributes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXAttributes, addAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addAttribute )( 
            IMXAttributes * This,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [in] */ BSTR strQName,
            /* [in] */ BSTR strType,
            /* [in] */ BSTR strValue);
        
        DECLSPEC_XFGVIRT(IMXAttributes, addAttributeFromIndex)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addAttributeFromIndex )( 
            IMXAttributes * This,
            /* [in] */ VARIANT varAtts,
            /* [in] */ int nIndex);
        
        DECLSPEC_XFGVIRT(IMXAttributes, clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *clear )( 
            IMXAttributes * This);
        
        DECLSPEC_XFGVIRT(IMXAttributes, removeAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeAttribute )( 
            IMXAttributes * This,
            /* [in] */ int nIndex);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setAttribute)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setAttribute )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strURI,
            /* [in] */ BSTR strLocalName,
            /* [in] */ BSTR strQName,
            /* [in] */ BSTR strType,
            /* [in] */ BSTR strValue);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setAttributes)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setAttributes )( 
            IMXAttributes * This,
            /* [in] */ VARIANT varAtts);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setLocalName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setLocalName )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strLocalName);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setQName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setQName )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strQName);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setType )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strType);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setURI)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setURI )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strURI);
        
        DECLSPEC_XFGVIRT(IMXAttributes, setValue)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *setValue )( 
            IMXAttributes * This,
            /* [in] */ int nIndex,
            /* [in] */ BSTR strValue);
        
        END_INTERFACE
    } IMXAttributesVtbl;

    interface IMXAttributes
    {
        CONST_VTBL struct IMXAttributesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXAttributes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXAttributes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXAttributes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXAttributes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXAttributes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXAttributes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXAttributes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXAttributes_addAttribute(This,strURI,strLocalName,strQName,strType,strValue)	\
    ( (This)->lpVtbl -> addAttribute(This,strURI,strLocalName,strQName,strType,strValue) ) 

#define IMXAttributes_addAttributeFromIndex(This,varAtts,nIndex)	\
    ( (This)->lpVtbl -> addAttributeFromIndex(This,varAtts,nIndex) ) 

#define IMXAttributes_clear(This)	\
    ( (This)->lpVtbl -> clear(This) ) 

#define IMXAttributes_removeAttribute(This,nIndex)	\
    ( (This)->lpVtbl -> removeAttribute(This,nIndex) ) 

#define IMXAttributes_setAttribute(This,nIndex,strURI,strLocalName,strQName,strType,strValue)	\
    ( (This)->lpVtbl -> setAttribute(This,nIndex,strURI,strLocalName,strQName,strType,strValue) ) 

#define IMXAttributes_setAttributes(This,varAtts)	\
    ( (This)->lpVtbl -> setAttributes(This,varAtts) ) 

#define IMXAttributes_setLocalName(This,nIndex,strLocalName)	\
    ( (This)->lpVtbl -> setLocalName(This,nIndex,strLocalName) ) 

#define IMXAttributes_setQName(This,nIndex,strQName)	\
    ( (This)->lpVtbl -> setQName(This,nIndex,strQName) ) 

#define IMXAttributes_setType(This,nIndex,strType)	\
    ( (This)->lpVtbl -> setType(This,nIndex,strType) ) 

#define IMXAttributes_setURI(This,nIndex,strURI)	\
    ( (This)->lpVtbl -> setURI(This,nIndex,strURI) ) 

#define IMXAttributes_setValue(This,nIndex,strValue)	\
    ( (This)->lpVtbl -> setValue(This,nIndex,strValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXAttributes_INTERFACE_DEFINED__ */


#ifndef __IMXReaderControl_INTERFACE_DEFINED__
#define __IMXReaderControl_INTERFACE_DEFINED__

/* interface IMXReaderControl */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IMXReaderControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("808f4e35-8d5a-4fbe-8466-33a41279ed30")
    IMXReaderControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE abort( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE resume( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE suspend( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXReaderControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXReaderControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXReaderControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXReaderControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXReaderControl * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXReaderControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXReaderControl * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXReaderControl * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXReaderControl, abort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *abort )( 
            IMXReaderControl * This);
        
        DECLSPEC_XFGVIRT(IMXReaderControl, resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *resume )( 
            IMXReaderControl * This);
        
        DECLSPEC_XFGVIRT(IMXReaderControl, suspend)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *suspend )( 
            IMXReaderControl * This);
        
        END_INTERFACE
    } IMXReaderControlVtbl;

    interface IMXReaderControl
    {
        CONST_VTBL struct IMXReaderControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXReaderControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXReaderControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXReaderControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXReaderControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXReaderControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXReaderControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXReaderControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXReaderControl_abort(This)	\
    ( (This)->lpVtbl -> abort(This) ) 

#define IMXReaderControl_resume(This)	\
    ( (This)->lpVtbl -> resume(This) ) 

#define IMXReaderControl_suspend(This)	\
    ( (This)->lpVtbl -> suspend(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXReaderControl_INTERFACE_DEFINED__ */


#ifndef __IMXSchemaDeclHandler_INTERFACE_DEFINED__
#define __IMXSchemaDeclHandler_INTERFACE_DEFINED__

/* interface IMXSchemaDeclHandler */
/* [unique][helpstring][uuid][nonextensible][oleautomation][dual][local][object] */ 


EXTERN_C const IID IID_IMXSchemaDeclHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa4bb38c-faf9-4cca-9302-d1dd0fe520db")
    IMXSchemaDeclHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE schemaElementDecl( 
            /* [in] */ ISchemaElement *oSchemaElement) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXSchemaDeclHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXSchemaDeclHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXSchemaDeclHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXSchemaDeclHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXSchemaDeclHandler * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXSchemaDeclHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXSchemaDeclHandler * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXSchemaDeclHandler * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXSchemaDeclHandler, schemaElementDecl)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *schemaElementDecl )( 
            IMXSchemaDeclHandler * This,
            /* [in] */ ISchemaElement *oSchemaElement);
        
        END_INTERFACE
    } IMXSchemaDeclHandlerVtbl;

    interface IMXSchemaDeclHandler
    {
        CONST_VTBL struct IMXSchemaDeclHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXSchemaDeclHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXSchemaDeclHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXSchemaDeclHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXSchemaDeclHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXSchemaDeclHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXSchemaDeclHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXSchemaDeclHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXSchemaDeclHandler_schemaElementDecl(This,oSchemaElement)	\
    ( (This)->lpVtbl -> schemaElementDecl(This,oSchemaElement) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXSchemaDeclHandler_INTERFACE_DEFINED__ */


#ifndef __IMXNamespacePrefixes_INTERFACE_DEFINED__
#define __IMXNamespacePrefixes_INTERFACE_DEFINED__

/* interface IMXNamespacePrefixes */
/* [unique][nonextensible][oleautomation][dual][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IMXNamespacePrefixes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c90352f4-643c-4fbc-bb23-e996eb2d51fd")
    IMXNamespacePrefixes : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ BSTR *prefix) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *length) = 0;
        
        virtual /* [id][hidden][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [retval][out] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXNamespacePrefixesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXNamespacePrefixes * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXNamespacePrefixes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXNamespacePrefixes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXNamespacePrefixes * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXNamespacePrefixes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXNamespacePrefixes * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXNamespacePrefixes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXNamespacePrefixes, get_item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            IMXNamespacePrefixes * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *prefix);
        
        DECLSPEC_XFGVIRT(IMXNamespacePrefixes, get_length)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IMXNamespacePrefixes * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(IMXNamespacePrefixes, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IMXNamespacePrefixes * This,
            /* [retval][out] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } IMXNamespacePrefixesVtbl;

    interface IMXNamespacePrefixes
    {
        CONST_VTBL struct IMXNamespacePrefixesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXNamespacePrefixes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXNamespacePrefixes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXNamespacePrefixes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXNamespacePrefixes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXNamespacePrefixes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXNamespacePrefixes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXNamespacePrefixes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXNamespacePrefixes_get_item(This,index,prefix)	\
    ( (This)->lpVtbl -> get_item(This,index,prefix) ) 

#define IMXNamespacePrefixes_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define IMXNamespacePrefixes_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXNamespacePrefixes_INTERFACE_DEFINED__ */


#ifndef __IVBMXNamespaceManager_INTERFACE_DEFINED__
#define __IVBMXNamespaceManager_INTERFACE_DEFINED__

/* interface IVBMXNamespaceManager */
/* [unique][nonextensible][oleautomation][dual][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IVBMXNamespaceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c90352f5-643c-4fbc-bb23-e996eb2d51fd")
    IVBMXNamespaceManager : public IDispatch
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_allowOverride( 
            /* [in] */ VARIANT_BOOL fOverride) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_allowOverride( 
            /* [retval][out] */ VARIANT_BOOL *fOverride) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE pushContext( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE pushNodeContext( 
            /* [in] */ IXMLDOMNode *contextNode,
            /* [defaultvalue][in] */ VARIANT_BOOL fDeep = -1) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE popContext( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE declarePrefix( 
            /* [in] */ BSTR prefix,
            /* [in] */ BSTR namespaceURI) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getDeclaredPrefixes( 
            /* [retval][out] */ IMXNamespacePrefixes **prefixes) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getPrefixes( 
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IMXNamespacePrefixes **prefixes) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getURI( 
            /* [in] */ BSTR prefix,
            /* [retval][out] */ VARIANT *uri) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getURIFromNode( 
            /* [in] */ BSTR strPrefix,
            /* [in] */ IXMLDOMNode *contextNode,
            /* [retval][out] */ VARIANT *uri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVBMXNamespaceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVBMXNamespaceManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVBMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVBMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IVBMXNamespaceManager * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IVBMXNamespaceManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IVBMXNamespaceManager * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IVBMXNamespaceManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, put_allowOverride)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_allowOverride )( 
            IVBMXNamespaceManager * This,
            /* [in] */ VARIANT_BOOL fOverride);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, get_allowOverride)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_allowOverride )( 
            IVBMXNamespaceManager * This,
            /* [retval][out] */ VARIANT_BOOL *fOverride);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, reset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *reset )( 
            IVBMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, pushContext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *pushContext )( 
            IVBMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, pushNodeContext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *pushNodeContext )( 
            IVBMXNamespaceManager * This,
            /* [in] */ IXMLDOMNode *contextNode,
            /* [defaultvalue][in] */ VARIANT_BOOL fDeep);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, popContext)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *popContext )( 
            IVBMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, declarePrefix)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *declarePrefix )( 
            IVBMXNamespaceManager * This,
            /* [in] */ BSTR prefix,
            /* [in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, getDeclaredPrefixes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getDeclaredPrefixes )( 
            IVBMXNamespaceManager * This,
            /* [retval][out] */ IMXNamespacePrefixes **prefixes);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, getPrefixes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getPrefixes )( 
            IVBMXNamespaceManager * This,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IMXNamespacePrefixes **prefixes);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, getURI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getURI )( 
            IVBMXNamespaceManager * This,
            /* [in] */ BSTR prefix,
            /* [retval][out] */ VARIANT *uri);
        
        DECLSPEC_XFGVIRT(IVBMXNamespaceManager, getURIFromNode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getURIFromNode )( 
            IVBMXNamespaceManager * This,
            /* [in] */ BSTR strPrefix,
            /* [in] */ IXMLDOMNode *contextNode,
            /* [retval][out] */ VARIANT *uri);
        
        END_INTERFACE
    } IVBMXNamespaceManagerVtbl;

    interface IVBMXNamespaceManager
    {
        CONST_VTBL struct IVBMXNamespaceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVBMXNamespaceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVBMXNamespaceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVBMXNamespaceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVBMXNamespaceManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IVBMXNamespaceManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IVBMXNamespaceManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IVBMXNamespaceManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IVBMXNamespaceManager_put_allowOverride(This,fOverride)	\
    ( (This)->lpVtbl -> put_allowOverride(This,fOverride) ) 

#define IVBMXNamespaceManager_get_allowOverride(This,fOverride)	\
    ( (This)->lpVtbl -> get_allowOverride(This,fOverride) ) 

#define IVBMXNamespaceManager_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IVBMXNamespaceManager_pushContext(This)	\
    ( (This)->lpVtbl -> pushContext(This) ) 

#define IVBMXNamespaceManager_pushNodeContext(This,contextNode,fDeep)	\
    ( (This)->lpVtbl -> pushNodeContext(This,contextNode,fDeep) ) 

#define IVBMXNamespaceManager_popContext(This)	\
    ( (This)->lpVtbl -> popContext(This) ) 

#define IVBMXNamespaceManager_declarePrefix(This,prefix,namespaceURI)	\
    ( (This)->lpVtbl -> declarePrefix(This,prefix,namespaceURI) ) 

#define IVBMXNamespaceManager_getDeclaredPrefixes(This,prefixes)	\
    ( (This)->lpVtbl -> getDeclaredPrefixes(This,prefixes) ) 

#define IVBMXNamespaceManager_getPrefixes(This,namespaceURI,prefixes)	\
    ( (This)->lpVtbl -> getPrefixes(This,namespaceURI,prefixes) ) 

#define IVBMXNamespaceManager_getURI(This,prefix,uri)	\
    ( (This)->lpVtbl -> getURI(This,prefix,uri) ) 

#define IVBMXNamespaceManager_getURIFromNode(This,strPrefix,contextNode,uri)	\
    ( (This)->lpVtbl -> getURIFromNode(This,strPrefix,contextNode,uri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVBMXNamespaceManager_INTERFACE_DEFINED__ */


#ifndef __IMXNamespaceManager_INTERFACE_DEFINED__
#define __IMXNamespaceManager_INTERFACE_DEFINED__

/* interface IMXNamespaceManager */
/* [unique][helpstring][uuid][local][object][hidden] */ 


EXTERN_C const IID IID_IMXNamespaceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c90352f6-643c-4fbc-bb23-e996eb2d51fd")
    IMXNamespaceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE putAllowOverride( 
            /* [in] */ VARIANT_BOOL fOverride) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getAllowOverride( 
            /* [retval][out] */ VARIANT_BOOL *fOverride) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE pushContext( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE pushNodeContext( 
            /* [in] */ IXMLDOMNode *contextNode,
            /* [in] */ VARIANT_BOOL fDeep) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE popContext( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE declarePrefix( 
            /* [in] */ LPCWSTR prefix,
            /* [in] */ LPCWSTR namespaceURI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getDeclaredPrefix( 
            /* [in] */ long nIndex,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchPrefix, *pcchPrefix)  wchar_t *pwchPrefix,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getPrefix( 
            /* [in] */ LPCWSTR pwszNamespaceURI,
            /* [in] */ long nIndex,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchPrefix, *pcchPrefix)  wchar_t *pwchPrefix,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getURI( 
            /* [in] */ LPCWSTR pwchPrefix,
            /* [in] */ IXMLDOMNode *pContextNode,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchUri, *pcchUri)  wchar_t *pwchUri,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXNamespaceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXNamespaceManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, putAllowOverride)
        HRESULT ( STDMETHODCALLTYPE *putAllowOverride )( 
            IMXNamespaceManager * This,
            /* [in] */ VARIANT_BOOL fOverride);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, getAllowOverride)
        HRESULT ( STDMETHODCALLTYPE *getAllowOverride )( 
            IMXNamespaceManager * This,
            /* [retval][out] */ VARIANT_BOOL *fOverride);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, reset)
        HRESULT ( STDMETHODCALLTYPE *reset )( 
            IMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, pushContext)
        HRESULT ( STDMETHODCALLTYPE *pushContext )( 
            IMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, pushNodeContext)
        HRESULT ( STDMETHODCALLTYPE *pushNodeContext )( 
            IMXNamespaceManager * This,
            /* [in] */ IXMLDOMNode *contextNode,
            /* [in] */ VARIANT_BOOL fDeep);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, popContext)
        HRESULT ( STDMETHODCALLTYPE *popContext )( 
            IMXNamespaceManager * This);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, declarePrefix)
        HRESULT ( STDMETHODCALLTYPE *declarePrefix )( 
            IMXNamespaceManager * This,
            /* [in] */ LPCWSTR prefix,
            /* [in] */ LPCWSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, getDeclaredPrefix)
        HRESULT ( STDMETHODCALLTYPE *getDeclaredPrefix )( 
            IMXNamespaceManager * This,
            /* [in] */ long nIndex,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchPrefix, *pcchPrefix)  wchar_t *pwchPrefix,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchPrefix);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, getPrefix)
        HRESULT ( STDMETHODCALLTYPE *getPrefix )( 
            IMXNamespaceManager * This,
            /* [in] */ LPCWSTR pwszNamespaceURI,
            /* [in] */ long nIndex,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchPrefix, *pcchPrefix)  wchar_t *pwchPrefix,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchPrefix);
        
        DECLSPEC_XFGVIRT(IMXNamespaceManager, getURI)
        HRESULT ( STDMETHODCALLTYPE *getURI )( 
            IMXNamespaceManager * This,
            /* [in] */ LPCWSTR pwchPrefix,
            /* [in] */ IXMLDOMNode *pContextNode,
            /* [annotation][out][in] */ 
            _Out_writes_to_(*pcchUri, *pcchUri)  wchar_t *pwchUri,
            /* [annotation][out][in] */ 
            _Inout_  int *pcchUri);
        
        END_INTERFACE
    } IMXNamespaceManagerVtbl;

    interface IMXNamespaceManager
    {
        CONST_VTBL struct IMXNamespaceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXNamespaceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXNamespaceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXNamespaceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXNamespaceManager_putAllowOverride(This,fOverride)	\
    ( (This)->lpVtbl -> putAllowOverride(This,fOverride) ) 

#define IMXNamespaceManager_getAllowOverride(This,fOverride)	\
    ( (This)->lpVtbl -> getAllowOverride(This,fOverride) ) 

#define IMXNamespaceManager_reset(This)	\
    ( (This)->lpVtbl -> reset(This) ) 

#define IMXNamespaceManager_pushContext(This)	\
    ( (This)->lpVtbl -> pushContext(This) ) 

#define IMXNamespaceManager_pushNodeContext(This,contextNode,fDeep)	\
    ( (This)->lpVtbl -> pushNodeContext(This,contextNode,fDeep) ) 

#define IMXNamespaceManager_popContext(This)	\
    ( (This)->lpVtbl -> popContext(This) ) 

#define IMXNamespaceManager_declarePrefix(This,prefix,namespaceURI)	\
    ( (This)->lpVtbl -> declarePrefix(This,prefix,namespaceURI) ) 

#define IMXNamespaceManager_getDeclaredPrefix(This,nIndex,pwchPrefix,pcchPrefix)	\
    ( (This)->lpVtbl -> getDeclaredPrefix(This,nIndex,pwchPrefix,pcchPrefix) ) 

#define IMXNamespaceManager_getPrefix(This,pwszNamespaceURI,nIndex,pwchPrefix,pcchPrefix)	\
    ( (This)->lpVtbl -> getPrefix(This,pwszNamespaceURI,nIndex,pwchPrefix,pcchPrefix) ) 

#define IMXNamespaceManager_getURI(This,pwchPrefix,pContextNode,pwchUri,pcchUri)	\
    ( (This)->lpVtbl -> getURI(This,pwchPrefix,pContextNode,pwchUri,pcchUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXNamespaceManager_INTERFACE_DEFINED__ */


#ifndef __IMXXMLFilter_INTERFACE_DEFINED__
#define __IMXXMLFilter_INTERFACE_DEFINED__

/* interface IMXXMLFilter */
/* [unique][nonextensible][oleautomation][dual][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IMXXMLFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c90352f7-643c-4fbc-bb23-e996eb2d51fd")
    IMXXMLFilter : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getFeature( 
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT_BOOL *fValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE putFeature( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT_BOOL fValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getProperty( 
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT *varValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE putProperty( 
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT varValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_entityResolver( 
            /* [retval][out] */ IUnknown **oResolver) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_entityResolver( 
            /* [in] */ IUnknown *oResolver) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_contentHandler( 
            /* [retval][out] */ IUnknown **oHandler) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_contentHandler( 
            /* [in] */ IUnknown *oHandler) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_dtdHandler( 
            /* [retval][out] */ IUnknown **oHandler) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_dtdHandler( 
            /* [in] */ IUnknown *oHandler) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_errorHandler( 
            /* [retval][out] */ IUnknown **oHandler) = 0;
        
        virtual /* [propputref][id] */ HRESULT STDMETHODCALLTYPE putref_errorHandler( 
            /* [in] */ IUnknown *oHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMXXMLFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMXXMLFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMXXMLFilter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IMXXMLFilter * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IMXXMLFilter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IMXXMLFilter * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMXXMLFilter * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, getFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getFeature )( 
            IMXXMLFilter * This,
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT_BOOL *fValue);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *putFeature )( 
            IMXXMLFilter * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT_BOOL fValue);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, getProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getProperty )( 
            IMXXMLFilter * This,
            /* [in] */ BSTR strName,
            /* [retval][out] */ VARIANT *varValue);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *putProperty )( 
            IMXXMLFilter * This,
            /* [in] */ BSTR strName,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, get_entityResolver)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_entityResolver )( 
            IMXXMLFilter * This,
            /* [retval][out] */ IUnknown **oResolver);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putref_entityResolver)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_entityResolver )( 
            IMXXMLFilter * This,
            /* [in] */ IUnknown *oResolver);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, get_contentHandler)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_contentHandler )( 
            IMXXMLFilter * This,
            /* [retval][out] */ IUnknown **oHandler);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putref_contentHandler)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_contentHandler )( 
            IMXXMLFilter * This,
            /* [in] */ IUnknown *oHandler);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, get_dtdHandler)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_dtdHandler )( 
            IMXXMLFilter * This,
            /* [retval][out] */ IUnknown **oHandler);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putref_dtdHandler)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_dtdHandler )( 
            IMXXMLFilter * This,
            /* [in] */ IUnknown *oHandler);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, get_errorHandler)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_errorHandler )( 
            IMXXMLFilter * This,
            /* [retval][out] */ IUnknown **oHandler);
        
        DECLSPEC_XFGVIRT(IMXXMLFilter, putref_errorHandler)
        /* [propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_errorHandler )( 
            IMXXMLFilter * This,
            /* [in] */ IUnknown *oHandler);
        
        END_INTERFACE
    } IMXXMLFilterVtbl;

    interface IMXXMLFilter
    {
        CONST_VTBL struct IMXXMLFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMXXMLFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMXXMLFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMXXMLFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMXXMLFilter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMXXMLFilter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMXXMLFilter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMXXMLFilter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMXXMLFilter_getFeature(This,strName,fValue)	\
    ( (This)->lpVtbl -> getFeature(This,strName,fValue) ) 

#define IMXXMLFilter_putFeature(This,strName,fValue)	\
    ( (This)->lpVtbl -> putFeature(This,strName,fValue) ) 

#define IMXXMLFilter_getProperty(This,strName,varValue)	\
    ( (This)->lpVtbl -> getProperty(This,strName,varValue) ) 

#define IMXXMLFilter_putProperty(This,strName,varValue)	\
    ( (This)->lpVtbl -> putProperty(This,strName,varValue) ) 

#define IMXXMLFilter_get_entityResolver(This,oResolver)	\
    ( (This)->lpVtbl -> get_entityResolver(This,oResolver) ) 

#define IMXXMLFilter_putref_entityResolver(This,oResolver)	\
    ( (This)->lpVtbl -> putref_entityResolver(This,oResolver) ) 

#define IMXXMLFilter_get_contentHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_contentHandler(This,oHandler) ) 

#define IMXXMLFilter_putref_contentHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_contentHandler(This,oHandler) ) 

#define IMXXMLFilter_get_dtdHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_dtdHandler(This,oHandler) ) 

#define IMXXMLFilter_putref_dtdHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_dtdHandler(This,oHandler) ) 

#define IMXXMLFilter_get_errorHandler(This,oHandler)	\
    ( (This)->lpVtbl -> get_errorHandler(This,oHandler) ) 

#define IMXXMLFilter_putref_errorHandler(This,oHandler)	\
    ( (This)->lpVtbl -> putref_errorHandler(This,oHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMXXMLFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msxml6_0000_0059 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if !defined(__msxml_som_enums__)
#define __msxml_som_enums__
typedef /* [helpstring] */ 
enum _SOMITEMTYPE
    {
        SOMITEM_SCHEMA	= 0x1000,
        SOMITEM_ATTRIBUTE	= 0x1001,
        SOMITEM_ATTRIBUTEGROUP	= 0x1002,
        SOMITEM_NOTATION	= 0x1003,
        SOMITEM_ANNOTATION	= 0x1004,
        SOMITEM_IDENTITYCONSTRAINT	= 0x1100,
        SOMITEM_KEY	= 0x1101,
        SOMITEM_KEYREF	= 0x1102,
        SOMITEM_UNIQUE	= 0x1103,
        SOMITEM_ANYTYPE	= 0x2000,
        SOMITEM_DATATYPE	= 0x2100,
        SOMITEM_DATATYPE_ANYTYPE	= 0x2101,
        SOMITEM_DATATYPE_ANYURI	= 0x2102,
        SOMITEM_DATATYPE_BASE64BINARY	= 0x2103,
        SOMITEM_DATATYPE_BOOLEAN	= 0x2104,
        SOMITEM_DATATYPE_BYTE	= 0x2105,
        SOMITEM_DATATYPE_DATE	= 0x2106,
        SOMITEM_DATATYPE_DATETIME	= 0x2107,
        SOMITEM_DATATYPE_DAY	= 0x2108,
        SOMITEM_DATATYPE_DECIMAL	= 0x2109,
        SOMITEM_DATATYPE_DOUBLE	= 0x210a,
        SOMITEM_DATATYPE_DURATION	= 0x210b,
        SOMITEM_DATATYPE_ENTITIES	= 0x210c,
        SOMITEM_DATATYPE_ENTITY	= 0x210d,
        SOMITEM_DATATYPE_FLOAT	= 0x210e,
        SOMITEM_DATATYPE_HEXBINARY	= 0x210f,
        SOMITEM_DATATYPE_ID	= 0x2110,
        SOMITEM_DATATYPE_IDREF	= 0x2111,
        SOMITEM_DATATYPE_IDREFS	= 0x2112,
        SOMITEM_DATATYPE_INT	= 0x2113,
        SOMITEM_DATATYPE_INTEGER	= 0x2114,
        SOMITEM_DATATYPE_LANGUAGE	= 0x2115,
        SOMITEM_DATATYPE_LONG	= 0x2116,
        SOMITEM_DATATYPE_MONTH	= 0x2117,
        SOMITEM_DATATYPE_MONTHDAY	= 0x2118,
        SOMITEM_DATATYPE_NAME	= 0x2119,
        SOMITEM_DATATYPE_NCNAME	= 0x211a,
        SOMITEM_DATATYPE_NEGATIVEINTEGER	= 0x211b,
        SOMITEM_DATATYPE_NMTOKEN	= 0x211c,
        SOMITEM_DATATYPE_NMTOKENS	= 0x211d,
        SOMITEM_DATATYPE_NONNEGATIVEINTEGER	= 0x211e,
        SOMITEM_DATATYPE_NONPOSITIVEINTEGER	= 0x211f,
        SOMITEM_DATATYPE_NORMALIZEDSTRING	= 0x2120,
        SOMITEM_DATATYPE_NOTATION	= 0x2121,
        SOMITEM_DATATYPE_POSITIVEINTEGER	= 0x2122,
        SOMITEM_DATATYPE_QNAME	= 0x2123,
        SOMITEM_DATATYPE_SHORT	= 0x2124,
        SOMITEM_DATATYPE_STRING	= 0x2125,
        SOMITEM_DATATYPE_TIME	= 0x2126,
        SOMITEM_DATATYPE_TOKEN	= 0x2127,
        SOMITEM_DATATYPE_UNSIGNEDBYTE	= 0x2128,
        SOMITEM_DATATYPE_UNSIGNEDINT	= 0x2129,
        SOMITEM_DATATYPE_UNSIGNEDLONG	= 0x212a,
        SOMITEM_DATATYPE_UNSIGNEDSHORT	= 0x212b,
        SOMITEM_DATATYPE_YEAR	= 0x212c,
        SOMITEM_DATATYPE_YEARMONTH	= 0x212d,
        SOMITEM_DATATYPE_ANYSIMPLETYPE	= 0x21ff,
        SOMITEM_SIMPLETYPE	= 0x2200,
        SOMITEM_COMPLEXTYPE	= 0x2400,
        SOMITEM_PARTICLE	= 0x4000,
        SOMITEM_ANY	= 0x4001,
        SOMITEM_ANYATTRIBUTE	= 0x4002,
        SOMITEM_ELEMENT	= 0x4003,
        SOMITEM_GROUP	= 0x4100,
        SOMITEM_ALL	= 0x4101,
        SOMITEM_CHOICE	= 0x4102,
        SOMITEM_SEQUENCE	= 0x4103,
        SOMITEM_EMPTYPARTICLE	= 0x4104,
        SOMITEM_NULL	= 0x800,
        SOMITEM_NULL_TYPE	= 0x2800,
        SOMITEM_NULL_ANY	= 0x4801,
        SOMITEM_NULL_ANYATTRIBUTE	= 0x4802,
        SOMITEM_NULL_ELEMENT	= 0x4803
    } 	SOMITEMTYPE;

typedef /* [helpstring] */ 
enum _SCHEMAUSE
    {
        SCHEMAUSE_OPTIONAL	= 0,
        SCHEMAUSE_PROHIBITED	= ( SCHEMAUSE_OPTIONAL + 1 ) ,
        SCHEMAUSE_REQUIRED	= ( SCHEMAUSE_PROHIBITED + 1 ) 
    } 	SCHEMAUSE;

typedef /* [helpstring] */ 
enum _SCHEMADERIVATIONMETHOD
    {
        SCHEMADERIVATIONMETHOD_EMPTY	= 0,
        SCHEMADERIVATIONMETHOD_SUBSTITUTION	= 0x1,
        SCHEMADERIVATIONMETHOD_EXTENSION	= 0x2,
        SCHEMADERIVATIONMETHOD_RESTRICTION	= 0x4,
        SCHEMADERIVATIONMETHOD_LIST	= 0x8,
        SCHEMADERIVATIONMETHOD_UNION	= 0x10,
        SCHEMADERIVATIONMETHOD_ALL	= 0xff,
        SCHEMADERIVATIONMETHOD_NONE	= 0x100
    } 	SCHEMADERIVATIONMETHOD;

typedef /* [helpstring] */ 
enum _SCHEMACONTENTTYPE
    {
        SCHEMACONTENTTYPE_EMPTY	= 0,
        SCHEMACONTENTTYPE_TEXTONLY	= ( SCHEMACONTENTTYPE_EMPTY + 1 ) ,
        SCHEMACONTENTTYPE_ELEMENTONLY	= ( SCHEMACONTENTTYPE_TEXTONLY + 1 ) ,
        SCHEMACONTENTTYPE_MIXED	= ( SCHEMACONTENTTYPE_ELEMENTONLY + 1 ) 
    } 	SCHEMACONTENTTYPE;

typedef /* [helpstring] */ 
enum _SCHEMAPROCESSCONTENTS
    {
        SCHEMAPROCESSCONTENTS_NONE	= 0,
        SCHEMAPROCESSCONTENTS_SKIP	= ( SCHEMAPROCESSCONTENTS_NONE + 1 ) ,
        SCHEMAPROCESSCONTENTS_LAX	= ( SCHEMAPROCESSCONTENTS_SKIP + 1 ) ,
        SCHEMAPROCESSCONTENTS_STRICT	= ( SCHEMAPROCESSCONTENTS_LAX + 1 ) 
    } 	SCHEMAPROCESSCONTENTS;

typedef /* [helpstring] */ 
enum _SCHEMAWHITESPACE
    {
        SCHEMAWHITESPACE_NONE	= -1,
        SCHEMAWHITESPACE_PRESERVE	= 0,
        SCHEMAWHITESPACE_REPLACE	= 1,
        SCHEMAWHITESPACE_COLLAPSE	= 2
    } 	SCHEMAWHITESPACE;

typedef /* [helpstring] */ 
enum _SCHEMATYPEVARIETY
    {
        SCHEMATYPEVARIETY_NONE	= -1,
        SCHEMATYPEVARIETY_ATOMIC	= 0,
        SCHEMATYPEVARIETY_LIST	= 1,
        SCHEMATYPEVARIETY_UNION	= 2
    } 	SCHEMATYPEVARIETY;

#endif // !defined(__msxml_som_enums__)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0059_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0059_v0_0_s_ifspec;

#ifndef __IXMLDOMSchemaCollection2_INTERFACE_DEFINED__
#define __IXMLDOMSchemaCollection2_INTERFACE_DEFINED__

/* interface IXMLDOMSchemaCollection2 */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_IXMLDOMSchemaCollection2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b0-dd1b-4664-9a50-c2f40f4bd79a")
    IXMLDOMSchemaCollection2 : public IXMLDOMSchemaCollection
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE validate( void) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_validateOnLoad( 
            /* [in] */ VARIANT_BOOL validateOnLoad) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_validateOnLoad( 
            /* [retval][out] */ VARIANT_BOOL *validateOnLoad) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getSchema( 
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ ISchema **schema) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getDeclaration( 
            /* [in] */ IXMLDOMNode *node,
            /* [retval][out] */ ISchemaItem **item) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLDOMSchemaCollection2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXMLDOMSchemaCollection2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXMLDOMSchemaCollection2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IXMLDOMSchemaCollection2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXMLDOMSchemaCollection2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *add )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ BSTR namespaceURI,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *get )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ IXMLDOMNode **schemaNode);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *remove )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ BSTR namespaceURI);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get_length)
        /* [propget][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            IXMLDOMSchemaCollection2 * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get_namespaceURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *length);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, addCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *addCollection )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ IXMLDOMSchemaCollection *otherCollection);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection, get__newEnum)
        /* [id][hidden][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            IXMLDOMSchemaCollection2 * This,
            /* [out][retval] */ IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection2, validate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *validate )( 
            IXMLDOMSchemaCollection2 * This);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection2, put_validateOnLoad)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_validateOnLoad )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ VARIANT_BOOL validateOnLoad);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection2, get_validateOnLoad)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_validateOnLoad )( 
            IXMLDOMSchemaCollection2 * This,
            /* [retval][out] */ VARIANT_BOOL *validateOnLoad);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection2, getSchema)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getSchema )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(IXMLDOMSchemaCollection2, getDeclaration)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getDeclaration )( 
            IXMLDOMSchemaCollection2 * This,
            /* [in] */ IXMLDOMNode *node,
            /* [retval][out] */ ISchemaItem **item);
        
        END_INTERFACE
    } IXMLDOMSchemaCollection2Vtbl;

    interface IXMLDOMSchemaCollection2
    {
        CONST_VTBL struct IXMLDOMSchemaCollection2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLDOMSchemaCollection2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLDOMSchemaCollection2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLDOMSchemaCollection2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLDOMSchemaCollection2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXMLDOMSchemaCollection2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXMLDOMSchemaCollection2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXMLDOMSchemaCollection2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IXMLDOMSchemaCollection2_add(This,namespaceURI,var)	\
    ( (This)->lpVtbl -> add(This,namespaceURI,var) ) 

#define IXMLDOMSchemaCollection2_get(This,namespaceURI,schemaNode)	\
    ( (This)->lpVtbl -> get(This,namespaceURI,schemaNode) ) 

#define IXMLDOMSchemaCollection2_remove(This,namespaceURI)	\
    ( (This)->lpVtbl -> remove(This,namespaceURI) ) 

#define IXMLDOMSchemaCollection2_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define IXMLDOMSchemaCollection2_get_namespaceURI(This,index,length)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,index,length) ) 

#define IXMLDOMSchemaCollection2_addCollection(This,otherCollection)	\
    ( (This)->lpVtbl -> addCollection(This,otherCollection) ) 

#define IXMLDOMSchemaCollection2_get__newEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppUnk) ) 


#define IXMLDOMSchemaCollection2_validate(This)	\
    ( (This)->lpVtbl -> validate(This) ) 

#define IXMLDOMSchemaCollection2_put_validateOnLoad(This,validateOnLoad)	\
    ( (This)->lpVtbl -> put_validateOnLoad(This,validateOnLoad) ) 

#define IXMLDOMSchemaCollection2_get_validateOnLoad(This,validateOnLoad)	\
    ( (This)->lpVtbl -> get_validateOnLoad(This,validateOnLoad) ) 

#define IXMLDOMSchemaCollection2_getSchema(This,namespaceURI,schema)	\
    ( (This)->lpVtbl -> getSchema(This,namespaceURI,schema) ) 

#define IXMLDOMSchemaCollection2_getDeclaration(This,node,item)	\
    ( (This)->lpVtbl -> getDeclaration(This,node,item) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLDOMSchemaCollection2_INTERFACE_DEFINED__ */


#ifndef __ISchemaStringCollection_INTERFACE_DEFINED__
#define __ISchemaStringCollection_INTERFACE_DEFINED__

/* interface ISchemaStringCollection */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaStringCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b1-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaStringCollection : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ BSTR *bstr) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *length) = 0;
        
        virtual /* [propget][restricted][hidden][id] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [retval][out] */ IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaStringCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaStringCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaStringCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaStringCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaStringCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaStringCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaStringCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaStringCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaStringCollection, get_item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            ISchemaStringCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ BSTR *bstr);
        
        DECLSPEC_XFGVIRT(ISchemaStringCollection, get_length)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            ISchemaStringCollection * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(ISchemaStringCollection, get__newEnum)
        /* [propget][restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            ISchemaStringCollection * This,
            /* [retval][out] */ IUnknown **ppunk);
        
        END_INTERFACE
    } ISchemaStringCollectionVtbl;

    interface ISchemaStringCollection
    {
        CONST_VTBL struct ISchemaStringCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaStringCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaStringCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaStringCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaStringCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaStringCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaStringCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaStringCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaStringCollection_get_item(This,index,bstr)	\
    ( (This)->lpVtbl -> get_item(This,index,bstr) ) 

#define ISchemaStringCollection_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define ISchemaStringCollection_get__newEnum(This,ppunk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaStringCollection_INTERFACE_DEFINED__ */


#ifndef __ISchemaItemCollection_INTERFACE_DEFINED__
#define __ISchemaItemCollection_INTERFACE_DEFINED__

/* interface ISchemaItemCollection */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaItemCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b2-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaItemCollection : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_item( 
            /* [in] */ long index,
            /* [retval][out] */ ISchemaItem **item) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE itemByName( 
            /* [in] */ BSTR name,
            /* [retval][out] */ ISchemaItem **item) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE itemByQName( 
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ ISchemaItem **item) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ long *length) = 0;
        
        virtual /* [propget][restricted][hidden][id] */ HRESULT STDMETHODCALLTYPE get__newEnum( 
            /* [retval][out] */ IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaItemCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaItemCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaItemCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaItemCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaItemCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaItemCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaItemCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaItemCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItemCollection, get_item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_item )( 
            ISchemaItemCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ ISchemaItem **item);
        
        DECLSPEC_XFGVIRT(ISchemaItemCollection, itemByName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *itemByName )( 
            ISchemaItemCollection * This,
            /* [in] */ BSTR name,
            /* [retval][out] */ ISchemaItem **item);
        
        DECLSPEC_XFGVIRT(ISchemaItemCollection, itemByQName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *itemByQName )( 
            ISchemaItemCollection * This,
            /* [in] */ BSTR name,
            /* [in] */ BSTR namespaceURI,
            /* [retval][out] */ ISchemaItem **item);
        
        DECLSPEC_XFGVIRT(ISchemaItemCollection, get_length)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            ISchemaItemCollection * This,
            /* [retval][out] */ long *length);
        
        DECLSPEC_XFGVIRT(ISchemaItemCollection, get__newEnum)
        /* [propget][restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *get__newEnum )( 
            ISchemaItemCollection * This,
            /* [retval][out] */ IUnknown **ppunk);
        
        END_INTERFACE
    } ISchemaItemCollectionVtbl;

    interface ISchemaItemCollection
    {
        CONST_VTBL struct ISchemaItemCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaItemCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaItemCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaItemCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaItemCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaItemCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaItemCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaItemCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaItemCollection_get_item(This,index,item)	\
    ( (This)->lpVtbl -> get_item(This,index,item) ) 

#define ISchemaItemCollection_itemByName(This,name,item)	\
    ( (This)->lpVtbl -> itemByName(This,name,item) ) 

#define ISchemaItemCollection_itemByQName(This,name,namespaceURI,item)	\
    ( (This)->lpVtbl -> itemByQName(This,name,namespaceURI,item) ) 

#define ISchemaItemCollection_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define ISchemaItemCollection_get__newEnum(This,ppunk)	\
    ( (This)->lpVtbl -> get__newEnum(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaItemCollection_INTERFACE_DEFINED__ */


#ifndef __ISchemaItem_INTERFACE_DEFINED__
#define __ISchemaItem_INTERFACE_DEFINED__

/* interface ISchemaItem */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b3-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaItem : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_name( 
            /* [retval][out] */ BSTR *name) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_namespaceURI( 
            /* [retval][out] */ BSTR *namespaceURI) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_schema( 
            /* [retval][out] */ ISchema **schema) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_id( 
            /* [retval][out] */ BSTR *id) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_itemType( 
            /* [retval][out] */ SOMITEMTYPE *itemType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_unhandledAttributes( 
            /* [retval][out] */ IVBSAXAttributes **attributes) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE writeAnnotation( 
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaItem * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaItem * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaItem * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaItem * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaItem * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaItem * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaItem * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaItem * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaItem * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaItem * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        END_INTERFACE
    } ISchemaItemVtbl;

    interface ISchemaItem
    {
        CONST_VTBL struct ISchemaItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaItem_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaItem_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaItem_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaItem_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaItem_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaItem_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaItem_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaItem_INTERFACE_DEFINED__ */


#ifndef __ISchema_INTERFACE_DEFINED__
#define __ISchema_INTERFACE_DEFINED__

/* interface ISchema */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b4-dd1b-4664-9a50-c2f40f4bd79a")
    ISchema : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_targetNamespace( 
            /* [retval][out] */ BSTR *targetNamespace) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_version( 
            /* [retval][out] */ BSTR *version) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_types( 
            /* [retval][out] */ ISchemaItemCollection **types) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_elements( 
            /* [retval][out] */ ISchemaItemCollection **elements) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [retval][out] */ ISchemaItemCollection **attributes) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_attributeGroups( 
            /* [retval][out] */ ISchemaItemCollection **attributeGroups) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_modelGroups( 
            /* [retval][out] */ ISchemaItemCollection **modelGroups) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_notations( 
            /* [retval][out] */ ISchemaItemCollection **notations) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_schemaLocations( 
            /* [retval][out] */ ISchemaStringCollection **schemaLocations) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchema * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchema * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchema * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchema * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchema * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchema * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchema * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchema * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchema * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchema * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchema * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchema * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchema * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchema, get_targetNamespace)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_targetNamespace )( 
            ISchema * This,
            /* [retval][out] */ BSTR *targetNamespace);
        
        DECLSPEC_XFGVIRT(ISchema, get_version)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_version )( 
            ISchema * This,
            /* [retval][out] */ BSTR *version);
        
        DECLSPEC_XFGVIRT(ISchema, get_types)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_types )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **types);
        
        DECLSPEC_XFGVIRT(ISchema, get_elements)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_elements )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **elements);
        
        DECLSPEC_XFGVIRT(ISchema, get_attributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **attributes);
        
        DECLSPEC_XFGVIRT(ISchema, get_attributeGroups)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_attributeGroups )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **attributeGroups);
        
        DECLSPEC_XFGVIRT(ISchema, get_modelGroups)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_modelGroups )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **modelGroups);
        
        DECLSPEC_XFGVIRT(ISchema, get_notations)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_notations )( 
            ISchema * This,
            /* [retval][out] */ ISchemaItemCollection **notations);
        
        DECLSPEC_XFGVIRT(ISchema, get_schemaLocations)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schemaLocations )( 
            ISchema * This,
            /* [retval][out] */ ISchemaStringCollection **schemaLocations);
        
        END_INTERFACE
    } ISchemaVtbl;

    interface ISchema
    {
        CONST_VTBL struct ISchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchema_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchema_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchema_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchema_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchema_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchema_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchema_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchema_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchema_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchema_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchema_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchema_get_targetNamespace(This,targetNamespace)	\
    ( (This)->lpVtbl -> get_targetNamespace(This,targetNamespace) ) 

#define ISchema_get_version(This,version)	\
    ( (This)->lpVtbl -> get_version(This,version) ) 

#define ISchema_get_types(This,types)	\
    ( (This)->lpVtbl -> get_types(This,types) ) 

#define ISchema_get_elements(This,elements)	\
    ( (This)->lpVtbl -> get_elements(This,elements) ) 

#define ISchema_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 

#define ISchema_get_attributeGroups(This,attributeGroups)	\
    ( (This)->lpVtbl -> get_attributeGroups(This,attributeGroups) ) 

#define ISchema_get_modelGroups(This,modelGroups)	\
    ( (This)->lpVtbl -> get_modelGroups(This,modelGroups) ) 

#define ISchema_get_notations(This,notations)	\
    ( (This)->lpVtbl -> get_notations(This,notations) ) 

#define ISchema_get_schemaLocations(This,schemaLocations)	\
    ( (This)->lpVtbl -> get_schemaLocations(This,schemaLocations) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchema_INTERFACE_DEFINED__ */


#ifndef __ISchemaParticle_INTERFACE_DEFINED__
#define __ISchemaParticle_INTERFACE_DEFINED__

/* interface ISchemaParticle */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaParticle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b5-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaParticle : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_minOccurs( 
            /* [retval][out] */ VARIANT *minOccurs) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_maxOccurs( 
            /* [retval][out] */ VARIANT *maxOccurs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaParticleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaParticle * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaParticle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaParticle * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaParticle * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaParticle * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaParticle * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaParticle * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaParticle * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaParticle * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaParticle * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaParticle * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaParticle * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaParticle * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaParticle * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_minOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minOccurs )( 
            ISchemaParticle * This,
            /* [retval][out] */ VARIANT *minOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_maxOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxOccurs )( 
            ISchemaParticle * This,
            /* [retval][out] */ VARIANT *maxOccurs);
        
        END_INTERFACE
    } ISchemaParticleVtbl;

    interface ISchemaParticle
    {
        CONST_VTBL struct ISchemaParticleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaParticle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaParticle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaParticle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaParticle_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaParticle_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaParticle_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaParticle_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaParticle_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaParticle_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaParticle_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaParticle_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaParticle_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaParticle_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaParticle_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaParticle_get_minOccurs(This,minOccurs)	\
    ( (This)->lpVtbl -> get_minOccurs(This,minOccurs) ) 

#define ISchemaParticle_get_maxOccurs(This,maxOccurs)	\
    ( (This)->lpVtbl -> get_maxOccurs(This,maxOccurs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaParticle_INTERFACE_DEFINED__ */


#ifndef __ISchemaAttribute_INTERFACE_DEFINED__
#define __ISchemaAttribute_INTERFACE_DEFINED__

/* interface ISchemaAttribute */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaAttribute;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b6-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaAttribute : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_type( 
            /* [retval][out] */ ISchemaType **type) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_scope( 
            /* [retval][out] */ ISchemaComplexType **scope) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_defaultValue( 
            /* [retval][out] */ BSTR *defaultValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_fixedValue( 
            /* [retval][out] */ BSTR *fixedValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_use( 
            /* [retval][out] */ SCHEMAUSE *use) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_isReference( 
            /* [retval][out] */ VARIANT_BOOL *reference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaAttributeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaAttribute * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaAttribute * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaAttribute * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaAttribute * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaAttribute * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaAttribute * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaAttribute * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaAttribute * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaAttribute * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaAttribute * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaAttribute * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaAttribute * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaAttribute * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaAttribute * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_type)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_type )( 
            ISchemaAttribute * This,
            /* [retval][out] */ ISchemaType **type);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_scope)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_scope )( 
            ISchemaAttribute * This,
            /* [retval][out] */ ISchemaComplexType **scope);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_defaultValue)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_defaultValue )( 
            ISchemaAttribute * This,
            /* [retval][out] */ BSTR *defaultValue);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_fixedValue)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_fixedValue )( 
            ISchemaAttribute * This,
            /* [retval][out] */ BSTR *fixedValue);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_use)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_use )( 
            ISchemaAttribute * This,
            /* [retval][out] */ SCHEMAUSE *use);
        
        DECLSPEC_XFGVIRT(ISchemaAttribute, get_isReference)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_isReference )( 
            ISchemaAttribute * This,
            /* [retval][out] */ VARIANT_BOOL *reference);
        
        END_INTERFACE
    } ISchemaAttributeVtbl;

    interface ISchemaAttribute
    {
        CONST_VTBL struct ISchemaAttributeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaAttribute_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaAttribute_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaAttribute_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaAttribute_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaAttribute_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaAttribute_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaAttribute_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaAttribute_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaAttribute_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaAttribute_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaAttribute_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaAttribute_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaAttribute_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaAttribute_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaAttribute_get_type(This,type)	\
    ( (This)->lpVtbl -> get_type(This,type) ) 

#define ISchemaAttribute_get_scope(This,scope)	\
    ( (This)->lpVtbl -> get_scope(This,scope) ) 

#define ISchemaAttribute_get_defaultValue(This,defaultValue)	\
    ( (This)->lpVtbl -> get_defaultValue(This,defaultValue) ) 

#define ISchemaAttribute_get_fixedValue(This,fixedValue)	\
    ( (This)->lpVtbl -> get_fixedValue(This,fixedValue) ) 

#define ISchemaAttribute_get_use(This,use)	\
    ( (This)->lpVtbl -> get_use(This,use) ) 

#define ISchemaAttribute_get_isReference(This,reference)	\
    ( (This)->lpVtbl -> get_isReference(This,reference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaAttribute_INTERFACE_DEFINED__ */


#ifndef __ISchemaElement_INTERFACE_DEFINED__
#define __ISchemaElement_INTERFACE_DEFINED__

/* interface ISchemaElement */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b7-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaElement : public ISchemaParticle
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_type( 
            /* [retval][out] */ ISchemaType **type) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_scope( 
            /* [retval][out] */ ISchemaComplexType **scope) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_defaultValue( 
            /* [retval][out] */ BSTR *defaultValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_fixedValue( 
            /* [retval][out] */ BSTR *fixedValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_isNillable( 
            /* [retval][out] */ VARIANT_BOOL *nillable) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_identityConstraints( 
            /* [retval][out] */ ISchemaItemCollection **constraints) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_substitutionGroup( 
            /* [retval][out] */ ISchemaElement **element) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_substitutionGroupExclusions( 
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *exclusions) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_disallowedSubstitutions( 
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *disallowed) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_isAbstract( 
            /* [retval][out] */ VARIANT_BOOL *abstract) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_isReference( 
            /* [retval][out] */ VARIANT_BOOL *reference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaElement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaElement * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaElement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaElement * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaElement * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaElement * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaElement * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaElement * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaElement * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaElement * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaElement * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaElement * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_minOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minOccurs )( 
            ISchemaElement * This,
            /* [retval][out] */ VARIANT *minOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_maxOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxOccurs )( 
            ISchemaElement * This,
            /* [retval][out] */ VARIANT *maxOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_type)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_type )( 
            ISchemaElement * This,
            /* [retval][out] */ ISchemaType **type);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_scope)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_scope )( 
            ISchemaElement * This,
            /* [retval][out] */ ISchemaComplexType **scope);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_defaultValue)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_defaultValue )( 
            ISchemaElement * This,
            /* [retval][out] */ BSTR *defaultValue);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_fixedValue)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_fixedValue )( 
            ISchemaElement * This,
            /* [retval][out] */ BSTR *fixedValue);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_isNillable)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_isNillable )( 
            ISchemaElement * This,
            /* [retval][out] */ VARIANT_BOOL *nillable);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_identityConstraints)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_identityConstraints )( 
            ISchemaElement * This,
            /* [retval][out] */ ISchemaItemCollection **constraints);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_substitutionGroup)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_substitutionGroup )( 
            ISchemaElement * This,
            /* [retval][out] */ ISchemaElement **element);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_substitutionGroupExclusions)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_substitutionGroupExclusions )( 
            ISchemaElement * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *exclusions);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_disallowedSubstitutions)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_disallowedSubstitutions )( 
            ISchemaElement * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *disallowed);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_isAbstract)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_isAbstract )( 
            ISchemaElement * This,
            /* [retval][out] */ VARIANT_BOOL *abstract);
        
        DECLSPEC_XFGVIRT(ISchemaElement, get_isReference)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_isReference )( 
            ISchemaElement * This,
            /* [retval][out] */ VARIANT_BOOL *reference);
        
        END_INTERFACE
    } ISchemaElementVtbl;

    interface ISchemaElement
    {
        CONST_VTBL struct ISchemaElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaElement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaElement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaElement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaElement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaElement_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaElement_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaElement_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaElement_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaElement_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaElement_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaElement_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaElement_get_minOccurs(This,minOccurs)	\
    ( (This)->lpVtbl -> get_minOccurs(This,minOccurs) ) 

#define ISchemaElement_get_maxOccurs(This,maxOccurs)	\
    ( (This)->lpVtbl -> get_maxOccurs(This,maxOccurs) ) 


#define ISchemaElement_get_type(This,type)	\
    ( (This)->lpVtbl -> get_type(This,type) ) 

#define ISchemaElement_get_scope(This,scope)	\
    ( (This)->lpVtbl -> get_scope(This,scope) ) 

#define ISchemaElement_get_defaultValue(This,defaultValue)	\
    ( (This)->lpVtbl -> get_defaultValue(This,defaultValue) ) 

#define ISchemaElement_get_fixedValue(This,fixedValue)	\
    ( (This)->lpVtbl -> get_fixedValue(This,fixedValue) ) 

#define ISchemaElement_get_isNillable(This,nillable)	\
    ( (This)->lpVtbl -> get_isNillable(This,nillable) ) 

#define ISchemaElement_get_identityConstraints(This,constraints)	\
    ( (This)->lpVtbl -> get_identityConstraints(This,constraints) ) 

#define ISchemaElement_get_substitutionGroup(This,element)	\
    ( (This)->lpVtbl -> get_substitutionGroup(This,element) ) 

#define ISchemaElement_get_substitutionGroupExclusions(This,exclusions)	\
    ( (This)->lpVtbl -> get_substitutionGroupExclusions(This,exclusions) ) 

#define ISchemaElement_get_disallowedSubstitutions(This,disallowed)	\
    ( (This)->lpVtbl -> get_disallowedSubstitutions(This,disallowed) ) 

#define ISchemaElement_get_isAbstract(This,abstract)	\
    ( (This)->lpVtbl -> get_isAbstract(This,abstract) ) 

#define ISchemaElement_get_isReference(This,reference)	\
    ( (This)->lpVtbl -> get_isReference(This,reference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaElement_INTERFACE_DEFINED__ */


#ifndef __ISchemaType_INTERFACE_DEFINED__
#define __ISchemaType_INTERFACE_DEFINED__

/* interface ISchemaType */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b8-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaType : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_baseTypes( 
            /* [retval][out] */ ISchemaItemCollection **baseTypes) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_final( 
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *final) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_variety( 
            /* [retval][out] */ SCHEMATYPEVARIETY *variety) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_derivedBy( 
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *derivedBy) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE isValid( 
            /* [in] */ BSTR data,
            /* [retval][out] */ VARIANT_BOOL *valid) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_minExclusive( 
            /* [retval][out] */ BSTR *minExclusive) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_minInclusive( 
            /* [retval][out] */ BSTR *minInclusive) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_maxExclusive( 
            /* [retval][out] */ BSTR *maxExclusive) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_maxInclusive( 
            /* [retval][out] */ BSTR *maxInclusive) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_totalDigits( 
            /* [retval][out] */ VARIANT *totalDigits) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_fractionDigits( 
            /* [retval][out] */ VARIANT *fractionDigits) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_length( 
            /* [retval][out] */ VARIANT *length) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_minLength( 
            /* [retval][out] */ VARIANT *minLength) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_maxLength( 
            /* [retval][out] */ VARIANT *maxLength) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_enumeration( 
            /* [retval][out] */ ISchemaStringCollection **enumeration) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_whitespace( 
            /* [retval][out] */ SCHEMAWHITESPACE *whitespace) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_patterns( 
            /* [retval][out] */ ISchemaStringCollection **patterns) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaType * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaType * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaType * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaType * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaType * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaType * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaType * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaType * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_baseTypes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_baseTypes )( 
            ISchemaType * This,
            /* [retval][out] */ ISchemaItemCollection **baseTypes);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_final)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_final )( 
            ISchemaType * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *final);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_variety)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_variety )( 
            ISchemaType * This,
            /* [retval][out] */ SCHEMATYPEVARIETY *variety);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_derivedBy)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_derivedBy )( 
            ISchemaType * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *derivedBy);
        
        DECLSPEC_XFGVIRT(ISchemaType, isValid)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *isValid )( 
            ISchemaType * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ VARIANT_BOOL *valid);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minExclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minExclusive )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *minExclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minInclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minInclusive )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *minInclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxExclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxExclusive )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *maxExclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxInclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxInclusive )( 
            ISchemaType * This,
            /* [retval][out] */ BSTR *maxInclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_totalDigits)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_totalDigits )( 
            ISchemaType * This,
            /* [retval][out] */ VARIANT *totalDigits);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_fractionDigits)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_fractionDigits )( 
            ISchemaType * This,
            /* [retval][out] */ VARIANT *fractionDigits);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_length)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            ISchemaType * This,
            /* [retval][out] */ VARIANT *length);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minLength)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minLength )( 
            ISchemaType * This,
            /* [retval][out] */ VARIANT *minLength);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxLength)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxLength )( 
            ISchemaType * This,
            /* [retval][out] */ VARIANT *maxLength);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_enumeration)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_enumeration )( 
            ISchemaType * This,
            /* [retval][out] */ ISchemaStringCollection **enumeration);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_whitespace)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_whitespace )( 
            ISchemaType * This,
            /* [retval][out] */ SCHEMAWHITESPACE *whitespace);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_patterns)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_patterns )( 
            ISchemaType * This,
            /* [retval][out] */ ISchemaStringCollection **patterns);
        
        END_INTERFACE
    } ISchemaTypeVtbl;

    interface ISchemaType
    {
        CONST_VTBL struct ISchemaTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaType_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaType_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaType_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaType_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaType_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaType_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaType_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaType_get_baseTypes(This,baseTypes)	\
    ( (This)->lpVtbl -> get_baseTypes(This,baseTypes) ) 

#define ISchemaType_get_final(This,final)	\
    ( (This)->lpVtbl -> get_final(This,final) ) 

#define ISchemaType_get_variety(This,variety)	\
    ( (This)->lpVtbl -> get_variety(This,variety) ) 

#define ISchemaType_get_derivedBy(This,derivedBy)	\
    ( (This)->lpVtbl -> get_derivedBy(This,derivedBy) ) 

#define ISchemaType_isValid(This,data,valid)	\
    ( (This)->lpVtbl -> isValid(This,data,valid) ) 

#define ISchemaType_get_minExclusive(This,minExclusive)	\
    ( (This)->lpVtbl -> get_minExclusive(This,minExclusive) ) 

#define ISchemaType_get_minInclusive(This,minInclusive)	\
    ( (This)->lpVtbl -> get_minInclusive(This,minInclusive) ) 

#define ISchemaType_get_maxExclusive(This,maxExclusive)	\
    ( (This)->lpVtbl -> get_maxExclusive(This,maxExclusive) ) 

#define ISchemaType_get_maxInclusive(This,maxInclusive)	\
    ( (This)->lpVtbl -> get_maxInclusive(This,maxInclusive) ) 

#define ISchemaType_get_totalDigits(This,totalDigits)	\
    ( (This)->lpVtbl -> get_totalDigits(This,totalDigits) ) 

#define ISchemaType_get_fractionDigits(This,fractionDigits)	\
    ( (This)->lpVtbl -> get_fractionDigits(This,fractionDigits) ) 

#define ISchemaType_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define ISchemaType_get_minLength(This,minLength)	\
    ( (This)->lpVtbl -> get_minLength(This,minLength) ) 

#define ISchemaType_get_maxLength(This,maxLength)	\
    ( (This)->lpVtbl -> get_maxLength(This,maxLength) ) 

#define ISchemaType_get_enumeration(This,enumeration)	\
    ( (This)->lpVtbl -> get_enumeration(This,enumeration) ) 

#define ISchemaType_get_whitespace(This,whitespace)	\
    ( (This)->lpVtbl -> get_whitespace(This,whitespace) ) 

#define ISchemaType_get_patterns(This,patterns)	\
    ( (This)->lpVtbl -> get_patterns(This,patterns) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaType_INTERFACE_DEFINED__ */


#ifndef __ISchemaComplexType_INTERFACE_DEFINED__
#define __ISchemaComplexType_INTERFACE_DEFINED__

/* interface ISchemaComplexType */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaComplexType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08b9-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaComplexType : public ISchemaType
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_isAbstract( 
            /* [retval][out] */ VARIANT_BOOL *abstract) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_anyAttribute( 
            /* [retval][out] */ ISchemaAny **anyAttribute) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [retval][out] */ ISchemaItemCollection **attributes) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_contentType( 
            /* [retval][out] */ SCHEMACONTENTTYPE *contentType) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_contentModel( 
            /* [retval][out] */ ISchemaModelGroup **contentModel) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_prohibitedSubstitutions( 
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *prohibited) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaComplexTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaComplexType * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaComplexType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaComplexType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaComplexType * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaComplexType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaComplexType * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaComplexType * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaComplexType * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaComplexType * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_baseTypes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_baseTypes )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaItemCollection **baseTypes);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_final)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_final )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *final);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_variety)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_variety )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMATYPEVARIETY *variety);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_derivedBy)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_derivedBy )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *derivedBy);
        
        DECLSPEC_XFGVIRT(ISchemaType, isValid)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *isValid )( 
            ISchemaComplexType * This,
            /* [in] */ BSTR data,
            /* [retval][out] */ VARIANT_BOOL *valid);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minExclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minExclusive )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *minExclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minInclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minInclusive )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *minInclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxExclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxExclusive )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *maxExclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxInclusive)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxInclusive )( 
            ISchemaComplexType * This,
            /* [retval][out] */ BSTR *maxInclusive);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_totalDigits)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_totalDigits )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT *totalDigits);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_fractionDigits)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_fractionDigits )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT *fractionDigits);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_length)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_length )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT *length);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_minLength)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minLength )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT *minLength);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_maxLength)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxLength )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT *maxLength);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_enumeration)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_enumeration )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaStringCollection **enumeration);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_whitespace)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_whitespace )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMAWHITESPACE *whitespace);
        
        DECLSPEC_XFGVIRT(ISchemaType, get_patterns)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_patterns )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaStringCollection **patterns);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_isAbstract)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_isAbstract )( 
            ISchemaComplexType * This,
            /* [retval][out] */ VARIANT_BOOL *abstract);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_anyAttribute)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_anyAttribute )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaAny **anyAttribute);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_attributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaItemCollection **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_contentType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_contentType )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMACONTENTTYPE *contentType);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_contentModel)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_contentModel )( 
            ISchemaComplexType * This,
            /* [retval][out] */ ISchemaModelGroup **contentModel);
        
        DECLSPEC_XFGVIRT(ISchemaComplexType, get_prohibitedSubstitutions)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_prohibitedSubstitutions )( 
            ISchemaComplexType * This,
            /* [retval][out] */ SCHEMADERIVATIONMETHOD *prohibited);
        
        END_INTERFACE
    } ISchemaComplexTypeVtbl;

    interface ISchemaComplexType
    {
        CONST_VTBL struct ISchemaComplexTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaComplexType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaComplexType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaComplexType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaComplexType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaComplexType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaComplexType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaComplexType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaComplexType_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaComplexType_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaComplexType_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaComplexType_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaComplexType_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaComplexType_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaComplexType_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaComplexType_get_baseTypes(This,baseTypes)	\
    ( (This)->lpVtbl -> get_baseTypes(This,baseTypes) ) 

#define ISchemaComplexType_get_final(This,final)	\
    ( (This)->lpVtbl -> get_final(This,final) ) 

#define ISchemaComplexType_get_variety(This,variety)	\
    ( (This)->lpVtbl -> get_variety(This,variety) ) 

#define ISchemaComplexType_get_derivedBy(This,derivedBy)	\
    ( (This)->lpVtbl -> get_derivedBy(This,derivedBy) ) 

#define ISchemaComplexType_isValid(This,data,valid)	\
    ( (This)->lpVtbl -> isValid(This,data,valid) ) 

#define ISchemaComplexType_get_minExclusive(This,minExclusive)	\
    ( (This)->lpVtbl -> get_minExclusive(This,minExclusive) ) 

#define ISchemaComplexType_get_minInclusive(This,minInclusive)	\
    ( (This)->lpVtbl -> get_minInclusive(This,minInclusive) ) 

#define ISchemaComplexType_get_maxExclusive(This,maxExclusive)	\
    ( (This)->lpVtbl -> get_maxExclusive(This,maxExclusive) ) 

#define ISchemaComplexType_get_maxInclusive(This,maxInclusive)	\
    ( (This)->lpVtbl -> get_maxInclusive(This,maxInclusive) ) 

#define ISchemaComplexType_get_totalDigits(This,totalDigits)	\
    ( (This)->lpVtbl -> get_totalDigits(This,totalDigits) ) 

#define ISchemaComplexType_get_fractionDigits(This,fractionDigits)	\
    ( (This)->lpVtbl -> get_fractionDigits(This,fractionDigits) ) 

#define ISchemaComplexType_get_length(This,length)	\
    ( (This)->lpVtbl -> get_length(This,length) ) 

#define ISchemaComplexType_get_minLength(This,minLength)	\
    ( (This)->lpVtbl -> get_minLength(This,minLength) ) 

#define ISchemaComplexType_get_maxLength(This,maxLength)	\
    ( (This)->lpVtbl -> get_maxLength(This,maxLength) ) 

#define ISchemaComplexType_get_enumeration(This,enumeration)	\
    ( (This)->lpVtbl -> get_enumeration(This,enumeration) ) 

#define ISchemaComplexType_get_whitespace(This,whitespace)	\
    ( (This)->lpVtbl -> get_whitespace(This,whitespace) ) 

#define ISchemaComplexType_get_patterns(This,patterns)	\
    ( (This)->lpVtbl -> get_patterns(This,patterns) ) 


#define ISchemaComplexType_get_isAbstract(This,abstract)	\
    ( (This)->lpVtbl -> get_isAbstract(This,abstract) ) 

#define ISchemaComplexType_get_anyAttribute(This,anyAttribute)	\
    ( (This)->lpVtbl -> get_anyAttribute(This,anyAttribute) ) 

#define ISchemaComplexType_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 

#define ISchemaComplexType_get_contentType(This,contentType)	\
    ( (This)->lpVtbl -> get_contentType(This,contentType) ) 

#define ISchemaComplexType_get_contentModel(This,contentModel)	\
    ( (This)->lpVtbl -> get_contentModel(This,contentModel) ) 

#define ISchemaComplexType_get_prohibitedSubstitutions(This,prohibited)	\
    ( (This)->lpVtbl -> get_prohibitedSubstitutions(This,prohibited) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaComplexType_INTERFACE_DEFINED__ */


#ifndef __ISchemaAttributeGroup_INTERFACE_DEFINED__
#define __ISchemaAttributeGroup_INTERFACE_DEFINED__

/* interface ISchemaAttributeGroup */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaAttributeGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08ba-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaAttributeGroup : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_anyAttribute( 
            /* [retval][out] */ ISchemaAny **anyAttribute) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_attributes( 
            /* [retval][out] */ ISchemaItemCollection **attributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaAttributeGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaAttributeGroup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaAttributeGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaAttributeGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaAttributeGroup * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaAttributeGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaAttributeGroup * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaAttributeGroup * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaAttributeGroup * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaAttributeGroup, get_anyAttribute)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_anyAttribute )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ ISchemaAny **anyAttribute);
        
        DECLSPEC_XFGVIRT(ISchemaAttributeGroup, get_attributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_attributes )( 
            ISchemaAttributeGroup * This,
            /* [retval][out] */ ISchemaItemCollection **attributes);
        
        END_INTERFACE
    } ISchemaAttributeGroupVtbl;

    interface ISchemaAttributeGroup
    {
        CONST_VTBL struct ISchemaAttributeGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaAttributeGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaAttributeGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaAttributeGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaAttributeGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaAttributeGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaAttributeGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaAttributeGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaAttributeGroup_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaAttributeGroup_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaAttributeGroup_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaAttributeGroup_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaAttributeGroup_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaAttributeGroup_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaAttributeGroup_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaAttributeGroup_get_anyAttribute(This,anyAttribute)	\
    ( (This)->lpVtbl -> get_anyAttribute(This,anyAttribute) ) 

#define ISchemaAttributeGroup_get_attributes(This,attributes)	\
    ( (This)->lpVtbl -> get_attributes(This,attributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaAttributeGroup_INTERFACE_DEFINED__ */


#ifndef __ISchemaModelGroup_INTERFACE_DEFINED__
#define __ISchemaModelGroup_INTERFACE_DEFINED__

/* interface ISchemaModelGroup */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaModelGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08bb-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaModelGroup : public ISchemaParticle
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_particles( 
            /* [retval][out] */ ISchemaItemCollection **particles) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaModelGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaModelGroup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaModelGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaModelGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaModelGroup * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaModelGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaModelGroup * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaModelGroup * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaModelGroup * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_minOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minOccurs )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ VARIANT *minOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_maxOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxOccurs )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ VARIANT *maxOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaModelGroup, get_particles)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_particles )( 
            ISchemaModelGroup * This,
            /* [retval][out] */ ISchemaItemCollection **particles);
        
        END_INTERFACE
    } ISchemaModelGroupVtbl;

    interface ISchemaModelGroup
    {
        CONST_VTBL struct ISchemaModelGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaModelGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaModelGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaModelGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaModelGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaModelGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaModelGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaModelGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaModelGroup_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaModelGroup_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaModelGroup_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaModelGroup_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaModelGroup_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaModelGroup_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaModelGroup_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaModelGroup_get_minOccurs(This,minOccurs)	\
    ( (This)->lpVtbl -> get_minOccurs(This,minOccurs) ) 

#define ISchemaModelGroup_get_maxOccurs(This,maxOccurs)	\
    ( (This)->lpVtbl -> get_maxOccurs(This,maxOccurs) ) 


#define ISchemaModelGroup_get_particles(This,particles)	\
    ( (This)->lpVtbl -> get_particles(This,particles) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaModelGroup_INTERFACE_DEFINED__ */


#ifndef __ISchemaAny_INTERFACE_DEFINED__
#define __ISchemaAny_INTERFACE_DEFINED__

/* interface ISchemaAny */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaAny;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08bc-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaAny : public ISchemaParticle
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_namespaces( 
            /* [retval][out] */ ISchemaStringCollection **namespaces) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_processContents( 
            /* [retval][out] */ SCHEMAPROCESSCONTENTS *processContents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaAnyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaAny * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaAny * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaAny * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaAny * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaAny * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaAny * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaAny * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaAny * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaAny * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaAny * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaAny * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaAny * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaAny * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaAny * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_minOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_minOccurs )( 
            ISchemaAny * This,
            /* [retval][out] */ VARIANT *minOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaParticle, get_maxOccurs)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_maxOccurs )( 
            ISchemaAny * This,
            /* [retval][out] */ VARIANT *maxOccurs);
        
        DECLSPEC_XFGVIRT(ISchemaAny, get_namespaces)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaces )( 
            ISchemaAny * This,
            /* [retval][out] */ ISchemaStringCollection **namespaces);
        
        DECLSPEC_XFGVIRT(ISchemaAny, get_processContents)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_processContents )( 
            ISchemaAny * This,
            /* [retval][out] */ SCHEMAPROCESSCONTENTS *processContents);
        
        END_INTERFACE
    } ISchemaAnyVtbl;

    interface ISchemaAny
    {
        CONST_VTBL struct ISchemaAnyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaAny_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaAny_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaAny_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaAny_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaAny_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaAny_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaAny_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaAny_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaAny_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaAny_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaAny_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaAny_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaAny_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaAny_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaAny_get_minOccurs(This,minOccurs)	\
    ( (This)->lpVtbl -> get_minOccurs(This,minOccurs) ) 

#define ISchemaAny_get_maxOccurs(This,maxOccurs)	\
    ( (This)->lpVtbl -> get_maxOccurs(This,maxOccurs) ) 


#define ISchemaAny_get_namespaces(This,namespaces)	\
    ( (This)->lpVtbl -> get_namespaces(This,namespaces) ) 

#define ISchemaAny_get_processContents(This,processContents)	\
    ( (This)->lpVtbl -> get_processContents(This,processContents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaAny_INTERFACE_DEFINED__ */


#ifndef __ISchemaIdentityConstraint_INTERFACE_DEFINED__
#define __ISchemaIdentityConstraint_INTERFACE_DEFINED__

/* interface ISchemaIdentityConstraint */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaIdentityConstraint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08bd-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaIdentityConstraint : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_selector( 
            /* [retval][out] */ BSTR *selector) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_fields( 
            /* [retval][out] */ ISchemaStringCollection **fields) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_referencedKey( 
            /* [retval][out] */ ISchemaIdentityConstraint **key) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaIdentityConstraintVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaIdentityConstraint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaIdentityConstraint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaIdentityConstraint * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaIdentityConstraint * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaIdentityConstraint * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaIdentityConstraint * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaIdentityConstraint * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaIdentityConstraint * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaIdentityConstraint, get_selector)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_selector )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ BSTR *selector);
        
        DECLSPEC_XFGVIRT(ISchemaIdentityConstraint, get_fields)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_fields )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ ISchemaStringCollection **fields);
        
        DECLSPEC_XFGVIRT(ISchemaIdentityConstraint, get_referencedKey)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_referencedKey )( 
            ISchemaIdentityConstraint * This,
            /* [retval][out] */ ISchemaIdentityConstraint **key);
        
        END_INTERFACE
    } ISchemaIdentityConstraintVtbl;

    interface ISchemaIdentityConstraint
    {
        CONST_VTBL struct ISchemaIdentityConstraintVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaIdentityConstraint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaIdentityConstraint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaIdentityConstraint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaIdentityConstraint_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaIdentityConstraint_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaIdentityConstraint_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaIdentityConstraint_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaIdentityConstraint_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaIdentityConstraint_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaIdentityConstraint_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaIdentityConstraint_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaIdentityConstraint_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaIdentityConstraint_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaIdentityConstraint_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaIdentityConstraint_get_selector(This,selector)	\
    ( (This)->lpVtbl -> get_selector(This,selector) ) 

#define ISchemaIdentityConstraint_get_fields(This,fields)	\
    ( (This)->lpVtbl -> get_fields(This,fields) ) 

#define ISchemaIdentityConstraint_get_referencedKey(This,key)	\
    ( (This)->lpVtbl -> get_referencedKey(This,key) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaIdentityConstraint_INTERFACE_DEFINED__ */


#ifndef __ISchemaNotation_INTERFACE_DEFINED__
#define __ISchemaNotation_INTERFACE_DEFINED__

/* interface ISchemaNotation */
/* [unique][helpstring][nonextensible][oleautomation][dual][uuid][object][local] */ 


EXTERN_C const IID IID_ISchemaNotation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50ea08be-dd1b-4664-9a50-c2f40f4bd79a")
    ISchemaNotation : public ISchemaItem
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_systemIdentifier( 
            /* [retval][out] */ BSTR *uri) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_publicIdentifier( 
            /* [retval][out] */ BSTR *uri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISchemaNotationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISchemaNotation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISchemaNotation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISchemaNotation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ISchemaNotation * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ISchemaNotation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ISchemaNotation * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISchemaNotation * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_name)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_name )( 
            ISchemaNotation * This,
            /* [retval][out] */ BSTR *name);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_namespaceURI)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_namespaceURI )( 
            ISchemaNotation * This,
            /* [retval][out] */ BSTR *namespaceURI);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_schema)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_schema )( 
            ISchemaNotation * This,
            /* [retval][out] */ ISchema **schema);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_id)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            ISchemaNotation * This,
            /* [retval][out] */ BSTR *id);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_itemType)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_itemType )( 
            ISchemaNotation * This,
            /* [retval][out] */ SOMITEMTYPE *itemType);
        
        DECLSPEC_XFGVIRT(ISchemaItem, get_unhandledAttributes)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_unhandledAttributes )( 
            ISchemaNotation * This,
            /* [retval][out] */ IVBSAXAttributes **attributes);
        
        DECLSPEC_XFGVIRT(ISchemaItem, writeAnnotation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *writeAnnotation )( 
            ISchemaNotation * This,
            /* [in] */ IUnknown *annotationSink,
            /* [retval][out] */ VARIANT_BOOL *isWritten);
        
        DECLSPEC_XFGVIRT(ISchemaNotation, get_systemIdentifier)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_systemIdentifier )( 
            ISchemaNotation * This,
            /* [retval][out] */ BSTR *uri);
        
        DECLSPEC_XFGVIRT(ISchemaNotation, get_publicIdentifier)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_publicIdentifier )( 
            ISchemaNotation * This,
            /* [retval][out] */ BSTR *uri);
        
        END_INTERFACE
    } ISchemaNotationVtbl;

    interface ISchemaNotation
    {
        CONST_VTBL struct ISchemaNotationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISchemaNotation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISchemaNotation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISchemaNotation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISchemaNotation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISchemaNotation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISchemaNotation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISchemaNotation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISchemaNotation_get_name(This,name)	\
    ( (This)->lpVtbl -> get_name(This,name) ) 

#define ISchemaNotation_get_namespaceURI(This,namespaceURI)	\
    ( (This)->lpVtbl -> get_namespaceURI(This,namespaceURI) ) 

#define ISchemaNotation_get_schema(This,schema)	\
    ( (This)->lpVtbl -> get_schema(This,schema) ) 

#define ISchemaNotation_get_id(This,id)	\
    ( (This)->lpVtbl -> get_id(This,id) ) 

#define ISchemaNotation_get_itemType(This,itemType)	\
    ( (This)->lpVtbl -> get_itemType(This,itemType) ) 

#define ISchemaNotation_get_unhandledAttributes(This,attributes)	\
    ( (This)->lpVtbl -> get_unhandledAttributes(This,attributes) ) 

#define ISchemaNotation_writeAnnotation(This,annotationSink,isWritten)	\
    ( (This)->lpVtbl -> writeAnnotation(This,annotationSink,isWritten) ) 


#define ISchemaNotation_get_systemIdentifier(This,uri)	\
    ( (This)->lpVtbl -> get_systemIdentifier(This,uri) ) 

#define ISchemaNotation_get_publicIdentifier(This,uri)	\
    ( (This)->lpVtbl -> get_publicIdentifier(This,uri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISchemaNotation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msxml6_0000_0074 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0074_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0074_v0_0_s_ifspec;


#ifndef __MSXML2_LIBRARY_DEFINED__
#define __MSXML2_LIBRARY_DEFINED__

/* library MSXML2 */
/* [lcid][helpstring][version][uuid] */ 























typedef /* [hidden] */ struct __msxml6_ReferenceRemainingTypes__
    {
    enum tagDOMNodeType __tagDomNodeType__;
    DOMNodeType __domNodeType__;
    enum _SERVERXMLHTTP_OPTION __serverXmlHttpOptionEnum__;
    SERVERXMLHTTP_OPTION __serverXmlHttpOption__;
    enum _SXH_SERVER_CERT_OPTION __serverCertOptionEnum__;
    SXH_SERVER_CERT_OPTION __serverCertOption__;
    enum _SXH_PROXY_SETTING __proxySettingEnum__;
    SXH_PROXY_SETTING __proxySetting__;
    enum _SOMITEMTYPE __somItemTypeEnum__;
    SOMITEMTYPE __somItemType__;
    enum _SCHEMAUSE __schemaUseEnum__;
    SCHEMAUSE __schemaUse__;
    enum _SCHEMADERIVATIONMETHOD __schemaDerivationMethodEnum__;
    SCHEMADERIVATIONMETHOD __schemaDerivationMethod__;
    enum _SCHEMACONTENTTYPE __schemaContentTypeEnum__;
    SCHEMACONTENTTYPE __schemaContentType__;
    enum _SCHEMAPROCESSCONTENTS __schemaProcessContentsEnum__;
    SCHEMAPROCESSCONTENTS __schemaProcessContents__;
    enum _SCHEMAWHITESPACE __schemaWhitespaceEnum__;
    SCHEMAWHITESPACE __schemaWhitespace__;
    enum _SCHEMATYPEVARIETY __schemaTypeVarietyEnum__;
    SCHEMATYPEVARIETY __schemaTypeVariety__;
    } 	__msxml6_ReferenceRemainingTypes__;

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#define CLSID_XmlHttpRequest CLSID_FreeThreadedXMLHTTP60
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

EXTERN_C const IID LIBID_MSXML2;

#ifndef __XMLDOMDocumentEvents_DISPINTERFACE_DEFINED__
#define __XMLDOMDocumentEvents_DISPINTERFACE_DEFINED__

/* dispinterface XMLDOMDocumentEvents */
/* [uuid][hidden] */ 


EXTERN_C const IID DIID_XMLDOMDocumentEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("3efaa427-272f-11d2-836f-0000f87a7782")
    XMLDOMDocumentEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct XMLDOMDocumentEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in XMLDOMDocumentEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in XMLDOMDocumentEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in XMLDOMDocumentEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in XMLDOMDocumentEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in XMLDOMDocumentEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in XMLDOMDocumentEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            XMLDOMDocumentEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } XMLDOMDocumentEventsVtbl;

    interface XMLDOMDocumentEvents
    {
        CONST_VTBL struct XMLDOMDocumentEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define XMLDOMDocumentEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define XMLDOMDocumentEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define XMLDOMDocumentEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define XMLDOMDocumentEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define XMLDOMDocumentEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define XMLDOMDocumentEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define XMLDOMDocumentEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __XMLDOMDocumentEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_DOMDocument60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a05-f192-11d4-a65f-0040963251e5")
DOMDocument60;
#endif

EXTERN_C const CLSID CLSID_FreeThreadedDOMDocument60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a06-f192-11d4-a65f-0040963251e5")
FreeThreadedDOMDocument60;
#endif

EXTERN_C const CLSID CLSID_XMLSchemaCache60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a07-f192-11d4-a65f-0040963251e5")
XMLSchemaCache60;
#endif

EXTERN_C const CLSID CLSID_XSLTemplate60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a08-f192-11d4-a65f-0040963251e5")
XSLTemplate60;
#endif

EXTERN_C const CLSID CLSID_XMLHTTP60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a0a-f192-11d4-a65f-0040963251e5")
XMLHTTP60;
#endif

EXTERN_C const CLSID CLSID_FreeThreadedXMLHTTP60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a09-f192-11d4-a65f-0040963251e5")
FreeThreadedXMLHTTP60;
#endif

EXTERN_C const CLSID CLSID_ServerXMLHTTP60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a0b-f192-11d4-a65f-0040963251e5")
ServerXMLHTTP60;
#endif

EXTERN_C const CLSID CLSID_SAXXMLReader60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a0c-f192-11d4-a65f-0040963251e5")
SAXXMLReader60;
#endif

EXTERN_C const CLSID CLSID_MXXMLWriter60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a0f-f192-11d4-a65f-0040963251e5")
MXXMLWriter60;
#endif

EXTERN_C const CLSID CLSID_MXHTMLWriter60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a10-f192-11d4-a65f-0040963251e5")
MXHTMLWriter60;
#endif

EXTERN_C const CLSID CLSID_SAXAttributes60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a0e-f192-11d4-a65f-0040963251e5")
SAXAttributes60;
#endif

EXTERN_C const CLSID CLSID_MXNamespaceManager60;

#ifdef __cplusplus

class DECLSPEC_UUID("88d96a11-f192-11d4-a65f-0040963251e5")
MXNamespaceManager60;
#endif
#endif /* __MSXML2_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msxml6_0000_0075 */
/* [local] */ 

//----------------------------
// MSXML SPECIFIC ERROR CODES 
//----------------------------
#define E_XML_NOTWF                0xC00CE223L  // Validate failed because the document is not well formed.
#define E_XML_NODTD                0xC00CE224L  // The node is neither Valid nor Invalid because no DTD/Schema declaration was found.
#define E_XML_INVALID              0xC00CE225L  // Validate failed because of a DTD/Schema violation.
#define E_XML_BUFFERTOOSMALL       0xC00CE226L  // Buffer passed in is too small to receive the data.
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if !defined(__msxml_xhr_enums)
#define __msxml_xhr_enums
typedef /* [v1_enum][helpstring] */ 
enum _XHR_COOKIE_STATE
    {
        XHR_COOKIE_STATE_UNKNOWN	= 0,
        XHR_COOKIE_STATE_ACCEPT	= 0x1,
        XHR_COOKIE_STATE_PROMPT	= 0x2,
        XHR_COOKIE_STATE_LEASH	= 0x3,
        XHR_COOKIE_STATE_DOWNGRADE	= 0x4,
        XHR_COOKIE_STATE_REJECT	= 0x5
    } 	XHR_COOKIE_STATE;

typedef /* [v1_enum][helpstring] */ 
enum _XHR_COOKIE_FLAG
    {
        XHR_COOKIE_IS_SECURE	= 0x1,
        XHR_COOKIE_IS_SESSION	= 0x2,
        XHR_COOKIE_THIRD_PARTY	= 0x10,
        XHR_COOKIE_PROMPT_REQUIRED	= 0x20,
        XHR_COOKIE_EVALUATE_P3P	= 0x40,
        XHR_COOKIE_APPLY_P3P	= 0x80,
        XHR_COOKIE_P3P_ENABLED	= 0x100,
        XHR_COOKIE_IS_RESTRICTED	= 0x200,
        XHR_COOKIE_IE6	= 0x400,
        XHR_COOKIE_IS_LEGACY	= 0x800,
        XHR_COOKIE_NON_SCRIPT	= 0x1000,
        XHR_COOKIE_HTTPONLY	= 0x2000
    } 	XHR_COOKIE_FLAG;

typedef /* [v1_enum][helpstring] */ 
enum _XHR_CRED_PROMPT
    {
        XHR_CRED_PROMPT_ALL	= 0,
        XHR_CRED_PROMPT_NONE	= 0x1,
        XHR_CRED_PROMPT_PROXY	= 0x2
    } 	XHR_CRED_PROMPT;

typedef /* [v1_enum][helpstring] */ 
enum _XHR_AUTH
    {
        XHR_AUTH_ALL	= 0,
        XHR_AUTH_NONE	= 0x1,
        XHR_AUTH_PROXY	= 0x2
    } 	XHR_AUTH;

typedef /* [v1_enum][helpstring] */ 
enum _XHR_PROPERTY
    {
        XHR_PROP_NO_CRED_PROMPT	= 0,
        XHR_PROP_NO_AUTH	= 0x1,
        XHR_PROP_TIMEOUT	= 0x2,
        XHR_PROP_NO_DEFAULT_HEADERS	= 0x3,
        XHR_PROP_REPORT_REDIRECT_STATUS	= 0x4,
        XHR_PROP_NO_CACHE	= 0x5,
        XHR_PROP_EXTENDED_ERROR	= 0x6,
        XHR_PROP_QUERY_STRING_UTF8	= 0x7,
        XHR_PROP_IGNORE_CERT_ERRORS	= 0x8,
        XHR_PROP_ONDATA_THRESHOLD	= 0x9,
        XHR_PROP_SET_ENTERPRISEID	= 0xa,
        XHR_PROP_MAX_CONNECTIONS	= 0xb
    } 	XHR_PROPERTY;

#define XHR_PROP_ONDATA_ALWAYS 0x0
#define XHR_PROP_ONDATA_NEVER 0xFFFFFFFFFFFFFFFF
typedef /* [v1_enum][helpstring] */ 
enum _XHR_CERT_IGNORE_FLAG
    {
        XHR_CERT_IGNORE_REVOCATION_FAILED	= 0x80UL,
        XHR_CERT_IGNORE_UNKNOWN_CA	= 0x100UL,
        XHR_CERT_IGNORE_CERT_CN_INVALID	= 0x1000UL,
        XHR_CERT_IGNORE_CERT_DATE_INVALID	= 0x2000UL,
        XHR_CERT_IGNORE_ALL_SERVER_ERRORS	= ( ( ( XHR_CERT_IGNORE_REVOCATION_FAILED | XHR_CERT_IGNORE_UNKNOWN_CA )  | XHR_CERT_IGNORE_CERT_CN_INVALID )  | XHR_CERT_IGNORE_CERT_DATE_INVALID ) 
    } 	XHR_CERT_IGNORE_FLAG;

typedef /* [v1_enum][helpstring] */ 
enum _XHR_CERT_ERROR_FLAG
    {
        XHR_CERT_ERROR_REVOCATION_FAILED	= 0x800000UL,
        XHR_CERT_ERROR_UNKNOWN_CA	= 0x1000000UL,
        XHR_CERT_ERROR_CERT_CN_INVALID	= 0x2000000UL,
        XHR_CERT_ERROR_CERT_DATE_INVALID	= 0x4000000UL,
        XHR_CERT_ERROR_ALL_SERVER_ERRORS	= ( ( ( XHR_CERT_ERROR_REVOCATION_FAILED | XHR_CERT_ERROR_UNKNOWN_CA )  | XHR_CERT_ERROR_CERT_CN_INVALID )  | XHR_CERT_ERROR_CERT_DATE_INVALID ) 
    } 	XHR_CERT_ERROR_FLAG;

#endif // !defined(__msxml_xhr_enums)
typedef struct tagXHR_COOKIE
    {
    /* [string][ref] */ WCHAR *pwszUrl;
    /* [string][ref] */ WCHAR *pwszName;
    /* [string][unique] */ WCHAR *pwszValue;
    /* [string][unique] */ WCHAR *pwszP3PPolicy;
    FILETIME ftExpires;
    DWORD dwFlags;
    } 	XHR_COOKIE;




extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0075_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0075_v0_0_s_ifspec;

#ifndef __IXMLHTTPRequest2Callback_INTERFACE_DEFINED__
#define __IXMLHTTPRequest2Callback_INTERFACE_DEFINED__

/* interface IXMLHTTPRequest2Callback */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXMLHTTPRequest2Callback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A44A9299-E321-40DE-8866-341B41669162")
    IXMLHTTPRequest2Callback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnRedirect( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszRedirectUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnHeadersAvailable( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ DWORD dwStatus,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDataAvailable( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResponseReceived( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnError( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ HRESULT hrError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLHTTPRequest2CallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXMLHTTPRequest2Callback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXMLHTTPRequest2Callback * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnRedirect)
        HRESULT ( STDMETHODCALLTYPE *OnRedirect )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszRedirectUrl);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnHeadersAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnHeadersAvailable )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ DWORD dwStatus,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnDataAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnDataAvailable )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnResponseReceived)
        HRESULT ( STDMETHODCALLTYPE *OnResponseReceived )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnError)
        HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in IXMLHTTPRequest2Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ HRESULT hrError);
        
        END_INTERFACE
    } IXMLHTTPRequest2CallbackVtbl;

    interface IXMLHTTPRequest2Callback
    {
        CONST_VTBL struct IXMLHTTPRequest2CallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLHTTPRequest2Callback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLHTTPRequest2Callback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLHTTPRequest2Callback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLHTTPRequest2Callback_OnRedirect(This,pXHR,pwszRedirectUrl)	\
    ( (This)->lpVtbl -> OnRedirect(This,pXHR,pwszRedirectUrl) ) 

#define IXMLHTTPRequest2Callback_OnHeadersAvailable(This,pXHR,dwStatus,pwszStatus)	\
    ( (This)->lpVtbl -> OnHeadersAvailable(This,pXHR,dwStatus,pwszStatus) ) 

#define IXMLHTTPRequest2Callback_OnDataAvailable(This,pXHR,pResponseStream)	\
    ( (This)->lpVtbl -> OnDataAvailable(This,pXHR,pResponseStream) ) 

#define IXMLHTTPRequest2Callback_OnResponseReceived(This,pXHR,pResponseStream)	\
    ( (This)->lpVtbl -> OnResponseReceived(This,pXHR,pResponseStream) ) 

#define IXMLHTTPRequest2Callback_OnError(This,pXHR,hrError)	\
    ( (This)->lpVtbl -> OnError(This,pXHR,hrError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLHTTPRequest2Callback_INTERFACE_DEFINED__ */


#ifndef __IXMLHTTPRequest2_INTERFACE_DEFINED__
#define __IXMLHTTPRequest2_INTERFACE_DEFINED__

/* interface IXMLHTTPRequest2 */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXMLHTTPRequest2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E5D37DC0-552A-4D52-9CC0-A14D546FBD04")
    IXMLHTTPRequest2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszMethod,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2Callback *pStatusCallback,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszPassword,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyPassword) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Send( 
            /* [unique][in] */ __RPC__in_opt ISequentialStream *pBody,
            /* [in] */ ULONGLONG cbBody) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCookie( 
            /* [ref][in] */ __RPC__in const XHR_COOKIE *pCookie,
            /* [out] */ __RPC__out DWORD *pdwCookieState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustomResponseStream( 
            /* [in] */ __RPC__in_opt ISequentialStream *pSequentialStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ XHR_PROPERTY eProperty,
            /* [in] */ ULONGLONG ullValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRequestHeader( 
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllResponseHeaders( 
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCookie( 
            /* [string][ref][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [string][unique][in] */ __RPC__in_opt_string const WCHAR *pwszName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out ULONG *pcCookies,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCookies) XHR_COOKIE **ppCookies) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResponseHeader( 
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLHTTPRequest2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszMethod,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2Callback *pStatusCallback,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszPassword,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyPassword);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Send)
        HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [unique][in] */ __RPC__in_opt ISequentialStream *pBody,
            /* [in] */ ULONGLONG cbBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IXMLHTTPRequest2 * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetCookie)
        HRESULT ( STDMETHODCALLTYPE *SetCookie )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [ref][in] */ __RPC__in const XHR_COOKIE *pCookie,
            /* [out] */ __RPC__out DWORD *pdwCookieState);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetCustomResponseStream)
        HRESULT ( STDMETHODCALLTYPE *SetCustomResponseStream )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [in] */ __RPC__in_opt ISequentialStream *pSequentialStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [in] */ XHR_PROPERTY eProperty,
            /* [in] */ ULONGLONG ullValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetRequestHeader)
        HRESULT ( STDMETHODCALLTYPE *SetRequestHeader )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetAllResponseHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetAllResponseHeaders )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszHeaders);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetCookie)
        HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [string][ref][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [string][unique][in] */ __RPC__in_opt_string const WCHAR *pwszName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out ULONG *pcCookies,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCookies) XHR_COOKIE **ppCookies);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetResponseHeader)
        HRESULT ( STDMETHODCALLTYPE *GetResponseHeader )( 
            __RPC__in IXMLHTTPRequest2 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszValue);
        
        END_INTERFACE
    } IXMLHTTPRequest2Vtbl;

    interface IXMLHTTPRequest2
    {
        CONST_VTBL struct IXMLHTTPRequest2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLHTTPRequest2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLHTTPRequest2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLHTTPRequest2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLHTTPRequest2_Open(This,pwszMethod,pwszUrl,pStatusCallback,pwszUserName,pwszPassword,pwszProxyUserName,pwszProxyPassword)	\
    ( (This)->lpVtbl -> Open(This,pwszMethod,pwszUrl,pStatusCallback,pwszUserName,pwszPassword,pwszProxyUserName,pwszProxyPassword) ) 

#define IXMLHTTPRequest2_Send(This,pBody,cbBody)	\
    ( (This)->lpVtbl -> Send(This,pBody,cbBody) ) 

#define IXMLHTTPRequest2_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IXMLHTTPRequest2_SetCookie(This,pCookie,pdwCookieState)	\
    ( (This)->lpVtbl -> SetCookie(This,pCookie,pdwCookieState) ) 

#define IXMLHTTPRequest2_SetCustomResponseStream(This,pSequentialStream)	\
    ( (This)->lpVtbl -> SetCustomResponseStream(This,pSequentialStream) ) 

#define IXMLHTTPRequest2_SetProperty(This,eProperty,ullValue)	\
    ( (This)->lpVtbl -> SetProperty(This,eProperty,ullValue) ) 

#define IXMLHTTPRequest2_SetRequestHeader(This,pwszHeader,pwszValue)	\
    ( (This)->lpVtbl -> SetRequestHeader(This,pwszHeader,pwszValue) ) 

#define IXMLHTTPRequest2_GetAllResponseHeaders(This,ppwszHeaders)	\
    ( (This)->lpVtbl -> GetAllResponseHeaders(This,ppwszHeaders) ) 

#define IXMLHTTPRequest2_GetCookie(This,pwszUrl,pwszName,dwFlags,pcCookies,ppCookies)	\
    ( (This)->lpVtbl -> GetCookie(This,pwszUrl,pwszName,dwFlags,pcCookies,ppCookies) ) 

#define IXMLHTTPRequest2_GetResponseHeader(This,pwszHeader,ppwszValue)	\
    ( (This)->lpVtbl -> GetResponseHeader(This,pwszHeader,ppwszValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLHTTPRequest2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msxml6_0000_0077 */
/* [local] */ 

typedef struct tagXHR_CERT
    {
    DWORD cbCert;
    /* [size_is][ref] */ BYTE *pbCert;
    } 	XHR_CERT;




extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0077_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0077_v0_0_s_ifspec;

#ifndef __IXMLHTTPRequest3Callback_INTERFACE_DEFINED__
#define __IXMLHTTPRequest3Callback_INTERFACE_DEFINED__

/* interface IXMLHTTPRequest3Callback */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXMLHTTPRequest3Callback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b9e57830-8c6c-4a6f-9c13-47772bb047bb")
    IXMLHTTPRequest3Callback : public IXMLHTTPRequest2Callback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnServerCertificateReceived( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest3 *pXHR,
            /* [in] */ DWORD dwCertificateErrors,
            /* [in] */ DWORD cServerCertificateChain,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cServerCertificateChain) const XHR_CERT *rgServerCertificateChain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClientCertificateRequested( 
            /* [in] */ __RPC__in_opt IXMLHTTPRequest3 *pXHR,
            /* [in] */ DWORD cIssuerList,
            /* [size_is][unique][string][in] */ __RPC__in_ecount_full_opt(cIssuerList) const WCHAR **rgpwszIssuerList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLHTTPRequest3CallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXMLHTTPRequest3Callback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXMLHTTPRequest3Callback * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnRedirect)
        HRESULT ( STDMETHODCALLTYPE *OnRedirect )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszRedirectUrl);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnHeadersAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnHeadersAvailable )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ DWORD dwStatus,
            /* [string][in] */ __RPC__in_string const WCHAR *pwszStatus);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnDataAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnDataAvailable )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnResponseReceived)
        HRESULT ( STDMETHODCALLTYPE *OnResponseReceived )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ __RPC__in_opt ISequentialStream *pResponseStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2Callback, OnError)
        HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2 *pXHR,
            /* [in] */ HRESULT hrError);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest3Callback, OnServerCertificateReceived)
        HRESULT ( STDMETHODCALLTYPE *OnServerCertificateReceived )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest3 *pXHR,
            /* [in] */ DWORD dwCertificateErrors,
            /* [in] */ DWORD cServerCertificateChain,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cServerCertificateChain) const XHR_CERT *rgServerCertificateChain);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest3Callback, OnClientCertificateRequested)
        HRESULT ( STDMETHODCALLTYPE *OnClientCertificateRequested )( 
            __RPC__in IXMLHTTPRequest3Callback * This,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest3 *pXHR,
            /* [in] */ DWORD cIssuerList,
            /* [size_is][unique][string][in] */ __RPC__in_ecount_full_opt(cIssuerList) const WCHAR **rgpwszIssuerList);
        
        END_INTERFACE
    } IXMLHTTPRequest3CallbackVtbl;

    interface IXMLHTTPRequest3Callback
    {
        CONST_VTBL struct IXMLHTTPRequest3CallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLHTTPRequest3Callback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLHTTPRequest3Callback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLHTTPRequest3Callback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLHTTPRequest3Callback_OnRedirect(This,pXHR,pwszRedirectUrl)	\
    ( (This)->lpVtbl -> OnRedirect(This,pXHR,pwszRedirectUrl) ) 

#define IXMLHTTPRequest3Callback_OnHeadersAvailable(This,pXHR,dwStatus,pwszStatus)	\
    ( (This)->lpVtbl -> OnHeadersAvailable(This,pXHR,dwStatus,pwszStatus) ) 

#define IXMLHTTPRequest3Callback_OnDataAvailable(This,pXHR,pResponseStream)	\
    ( (This)->lpVtbl -> OnDataAvailable(This,pXHR,pResponseStream) ) 

#define IXMLHTTPRequest3Callback_OnResponseReceived(This,pXHR,pResponseStream)	\
    ( (This)->lpVtbl -> OnResponseReceived(This,pXHR,pResponseStream) ) 

#define IXMLHTTPRequest3Callback_OnError(This,pXHR,hrError)	\
    ( (This)->lpVtbl -> OnError(This,pXHR,hrError) ) 


#define IXMLHTTPRequest3Callback_OnServerCertificateReceived(This,pXHR,dwCertificateErrors,cServerCertificateChain,rgServerCertificateChain)	\
    ( (This)->lpVtbl -> OnServerCertificateReceived(This,pXHR,dwCertificateErrors,cServerCertificateChain,rgServerCertificateChain) ) 

#define IXMLHTTPRequest3Callback_OnClientCertificateRequested(This,pXHR,cIssuerList,rgpwszIssuerList)	\
    ( (This)->lpVtbl -> OnClientCertificateRequested(This,pXHR,cIssuerList,rgpwszIssuerList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLHTTPRequest3Callback_INTERFACE_DEFINED__ */


#ifndef __IXMLHTTPRequest3_INTERFACE_DEFINED__
#define __IXMLHTTPRequest3_INTERFACE_DEFINED__

/* interface IXMLHTTPRequest3 */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXMLHTTPRequest3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a1c9feee-0617-4f23-9d58-8961ea43567c")
    IXMLHTTPRequest3 : public IXMLHTTPRequest2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetClientCertificate( 
            /* [in] */ DWORD cbClientCertificateHash,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbClientCertificateHash) const BYTE *pbClientCertificateHash,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszPin) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXMLHTTPRequest3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXMLHTTPRequest3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXMLHTTPRequest3 * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszMethod,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [in] */ __RPC__in_opt IXMLHTTPRequest2Callback *pStatusCallback,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszPassword,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyUserName,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszProxyPassword);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Send)
        HRESULT ( STDMETHODCALLTYPE *Send )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [unique][in] */ __RPC__in_opt ISequentialStream *pBody,
            /* [in] */ ULONGLONG cbBody);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IXMLHTTPRequest3 * This);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetCookie)
        HRESULT ( STDMETHODCALLTYPE *SetCookie )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [ref][in] */ __RPC__in const XHR_COOKIE *pCookie,
            /* [out] */ __RPC__out DWORD *pdwCookieState);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetCustomResponseStream)
        HRESULT ( STDMETHODCALLTYPE *SetCustomResponseStream )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [in] */ __RPC__in_opt ISequentialStream *pSequentialStream);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [in] */ XHR_PROPERTY eProperty,
            /* [in] */ ULONGLONG ullValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, SetRequestHeader)
        HRESULT ( STDMETHODCALLTYPE *SetRequestHeader )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetAllResponseHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetAllResponseHeaders )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszHeaders);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetCookie)
        HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [string][ref][in] */ __RPC__in_string const WCHAR *pwszUrl,
            /* [string][unique][in] */ __RPC__in_opt_string const WCHAR *pwszName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out ULONG *pcCookies,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCookies) XHR_COOKIE **ppCookies);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest2, GetResponseHeader)
        HRESULT ( STDMETHODCALLTYPE *GetResponseHeader )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [ref][string][in] */ __RPC__in_string const WCHAR *pwszHeader,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppwszValue);
        
        DECLSPEC_XFGVIRT(IXMLHTTPRequest3, SetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetClientCertificate )( 
            __RPC__in IXMLHTTPRequest3 * This,
            /* [in] */ DWORD cbClientCertificateHash,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbClientCertificateHash) const BYTE *pbClientCertificateHash,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pwszPin);
        
        END_INTERFACE
    } IXMLHTTPRequest3Vtbl;

    interface IXMLHTTPRequest3
    {
        CONST_VTBL struct IXMLHTTPRequest3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXMLHTTPRequest3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXMLHTTPRequest3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXMLHTTPRequest3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXMLHTTPRequest3_Open(This,pwszMethod,pwszUrl,pStatusCallback,pwszUserName,pwszPassword,pwszProxyUserName,pwszProxyPassword)	\
    ( (This)->lpVtbl -> Open(This,pwszMethod,pwszUrl,pStatusCallback,pwszUserName,pwszPassword,pwszProxyUserName,pwszProxyPassword) ) 

#define IXMLHTTPRequest3_Send(This,pBody,cbBody)	\
    ( (This)->lpVtbl -> Send(This,pBody,cbBody) ) 

#define IXMLHTTPRequest3_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IXMLHTTPRequest3_SetCookie(This,pCookie,pdwCookieState)	\
    ( (This)->lpVtbl -> SetCookie(This,pCookie,pdwCookieState) ) 

#define IXMLHTTPRequest3_SetCustomResponseStream(This,pSequentialStream)	\
    ( (This)->lpVtbl -> SetCustomResponseStream(This,pSequentialStream) ) 

#define IXMLHTTPRequest3_SetProperty(This,eProperty,ullValue)	\
    ( (This)->lpVtbl -> SetProperty(This,eProperty,ullValue) ) 

#define IXMLHTTPRequest3_SetRequestHeader(This,pwszHeader,pwszValue)	\
    ( (This)->lpVtbl -> SetRequestHeader(This,pwszHeader,pwszValue) ) 

#define IXMLHTTPRequest3_GetAllResponseHeaders(This,ppwszHeaders)	\
    ( (This)->lpVtbl -> GetAllResponseHeaders(This,ppwszHeaders) ) 

#define IXMLHTTPRequest3_GetCookie(This,pwszUrl,pwszName,dwFlags,pcCookies,ppCookies)	\
    ( (This)->lpVtbl -> GetCookie(This,pwszUrl,pwszName,dwFlags,pcCookies,ppCookies) ) 

#define IXMLHTTPRequest3_GetResponseHeader(This,pwszHeader,ppwszValue)	\
    ( (This)->lpVtbl -> GetResponseHeader(This,pwszHeader,ppwszValue) ) 


#define IXMLHTTPRequest3_SetClientCertificate(This,cbClientCertificateHash,pbClientCertificateHash,pwszPin)	\
    ( (This)->lpVtbl -> SetClientCertificate(This,cbClientCertificateHash,pbClientCertificateHash,pwszPin) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXMLHTTPRequest3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msxml6_0000_0079 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#ifdef __USE_MSXML6_NAMESPACE__
}
#endif


extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0079_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msxml6_0000_0079_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


