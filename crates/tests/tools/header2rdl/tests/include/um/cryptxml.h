/*-------------------------------------------------------------------
//
//  Copyright (C) Microsoft, 2007
//
//
//  File:       CryptXml.h
//
//  Contents:   
//              XML DigSig
//
--------------------------------------------------------------------*/
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or Appx Deployment Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPXDEPLOYMENT)


#include <specstrings.h>        /* for SAL annotations */
#include <wincrypt.h>
#include <bcrypt.h>
#include <ncrypt.h>

#ifndef WINAPI
#define WINAPI __stdcall
#endif

/* "C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C" */
#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

typedef struct _CRYPT_XML_BLOB                  CRYPT_XML_BLOB, *PCRYPT_XML_BLOB;
typedef struct _CRYPT_XML_DATA_BLOB             CRYPT_XML_DATA_BLOB, *PCRYPT_XML_DATA_BLOB;
typedef struct _CRYPT_XML_STATUS                CRYPT_XML_STATUS, *PCRYPT_XML_STATUS;
typedef struct _CRYPT_XML_TRANSFORM_INFO        CRYPT_XML_TRANSFORM_INFO, *PCRYPT_XML_TRANSFORM_INFO;
typedef struct _CRYPT_XML_TRANSFORM_CHAIN_CONFIG CRYPT_XML_TRANSFORM_CHAIN_CONFIG, *PCRYPT_XML_TRANSFORM_CHAIN_CONFIG;
typedef struct _CRYPT_XML_KEY_INFO              CRYPT_XML_KEY_INFO, *PCRYPT_XML_KEY_INFO;
typedef struct _CRYPT_XML_ALGORITHM             CRYPT_XML_ALGORITHM, *PCRYPT_XML_ALGORITHM;
typedef struct _CRYPT_XML_REFERENCE             CRYPT_XML_REFERENCE, *PCRYPT_XML_REFERENCE;
typedef struct _CRYPT_XML_REFERENCES            CRYPT_XML_REFERENCES, *PCRYPT_XML_REFERENCES;
typedef struct _CRYPT_XML_SIGNED_INFO           CRYPT_XML_SIGNED_INFO, *PCRYPT_XML_SIGNED_INFO;
typedef struct _CRYPT_XML_OBJECT                CRYPT_XML_OBJECT, *PCRYPT_XML_OBJECT;
typedef struct _CRYPT_XML_SIGNATURE             CRYPT_XML_SIGNATURE, *PCRYPT_XML_SIGNATURE;
typedef struct _CRYPT_XML_ALGORITHM_INFO        CRYPT_XML_ALGORITHM_INFO, *PCRYPT_XML_ALGORITHM_INFO;
typedef struct _CRYPT_XML_CRYPTOGRAPHIC_INTERFACE CRYPT_XML_CRYPTOGRAPHIC_INTERFACE, *PCRYPT_XML_CRYPTO_PROVIDER;

typedef void*   HCRYPTXML;

#define wszXMLNS_DIGSIG                         L"http://www.w3.org/2000/09/xmldsig#"

#define wszXMLNS_DIGSIG_SignatureProperties     L"http://www.w3.org/2000/09/xmldsig#SignatureProperties"

//
// The Id attribute must be unique withing the XML document.
// It's used to identify same-document (internal) references.
//

#define wszXMLNS_DIGSIG_Id                       L"Id"

#define wszURI_XMLNS_DIGSIG_BASE64               L"http://www.w3.org/2000/09/xmldsig#base64"

#define wszURI_XMLNS_DIGSIG_SHA1                 L"http://www.w3.org/2000/09/xmldsig#sha1"
#define wszURI_XMLNS_DIGSIG_SHA256               L"http://www.w3.org/2001/04/xmlenc#sha256"
#define wszURI_XMLNS_DIGSIG_SHA384               L"http://www.w3.org/2001/04/xmldsig-more#sha384"
#define wszURI_XMLNS_DIGSIG_SHA512               L"http://www.w3.org/2001/04/xmlenc#sha512"

#define wszURI_XMLNS_DIGSIG_RSA_SHA1             L"http://www.w3.org/2000/09/xmldsig#rsa-sha1"
#define wszURI_XMLNS_DIGSIG_DSA_SHA1             L"http://www.w3.org/2000/09/xmldsig#dsa-sha1"

#define wszURI_XMLNS_DIGSIG_RSA_SHA256           L"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"
#define wszURI_XMLNS_DIGSIG_RSA_SHA384           L"http://www.w3.org/2001/04/xmldsig-more#rsa-sha384"
#define wszURI_XMLNS_DIGSIG_RSA_SHA512           L"http://www.w3.org/2001/04/xmldsig-more#rsa-sha512"

#define wszURI_XMLNS_DIGSIG_ECDSA_SHA1           L"http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1"
#define wszURI_XMLNS_DIGSIG_ECDSA_SHA256         L"http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256"
#define wszURI_XMLNS_DIGSIG_ECDSA_SHA384         L"http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384"
#define wszURI_XMLNS_DIGSIG_ECDSA_SHA512         L"http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512"

#define wszURI_XMLNS_DIGSIG_HMAC_SHA1            L"http://www.w3.org/2000/09/xmldsig#hmac-sha1"
#define wszURI_XMLNS_DIGSIG_HMAC_SHA256          L"http://www.w3.org/2001/04/xmldsig-more#hmac-sha256"
#define wszURI_XMLNS_DIGSIG_HMAC_SHA384          L"http://www.w3.org/2001/04/xmldsig-more#hmac-sha384"
#define wszURI_XMLNS_DIGSIG_HMAC_SHA512          L"http://www.w3.org/2001/04/xmldsig-more#hmac-sha512"

#define wszURI_CANONICALIZATION_C14N             L"http://www.w3.org/TR/2001/REC-xml-c14n-20010315"
#define wszURI_CANONICALIZATION_C14NC            L"http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments"
#define wszURI_CANONICALIZATION_EXSLUSIVE_C14N   L"http://www.w3.org/2001/10/xml-exc-c14n#"
#define wszURI_CANONICALIZATION_EXSLUSIVE_C14NC  L"http://www.w3.org/2001/10/xml-exc-c14n#WithComments"

#define wszURI_TRANSFORM_XPATH                   L"http://www.w3.org/TR/1999/REC-xpath-19991116"
#define wszURI_XMLNS_TRANSFORM_BASE64            L"http://www.w3.org/2000/09/xmldsig#base64"
#define wszURI_XMLNS_TRANSFORM_ENVELOPED         L"http://www.w3.org/2000/09/xmldsig#enveloped-signature"

/****************************************************************************
 Limits

    reduce Integer overflows and security threats surface

****************************************************************************/

// Encoded data may not exceed 2 Gb
#define CRYPT_XML_BLOB_MAX              0x7FFFFFF8

// Id may not exceed 256 characters
#define CRYPT_XML_ID_MAX                256

// URI may not exceed 8K characters
#define CRYPT_XML_URI_MAX               8*1024

// Maximum number of signature elements per document, by default
#define CRYPT_XML_SIGNATURES_MAX        16

// Maximum number of transforms per reference
#define CRYPT_XML_TRANSFORM_MAX         16

#define CRYPT_XML_SIGNATURE_VALUE_MAX   2048

#define CRYPT_XML_DIGEST_VALUE_MAX      128

// Maximum number of Object elements per Signature
#define CRYPT_XML_OBJECTS_MAX           256

// Maximum number of Reference elements
#define CRYPT_XML_REFERENCES_MAX        0x7FF8


/****************************************************************************
 Error Codes

****************************************************************************/

#define CRYPT_XML_E_BASE                _HRESULT_TYPEDEF_(0x80092100L)

// The value is too large
#define CRYPT_XML_E_LARGE               _HRESULT_TYPEDEF_(0x80092101L)

// Too many transforms
#define CRYPT_XML_E_TOO_MANY_TRANSFORMS _HRESULT_TYPEDEF_(0x80092102L)

// Unsupported XML Encoding
#define CRYPT_XML_E_ENCODING            _HRESULT_TYPEDEF_(0x80092103L)

// Unsupported XML Algorithm
#define CRYPT_XML_E_ALGORITHM           _HRESULT_TYPEDEF_(0x80092104L)

// Unsupported Transform
#define CRYPT_XML_E_TRANSFORM           _HRESULT_TYPEDEF_(0x80092105L)

// Invalid handle
#define CRYPT_XML_E_HANDLE              _HRESULT_TYPEDEF_(0x80092106L)

// Invalid operation
#define CRYPT_XML_E_OPERATION           _HRESULT_TYPEDEF_(0x80092107L)

// Unable to resolve Reference
#define CRYPT_XML_E_UNRESOLVED_REFERENCE _HRESULT_TYPEDEF_(0x80092108L)

// Invalid digest value
#define CRYPT_XML_E_INVALID_DIGEST      _HRESULT_TYPEDEF_(0x80092109L)

// Invalid signature value
#define CRYPT_XML_E_INVALID_SIGNATURE   _HRESULT_TYPEDEF_(0x8009210AL)

// Unable to create or calculate the hash
#define CRYPT_XML_E_HASH_FAILED         _HRESULT_TYPEDEF_(0x8009210BL)

// Cryptographic signature operation failed
#define CRYPT_XML_E_SIGN_FAILED         _HRESULT_TYPEDEF_(0x8009210CL)

// Signature verification failed
#define CRYPT_XML_E_VERIFY_FAILED       _HRESULT_TYPEDEF_(0x8009210DL)

// Too many signatures
#define CRYPT_XML_E_TOO_MANY_SIGNATURES _HRESULT_TYPEDEF_(0x8009210EL)

// Invalid key value
#define CRYPT_XML_E_INVALID_KEYVALUE    _HRESULT_TYPEDEF_(0x8009210FL)

// Unexpected XML
#define CRYPT_XML_E_UNEXPECTED_XML      _HRESULT_TYPEDEF_(0x80092110L)

// Unable to find signer's key
#define CRYPT_XML_E_SIGNER              _HRESULT_TYPEDEF_(0x80092111L)

// Non-unique Id attribute found
#define CRYPT_XML_E_NON_UNIQUE_ID       _HRESULT_TYPEDEF_(0x80092112L)

