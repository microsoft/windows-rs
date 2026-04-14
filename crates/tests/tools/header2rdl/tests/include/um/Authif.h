/*/////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// FILE
//
//    authif.h
//
// SYNOPSIS
//
//    Declares the interface for extensions to the Internet Authentication
//    Service.
//
/////////////////////////////////////////////////////////////////////////////*/

#ifndef _AUTHIF_H_
#define _AUTHIF_H_
#if _MSC_VER >= 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/*
 *  Enumerates the attribute types that are passed to the extension DLL.  The
 *  RADIUS standard attributes are included for convenience and should not be
 *  considered exhaustive.
 */
typedef enum _RADIUS_ATTRIBUTE_TYPE {

    /* Used to terminate attribute arrays. */
    ratMinimum = 0,

    /* RADIUS standard attributes. */
    ratUserName = 1,
    ratUserPassword = 2,
    ratCHAPPassword = 3,
    ratNASIPAddress = 4,
    ratNASPort = 5,
    ratServiceType = 6,
    ratFramedProtocol = 7,
    ratFramedIPAddress = 8,
    ratFramedIPNetmask = 9,
    ratFramedRouting = 10,
    ratFilterId = 11,
    ratFramedMTU = 12,
    ratFramedCompression = 13,
    ratLoginIPHost = 14,
    ratLoginService = 15,
    ratLoginPort = 16,
    ratReplyMessage = 18,
    ratCallbackNumber = 19,
    ratCallbackId = 20,
    ratFramedRoute = 22,
    ratFramedIPXNetwork = 23,
    ratState = 24,
    ratClass = 25,
    ratVendorSpecific = 26,
    ratSessionTimeout = 27,
    ratIdleTimeout = 28,
    ratTerminationAction = 29,
    ratCalledStationId = 30,
    ratCallingStationId = 31,
    ratNASIdentifier = 32,
    ratProxyState = 33,
    ratLoginLATService = 34,
    ratLoginLATNode = 35,
    ratLoginLATGroup = 36,
    ratFramedAppleTalkLink = 37,
    ratFramedAppleTalkNetwork = 38,
    ratFramedAppleTalkZone = 39,
    ratAcctStatusType = 40,
    ratAcctDelayTime = 41,
    ratAcctInputOctets = 42,
    ratAcctOutputOctets = 43,
    ratAcctSessionId = 44,
    ratAcctAuthentic = 45,
    ratAcctSessionTime = 46,
    ratAcctInputPackets = 47,
    ratAcctOutputPackets = 48,
    ratAcctTerminationCause = 49,
    ratCHAPChallenge = 60,
    ratNASPortType = 61,
    ratPortLimit = 62,
    ratTunnelType = 64,
    ratMediumType = 65,
    ratTunnelPassword = 69,
    ratTunnelPrivateGroupID = 81,
    ratNASIPv6Address = 95,
    ratFramedInterfaceId,
    ratFramedIPv6Prefix,
    ratLoginIPv6Host,
    ratFramedIPv6Route,
    ratFramedIPv6Pool,
    
    /* Extended attribute types used to pass additional information. */
    ratCode = 262,             /* Request type code. */
    ratIdentifier = 263,       /* Request identifier. */
    ratAuthenticator = 264,    /* Request authenticator. */
    ratSrcIPAddress = 265,     /* Source IP address. */
    ratSrcPort = 266,          /* Source IP port. */
    ratProvider = 267,         /* Authentication provider. */
    ratStrippedUserName = 268, /* User-Name with realm stripped. */
    ratFQUserName = 269,       /* Fully-Qualified-User-Name. */
    ratPolicyName = 270,       /* Remote Access Policy name. */
    ratUniqueId = 271,         /* Unique ID identifying the request. */
    ratExtensionState = 272,   /* Used to pass state between extensions. */
    ratEAPTLV = 273,           /* EAP-TLV packet */
    ratRejectReasonCode = 274, /* The reason code for radius reject */
    ratCRPPolicyName = 275,    /* CRP Policy Name */
    ratProviderName = 276,     /* Remote RADIUS server group name */
    ratClearTextPassword = 277,   /* The user password in clear text */
    ratSrcIPv6Address = 278,   /* Source IPv6 address */
    ratCertificateThumbprint = 279, /* The thumbprint of client auth certificate */
} RADIUS_ATTRIBUTE_TYPE;

/*
 *  Enumerates the different RADIUS packet codes. Used for the ratCode extended
 *  attribute.
 */
typedef enum _RADIUS_CODE {
    rcUnknown = 0,
    rcAccessRequest = 1,
    rcAccessAccept = 2,
    rcAccessReject = 3,
    rcAccountingRequest = 4,
    rcAccountingResponse = 5,
    rcAccessChallenge = 11,
    rcDiscard = 256
} RADIUS_CODE;

