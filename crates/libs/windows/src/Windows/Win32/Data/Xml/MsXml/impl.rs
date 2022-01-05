#[cfg(feature = "Win32_System_Com")]
pub trait IMXAttributesImpl: Sized + IDispatchImpl {
    fn addAttribute();
    fn addAttributeFromIndex();
    fn clear();
    fn removeAttribute();
    fn setAttribute();
    fn setAttributes();
    fn setLocalName();
    fn setQName();
    fn setType();
    fn setURI();
    fn setValue();
}
pub trait IMXNamespaceManagerImpl: Sized {
    fn putAllowOverride();
    fn getAllowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefix();
    fn getPrefix();
    fn getURI();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXNamespacePrefixesImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXReaderControlImpl: Sized + IDispatchImpl {
    fn abort();
    fn resume();
    fn suspend();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXSchemaDeclHandlerImpl: Sized + IDispatchImpl {
    fn schemaElementDecl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXWriterImpl: Sized + IDispatchImpl {
    fn Setoutput();
    fn output();
    fn Setencoding();
    fn encoding();
    fn SetbyteOrderMark();
    fn byteOrderMark();
    fn Setindent();
    fn indent();
    fn Setstandalone();
    fn standalone();
    fn SetomitXMLDeclaration();
    fn omitXMLDeclaration();
    fn Setversion();
    fn version();
    fn SetdisableOutputEscaping();
    fn disableOutputEscaping();
    fn flush();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXXMLFilterImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
}
pub trait ISAXAttributesImpl: Sized {
    fn getLength();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
pub trait ISAXContentHandlerImpl: Sized {
    fn putDocumentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
pub trait ISAXDTDHandlerImpl: Sized {
    fn notationDecl();
    fn unparsedEntityDecl();
}
pub trait ISAXDeclHandlerImpl: Sized {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
pub trait ISAXEntityResolverImpl: Sized {
    fn resolveEntity();
}
pub trait ISAXErrorHandlerImpl: Sized {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
pub trait ISAXLexicalHandlerImpl: Sized {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
pub trait ISAXLocatorImpl: Sized {
    fn getColumnNumber();
    fn getLineNumber();
    fn getPublicId();
    fn getSystemId();
}
pub trait ISAXXMLFilterImpl: Sized + ISAXXMLReaderImpl {
    fn getParent();
    fn putParent();
}
pub trait ISAXXMLReaderImpl: Sized {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn getEntityResolver();
    fn putEntityResolver();
    fn getContentHandler();
    fn putContentHandler();
    fn getDTDHandler();
    fn putDTDHandler();
    fn getErrorHandler();
    fn putErrorHandler();
    fn getBaseURL();
    fn putBaseURL();
    fn getSecureBaseURL();
    fn putSecureBaseURL();
    fn parse();
    fn parseURL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn targetNamespace();
    fn version();
    fn types();
    fn elements();
    fn attributes();
    fn attributeGroups();
    fn modelGroups();
    fn notations();
    fn schemaLocations();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAnyImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn namespaces();
    fn processContents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttributeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn r#use();
    fn isReference();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttributeGroupImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn anyAttribute();
    fn attributes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaComplexTypeImpl: Sized + ISchemaTypeImpl + ISchemaItemImpl + IDispatchImpl {
    fn isAbstract();
    fn anyAttribute();
    fn attributes();
    fn contentType();
    fn contentModel();
    fn prohibitedSubstitutions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaElementImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn isNillable();
    fn identityConstraints();
    fn substitutionGroup();
    fn substitutionGroupExclusions();
    fn disallowedSubstitutions();
    fn isAbstract();
    fn isReference();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaIdentityConstraintImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn selector();
    fn fields();
    fn referencedKey();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItemImpl: Sized + IDispatchImpl {
    fn name();
    fn namespaceURI();
    fn schema();
    fn id();
    fn itemType();
    fn unhandledAttributes();
    fn writeAnnotation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItemCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn itemByName();
    fn itemByQName();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaModelGroupImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn particles();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaNotationImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn systemIdentifier();
    fn publicIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaParticleImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn minOccurs();
    fn maxOccurs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaStringCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaTypeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn baseTypes();
    fn r#final();
    fn variety();
    fn derivedBy();
    fn isValid();
    fn minExclusive();
    fn minInclusive();
    fn maxExclusive();
    fn maxInclusive();
    fn totalDigits();
    fn fractionDigits();
    fn length();
    fn minLength();
    fn maxLength();
    fn enumeration();
    fn whitespace();
    fn patterns();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequestImpl: Sized + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setTimeouts();
    fn waitForResponse();
    fn getOption();
    fn setOption();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequest2Impl: Sized + IServerXMLHTTPRequestImpl + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setProxy();
    fn setProxyCredentials();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBMXNamespaceManagerImpl: Sized + IDispatchImpl {
    fn SetallowOverride();
    fn allowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefixes();
    fn getPrefixes();
    fn getURI();
    fn getURIFromNode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXAttributesImpl: Sized + IDispatchImpl {
    fn length();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXContentHandlerImpl: Sized + IDispatchImpl {
    fn putref_documentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDTDHandlerImpl: Sized + IDispatchImpl {
    fn notationDecl();
    fn unparsedEntityDecl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDeclHandlerImpl: Sized + IDispatchImpl {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXEntityResolverImpl: Sized + IDispatchImpl {
    fn resolveEntity();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXErrorHandlerImpl: Sized + IDispatchImpl {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLexicalHandlerImpl: Sized + IDispatchImpl {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLocatorImpl: Sized + IDispatchImpl {
    fn columnNumber();
    fn lineNumber();
    fn publicId();
    fn systemId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLFilterImpl: Sized + IDispatchImpl {
    fn parent();
    fn putref_parent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLReaderImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
    fn baseURL();
    fn SetbaseURL();
    fn secureBaseURL();
    fn SetsecureBaseURL();
    fn parse();
    fn parseURL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLAttributeImpl: Sized + IDispatchImpl {
    fn name();
    fn value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMAttributeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn value();
    fn Setvalue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCDATASectionImpl: Sized + IXMLDOMTextImpl + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCharacterDataImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn data();
    fn Setdata();
    fn length();
    fn substringData();
    fn appendData();
    fn insertData();
    fn deleteData();
    fn replaceData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCommentImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn doctype();
    fn implementation();
    fn documentElement();
    fn putref_documentElement();
    fn createElement();
    fn createDocumentFragment();
    fn createTextNode();
    fn createComment();
    fn createCDATASection();
    fn createProcessingInstruction();
    fn createAttribute();
    fn createEntityReference();
    fn getElementsByTagName();
    fn createNode();
    fn nodeFromID();
    fn load();
    fn readyState();
    fn parseError();
    fn url();
    fn r#async();
    fn Setasync();
    fn abort();
    fn loadXML();
    fn save();
    fn validateOnParse();
    fn SetvalidateOnParse();
    fn resolveExternals();
    fn SetresolveExternals();
    fn preserveWhiteSpace();
    fn SetpreserveWhiteSpace();
    fn Setonreadystatechange();
    fn Setondataavailable();
    fn Setontransformnode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument2Impl: Sized + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn namespaces();
    fn schemas();
    fn putref_schemas();
    fn validate();
    fn setProperty();
    fn getProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument3Impl: Sized + IXMLDOMDocument2Impl + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn validateNode();
    fn importNode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentFragmentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentTypeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn entities();
    fn notations();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMElementImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn tagName();
    fn getAttribute();
    fn setAttribute();
    fn removeAttribute();
    fn getAttributeNode();
    fn setAttributeNode();
    fn removeAttributeNode();
    fn getElementsByTagName();
    fn normalize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntityImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
    fn notationName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntityReferenceImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMImplementationImpl: Sized + IDispatchImpl {
    fn hasFeature();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNamedNodeMapImpl: Sized + IDispatchImpl {
    fn getNamedItem();
    fn setNamedItem();
    fn removeNamedItem();
    fn item();
    fn length();
    fn getQualifiedItem();
    fn removeQualifiedItem();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNodeImpl: Sized + IDispatchImpl {
    fn nodeName();
    fn nodeValue();
    fn SetnodeValue();
    fn nodeType();
    fn parentNode();
    fn childNodes();
    fn firstChild();
    fn lastChild();
    fn previousSibling();
    fn nextSibling();
    fn attributes();
    fn insertBefore();
    fn replaceChild();
    fn removeChild();
    fn appendChild();
    fn hasChildNodes();
    fn ownerDocument();
    fn cloneNode();
    fn nodeTypeString();
    fn text();
    fn Settext();
    fn specified();
    fn definition();
    fn nodeTypedValue();
    fn SetnodeTypedValue();
    fn dataType();
    fn SetdataType();
    fn xml();
    fn transformNode();
    fn selectNodes();
    fn selectSingleNode();
    fn parsed();
    fn namespaceURI();
    fn prefix();
    fn baseName();
    fn transformNodeToObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNodeListImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNotationImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseErrorImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn url();
    fn reason();
    fn srcText();
    fn line();
    fn linepos();
    fn filepos();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseError2Impl: Sized + IXMLDOMParseErrorImpl + IDispatchImpl {
    fn errorXPath();
    fn allErrors();
    fn errorParameters();
    fn errorParametersCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseErrorCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn next();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMProcessingInstructionImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn target();
    fn data();
    fn Setdata();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollectionImpl: Sized + IDispatchImpl {
    fn add();
    fn get();
    fn remove();
    fn length();
    fn namespaceURI();
    fn addCollection();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollection2Impl: Sized + IXMLDOMSchemaCollectionImpl + IDispatchImpl {
    fn validate();
    fn SetvalidateOnLoad();
    fn validateOnLoad();
    fn getSchema();
    fn getDeclaration();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSelectionImpl: Sized + IXMLDOMNodeListImpl + IDispatchImpl {
    fn expr();
    fn Setexpr();
    fn context();
    fn putref_context();
    fn peekNode();
    fn matches();
    fn removeNext();
    fn removeAll();
    fn clone();
    fn getProperty();
    fn setProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMTextImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn splitText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDSOControlImpl: Sized + IDispatchImpl {
    fn XMLDocument();
    fn SetXMLDocument();
    fn JavaDSOCompatible();
    fn SetJavaDSOCompatible();
    fn readyState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocumentImpl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocument2Impl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
    fn r#async();
    fn Setasync();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElementImpl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElement2Impl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
    fn attributes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElementCollectionImpl: Sized + IDispatchImpl {
    fn Setlength();
    fn length();
    fn _newEnum();
    fn item();
}
pub trait IXMLErrorImpl: Sized {
    fn GetErrorInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
pub trait IXMLHTTPRequest2Impl: Sized {
    fn Open();
    fn Send();
    fn Abort();
    fn SetCookie();
    fn SetCustomResponseStream();
    fn SetProperty();
    fn SetRequestHeader();
    fn GetAllResponseHeaders();
    fn GetCookie();
    fn GetResponseHeader();
}
pub trait IXMLHTTPRequest2CallbackImpl: Sized {
    fn OnRedirect();
    fn OnHeadersAvailable();
    fn OnDataAvailable();
    fn OnResponseReceived();
    fn OnError();
}
pub trait IXMLHTTPRequest3Impl: Sized + IXMLHTTPRequest2Impl {
    fn SetClientCertificate();
}
pub trait IXMLHTTPRequest3CallbackImpl: Sized + IXMLHTTPRequest2CallbackImpl {
    fn OnServerCertificateReceived();
    fn OnClientCertificateRequested();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHttpRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLProcessorImpl: Sized + IDispatchImpl {
    fn Setinput();
    fn input();
    fn ownerTemplate();
    fn setStartMode();
    fn startMode();
    fn startModeURI();
    fn Setoutput();
    fn output();
    fn transform();
    fn reset();
    fn readyState();
    fn addParameter();
    fn addObject();
    fn stylesheet();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLTemplateImpl: Sized + IDispatchImpl {
    fn putref_stylesheet();
    fn stylesheet();
    fn createProcessor();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXTLRuntimeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn uniqueID();
    fn depth();
    fn childNumber();
    fn ancestorChildNumber();
    fn absoluteChildNumber();
    fn formatIndex();
    fn formatNumber();
    fn formatDate();
    fn formatTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait XMLDOMDocumentEventsImpl: Sized + IDispatchImpl {}