#define CRYPT_XML_E_LAST                _HRESULT_TYPEDEF_(0x80092112L)


/****************************************************************************
 Global Flags

****************************************************************************/

// Serialization ensures mutual exclusion when two or more threads attempt 
// to simultaneously accept a CryptXml object, or memory.
// There is a small performance cost to serialization, 
// but it must be used whenever multiple threads access a CryptXml object.
// Set this flag to inhibit serialization.

#define CRYPT_XML_FLAG_NO_SERIALIZE                     0x80000000

// CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT flag is applicable to:
//   CryptXmlOpenToEncode
//   CryptXmlOpenToDecode
// When this flag is set, then CRYPT_XML_OBJECT structure will always 
// return encoded <Object> element. See CRYPT_XML_OBJECT for details.

#define CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT     0x40000000

// CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT flag is applicable to:
//   CryptXmlOpenToEncode
//   CryptXmlOpenToDecode
// When this flag is set, then Id attibutes must conform to
// NCName syntax (http://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName).
// By default, CryptXml accepts any valid string for Id attribute.

#define CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT         0x20000000

// CRYPT_XML_FLAG_DISABLE_EXTENSIONS flag is applicable to:
//   CryptXmlOpenToEncode
//   CryptXmlOpenToDecode
//   CryptXmlEnumAlgorithmInfo
// When this flag is set only default implementations for signature and
// digest  will be utilized and no registered Extensions will be loaded.

#define CRYPT_XML_FLAG_DISABLE_EXTENSIONS               0x10000000

// CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT flag is applicable to:
//   CryptXmlOpenToEncode
//   CryptXmlOpenToDecode
// When this flag is set, then Id attibutes must conform to
// Name syntax (http://www.w3.org/TR/2000/WD-xml-2e-2000814#idref).
// By default, CryptXml accepts any valid string for Id attribute.

#define CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT           0x08000000

//CRYPT_XML_FLAG_ECDSA_DISIG11 flag is applicable to:
//CryptXmlOpenToEncode
//When this flag is set, ECKeyValue schema defined in XML Signature Syntax and Processing Version 1.1
//is used to encode the ECC signing key.  Otherwise,  ECC key value uses the schema defined
//in RFC4050.

#define CRYPT_XML_FLAG_ECDSA_DSIG11 			0x04000000

/****************************************************************************
 CRYPT_XML_CHARSET
    
****************************************************************************/
typedef enum
{
    //
    // CRYPT_XML_CHARSET_AUTO is supported only in CryptXmlOpenToDecode mode.
    // The encoded XML character set will be determined by the parser from 
    // the XML declaration or the best guess on the characters.
    CRYPT_XML_CHARSET_AUTO        = 0,

    CRYPT_XML_CHARSET_UTF8        = 1,

    CRYPT_XML_CHARSET_UTF16LE     = 2,

    CRYPT_XML_CHARSET_UTF16BE     = 3,
}CRYPT_XML_CHARSET;

/****************************************************************************
 CRYPT_XML_BLOB

****************************************************************************/
typedef struct _CRYPT_XML_BLOB{
    CRYPT_XML_CHARSET                   dwCharset;
    _Field_range_( 0, CRYPT_XML_BLOB_MAX )
    ULONG                               cbData;    
    _Field_size_(cbData)
    BYTE                                *pbData;    
}CRYPT_XML_BLOB, *PCRYPT_XML_BLOB;

/****************************************************************************
 CRYPT_XML_DATA_BLOB

****************************************************************************/
typedef struct _CRYPT_XML_DATA_BLOB{
    _Field_range_( 0, CRYPT_XML_BLOB_MAX )
    ULONG                               cbData;    
    _Field_size_(cbData)
    BYTE                                *pbData;    
}CRYPT_XML_DATA_BLOB, *PCRYPT_XML_DATA_BLOB;


/****************************************************************************
 CRYPT_XML_PROPERTY

    CRYPT_XML_PROPERTY_MAX_HEAP_SIZE
        Specifies the maximum heap size to be used by XML layer.
        This property is also applied for intermediate buffers used to parse
        or construct XML parts.         
        By default, the limit is equal to CRYPT_XML_BLOB_MAX.

    CRYPT_XML_PROPERTY_SIGNATURE_LOCATION
        Specifies the location in XML document where the Signature will be 
        created. The following formats are supported:
            #id  - the Id Attribute of the element to insert the Signature;
            /a/b/c - the absolute path of the element to insert the Signature;

    CRYPT_XML_PROPERTY_MAX_SIGNATURES
        Specifies the maximum number of <Signature> elements when
        parsing an XML document.
        This property overrides the default CRYPT_XML_SIGNATURES_MAX value.

    CRYPT_XML_PROPERTY_DOC_DECLARATION
        Specifies whether to write an XML document declaration.
        This property is used with CryptXmlEncode. The default property is TRUE.

    CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET
        Specifies an encoding charset of XML fragments for custom elements.
        This property is used with CryptXmlOpenToDecode. 
        The default charset is inherited from the opened document.

 NOTE: If a property value is defined as a pointer to data, 
       then the pointer must be valid for the entire period of signature 
       operation.

****************************************************************************/
typedef enum
{
    CRYPT_XML_PROPERTY_MAX_HEAP_SIZE        =   1,  // ULONG, sizeof(ULONG)
    CRYPT_XML_PROPERTY_SIGNATURE_LOCATION   =   2,  // LPCWSTR*, sizeof(LPCWSTR)
    CRYPT_XML_PROPERTY_MAX_SIGNATURES       =   3,  // ULONG, sizeof(ULONG)
    CRYPT_XML_PROPERTY_DOC_DECLARATION      =   4,  // BOOL, sizeof(BOOL)
    CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET   =   5,  // CRYPT_XML_CHARSET, sizeof(CRYPT_XML_CHARSET)
}CRYPT_XML_PROPERTY_ID;

typedef struct _CRYPT_XML_PROPERTY{
    CRYPT_XML_PROPERTY_ID               dwPropId;
    _Field_size_bytes_( cbValue )
    const void*                         pvValue;
    ULONG                               cbValue;
}CRYPT_XML_PROPERTY, *PCRYPT_XML_PROPERTY;

/****************************************************************************
 PFN_CRYPT_XML_WRITE_CALLBACK

 Callback function used to write data.

 pvCallbackState
    [in] An application defined argument for the callback.

 pbData
    [in] Pointer to a block of data to be written.

 cbData
    Size, in bytes, of the block of data at pbData.

****************************************************************************/
typedef 
_Success_( return == 0 )
HRESULT 
(CALLBACK* PFN_CRYPT_XML_WRITE_CALLBACK)(
    _Inout_         void                *pvCallbackState, 
    _In_reads_bytes_( cbData )		
                    const BYTE          *pbData, 
                    ULONG               cbData
    );

/****************************************************************************
 PFN_CRYPT_XML_DATA_PROVIDER_READ

 Callback function used to provide data.

 pvCallbackState
    [in] An application defined argument for the callback.

 pbData
    [out] Pointer to the buffer that receives the data.

 cbData
    [in] Specifies the number of bites to read.

 pcbRead
    [out] A pointer to the variable that receives the number of bytes read.

Return Values:

    The PFN_CRYPT_XML_DATA_PROVIDER callback returns when one of the 
    following conditions occurs: 
    - A write operation completes on the write end of the data provider. 
    - The number of bytes requested is read. 
    - An error occurs. 

 If the function succeeds, the return value is NO_ERROR.
 If the function fails, the error code is determined by HRESULT.
 If the *pcbRead value is 0, then there is no more data available.

Remarks:

 The callback shall not return unless number of bytes specified in cbData 
 is available, or it's the last block of data.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(CALLBACK *PFN_CRYPT_XML_DATA_PROVIDER_READ)(
    _Inout_         void                *pvCallbackState,
    _Out_writes_bytes_to_( cbData, *pcbRead )
                    BYTE                *pbData,
    _In_            ULONG               cbData,
    _Out_range_( 0, cbData )
                    ULONG               *pcbRead
    );

/****************************************************************************
 PFN_CRYPT_XML_DATA_PROVIDER_CLOSE

 Callback function used to release the data provider.

 pvCallbackState
    [in] An application defined argument for the callback.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(CALLBACK *PFN_CRYPT_XML_DATA_PROVIDER_CLOSE)(
    _Inout_         void                *pvCallbackState
    );

/****************************************************************************
 CRYPT_XML_DATA_PROVIDER

 pvCallbackState
    [in] An application defined argument to be passed to
    the pfnRead and pfnClose callbacks.

 cbBufferSize
    [in] Specifies the size of data provider's buffer, 
    that can be 0 if the size does not matter or can't be determined by the provider.
    This value is used by a caller of pfnRead to determine a size of the receiving buffer.

 pfnRead
    [in] Callback function used to read data.

 pfnClose
    [in] Callback function used to release the data provider.
    The caller must always release the data provider after using it,
    even when the read operation failed.

****************************************************************************/
typedef struct _CRYPT_XML_DATA_PROVIDER{
        void                            *pvCallbackState;
        ULONG                           cbBufferSize;
        PFN_CRYPT_XML_DATA_PROVIDER_READ pfnRead;
        PFN_CRYPT_XML_DATA_PROVIDER_CLOSE pfnClose;
}CRYPT_XML_DATA_PROVIDER, *PCRYPT_XML_DATA_PROVIDER;

/****************************************************************************
 PFN_CRYPT_XML_CREATE_TRANSFORM

 Callback function used to create transform routine.

 pTransform
    [in] Specifies the Transform to apply.

 pProviderIn
    [in] Address of the data provider to be used as input for transform. 

 pfpProviderOut
    [out] Address of the variable that receives a pointer to 
    the transform's data provider.

 NOTE: In the transform chain, the output of the trasnform is the input of the next one.

 The pProviderOut implementation is responsible to call pProviderIn->pfnClose
 to relase the input provider. Usually it's implemented in its own pfnClose.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(CALLBACK *PFN_CRYPT_XML_CREATE_TRANSFORM)(
    _In_            const CRYPT_XML_ALGORITHM *pTransform,
    _In_            CRYPT_XML_DATA_PROVIDER *pProviderIn,
    _Out_           CRYPT_XML_DATA_PROVIDER *pProviderOut
    );


/****************************************************************************
 CRYPT_XML_STATUS

  Structure contains information about signature validation status, 
  summary status information about a SignedInfo, or summary information 
  status about an array of Reference.

 cbSize
    Size of this structure. 

 dwErrorStatus
    Error flags.

 dwInfoStatus
    Informational flags.

****************************************************************************/
typedef struct _CRYPT_XML_STATUS{
    ULONG                               cbSize;    
    DWORD                               dwErrorStatus;
    DWORD                               dwInfoStatus;
}CRYPT_XML_STATUS, *PCRYPT_XML_STATUS;