/*
 *  Enumerates the different authentication providers used for processing a
 *  request. Used for the ratProvider extended attribute.
 */
typedef enum _RADIUS_AUTHENTICATION_PROVIDER {
    rapUnknown,
    rapUsersFile,
    rapProxy,
    rapWindowsNT,
    rapMCIS,
    rapODBC,
    rapNone
} RADIUS_AUTHENTICATION_PROVIDER;

/*
 *  Enumerates a subset of reason codes for radius reject.
 *  Used for the ratReasonCode extended attribute.
 */
typedef enum _RADIUS_REJECT_REASON_CODE {
    rrrcUndefined = 0,
    rrrcAccountUnknown = 1,
    rrrcAccountDisabled = 2,
    rrrcAccountExpired = 3,
    rrrcAuthenticationFailure = 4,
} RADIUS_REJECT_REASON_CODE;

/*
 *  Enumerates the different RADIUS data types. A type of 'rdtUnknown' means
 *  the attribute was not recognized by the dictionary.
 */
typedef enum _RADIUS_DATA_TYPE {
   rdtUnknown,
   rdtString,
   rdtAddress,
   rdtInteger,
   rdtTime,
   rdtIpv6Address
} RADIUS_DATA_TYPE;

/*
 *  Struct representing a RADIUS or extended attribute.
 */
typedef struct _RADIUS_ATTRIBUTE {
    DWORD dwAttrType;            /* Attribute type */
    RADIUS_DATA_TYPE fDataType;  /* RADIUS_DATA_TYPE of the value */
    DWORD cbDataLength;          /* Length of the value (in bytes) */
    union {
        DWORD dwValue;           /* For rdtAddress, rdtInteger, and rdtTime */
        CONST BYTE* lpValue;     /* For rdtUnknown, rdtIpv6Address and rdtString */
    };
} RADIUS_ATTRIBUTE, *PRADIUS_ATTRIBUTE;

/*
 *  Struct representing the layout of a RADIUS Vendor-Specific attribute. This
 *  is useful when interpreting the RADIUS_ATTRIBUTE lpValue field when
 *  dwAttrType is ratVendorSpecific.
 */
typedef struct _RADIUS_VSA_FORMAT {
   BYTE VendorId[4];
   BYTE VendorType;
   BYTE VendorLength;
   BYTE AttributeSpecific[1];
} RADIUS_VSA_FORMAT;

/*
 *  Enumerates the different actions an extension DLL can generate in
 *  response to an Access-Request.
 */
typedef enum _RADIUS_ACTION {
   raContinue,
   raReject,
   raAccept
} RADIUS_ACTION, *PRADIUS_ACTION;


/*
 * Routines exported by a RADIUS extension DLL.
 */

/*
 * RadiusExtensionInit is optional. If it exists, it will be invoked prior to
 * the service coming on-line. A return value other than NO_ERROR prevents the
 * service from initializing.
 */
#define RADIUS_EXTENSION_INIT "RadiusExtensionInit"
typedef DWORD (WINAPI *PRADIUS_EXTENSION_INIT)( VOID );

/*
 * RadiusExtensionTerm is optional. If it exists, it will be invoked prior to
 * unloading the DLL to give the extension a chance to clean-up.
 */
#define RADIUS_EXTENSION_TERM "RadiusExtensionTerm"
typedef VOID (WINAPI *PRADIUS_EXTENSION_TERM)( VOID );

/*
 * RadiusExtensionProcess is mandatory for NT4. For Windows 2000, an
 * extension may export RadiusExtensionProcessEx (q.v.) instead.
 *
 * Parameters:
 *   pAttrs      Array of attributes from the request. It is terminated by an
 *               attribute with dwAttrType set to ratMinimum. These attributes
 *               should be treated as read-only and must not be referenced
 *               after the function returns.
 *   pfAction    For Access-Requests, this parameter will be non-NULL with
 *               *pfAction == raContinue. The extension DLL can set *pfAction
 *               to abort further processing and force an Access-Accept or
 *               Access-Reject.  For all other request types, this parameter
 *               will be NULL.
 *
 * Return Value:
 *     A return value other than NO_ERROR causes the request to be discarded.
 */
#define RADIUS_EXTENSION_PROCESS "RadiusExtensionProcess"
typedef DWORD (WINAPI *PRADIUS_EXTENSION_PROCESS)(
    IN const RADIUS_ATTRIBUTE *pAttrs,
    OUT OPTIONAL PRADIUS_ACTION pfAction
    );