#define CRYPT_XML_STATUS_NO_ERROR                           0x00000000

//===========================================================================
// Error Status
//===========================================================================

// One of the References could not be resolved to digest the data
#define CRYPT_XML_STATUS_ERROR_NOT_RESOLVED                     0x00000001

// Digest value was not verified successfully
#define CRYPT_XML_STATUS_ERROR_DIGEST_INVALID                   0x00000002

// One of the algorithm URIs specified in XML is not supported
#define CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_ALGORITHM          0x00000004

// One of the transform URIs specified in XML is not supported
#define CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_TRANSFORM          0x00000008

// Signature value was not verified successfully
#define CRYPT_XML_STATUS_ERROR_SIGNATURE_INVALID                0x00010000

// Unable to parse the KeyInfo element
#define CRYPT_XML_STATUS_ERROR_KEYINFO_NOT_PARSED               0x00020000

//===========================================================================
// Info Status
//===========================================================================

// The Reference URI points to an internal element in XML 
// and can be resolved automatically
#define CRYPT_XML_STATUS_INTERNAL_REFERENCE                     0x00000001

// The KeyValue element parsed and a key handle imported successfully
#define CRYPT_XML_STATUS_KEY_AVAILABLE                          0x00000002

// The reference is being digetsed
#define CRYPT_XML_STATUS_DIGESTING                              0x00000004

// The digest value successfully verified
#define CRYPT_XML_STATUS_DIGEST_VALID                           0x00000008

// The signature value successfully verified
#define CRYPT_XML_STATUS_SIGNATURE_VALID                        0x00010000

// The document is opened to encode
#define CRYPT_XML_STATUS_OPENED_TO_ENCODE                       0x80000000


/****************************************************************************
 CRYPT_XML_ALGORITHM

 cbSize
    Size of this structure. 

 wszAlgorithm
    Specifies the Algorithm attribute. 
    This parameter must be NULL, when Encoded is provided by an application.

 Encoded
    [optional] Contains the XML encoded element. 
    This value is set only when ANY element is present.

    <element name="CanonicalizationMethod" type="ds:CanonicalizationMethodType"/> 
    <complexType name="CanonicalizationMethodType" mixed="true">
     <sequence>
       <any namespace="##any" minOccurs="0" maxOccurs="unbounded"/>
       <!-- (0,unbounded) elements from (1,1) namespace -->
     </sequence>
     <attribute name="Algorithm" type="anyURI" use="required"/> 
    </complexType>

    <element name="SignatureMethod" type="ds:SignatureMethodType"/>
    <complexType name="SignatureMethodType" mixed="true">
     <sequence>
       <element name="HMACOutputLength" minOccurs="0" type="ds:HMACOutputLengthType"/>
       <any namespace="##other" minOccurs="0" maxOccurs="unbounded"/>
       <!-- (0,unbounded) elements from (1,1) external namespace -->
      </sequence>
    <attribute name="Algorithm" type="anyURI" use="required"/> 
    </complexType>

   <element name="DigestMethod" type="ds:DigestMethodType"/>
   <complexType name="DigestMethodType" mixed="true"> 
     <sequence>
       <any namespace="##other" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
     </sequence>    
     <attribute name="Algorithm" type="anyURI" use="required"/> 
   </complexType>

    <element name="Transform" type="ds:TransformType"/>
    <complexType name="TransformType" mixed="true">
     <choice minOccurs="0" maxOccurs="unbounded"> 
       <any namespace="##other" processContents="lax"/>
       <!-- (1,1) elements from (0,unbounded) namespaces -->
       <element name="XPath" type="string"/> 
     </choice>
     <attribute name="Algorithm" type="anyURI" use="required"/> 
    </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_ALGORITHM{
    ULONG                               cbSize;    
    LPCWSTR                             wszAlgorithm;
    CRYPT_XML_BLOB                      Encoded;
}CRYPT_XML_ALGORITHM, *PCRYPT_XML_ALGORITHM;

/****************************************************************************
 CRYPT_XML_TRANSFORM_INFO

 cbSize
    Size of this structure. 

 wszAlgorithm
    Specifies the Algorithm attribute. 

 cbBufferSize
    Maximum buffer size for transformed data.
    This value can be 0 if the size can't be determined at the initialization time.

 dwFlags
    The following flags are supported
            CRYPT_XML_TRANSFORM_ON_STREAM
            CRYPT_XML_TRANSFORM_ON_NODESET
            CRYPT_XML_TRANSFORM_URI_QUERY_STRING

 pfnCreateTransform
    [callback] Pointer to a function to create a transform. 

****************************************************************************/
typedef struct _CRYPT_XML_TRANSFORM_INFO{
    ULONG                               cbSize;    
    LPCWSTR                             wszAlgorithm;
    ULONG                               cbBufferSize;
    DWORD                               dwFlags;
    PFN_CRYPT_XML_CREATE_TRANSFORM      pfnCreateTransform;
}CRYPT_XML_TRANSFORM_INFO, *PCRYPT_XML_TRANSFORM_INFO;

//
// Informational flag, specifies that transform is implemented
// on a stream of bytes

#define CRYPT_XML_TRANSFORM_ON_STREAM           0x00000001

//
// Informational flag, specifies that transform is implemented
// on XML node set

#define CRYPT_XML_TRANSFORM_ON_NODESET          0x00000002

//
// In some cases, the URI may contain additional information 
// in the QueryString after the & sign.
// When this flag is set, the URI comparisson will be performed on the core URI 
// without the QueryString

#define CRYPT_XML_TRANSFORM_URI_QUERY_STRING    0x00000004

/****************************************************************************
 CRYPT_XML_TRANSFORM_CHAIN_CONFIG

 cTransformInfo
    Count of elements in rgTransformInfo. 

 rgTransformInfo
    Array of  PCRYPT_XML_TRANSFORM_INFO

 ****************************************************************************/
typedef struct _CRYPT_XML_TRANSFORM_CHAIN_CONFIG{
    ULONG                               cbSize;    
    ULONG                               cTransformInfo;
    _Field_size_(cTransformInfo)
    PCRYPT_XML_TRANSFORM_INFO           *rgpTransformInfo;
} CRYPT_XML_TRANSFORM_CHAIN_CONFIG, *PCRYPT_XML_TRANSFORM_CHAIN_CONFIG;

/****************************************************************************
 CRYPT_XML_KEY_DSA_KEY_VALUE

 Encapsulates DSA key value.

  P
    BLOB containing DSA key P parameter.

  Q
    BLOB containing DSA key Q parameter.

  G
    BLOB containing DSA key G parameter.

  Y
    BLOB containing DSA key Y parameter.

  J
    BLOB containing DSA key J parameter.

  Seed
    BLOB containing DSA key seed.

  Counter
    BLOB containing DSA key counter.

****************************************************************************/
typedef struct _CRYPT_XML_KEY_DSA_KEY_VALUE{
    CRYPT_XML_DATA_BLOB                 P;
    CRYPT_XML_DATA_BLOB                 Q;
    CRYPT_XML_DATA_BLOB                 G;
    CRYPT_XML_DATA_BLOB                 Y;
    CRYPT_XML_DATA_BLOB                 J;
    CRYPT_XML_DATA_BLOB                 Seed;
    CRYPT_XML_DATA_BLOB                 Counter;
} CRYPT_XML_KEY_DSA_KEY_VALUE;

/****************************************************************************
 CRYPT_XML_KEY_ECDSA_KEY_VALUE

 Encapsulates ECDSA key value.

  wszNamedCurve
    Specifies named curve as a Unicode string in URN format,
    for example "urn:oid:1.2.3.4"

  X
    BLOB containing ECDSA key X parameter.

  Y
    BLOB containing ECDSA key Y parameter.

  ExplicitPara
    [optional] XML part containing the <ExplicitParams> element.
    
 NOTE:
    CryptXml does not support explicit parameters due to CNG ECDSA key support.

****************************************************************************/
typedef struct _CRYPT_XML_KEY_ECDSA_KEY_VALUE{
    LPCWSTR                             wszNamedCurve;  // URN Format urn:oid:1.2.3.4
    CRYPT_XML_DATA_BLOB                 X;
    CRYPT_XML_DATA_BLOB                 Y;
    CRYPT_XML_BLOB                      ExplicitPara;   // Encoded <ExplicitParams>
} CRYPT_XML_KEY_ECDSA_KEY_VALUE;

/****************************************************************************
 CRYPT_XML_KEY_RSA_KEY_VALUE

 Encapsulates RSA key value.

  Modulus
    BLOB containing RSA key modulus.

  Exponent
    BLOB containing RSA key exponent.

****************************************************************************/
typedef struct _CRYPT_XML_KEY_RSA_KEY_VALUE{
    CRYPT_XML_DATA_BLOB                 Modulus;
    CRYPT_XML_DATA_BLOB                 Exponent;
} CRYPT_XML_KEY_RSA_KEY_VALUE;

/****************************************************************************
 CRYPT_XML_KEY_VALUE

 Represents the KeyValueType choice in the <KeyValue> element.

 dwType
    Indicates the union variant used for the key value.

    This can be one of the following values:
        CRYPT_XML_KEY_VALUE_TYPE_DSA    
        CRYPT_XML_KEY_VALUE_TYPE_RSA    
        CRYPT_XML_KEY_VALUE_TYPE_ECDSA  
        CRYPT_XML_KEY_VALUE_TYPE_CUSTOM 

  DSAKeyValue
    CRYPT_XML_KEY_DSA_KEY_VALUE representing the DSA key

  RSAKeyValue
    CRYPT_XML_KEY_RSA_KEY_VALUE representing the RSA key

  ECDSAKeyValue
    CRYPT_XML_KEY_ECDSA_KEY_VALUE representing the ECDSA key

  Custom
    XML part containing unsupported elements of the <KeyValue> element.

Schema:

     <element name="KeyValue" type="ds:KeyValueType"/> 
       <complexType name="KeyValueType" mixed="true">
        <choice>
          <element ref="ds:DSAKeyValue"/>
          <element ref="ds:RSAKeyValue"/>
          <any namespace="##other" processContents="lax"/>
        </choice>
       </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_KEY_VALUE{
    DWORD                               dwType;
    union
    {
        CRYPT_XML_KEY_DSA_KEY_VALUE     DSAKeyValue;
        CRYPT_XML_KEY_RSA_KEY_VALUE     RSAKeyValue;
        CRYPT_XML_KEY_ECDSA_KEY_VALUE   ECDSAKeyValue;
        CRYPT_XML_BLOB                  Custom;            // XML Encoded element
    };
} CRYPT_XML_KEY_VALUE;

#define CRYPT_XML_KEY_VALUE_TYPE_DSA    0x00000001
#define CRYPT_XML_KEY_VALUE_TYPE_RSA    0x00000002
#define CRYPT_XML_KEY_VALUE_TYPE_ECDSA  0x00000003
#define CRYPT_XML_KEY_VALUE_TYPE_CUSTOM 0x00000004

/****************************************************************************
 CRYPT_XML_ISSUER_SERIAL

   <complexType name="X509IssuerSerialType"> 
     <sequence> 
       <element name="X509IssuerName" type="string"/> 
       <element name="X509SerialNumber" type="integer"/> 
     </sequence>
   </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_ISSUER_SERIAL{
    LPCWSTR                             wszIssuer;
    LPCWSTR                             wszSerial;
} CRYPT_XML_ISSUER_SERIAL;

/****************************************************************************
 CRYPT_XML_X509DATA_ITEM

 Represents the X509DataType choice in the <X509Data> element.

 dwType
    Indicates the union variant used for the X509Data.

    This can be one of the following values:
        CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL   
        CRYPT_XML_X509DATA_TYPE_SKI             
        CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME    
        CRYPT_XML_X509DATA_TYPE_CERTIFICATE     
        CRYPT_XML_X509DATA_TYPE_CRL             
        CRYPT_XML_X509DATA_TYPE_CUSTOM          

  IssuerSerial
    CRYPT_XML_ISSUER_SERIAL representing the <X509IssuerSerial> element

  SKI
    BLOB containing the <X509SKI> element

  wszSubjectName
    Subject name as a Unicode string

  Certificate
    BLOB containing X.509 certificate

  CRL
    BLOB containing X.509 certificate revocation list

  Custom
    XML part containing unsupported elements of the <X509Data> element.

****************************************************************************/
typedef struct _CRYPT_XML_X509DATA_ITEM{
    DWORD                               dwType;
    union
    {
        CRYPT_XML_ISSUER_SERIAL         IssuerSerial;
        CRYPT_XML_DATA_BLOB             SKI;
        LPCWSTR                         wszSubjectName;
        CRYPT_XML_DATA_BLOB             Certificate;
        CRYPT_XML_DATA_BLOB             CRL;
        CRYPT_XML_BLOB                  Custom;            // XML Encoded element
    };
} CRYPT_XML_X509DATA_ITEM;

#define CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL   0x00000001
#define CRYPT_XML_X509DATA_TYPE_SKI             0x00000002
#define CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME    0x00000003
#define CRYPT_XML_X509DATA_TYPE_CERTIFICATE     0x00000004
#define CRYPT_XML_X509DATA_TYPE_CRL             0x00000005
#define CRYPT_XML_X509DATA_TYPE_CUSTOM          0x00000006

/****************************************************************************
 CRYPT_XML_X509DATA

  Represents the sequence of choices in the <X509Data> element.

  cX509Data
    Number of elements in rgX509Data

  rgX509Data
    Array of CRYPT_XML_X509DATA_ITEM

Schema:

   <element name="X509Data" type="ds:X509DataType"/> 
   <complexType name="X509DataType">
     <sequence maxOccurs="unbounded">
       <choice>
         <element name="X509IssuerSerial" type="ds:X509IssuerSerialType"/>
         <element name="X509SKI" type="base64Binary"/>
         <element name="X509SubjectName" type="string"/>
         <element name="X509Certificate" type="base64Binary"/>
         <element name="X509CRL" type="base64Binary"/>
         <any namespace="##other" processContents="lax"/>
       </choice>
     </sequence>
   </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_X509DATA{
    UINT                                cX509Data;
    _Field_size_(cX509Data)
    CRYPT_XML_X509DATA_ITEM             *rgX509Data;
} CRYPT_XML_X509DATA;

/****************************************************************************
 CRYPT_XML_KEY_INFO_ITEM

 Represents the KeyInfoType choice in the <KeyInfo> element.

 dwType
    Indicates the union variant used for the key info.

    This can be one of the following values:
        CRYPT_XML_KEYINFO_TYPE_KEYNAME      
        CRYPT_XML_KEYINFO_TYPE_KEYVALUE     
        CRYPT_XML_KEYINFO_TYPE_RETRIEVAL    
        CRYPT_XML_KEYINFO_TYPE_X509DATA     
        CRYPT_XML_KEYINFO_TYPE_CUSTOM       

  wszKeyName
    Key name is a Unicode string

  KeyValue
    CRYPT_XML_KEY_VALUE representing the <KeyValue> element

  RetrievalMethod
    BLOB containing the <RetrievalMethod> element

  X509Data
    CRYPT_XML_X509DATA representing the <X509Data> element
  
  Custom            
    XML part containing unsupported elements of the key info.

****************************************************************************/
typedef struct _CRYPT_XML_KEY_INFO_ITEM{
    DWORD                               dwType;
    union
    {
        LPCWSTR                         wszKeyName;
        CRYPT_XML_KEY_VALUE             KeyValue;
        CRYPT_XML_BLOB                  RetrievalMethod;    // XML Encoded element
        CRYPT_XML_X509DATA              X509Data;
        CRYPT_XML_BLOB                  Custom;             // XML Encoded element
    };
} CRYPT_XML_KEY_INFO_ITEM;

#define CRYPT_XML_KEYINFO_TYPE_KEYNAME      0x00000001
#define CRYPT_XML_KEYINFO_TYPE_KEYVALUE     0x00000002
#define CRYPT_XML_KEYINFO_TYPE_RETRIEVAL    0x00000003
#define CRYPT_XML_KEYINFO_TYPE_X509DATA     0x00000004
#define CRYPT_XML_KEYINFO_TYPE_CUSTOM       0x00000005

/****************************************************************************
 CRYPT_XML_KEY_INFO

 Encapsulates the <KeyInfo> which is an optional element that enables 
 the recipient(s) to obtain the key needed to validate the signature.

 cbSize
    Size of this structure. 

 wszId
    [optional] Specifies the ID attribute.

 cKeyInfo
    Number of elements in rgKeyInfo

 rgKeyInfo
    Array of CRYPT_XML_KEY_INFO_ITEM.

 hVerifyKey
    A BCrypt key handle which is resolved from the first <KeyValue> element.
    NOTE: If more than one <KeyValue> elements included, then only the first
          one is used, and the rest are ignored. 

Schema:

   <element name="KeyInfo" type="ds:KeyInfoType"/> 
   <complexType name="KeyInfoType" mixed="true">
     <choice maxOccurs="unbounded">     
       <element ref="ds:KeyName"/> 
       <element ref="ds:KeyValue"/> 
       <element ref="ds:RetrievalMethod"/> 
       <element ref="ds:X509Data"/> 
       <element ref="ds:PGPData"/> 
       <element ref="ds:SPKIData"/>
       <element ref="ds:MgmtData"/>
       <any processContents="lax" namespace="##other"/>
       <!-- (1,1) elements from (0,unbounded) namespaces -->
     </choice>
     <attribute name="Id" type="ID" use="optional"/>
   </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_KEY_INFO{
    ULONG                               cbSize;
    LPCWSTR                             wszId;
    UINT                                cKeyInfo;
    _Field_size_(cKeyInfo)
    CRYPT_XML_KEY_INFO_ITEM             *rgKeyInfo;
    BCRYPT_KEY_HANDLE                   hVerifyKey;         // <= A handle, resolved from the first KeyValue, if any
}CRYPT_XML_KEY_INFO, *PCRYPT_XML_KEY_INFO;


/****************************************************************************
 CRYPT_XML_REFERENCE

 cbSize
    Size of this structure. 

 hReference
    Handle to the Reference object.

 wszId
    [optional] Specifies unique identifier attribute. 

 wszUri
    [optional] Specifies the URI attribute. 

 wszType
    [optional] Specifies the Type attribute. 

 DigestMethod
    Specifies digest method.

 DigestValue
    Specifies hash value. 

 cTransform
    Number of elements in the array rgTransform.

 rgTransform
    Array of structures  CRYPT_XML_TRANSFORM_INFO, 
    each holding information about Transform applied to the signed data.

****************************************************************************/
typedef struct _CRYPT_XML_REFERENCE{
    ULONG                               cbSize;    
    HCRYPTXML                           hReference;
    LPCWSTR                             wszId;
    LPCWSTR                             wszUri;
    LPCWSTR                             wszType;
    CRYPT_XML_ALGORITHM 	            DigestMethod;
    CRYPT_DATA_BLOB		                DigestValue;
    ULONG			                    cTransform;
    _Field_size_(cTransform)
    CRYPT_XML_ALGORITHM                 *rgTransform;
}CRYPT_XML_REFERENCE, *PCRYPT_XML_REFERENCE;

/****************************************************************************
 CRYPT_XML_REFERENCES

****************************************************************************/
typedef struct _CRYPT_XML_REFERENCES{
    ULONG                               cReference;
    _Field_size_(cReference)
    PCRYPT_XML_REFERENCE                *rgpReference;
}CRYPT_XML_REFERENCES, *PCRYPT_XML_REFERENCES;