/*
 * RadiusExtensionProcessEx is only supported on Windows 2000. If it exits,
 * RadiusExtensionProcess is ignored.
 *
 * Parameters:
 *   pInAttrs    Array of attributes from the request. It is terminated by an
 *               attribute with dwAttrType set to ratMinimum. These attributes
 *               should be treated as read-only and must not be referenced
 *               after the function returns.
 *   pOutAttrs   Array of attributes to add to the response. It is terminated
 *               by an attribute with dwAttrType set to ratMinimum.
 *               *pOutAttrs may be set to NULL if no attributes are returned.
 *   pfAction    For Access-Requests, this parameter will be non-NULL with
 *               *pfAction == raContinue. The extension DLL can set *pfAction
 *               to abort further processing and force an Access-Accept or
 *               Access-Reject.  For all other request types, this parameter
 *               will be NULL.
 *
 * Return Value:
 *     A return value other than NO_ERROR causes the request to be discarded.
 */
#define RADIUS_EXTENSION_PROCESS_EX "RadiusExtensionProcessEx"
typedef DWORD (WINAPI *PRADIUS_EXTENSION_PROCESS_EX)(
    IN const RADIUS_ATTRIBUTE *pInAttrs,
    OUT PRADIUS_ATTRIBUTE *pOutAttrs,
    OUT OPTIONAL PRADIUS_ACTION pfAction
    );

/*
 * RadiusExtensionFreeAttributes must be defined if RadiusExtensionProcessEx
 * is defined. It is used to free the attributes returned by
 * RadiusExtensionProcessEx
 *
 * Parameters:
 *   pAttrs     Array of attributes to be freed.
 */
#define RADIUS_EXTENSION_FREE_ATTRIBUTES "RadiusExtensionFreeAttributes"
typedef VOID (WINAPI *PRADIUS_EXTENSION_FREE_ATTRIBUTES)(
    IN PRADIUS_ATTRIBUTE pAttrs
    );

/*
 *  Defines used for installation of an extension DLL.
 *  The following registry values are used for loading extensions:
 *
 *      HKLM\System\CurrentControlSet\Services\AuthSrv\Parameters
 *          ExtensionDLLs      (REG_MULTI_SZ)  <list of DLL paths>
 *          AuthorizationDLLs  (REG_MULTI_SZ)  <list of DLL paths>
 *
 *  ExtensionDLLs are invoked before any of the built-in authentication
 *  providers. They receive all the attributes from the request plus all
 *  the extended attribute types.
 *
 *  AuthorizationDLLs are invoked after the built-in authentication and
 *  authorization providers. They receive all the attributes from the
 *  response plus all the extended attributes types. AuthorizationDLLs may
 *  not return an action of raAccept.
 */

#define AUTHSRV_PARAMETERS_KEY_W \
    L"System\\CurrentControlSet\\Services\\AuthSrv\\Parameters"

#define AUTHSRV_EXTENSIONS_VALUE_W \
    L"ExtensionDLLs"

#define AUTHSRV_AUTHORIZATION_VALUE_W \
    L"AuthorizationDLLs"

#define AUTHSRV_ENFORCE_NP_FOR_PAP_CHALLENGE_RESPONSE_VALUE_W \
    L"EnforceNetworkPolicyForPAPBasedChallengeResponse"
	
#if _WIN32_WINNT >= 0x0501

/* Version of this spec. */
#define RADIUS_EXTENSION_VERSION (1)

/*
 *  Enumerates the different points during request processing where an
 *  extension can be invoked.
 */
typedef enum _RADIUS_EXTENSION_POINT {
   repAuthentication,     /* ExtensionDLLs */
   repAuthorization       /* AuthorizationDLLs */
} RADIUS_EXTENSION_POINT;

/*
 * Struct representing an array of RADIUS_ATTRIBUTE structs. All the functions
 * for adding attributes to a request copy the supplied memory, so there is no
 * need for the extension to export RadiusExtensionFreeAttributes.  The
 * extension must not modify this struct. All changes must be made by using the
 * supplied callback functions.
 */