/****************************************************************************
 CRYPT_XML_SIGNED_INFO

 cbSize
    Size of this structure. 

 wszId
    [optional] Specifies unique identifier attribute. 

 Canonicalization
    Specifies a canonicalization algorithm. 

 SignatureMethod
    Specifies a signature algorithm. 

 cReference
    Number of elements in the array rgReference. 

 rgpReference
    Array of pointers to CRYPT_XML_REFERENCE. 

 Encoded
    Contains canonicalized <SignedInfo> element.

****************************************************************************/
typedef struct _CRYPT_XML_SIGNED_INFO{
    ULONG                               cbSize;    
    LPCWSTR                             wszId;
    CRYPT_XML_ALGORITHM                 Canonicalization;
    CRYPT_XML_ALGORITHM                 SignatureMethod;
    ULONG                               cReference;
    _Field_size_(cReference)
    PCRYPT_XML_REFERENCE                *rgpReference;
    CRYPT_XML_BLOB                      Encoded;
}CRYPT_XML_SIGNED_INFO, *PCRYPT_XML_SIGNED_INFO;

/****************************************************************************
 CRYPT_XML_OBJECT

 cbSize
    Size of this structure. 

 hObject
    Handle to the Object object.

 wszId
    [optional] Specifies unique identifier attribute. 

 wszMimeType
    [optional] Specifies MIME-type attribute. 

 wszEncoding
    [optional] Specifies encoding method attribute. 

 Manifest
    [optional] Specifies array of References in Manifest.

 Encoded
    [optional] An XML part of the entire <Object> element.
    This field is empty when the <Object> element does not contain
    '##any' elements, therefore applications do not need to parse 
    it while validating the signature.
    Applications may use CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT flag
    to always receive encoded <Object> element.

 XML Schema:

    <element name="Object" type="ds:ObjectType"/>
    <complexType name="ObjectType" mixed="true">
       <element ref="ds:Manifest" minOccurs="0" maxOccurs="1"/> 
        <sequence minOccurs="0" maxOccurs="unbounded">
            <any namespace="##any" processContents="lax"/>
        </sequence>
        <attribute name="Id" type="ID" use="optional"/>
        <attribute name="MimeType" type="string" use="optional"/>
        <attribute name="Encoding" type="anyURI" use="optional"/>
    </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_OBJECT{
    ULONG                               cbSize;    
    HCRYPTXML                           hObject;
    LPCWSTR                             wszId;
    LPCWSTR                             wszMimeType;
    LPCWSTR                             wszEncoding;
    CRYPT_XML_REFERENCES                Manifest;      // OPTIONAL
    CRYPT_XML_BLOB                      Encoded;
}CRYPT_XML_OBJECT, *PCRYPT_XML_OBJECT;

/****************************************************************************
 CRYPT_XML_SIGNATURE

 cbSize
    Size of this structure. 

 hSignature
    Handle to the Signature object.

 SignedInfo
    The structure of CRYPT_XML_SIGNED_INFO includes the canonicalization algorithm, 
    a signature algorithm, and one or more references. 
    The SignedInfo element may contain an optional ID attribute that will allow 
    it to be referenced by other signatures and objects. 

SignatureValue
    CRYPT_DATA_BLOB that contains a cryptographic signature value over the <SignedInfo> element. 

KeyInfo
    [optional] Specifies key info. 

cObject
    Number of elements in the array rgObject. 

rgObject
    Array of structures, each holding information of type CRYPT_XML_OBJECT. 

 XML Schema:

    <element name="Signature" type="ds:SignatureType"/>
    <complexType name="SignatureType">
        <sequence>
            <element ref="ds:SignedInfo"/>
            <element ref="ds:SignatureValue"/>
            <element ref="ds:KeyInfo" minOccurs="0"/>
            <element ref="ds:Object" minOccurs="0" maxOccurs="unbounded"/>
        </sequence>
        <attribute name="Id" type="ID" use="optional"/>
    </complexType>

****************************************************************************/
typedef struct _CRYPT_XML_SIGNATURE{
    ULONG                               cbSize;    
    HCRYPTXML                           hSignature;
    LPCWSTR                             wszId;
    CRYPT_XML_SIGNED_INFO               SignedInfo;
    CRYPT_DATA_BLOB                     SignatureValue;
    CRYPT_XML_KEY_INFO                  *pKeyInfo;          // OPTIONAL
    ULONG                               cObject;
    _Field_size_(cObject)
    PCRYPT_XML_OBJECT                   *rgpObject;         // OPTIONAL
}CRYPT_XML_SIGNATURE, *PCRYPT_XML_SIGNATURE;

/****************************************************************************
 CRYPT_XML_DOC_CTXT

 cbSize
    Size of this structure. 

 hDocCtxt
    Handle to Document Context.

 cSignature
    Number of elements in the array rgSignature. 

 rgSignature
    Array of structures, each holding information of type CRYPT_XML_SIGNATURE. 

 pEncoded
    Contains XML document.

****************************************************************************/
typedef struct _CRYPT_XML_DOC_CTXT{
    ULONG                               cbSize;    
    HCRYPTXML                           hDocCtxt;
    CRYPT_XML_TRANSFORM_CHAIN_CONFIG    *pTransformsConfig;
    ULONG                               cSignature;
    _Field_size_(cSignature)
    PCRYPT_XML_SIGNATURE                *rgpSignature;
}CRYPT_XML_DOC_CTXT, *PCRYPT_XML_DOC_CTXT;

/****************************************************************************
 CryptXmlClose

 The CryptXmlClose function closes a cryptographic XML object handle. 
 At each call to this function, the reference count on the handle 
 is reduced by one.  When the reference count reaches zero, an object 
 encapsulated by the handle is fully released.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlClose(
    _In_            HCRYPTXML            hCryptXml
    );

/****************************************************************************
 CryptXmlGetTransforms

 The CryptXmlGetTransforms function returns information on 
 default Transform Chain Engine.

****************************************************************************/
_Success_( return == 0 )
HRESULT
WINAPI
CryptXmlGetTransforms(
    _Outptr_ const CRYPT_XML_TRANSFORM_CHAIN_CONFIG   **ppConfig
    );

/****************************************************************************
 CryptXmlOpenToEncode

 The CryptXmlOpenToEndoce function opens an XML digital signature to encode 
 and returns a handle of the opened signature object. 
 The handle encapsulates a Document Context with single Signature object and 
 remains open until CryptXmlClose is called.

 pConfig
    [in, optional] Specifies the transform chain engine. 
    If this parameter is NULL, 
    then a default engine will be used to apply transforms.

 dwFlags 
    [in] Currently defined dwFlags are shown in the following table. 

        CRYPT_XML_FLAG_NO_SERIALIZE
        CRYPT_XML_FLAG_DISABLE_EXTENSIONS
        CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT | CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT

 wszId 
 	[in, optional] Specifies the Id attribute of the <Signature> element.
    If this parameter is NULL or an empty string, then no Id attribute will be produced. 

 rgProperty
    [in, optional] Specifies additional properties.
    This pointer must be valid until CryptXmlClose is 
    called on the document context.

 phSignature
 	[out] Handle to the Signature object.

****************************************************************************/
_Success_( return == 0 )
HRESULT
WINAPI
CryptXmlOpenToEncode(
    _In_opt_        const CRYPT_XML_TRANSFORM_CHAIN_CONFIG    *pConfig,
                    DWORD               dwFlags,
    _In_opt_        LPCWSTR             wszId,
    _In_reads_opt_(cProperty) 
                    const CRYPT_XML_PROPERTY* rgProperty, 
    _In_            ULONG               cProperty, 
    _In_opt_        const CRYPT_XML_BLOB *pEncoded,
    _Outptr_     HCRYPTXML           *phSignature
    );

/****************************************************************************
 CryptXmlOpenToDecode

 The CryptXmlOpenToDecode function opens an XML digital signature to decode 
 and returns a handle of the Document Context that encapsulates Signatures 
 object. The handle remains open until CryptXmlClose is called. 
 The Document Context may include one or more Signature objects.

 hEngine
    [in, optional] Handle of the transform chain engine. 
    If this parameter is NULL, then a default engine will be 
    used to apply transforms.

 dwFlags 
    [in] Currently defined dwFlags are shown in the following table. 

        CRYPT_XML_FLAG_NO_SERIALIZE
        CRYPT_XML_FLAG_DISABLE_EXTENSIONS
        CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT
        CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT | CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT

 rgProperty
    [in, optional] Specifies additional properties.

 phCryptXml
 	[out] Handle to the Document Context object.

****************************************************************************/
_Success_( return == 0 )
HRESULT
WINAPI
CryptXmlOpenToDecode(
    _In_opt_        const CRYPT_XML_TRANSFORM_CHAIN_CONFIG    *pConfig,
                    DWORD               dwFlags,
    _In_reads_opt_(cProperty) 
                    const CRYPT_XML_PROPERTY* rgProperty, 
    _In_            ULONG               cProperty, 
    _In_            const CRYPT_XML_BLOB *pEncoded,
    _Outptr_     HCRYPTXML           *phCryptXml
    );

/****************************************************************************
 CryptXmlAddObject

 The CryptXmlAddObject function adds the <Object> element to Signature in 
 the Document Context opened to encode. 
 See Remarks.

 hSignatureOrObject
    [in] Handle to the Signature returned by CryptXmlOpenToEncode;
    or handle to a Reference returned by CryptXmlCreateReference with 
    CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT flag. See Remarks.

 dwFlags
    [in] The following flags are defined.
        CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY

 rgProperty
    [in, optional] Specifies additional properties used to decode the <Object> element.

 cProperty
    [in] Specifies number of elements in the rgProperty array.

 pEncoded 
    [in] Specifies the <Object> element or a part of it, see Remarks.

 ppObject
    [out, optional] Pointer to CRYPT_XML_OBJECT* to receive a decoded structure.
    This parameter must be NULL when hSignatureOrObject is a handle to the Object.

 REMARKS:
    When the hSignatureOrObject specifies a handle to a Reference returned 
    by CryptXmlCreateReference, then pEncoded specifies XML content to be included
    inside the <Object> node after the optional <Manifest> element.
    The pEncoded pointer must be valid till the signature is complete. 
    Otherwise, use CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY flag to create in-memory copy.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlAddObject(
    _In_            HCRYPTXML           hSignatureOrObject,
                    DWORD               dwFlags,
    _In_reads_opt_(cProperty) 
                    const CRYPT_XML_PROPERTY* rgProperty, 
    _In_            ULONG               cProperty, 
    _In_            const CRYPT_XML_BLOB *pEncoded,
    _Outptr_result_maybenull_ const CRYPT_XML_OBJECT **ppObject
    );

// Used to create in-memory copy of XML part to be included in the <Object>
#define CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY   0x00000001

/****************************************************************************
 CryptXmlCreateReference

 hCryptXml 
    [in] Handle of the XML signature.

 dwFlags 
    [in] Currently defined dwFlags are shown in the following table. 
        CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT

 pwszId 
 	[in, optional] Specifies the Id attribute of the <Reference> element.
	If this parameter is NULL, then the Id attribute will not be created. 
	If this parameter is an empty string, then the Id attribute with empty 
        value will be created.

 pwszURI 
    [in] Specifies the URI attribute of the <Reference> element. 
    If this parameter is an empty string, 
    then the Uri attribute with empty value will be created.

 pwszType 
    [in, optional] Specifies the Type attribute of the <Reference> element. 
    The processing Engine does not check or use this attribute.

 pDigestMethod
    [in] Specifies the digest method.

 cTransform
    [in] Number of elements in rgTransform array.

 rgTransform
    [in, optional] Ordered array of transform algorithms to be applied to 
    the reference data before digest calculation.

 phReference
    [out] Pointer to a Reference Handle.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlCreateReference(
    _In_	        HCRYPTXML           hCryptXml,
                    DWORD               dwFlags,
    _In_opt_	    LPCWSTR             wszId,
    _In_opt_	    LPCWSTR             wszURI,
    _In_opt_	    LPCWSTR             wszType,
    _In_            const CRYPT_XML_ALGORITHM *pDigestMethod,
                    ULONG               cTransform,
    _In_reads_opt_(cTransform)   	
                    const CRYPT_XML_ALGORITHM *rgTransform,
    _Outptr_     HCRYPTXML            *phReference
    );

// When this flag is set an Object will be created and added to the Signature, 
// and a Reference to the Object will be created in SignedInfo.
// The returned handle is encapsulated Object and can be used
// in subsequent calls to CryptXmlCreateReference to create references in
// Manifest.

#define CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT      0x00000001

/****************************************************************************
 CryptXmlDigestReference

 The CryptXmlDigestReference function is used by an application to digest 
 the resolved reference. 
 This function applies Transforms before updating the digest.

 hReference
    [in] Handle to Reference object.

 dwFlags 
    [in] Currently defined dwFlags are shown in the following table. 

 pDataProviderIn 
    [in,out] Specifies the data provider.

Remarks:

    When CRYPT CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED flag is set,
    the processing engine directly digests received data without 
    applying the transform chain engine.
 
    The CryptXmlDigestReference function always calls pDataProviderIn->fpnClose.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlDigestReference(
    _In_            HCRYPTXML           hReference,
                    DWORD               dwFlags,
    _In_            CRYPT_XML_DATA_PROVIDER *pDataProviderIn
    );

// Specifies that the processing engine creates the digest 
// without applying the transform chain engine.
#define CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED           0x00000001

/****************************************************************************
 CryptXmlSetHMACSecret

 The CryptXmlSetHMACSecret function set HMAC secret on the handle before
 calling CryptXmlSign or CryptXmlVerify.

 hSignature
    [in] Handle to Signature. 

 pbSecret
    [in] Pointer to a block of bytes. 
    The pointer must be valid during calls to CryptXmlSign or CryptXmlVerify.

 cbSecret
    Size, in bytes, of the block of data at pbSecret.

 NOTE: When using HMAC, the key handles passed to CryptXmlSign or CryptXmlVerify
 must be NULL.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlSetHMACSecret(
    _In_            HCRYPTXML           hSignature,
    _In_reads_bytes_( cbSecret )		
                    const BYTE          *pbSecret, 
                    ULONG               cbSecret
    );

/****************************************************************************
 CRYPT_XML_KEYINFO_PARAMS

  This structure is passed to CryptXmlSign specifying
  the members of <KeyInfo> element to be encoded.

  wszId
    Specifies the Id attribute of the <KeyInfo> element

  wszKeyName
    Specifies value for the <KeyName> element

  SKI
    Specifies value for the <X509SKI> element

  wszSubjectName
    Specifies value for the <X509SubjectName> element

  cCertificate
    Specifies number of items in the rgCertificate array
  
  rgCertificate
    Specifies array of CERT_BLOB to populate <X509Certificate> elements

  cCRL;
    Specifies number of items in the rgCRL array

  rgCRL
    Specifies array of CERT_BLOB to populate <X509CRL> elements

****************************************************************************/
typedef struct _CRYPT_XML_KEYINFO_PARAM
{
    LPCWSTR                             wszId;
    LPCWSTR                             wszKeyName;
    CERT_BLOB                           SKI;
    LPCWSTR                             wszSubjectName;
    ULONG                               cCertificate;
    _Field_size_(cCertificate)
    CERT_BLOB                           *rgCertificate;
    ULONG                               cCRL;
    _Field_size_(cCRL)
    CERT_BLOB                           *rgCRL;
}CRYPT_XML_KEYINFO_PARAM;

/****************************************************************************
 CRYPT_XML_KEYINFO_SPEC

****************************************************************************/
typedef enum
{
    CRYPT_XML_KEYINFO_SPEC_NONE    = 0,     // No KeyInfo
    CRYPT_XML_KEYINFO_SPEC_ENCODED = 1,     // CRYPT_XML_BLOB*
    CRYPT_XML_KEYINFO_SPEC_PARAM   = 2,     // CRYPT_XML_KEYINFO_PARAM*
}CRYPT_XML_KEYINFO_SPEC;

/****************************************************************************
 CryptXmlSign

 The CryptXmlSign function creates a cryptographic signature over SignedInfo 
 element in a document context opened for encode.

 hSignature
    [in] Handle to Signature. 

 hKey
    [in] Handle to a Private Key used to sign the SignedInfo element.
    This parameter must be NULL for HMAC-based signature algorithms.

 dwKeySpec
    [in] Identifies the private key to use from the provider's container. 
    It can be AT_KEYEXCHANGE, AT_SIGNATURE or 0.
    This parameter is ignored if an NCRYPT_KEY_HANDLE is used in 
    the hCryptProvOrNCryptKey parameter.

 dwFlags 
    [in] The following value is defined. 
        CRYPT_XML_SIGN_ADD_KEYVALUE
        CRYPT_XML_FLAG_DISABLE_EXTENSIONS

 dwKeyInfoSpec 
    [in] Specifies the type of pvKeyInfoSpec data structure. 
    
 pvKeyInfoSpec
    [in] Pointer to a structure determined by the value of dwKeyInfoSpec. 
    The following table specifies possible combinations for 
    dwKeyInfoSpec and pvKeyInfoSpec

    dwKeyInfoSpec                           pvKeyInfoSpec
    -----------------------------           ------------------------
    CRYPT_XML_KEYINFO_SPEC_NONE             NULL
    CRYPT_XML_KEYINFO_SPEC_ENCODED          CRYPT_XML_BLOB*
    CRYPT_XML_KEYINFO_SPEC_PARAM            CRYPT_XML_KEYINFO_PARAM*

 pSignatureMethod
    [in] Specifies the signature method.

 pCanonicalization
    [in] Specifies the canonicalization method.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlSign(
    _In_            HCRYPTXML           hSignature,
    _In_opt_        HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hKey,
                    DWORD               dwKeySpec,
                    DWORD               dwFlags,
                    CRYPT_XML_KEYINFO_SPEC dwKeyInfoSpec,
    _In_opt_        const void          *pvKeyInfoSpec,
    _In_            const CRYPT_XML_ALGORITHM *pSignatureMethod,
    _In_            const CRYPT_XML_ALGORITHM *pCanonicalization
    );

// Specify this flag to populate <KeyValue> element from 
// the hKey handle.
// NOTE: This flag can not be used when dwKeyInfoSpec is 
//       set to CRYPT_XML_KEYINFO_SPEC_ENCODED
#define CRYPT_XML_SIGN_ADD_KEYVALUE         0x00000001

/****************************************************************************
 CryptXmlImportPublicKey

    dwFlags
        The following flags are supported:
            CRYPT_XML_FLAG_DISABLE_EXTENSIONS

    pKeyValue
        CRYPT_XML_KEY_VALUE to be imported.

    phKey
        [out] Pointer to BCRYPT_KEY_HANDLE that receives imported key handle.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlImportPublicKey(
                    DWORD               dwFlags,
    _In_            const CRYPT_XML_KEY_VALUE *pKeyValue,
    _Out_           BCRYPT_KEY_HANDLE   *phKey
    );

/****************************************************************************
 CryptXmlVerifySignature

 The CryptXmlVerifySignature function performs a cryptographic signature 
 validation over SignedInfo element in a document context opened for decode. 
 hSignature
    [in] Handle to Signature. 

 hKey
    [in] Handle to a Public Key to verify the signature value on 
    the SignedInfo element.
    This parameter must be NULL for HMAC-based signature algorithms.

 dwFlags
    [in] The following flags are supported:
        CRYPT_XML_FLAG_DISABLE_EXTENSIONS

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlVerifySignature(
    _In_            HCRYPTXML           hSignature,
    _In_opt_        BCRYPT_KEY_HANDLE   hKey,
                    DWORD               dwFlags
    );

/****************************************************************************
 CryptXmlGetDocContext

 The CryptXmlGetDocContext function returns data structure encapsulated by 
 a handle to Document Context. 

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlGetDocContext(
    _In_            HCRYPTXML           hCryptXml,
    _Outptr_     const CRYPT_XML_DOC_CTXT **ppStruct	
    );

/****************************************************************************
 CryptXmlGetSignature

 The CryptXmlGetSignature function returns data structure encapsulated 
 by a handle to Signature.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlGetSignature(
    _In_            HCRYPTXML           hCryptXml,
    _Outptr_     const CRYPT_XML_SIGNATURE **ppStruct	
    );

/****************************************************************************
 CryptXmlGetReference

The CryptXmlGetReference function returns data structure encapsulated 
by a handle to Reference.
****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlGetReference(
    _In_            HCRYPTXML           hCryptXml,
    _Outptr_     const CRYPT_XML_REFERENCE **ppStruct	
    );

/****************************************************************************
 CryptXmlGetStatus

The CryptXmlGetStatus function returns a CRYPT_XML_STATUS for array 
of Signature, single Signature, Reference or Manifest.

****************************************************************************/
_Success_( return == 0 )
HRESULT 
WINAPI 
CryptXmlGetStatus(
    _In_            HCRYPTXML           hCryptXml,
    _Out_           CRYPT_XML_STATUS    *pStatus
    );