typedef struct _RADIUS_ATTRIBUTE_ARRAY {

   /* Size of this structure in bytes. */
   DWORD cbSize;

   /* Adds a new attribute to the end of the array. */
   DWORD (WINAPI *Add)(
       IN struct _RADIUS_ATTRIBUTE_ARRAY *This,
       IN const RADIUS_ATTRIBUTE *pAttr
       );

   /*
    * Returns a const pointer to the specified attribute within the array or
    * NULL if the index is out of range.
    */
   const RADIUS_ATTRIBUTE * (WINAPI *AttributeAt)(
       IN const struct _RADIUS_ATTRIBUTE_ARRAY *This,
       IN DWORD dwIndex
       );

   /*
    * Returns the size of the array. Since indexes are zero-based, the size is
    * 1 greater than the largest index.
    */
   DWORD (WINAPI *GetSize)(
                     IN const struct _RADIUS_ATTRIBUTE_ARRAY *This
                     );

   /*
    * Inserts a new attribute at a specified index in the array. In the
    * process, it shifts up (by incrementing the index) the existing attribute
    * at this index, and it shifts up all the attributes above it. Returns
    * ERROR_INVALID_PARAMETER if the index is out of range.
    */
   DWORD (WINAPI *InsertAt)(
       IN struct _RADIUS_ATTRIBUTE_ARRAY *This,
       IN DWORD dwIndex,
       IN const RADIUS_ATTRIBUTE *pAttr
       );

   /*
    * Removes the attribute at the specified index in the array. In the
    * process, it shifts down all the attributes above the removed attribute.
    * Returns ERROR_ACCESS_DENIED if the specified attribute is read-only.
    * Returns ERROR_INVALID_PARAMETER if the index is out of range.
    */
   DWORD (WINAPI *RemoveAt)(
       IN struct _RADIUS_ATTRIBUTE_ARRAY *This,
       IN DWORD dwIndex
       );

   /*
    * Sets the array element at the specified index, replacing the existing
    * attribute.  Returns ERROR_INVALID_PARAMETER if the index is out of range.
    */
   DWORD (WINAPI *SetAt)(
       IN struct _RADIUS_ATTRIBUTE_ARRAY *This,
       IN DWORD dwIndex,
       IN const RADIUS_ATTRIBUTE *pAttr
       );

} RADIUS_ATTRIBUTE_ARRAY, *PRADIUS_ATTRIBUTE_ARRAY;

/*
 * Struct used to exchange information with the extension during request
 * processing. The extension must not modify this struct. All changes must be
 * made by using the supplied callback functions.
 */
typedef struct _RADIUS_EXTENSION_CONTROL_BLOCK  {

   /* Size of this structure. */
   DWORD cbSize;

   /* Version info of this specification. */
   DWORD dwVersion;

   /* Point during request processing where the extension is being invoked. */
   RADIUS_EXTENSION_POINT repPoint;

   /* Type of RADIUS request being processed. */
   RADIUS_CODE rcRequestType;

   /*
    * Final disposition of the request. This field must not be modified
    * directly; use the SetResponseType callback function instead. At the
    * repAuthentication point, this may be set to rcUnknown to indicate that no
    * decision has been made yet.
    */
   RADIUS_CODE rcResponseType;

   /*
    * Returns the attributes received in the RADIUS request and any internal
    * attributes describing the request state. The extenstion can modify the
    * request attributes. For example, when IAS is acting as a RADIUS proxy, an
    * extension could filter which attributes are forwarded to a remote RADIUS
    * server.
    */
   PRADIUS_ATTRIBUTE_ARRAY (WINAPI *GetRequest)(
       IN struct _RADIUS_EXTENSION_CONTROL_BLOCK *This
       );

   /*
    * Returns the attributes that will be sent in the response if the final
    * outcome of request processing matches the specified response type.
    * Returns NULL if rcResponseType is invalid. Note that an extension may
    * retrieve and modify the attributes for any valid response type regardless
    * of the request's current disposition. For example, an extension can set
    * the response type to rcAccessAccept, but still add attributes to the
    * Access-Reject in case the response type is overridden during further
    * processing.
    */
   PRADIUS_ATTRIBUTE_ARRAY (WINAPI *GetResponse)(
       IN struct _RADIUS_EXTENSION_CONTROL_BLOCK *This,
       IN RADIUS_CODE rcResponseType
       );

   /*
    * Sets the final disposition of the request.
    * Returns ERROR_INVALID_PARAMETER if the specified response type is invalid
    * for the request type.
    */
   DWORD (WINAPI *SetResponseType)(
       IN struct _RADIUS_EXTENSION_CONTROL_BLOCK *This,
       IN RADIUS_CODE rcResponseType
       );

} RADIUS_EXTENSION_CONTROL_BLOCK, *PRADIUS_EXTENSION_CONTROL_BLOCK;

/*
 * If RadiusExtensionProcess2 exists, RadiusExtensionProcess and
 * RadiusExtensionProcessEx are ignored.
 *
 * Parameters:
 *   pECB      Info exchanged with the extension.
 *
 * Return Value:
 *     A return value other than NO_ERROR causes the request to be discarded.
 */
#define RADIUS_EXTENSION_PROCESS2 "RadiusExtensionProcess2"
typedef DWORD (WINAPI *PRADIUS_EXTENSION_PROCESS_2)(
   IN OUT PRADIUS_EXTENSION_CONTROL_BLOCK pECB
   );

#endif // _WIN32_WINNT

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _AUTHIF_H_ */