/****************************************************************************
 CryptXmlEncode

 The CryptXmlEncode function constructs XML document using the XML writer 
 callback that allows applications to stream XML being constructed.

 hCryptXml
    [in] Specifies the handle of an object to be serialized. 
    The handle can be of Signature, Object, Reference types. 

 rgProperty
    [in, optional] Specifies additional properties used to encode XML.

 cProperty
    [in] Specifies number of elements in the rgProperty array.

 pvCallbackState
    [in] An application-defined argument to be passed to the callback.

 pfnWrite
    [in, callback] An application-defined callback to receive constructed XML.

****************************************************************************/
_Success_( return == 0 )
HRESULT
WINAPI
CryptXmlEncode(
    _In_            HCRYPTXML           hCryptXml,
                    CRYPT_XML_CHARSET   dwCharset,
    _In_reads_opt_(cProperty) 
                    const CRYPT_XML_PROPERTY* rgProperty, 
    _In_            ULONG               cProperty, 
    _Inout_         void                *pvCallbackState,
    _In_            PFN_CRYPT_XML_WRITE_CALLBACK pfnWrite
    );

/****************************************************************************
 CRYPT_XML_ALGORITHM_INFO

    cbSize
        The size, in bytes, of this structure.

    wszAlgorithmURI
        The URI associated with attribute of the SignatureMethod or DigestMethod elements

    wszName
        [optional] Friendly name of the algorithm.

    dwGroupId
        Can be one of the following values:
            CRYPT_XML_GROUP_ID_HASH         Hash algorithms
            CRYPT_XML_GROUP_ID_SIGN         Signature algorithms

    wszCNGAlgid
        The algorithm identifier string passed to the CNG functions 
        CNG functions use algorithm identifier strings, such as L"SHA1", 

    wszCNGExtraAlgid
        An extra algorithm string, other than the string in the pwszCNGAlgid member, 
        that can be passed to the CNG functions.
    
    dwSignFlags
        This value is passed to NCryptSignHash
    
    dwVerifyFlags
        This value is passed to BCryptVerifySignature.
    
    pvPaddingInfo
        A pointer to a structure that contains padding information. 
        The actual type of structure this parameter points to depends on 
        the value of the dwFlags parameter.
        This pointer is passed to NCryptSignHash or BCryptVerifySignature

    pvExtraInfo
        [optional] A pointer to a structure that contains extra information 
        that can be passed to the CNG functions.

NOTE: the BCrypt* and NCrypt* functions that are defined in Bcrypt.h and Ncrypt.h

****************************************************************************/
typedef 
struct _CRYPT_XML_ALGORITHM_INFO
{
    DWORD                               cbSize;
    WCHAR                               *wszAlgorithmURI;
    WCHAR                               *wszName;
    DWORD                               dwGroupId;
    WCHAR                               *wszCNGAlgid;
    WCHAR                               *wszCNGExtraAlgid;
    DWORD                               dwSignFlags;	    // NCryptSignHash flags
    DWORD                               dwVerifyFlags;	    // BCryptVerifySignature flags
    void                                *pvPaddingInfo;
    void                                *pvExtraInfo;
}CRYPT_XML_ALGORITHM_INFO, *PCRYPT_XML_ALGORITHM_INFO;

#define CRYPT_XML_GROUP_ID_HASH         1
#define CRYPT_XML_GROUP_ID_SIGN         2

/****************************************************************************
 CryptXmlGetAlgorithmInfo

    Decodes the XML Algorithm and returns information about the algorithm.

    The following flags are supported:
        CRYPT_XML_FLAG_DISABLE_EXTENSIONS

    The caller must free ppAlgInfo by calling LocalFree()

****************************************************************************/
_Success_( return == 0 )
HRESULT
WINAPI 
CryptXmlGetAlgorithmInfo(
    _In_            const CRYPT_XML_ALGORITHM       *pXmlAlgorithm,
                    DWORD                           dwFlags,
    _Outptr_     CRYPT_XML_ALGORITHM_INFO        **ppAlgInfo
    );

/****************************************************************************
 CryptXmlFindAlgorithmInfo

    dwFindByType
        [in] Specifies the pvFindBy parameter type.
            The following values are supported:
                CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI            - wszAlgorithmURI
                CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME           - wszName
                CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID      - wszCNGAlgid
                CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID - pwszCNGAlgid
                        pwszCNGAlgid[0] - Hash pwszCNGAlgid
                        pwszCNGAlgid[1] - PubKey pwszCNGAlgid

    pvFindBy
        [in] Specifies the search parameter.

    dwGroupId
        [in] Specifies the algorithm group.

    dwFlags
        [in] The following flags are supported:
                CRYPT_XML_FLAG_DISABLE_EXTENSIONS

    ppAlgInfo
        [out] Pointer to CRYPT_XML_ALGORITHM_INFO pointer.

****************************************************************************/
const CRYPT_XML_ALGORITHM_INFO*
WINAPI 
CryptXmlFindAlgorithmInfo(
    _In_            DWORD                           dwFindByType,
    _In_            const void                      *pvFindBy,
    _In_            DWORD                           dwGroupId,
                    DWORD                           dwFlags
    );

#define CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI                1
#define CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME               2
#define CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID          3
#define CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID     4

/****************************************************************************
 CryptXmlEnumAlgorithmInfo

 The CryptXmlEnumAlgorithmInfo function enumerates predefined and registered 
 CryptXml CRYPT_XML_ALGORITHM_INFO entries. 
 This function enumerates either all of the predefined and registered 
 entries or only structures identified by a selected URI group. 
 For each URI information structure enumerated,
 an application provided callback function, pfnEnumAlgInfo, is called.

    dwGroupId
        [in] Specifies an algorithm group to be searched (Digest, Signature)

    dwFlags
        [in] The following flags are defined.

        CRYPT_XML_INHIBIT_EXTENSIONS	
            Disable XML Cryptographic Extension algorithms.

    pvArg
        [in, optional] Specifies an application defined parameter for the callback. 

    pfnEnumAlgInfo
        [callback] Address of the callback function that will be called 
        for each Algorithm found. 

****************************************************************************/

//
// If the callback returns FALSE, then stop the enumeration.
//

typedef BOOL (WINAPI * PFN_CRYPT_XML_ENUM_ALG_INFO)(
    _In_            const CRYPT_XML_ALGORITHM_INFO *pInfo,
    _Inout_opt_     void                *pvArg
    );

_Success_( return == 0 )
HRESULT
WINAPI
CryptXmlEnumAlgorithmInfo(
    _In_            DWORD               dwGroupId,
    _In_            DWORD               dwFlags,
    _Inout_opt_     void                *pvArg,
    __callback PFN_CRYPT_XML_ENUM_ALG_INFO pfnEnumAlgInfo
    );

/****************************************************************************
 * CRYPTO EXTENSIBILITY
 *
 ***************************************************************************/

/****************************************************************************
 CryptXmlGetInterface

 Cryptographic Extensions DLL must export CryptXmlGetInterface entry.

    dwFlags 
        [in] Not used at the moment and must be 0.

    pMethod 
        [in] A pointer to a CRYPT_XML_ALGORITHM_INFO structure 
        to retrieve the interface of.

    pInterface 
        [out] A pointer to a CRYPT_XML_ALGORITHM_INFO structure 
        to receive the interface information.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllGetInterface)(
                    DWORD               dwFlags,
    _In_            const CRYPT_XML_ALGORITHM_INFO  *pMethod,
    _Out_           CRYPT_XML_CRYPTOGRAPHIC_INTERFACE *pInterface
    );

/****************************************************************************
 CryptXmlDllEncodeAlgorithm

    The CryptXmlDllEncodeAlgorithm function is used to encode <SignatureMethod>
    or <DigestMethod> elements for agile algorithms with default parameters.
    The CryptXmlDllEncodeAlgorithm function is exposed though 
    the exported CryptXmlDllGetInterface fucntion.

    pAlgInfo 
        [in] A pointer to a CRYPT_XML_ALGORITHM_INFO structure. 
    
    dwCharset 
        A CRYPT_XML_CHARSET value that specifies the character set of the encoded XML.
    
    pvCallbackState 
        [in, out] A poniter to an argument that is passed to the callback 
        function pointed to by the pfnWrite parameter.
    
    pfnWrite 
        [in] A PFN_CRYPT_XML_WRITE_CALLBACK callback to receive the encoded XML.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllEncodeAlgorithm)(
    _In_            const CRYPT_XML_ALGORITHM_INFO *pAlgInfo,
                    CRYPT_XML_CHARSET   dwCharset,
    _Inout_         void                *pvCallbackState,
    _In_            PFN_CRYPT_XML_WRITE_CALLBACK pfnWrite
    );

/****************************************************************************
 CryptXmlDllCreateDigest

    pDigestMethod 
        [in] A pointer to a CRYPT_XML_ALGORITHM structure that specifies 
        the algorithm to use to create the digest.

    pcbSize 
        [out] A pointer to a ULONG value to receive the size, 
        in bytes, of the digest.

    phDigest 
        [out] A pointer to a CRYPT_XML_DIGEST variable to receive a pointer 
        to the digest.
        CryptXml uses CryptXmlDllCloseDigest to free resources allocated
        in phDigest.

****************************************************************************/
typedef void*   CRYPT_XML_DIGEST;

typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllCreateDigest)(
    _In_            const CRYPT_XML_ALGORITHM *pDigestMethod,
    _Out_range_( 0, CRYPT_XML_DIGEST_VALUE_MAX )
                    ULONG               *pcbSize,
    _Out_           CRYPT_XML_DIGEST    *phDigest
    );

/****************************************************************************
 CryptXmlDllDigestData

    The CryptXmlDllDigestData function is used to digest data.
    The CryptXmlDllDigestData function is exposed though 
    the exported CryptXmlDllGetInterface fucntion.

    hDigest 
        [in] The handle of the hash object used to perform the digest. 
        This handle is obtained by calling the CryptXmlDllCreateDigest function.

    pbData 
        [in] A pointer to a block of data to be processed.

    cbDigest 
        [in] The size, in bytes, of the block of data pointed 
        to bye the pbData parameter.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllDigestData)(
    _In_	        CRYPT_XML_DIGEST    hDigest,
    _In_reads_bytes_(cbData) 
		            const BYTE          *pbData,      
		            ULONG               cbData
    );

/****************************************************************************
 CryptXmlDllFinalizeDigest

    The CryptXmlDllFinalizeDigest function isused to retrieve the digest value.
    The CryptXmlDllFinalizeDigest function is exposed though 
    the exported CryptXmlDllGetInterface fucntion.

    hDigest 
        [in] The handle of the hash object used to perform the digest. 
        This handle is obtained by calling the CryptXmlDllCreateDigest function.

    pbDigest 
        [out] A pointer to a buffer that receives the digest value.
    
    cbDigest 
        [in] The size, in bytes, of the buffer pointed to by the pbDigest.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllFinalizeDigest)(
    _In_            CRYPT_XML_DIGEST    hDigest,
    _Out_writes_bytes_(cbDigest) 
                    BYTE                *pbDigest,      
                    ULONG               cbDigest
    );

/****************************************************************************
 CryptXmlDllCloseDigest

    hDigest 
        [in] The handle of the digest object. 
        This handle is obtained by calling the CryptXmlCreateDigest function. 
        After the function has been called, the digest handle passed 
        to this function is released and cannot be used again.
****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllCloseDigest)(
    _In_            CRYPT_XML_DIGEST    hDigest
    );

/****************************************************************************
 CryptXmlDllSignData

    The CryptXmlDllSignData function is used to sign data.
    The CryptXmlDllSignData function is exposed though 
    the exported CryptXmlDllGetInterface fucntion.

    pSignatureMethod 
        [in] A pointer to a CRYPT_XML_ALGORITHM structure that specifies the algorithm.

    hCryptProvOrNCryptKey 
        [in] The handle of the CSP that creates the signature. 
        This handle must be an HCRYPTPROV handle that obtained from a call to
        the CryptAcquireContext function or an NCRYPT_KEY_HANDLE handle that has been 
        created by using the NCryptOpenKey function. 
        New applications should always pass in the NCRYPT_KEY_HANDLE handle.

    dwKeySpec 
        [in] Identifies the private key to use from the provider's container. 
        It can be AT_KEYEXCHANGE or AT_SIGNATURE. 
        This parameter is ignored if an NCRYPT_KEY_HANDLE is used in 
        the hCryptProvOrNCryptKey parameter.
    
    pbInput 
        [in] A pointer to a buffer that contains the digest value to sign. 
        The cbInput parameter contains the size of this buffer.
    
    cbInput 
        [in] The size, in bytes, of the buffer pointed to by the pbInput parameter.

    pbOutput 
        [out, optional] The address of a buffer to receive the signature 
        produced by this function. The cbOutput parameter contains the s
        ize of this buffer.
        If this parameter is NULL, this function will calculate the size needed for 
        the encrypted data and return the size in the location pointed to 
        by the pcbResult parameter.

    cbOutput 
        [in] The size, in bytes, of the buffer pointed to by the pbOutput pramater.
    
    pcbResult 
        [out] A pointer to a DWORD variable that receives the number of bytes copied 
        to the pbOutput buffer. 
        If pbOutput is NULL, this receives the size, in bytes, required 
        for the signature.


****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllSignData)(
    _In_            const CRYPT_XML_ALGORITHM *pSignatureMethod,
    _In_            HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
    _In_            DWORD               dwKeySpec,
    _In_reads_bytes_(cbInput)            
                    const BYTE          *pbInput,
    _In_            ULONG               cbInput,
    _Out_writes_bytes_to_opt_(cbOutput, *pcbResult) 
                    BYTE                *pbOutput,
    _In_            ULONG               cbOutput,
    _Out_range_( 0, cbOutput )
                    ULONG               *pcbResult
    );

/****************************************************************************
 CryptXmlDllVerifySignature

    The CryptXmlDllVerifySignature function is used to verify signature.
    The CryptXmlDllVerifySignature function is exposed though 
    the exported CryptXmlDllGetInterface fucntion.

    pSignatureMethod 
        [in] A pointer to a CRYPT_XML_ALGORITHM structure that specifies the algorithm.

    hCryptProv 
        [in] A handle to the cryptographic provider. 
        Windows Vista and higher:  Must be set to NULL with BCrypt key handle.
        Windows XP:  Must be a HCRYPTPROV returned by the CryptAcquireContext function.

    hKey 
        [in] A handle to the Public Key. 

    pbInput 
        [in] A pointer to a buffer that contains the signed data. 
        The cbInput parameter contains the size of this buffer. 

    cbInput 
        [in] The size, in bytes, of the buffer pointed to by teh pbInput paramter.

    pbSignature 
        [in] A pointer to a buffer that contains the signature value to be verified. 
        The cbSignature parameter contains the size of this buffer. 
    
    cbSignature 
        [in] The size, in bytes, of the pbSignature buffer.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllVerifySignature)(
    _In_            const CRYPT_XML_ALGORITHM *pSignatureMethod,
    _In_            BCRYPT_KEY_HANDLE   hKey,
    _In_reads_bytes_(cbInput)            
                    const BYTE          *pbInput,
    _In_            ULONG               cbInput,
    _In_reads_bytes_(cbSignature) 
                    const BYTE          *pbSignature,
    _In_            ULONG               cbSignature
    );

/****************************************************************************
 CryptXmlDllGetAlgorithmInfo

  The CryptXmlDllGetAlgorithmInfo fucntion decodes the XML Algorithm and 
  returns information about the algorithm.
  The CryptXmlDllGetAlgorithmInfo function is exposed though the exported 
  CryptXmlDllGetInterface fucntion.

    pXmlAlgorithm 
        [in] A Pointer to a CRYPT_XML_ALGORITHM structure that specifies 
        the algorithm for which to return information.

    ppAlgInfo 
        [out] A pointer to a pointer to a CRYPT_XML_ALGORITHM_INFO structure.
        The caller must free ppAlgInfo by calling LocalFree()

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI *CryptXmlDllGetAlgorithmInfo)(
    _In_            const CRYPT_XML_ALGORITHM       *pXmlAlgorithm,
    _Outptr_     CRYPT_XML_ALGORITHM_INFO        **ppAlgInfo
    );

/****************************************************************************
 CRYPT_XML_CRYPTOGRAPHIC_INTERFACE

    fpCryptXmlEncodeAlgorithm
        A pointer to the implementation of the CryptXmlDllEncodeAlgorithm. 

    fpCryptXmlCreateDigest
        A pointer to the implementation of the CryptXmlDllCreateDigest.

    fpCryptXmlDigestData
        A pointer to the implementation of the CryptXmlDllCreateDigest.

    fpCryptXmlFinalizeDigest
        A pointer to the implementation of the CryptXmlDllFinalizeDigest.

    fpCryptXmlCloseDigest
        A pointer to the implementation of the CryptXmlDllCloseDigest.

    fpCryptXmlSignData
        A pointer to the implementation of the CryptXmlDllSignData.

    fpCryptXmlVerifySignature
        A pointer to the implementation of the CryptXmlDllVerifySignature.

    fpCryptXmlGetAlgorithmInfo
        A pointer to the implementation of the CryptXmlDllGetAlgorithmInfo.

****************************************************************************/
typedef struct _CRYPT_XML_CRYPTOGRAPHIC_INTERFACE{
    ULONG                               cbSize;
    CryptXmlDllEncodeAlgorithm          fpCryptXmlEncodeAlgorithm;
    CryptXmlDllCreateDigest             fpCryptXmlCreateDigest;
    CryptXmlDllDigestData               fpCryptXmlDigestData;
    CryptXmlDllFinalizeDigest           fpCryptXmlFinalizeDigest;
    CryptXmlDllCloseDigest              fpCryptXmlCloseDigest;
    CryptXmlDllSignData                 fpCryptXmlSignData;
    CryptXmlDllVerifySignature          fpCryptXmlVerifySignature;
    CryptXmlDllGetAlgorithmInfo         fpCryptXmlGetAlgorithmInfo;
}CRYPT_XML_CRYPTOGRAPHIC_INTERFACE, *PCRYPT_XML_CRYPTOGRAPHIC_INTERFACE;

/****************************************************************************
 * KEYINFO EXTENSIBILITY
 *
 ***************************************************************************/

/****************************************************************************
 CryptXmlDllEncodeKeyValue

    Address of installable function used to encode <*KeyValue> element.

    hKey
        [in] The handle of the key value to encode. 

    dwCharset
        [in] Specifies a charset of the XML to be written to pfnWrite.

    pvCallbackState
        [in] An argument to be passed to the callback.

    pfnWrite
        [in, callback] A CryptXml defined callback to receive constructed XML.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI * CryptXmlDllEncodeKeyValue)(
    _In_            NCRYPT_KEY_HANDLE   hKey,
                    CRYPT_XML_CHARSET   dwCharset,
    _Inout_         void                *pvCallbackState,
    _In_            PFN_CRYPT_XML_WRITE_CALLBACK pfnWrite
    );

/****************************************************************************
 CryptXmlDllCreateKey

    Address of installable function used to parse <*KeyValue> element and 
    create a CNG NCrypt key handle to verify signature.

    pEncoded
        [in] Specifies XML encoded <*KeyValue>.
    
    phKey
        [out] A key handle used to verify signature. 
        CryptXml calls BCryptDestroyKey when it's done using it.

****************************************************************************/
typedef
_Success_( return == 0 )
HRESULT
(WINAPI * CryptXmlDllCreateKey)(
    _In_            const CRYPT_XML_BLOB *pEncoded,
    _Out_           BCRYPT_KEY_HANDLE   *phKey
    );

#ifdef __cplusplus
} //extern "C"
#endif /* __cplusplus */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPXDEPLOYMENT) */
#pragma endregion

