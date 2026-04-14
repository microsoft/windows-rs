////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation.
//
// SYNOPSIS
//
//    IDL source for interaction with EAPHost supplicants.
//
////////////////////////////////////////////////////////////

#ifndef EAPTYPES_H
#define EAPTYPES_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <guiddef.h>  // Used by EAP_ERROR structure.


// Properties that EAP Methods support
#define eapPropCipherSuiteNegotiation      0x00000001
#define eapPropMutualAuth                  0x00000002
#define eapPropIntegrity                   0x00000004
#define eapPropReplayProtection            0x00000008
#define eapPropConfidentiality             0x00000010
#define eapPropKeyDerivation               0x00000020
#define eapPropKeyStrength64               0x00000040
#define eapPropKeyStrength128              0x00000080
#define eapPropKeyStrength256              0x00000100
#define eapPropKeyStrength512              0x00000200
#define eapPropKeyStrength1024             0x00000400
#define eapPropDictionaryAttackResistance  0x00000800
#define eapPropFastReconnect               0x00001000
#define eapPropCryptoBinding               0x00002000
#define eapPropSessionIndependence         0x00004000
#define eapPropFragmentation               0x00008000
#define eapPropChannelBinding              0x00010000
#define eapPropNap                         0x00020000
#define eapPropStandalone                  0x00040000
#define eapPropMppeEncryption              0x00080000
#define eapPropTunnelMethod                0x00100000
#define eapPropSupportsConfig              0x00200000
#define eapPropCertifiedMethod             0x00400000
#define eapPropHiddenMethod                0x00800000
#define eapPropMachineAuth                 0x01000000
#define eapPropUserAuth                    0x02000000
#define eapPropIdentityPrivacy             0x04000000
#define eapPropMethodChaining              0x08000000
#define eapPropSharedStateEquivalence      0x10000000
#define eapPropReserved                    0x80000000

// Registry NameValue that stores Properties supported by EAP Method
#define EAP_VALUENAME_PROPERTIES L"Properties"

typedef DWORD EAP_SESSIONID;

typedef struct _EAP_TYPE
{
   BYTE type;
   DWORD dwVendorId;
   DWORD dwVendorType;
} EAP_TYPE;


// definition of EAP_METHOD_TYPE used for describing an EAP method
typedef struct _EAP_METHOD_TYPE
{
   EAP_TYPE eapType;
   DWORD dwAuthorId;
} EAP_METHOD_TYPE;


// Information pertaining to each EAP method is populated in this structure format
typedef struct _EAP_METHOD_INFO
{
   EAP_METHOD_TYPE eaptype;
   LPWSTR pwszAuthorName;
   LPWSTR pwszFriendlyName;
   DWORD eapProperties;
   struct _EAP_METHOD_INFO* pInnerMethodInfo;
} EAP_METHOD_INFO;


// EAPHost populates this array with the information for all the eap-methods installed
// on machine and passes it to a supplicant
struct _EAP_METHOD_INFO_ARRAY_EX;


typedef struct _EAP_METHOD_INFO_EX
{
   EAP_METHOD_TYPE eaptype;
   LPWSTR pwszAuthorName;
   LPWSTR pwszFriendlyName;
   DWORD eapProperties;
   struct _EAP_METHOD_INFO_ARRAY_EX* pInnerMethodInfoArray;
} EAP_METHOD_INFO_EX;


typedef struct _EAP_METHOD_INFO_ARRAY
{
   DWORD dwNumberOfMethods;
   EAP_METHOD_INFO* pEapMethods;
} EAP_METHOD_INFO_ARRAY;

// EAPHost populates this error structs with appropriate information and passes it to
// supplicant, which can use this information for error reporting and troubleshooting
typedef struct _EAP_METHOD_INFO_ARRAY_EX
{
   DWORD dwNumberOfMethods;
   EAP_METHOD_INFO_EX* pEapMethods;
} EAP_METHOD_INFO_ARRAY_EX;

typedef struct _EAP_ERROR
{
   DWORD dwWinError;
   EAP_METHOD_TYPE type;
   DWORD dwReasonCode;

   GUID   rootCauseGuid;
   GUID   repairGuid;
   GUID   helpLinkGuid;

   LPWSTR pRootCauseString;
   LPWSTR pRepairString;
} EAP_ERROR;


EXTERN_C const CLSID GUID_EapHost_Default;

EXTERN_C const CLSID GUID_EapHost_Cause_MethodDLLNotFound;
EXTERN_C const CLSID GUID_EapHost_Cause_EapNegotiationFailed;    //*!*
EXTERN_C const CLSID GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset;
EXTERN_C const CLSID GUID_EapHost_Cause_XmlMalformed;
EXTERN_C const CLSID GUID_EapHost_Cause_MethodDoesNotSupportOperation;
EXTERN_C const CLSID GUID_EapHost_Cause_EapQecInaccessible;      //*!*

EXTERN_C const CLSID GUID_EapHost_Cause_Generic_AuthFailure;
EXTERN_C const CLSID GUID_EapHost_Cause_IdentityUnknown;
EXTERN_C const CLSID GUID_EapHost_Cause_SimNotValid;
EXTERN_C const CLSID GUID_EapHost_Cause_CertStoreInaccessible;

EXTERN_C const CLSID GUID_EapHost_Cause_User_CertExpired;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CertInvalid;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CertNotFound;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CertOtherError;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CertRejected;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CertRevoked;
EXTERN_C const CLSID GUID_EapHost_Cause_User_Account_OtherProblem;
EXTERN_C const CLSID GUID_EapHost_Cause_User_CredsRejected;

EXTERN_C const CLSID GUID_EapHost_Cause_Server_CertExpired;
EXTERN_C const CLSID GUID_EapHost_Cause_Server_CertInvalid;
EXTERN_C const CLSID GUID_EapHost_Cause_Server_CertNotFound;
EXTERN_C const CLSID GUID_EapHost_Cause_Server_CertOtherError;
EXTERN_C const CLSID GUID_EapHost_Cause_Server_CertRevoked;

EXTERN_C const CLSID GUID_EapHost_Cause_User_Root_CertExpired;
EXTERN_C const CLSID GUID_EapHost_Cause_User_Root_CertInvalid;
EXTERN_C const CLSID GUID_EapHost_Cause_User_Root_CertNotFound;

EXTERN_C const CLSID GUID_EapHost_Cause_Server_Root_CertNotFound;
EXTERN_C const CLSID GUID_EapHost_Cause_Server_Root_CertNameRequired;
EXTERN_C const CLSID GUID_EapHost_Cause_No_SmartCardReader_Found;

EXTERN_C const CLSID GUID_EapHost_Repair_ContactSysadmin;
EXTERN_C const CLSID GUID_EapHost_Repair_Retry_Authentication;

EXTERN_C const CLSID GUID_EapHost_Repair_User_AuthFailure;
EXTERN_C const CLSID GUID_EapHost_Repair_User_GetNewCert;
EXTERN_C const CLSID GUID_EapHost_Repair_User_SelectValidCert;

EXTERN_C const CLSID GUID_EapHost_Repair_Server_ClientSelectServerCert;

EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_AuthFailure;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_IdentityUnknown;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_NegotiationFailed;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_MethodNotFound;
EXTERN_C const CLSID GUID_EapHost_Repair_RestartNap;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_InvalidUserCert;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_RootCertInvalid;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_RootCertNotFound;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_RootExpired;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_CertNameAbsent;
EXTERN_C const CLSID GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader;
EXTERN_C const CLSID GUID_EapHost_Repair_Method_Not_Support_Sso;
EXTERN_C const CLSID GUID_EapHost_Repair_No_ValidSim_Found;
      
EXTERN_C const CLSID GUID_EapHost_Help_ObtainingCerts;
EXTERN_C const CLSID GUID_EapHost_Help_Troubleshooting;
EXTERN_C const CLSID GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso;

//--------------------------


typedef
#ifdef __midl
   [v1_enum]
#endif

// Attributes that EAP Methods support
enum _EAP_ATTRIBUTE_TYPE
{
    eatMinimum = 0,                // Undefined
    eatUserName,                   // Value field is a Pointer
    eatUserPassword,               // Value field is a Pointer
    eatMD5CHAPPassword,            // Value field is a Pointer
    eatNASIPAddress,               // Value field is a 32 bit integral value
    eatNASPort,                    // Value field is a 32 bit integral value
    eatServiceType,                // Value field is a 32 bit integral value
    eatFramedProtocol,             // Value field is a 32 bit integral value
    eatFramedIPAddress,            // Value field is a 32 bit integral value
    eatFramedIPNetmask,            // Value field is a 32 bit integral value
    eatFramedRouting = 10,         // Value field is a 32 bit integral value
    eatFilterId,                   // Value field is a Pointer
    eatFramedMTU,                  // Value field is a 32 bit integral value
    eatFramedCompression,          // Value field is a 32 bit integral value
    eatLoginIPHost,                // Value field is a 32 bit integral value
    eatLoginService,               // Value field is a 32 bit integral value
    eatLoginTCPPort,               // Value field is a 32 bit integral value
    eatUnassigned17,               // Undefined
    eatReplyMessage,               // Value field is a Pointer
    eatCallbackNumber,             // Value field is a Pointer
    eatCallbackId =20,             // Value field is a Pointer
    eatUnassigned21,               // Undefined
    eatFramedRoute,                // Value field is a Pointer
    eatFramedIPXNetwork,           // Value field is a 32 bit integral value
    eatState,                      // Value field is a Pointer
    eatClass,                      // Value field is a Pointer
    eatVendorSpecific,             // Value field is a Pointer
    eatSessionTimeout,             // Value field is a 32 bit integral value
    eatIdleTimeout,                // Value field is a 32 bit integral value
    eatTerminationAction,          // Value field is a 32 bit integral value
    eatCalledStationId = 30,       // Value field is a Pointer
    eatCallingStationId,           // Value field is a Pointer
    eatNASIdentifier,              // Value field is a Pointer
    eatProxyState,                 // Value field is a Pointer
    eatLoginLATService,            // Value field is a Pointer
    eatLoginLATNode,               // Value field is a Pointer
    eatLoginLATGroup,              // Value field is a Pointer
    eatFramedAppleTalkLink,        // Value field is a 32 bit integral value
    eatFramedAppleTalkNetwork,     // Value field is a 32 bit integral value
    eatFramedAppleTalkZone,        // Value field is a Pointer
    eatAcctStatusType = 40,        // Value field is a 32 bit integral value
    eatAcctDelayTime,              // Value field is a 32 bit integral value
    eatAcctInputOctets,            // Value field is a 32 bit integral value
    eatAcctOutputOctets,           // Value field is a 32 bit integral value
    eatAcctSessionId,              // Value field is a Pointer
    eatAcctAuthentic,              // Value field is a 32 bit integral value
    eatAcctSessionTime,            // Value field is a 32 bit integral value
    eatAcctInputPackets,           // Value field is a 32 bit integral value
    eatAcctOutputPackets,          // Value field is a 32 bit integral value
    eatAcctTerminateCause,         // Value field is a 32 bit integral value
    eatAcctMultiSessionId = 50,    // Value field is a Pointer
    eatAcctLinkCount,              // Value field is a 32 bit integral value
    eatAcctEventTimeStamp = 55,    // Value field is a 32 bit integral value
    eatMD5CHAPChallenge = 60,      // Value field is a Pointer
    eatNASPortType,                // Value field is a 32 bit integral value
    eatPortLimit,                  // Value field is a 32 bit integral value
    eatLoginLATPort,               // Value field is a Pointer
    eatTunnelType,                 // Value field is a 32 bit integral value
    eatTunnelMediumType,           // Value field is a 32 bit integral value
    eatTunnelClientEndpoint,       // Value field is a Pointer
    eatTunnelServerEndpoint,       // Value field is a Pointer
    eatARAPPassword = 70,          // Value field is a Pointer
    eatARAPFeatures,               // Value field is a Pointer
    eatARAPZoneAccess,             // Value field is a 32 bit integral value
    eatARAPSecurity,               // Value field is a 32 bit integral value
    eatARAPSecurityData,           // Value field is a Pointer
    eatPasswordRetry,              // Value field is a 32 bit integral value
    eatPrompt,                     // Value field is a 32 bit integral value
    eatConnectInfo,                // Value field is a Pointer
    eatConfigurationToken,         // Value field is a Pointer
    eatEAPMessage,                 // Value field is a Pointer
    eatSignature = 80,             // Value field is a Pointer
    eatARAPChallengeResponse = 84, // Value field is a Pointer
    eatAcctInterimInterval = 85,   // Value field is a 32 bit integral value
    eatNASIPv6Address = 95,  // Value field is a Pointer
    eatFramedInterfaceId, // Value field is a Pointer
    eatFramedIPv6Prefix, // Value field is a Pointer
    eatLoginIPv6Host, // Value field is a Pointer
    eatFramedIPv6Route, // Value field is a Pointer
    eatFramedIPv6Pool, // Value field is a Pointer
    eatARAPGuestLogon = 8096,      // Value field is a 32 bit integral value
    eatCertificateOID,             // Value field is a Pointer
    eatEAPConfiguration,           // Value field is a Pointer
    eatPEAPEmbeddedEAPTypeId,      // Value field is a 32 bit integral value
    eatPEAPFastRoamedSession = 8100,   // Value field is a 32 bit integral value
    eatFastRoamedSession = 8100,       // Value field is a 32 bit integral value
    eatEAPTLV = 8102,              // Value field is a Pointer
    eatCredentialsChanged,        // Value field is a Integer with boolean
                                   // semantics
    eatInnerEapMethodType,         // Value field is a pointer
                                   // EapMethodType of the inner method in
                                   // case of Tunnelled method
    eatClearTextPassword = 8107,   // Value field is a pointer
                                   // Clear text password that can be used
                                   // by certain password based eap methods
                                   // to authenticate a user.
    eatQuarantineSoH     = 8150, // Value field is a pointer
    eatCertificateThumbprint = 8250, // Value field is a pointer
    eatPeerId                 = 9000,  // Value field is a pointer
    eatServerId,                          // Value field is a pointer
    eatMethodId,                          // Value field is a pointer
    eatEMSK,                               // Value field is a pointer
    eatSessionId,                               // Value field is a pointer
    eatReserved = 0xFFFFFFFF       // Undefined
} EAP_ATTRIBUTE_TYPE, EapAttributeType;


// format for carrying EAP attribute TLV (type, length and value)
typedef struct _EAP_ATTRIBUTE
{
        EAP_ATTRIBUTE_TYPE eaType;
        DWORD dwLength;
#ifdef __midl
        [size_is(dwLength)] BYTE* pValue;
#else
    BYTE *pValue;
#endif
} EAP_ATTRIBUTE, EapAttribute;


// List of EAP-attributes
typedef struct _EAP_ATTRIBUTES
{
        DWORD dwNumberOfAttributes;
#ifdef __midl
        [size_is(dwNumberOfAttributes)] EAP_ATTRIBUTE* pAttribs;
#else
    EAP_ATTRIBUTE *pAttribs;
#endif
} EAP_ATTRIBUTES, EapAttributes;


// No UI should be displayed
#define EAP_FLAG_Reserved1              0x00000001
#define EAP_FLAG_NON_INTERACTIVE        0x00000002

// The user data was obtained from Winlogon
#define EAP_FLAG_LOGON                  0x00000004

// Show the Credentails UI before authenticating even if cached creds are present
#define EAP_FLAG_PREVIEW                0x00000008

#define EAP_FLAG_Reserved2              0x00000010

// Authentication is for machine, i.e. not for user. Not setting this flag means
// it is user authentication.
#define EAP_FLAG_MACHINE_AUTH           0x00000020

// Request to provide guest access
#define EAP_FLAG_GUEST_ACCESS           0x00000040

#define EAP_FLAG_Reserved3              0x00000080

#define EAP_FLAG_Reserved4              0x00000100

// Indicates this is the first call after m/c resumed from hibernation.
#define EAP_FLAG_RESUME_FROM_HIBERNATE  0x00000200

#define EAP_FLAG_Reserved5              0x00000400

#define EAP_FLAG_Reserved6              0x00000800

// Should have this or make it reserved?
#define EAP_FLAG_FULL_AUTH              0x00001000

///   New EapHost Flags

// Prefer Credentails passed in BeginSession instead of other form of
// credentials, even if config says use certain way of cred retrieval. If this fails, fallback to
// Method specific credentail retrieval
#define EAP_FLAG_PREFER_ALT_CREDENTIALS     0x00002000

#define EAP_FLAG_Reserved7                  0x00004000

// To inform the cause of reauth is a NAP callback
#define EAP_PEER_FLAG_HEALTH_STATE_CHANGE   0x00008000

// Continue authentication with information available. If can not proceed, fail
// the authentication
#define EAP_FLAG_SUPRESS_UI                 0x00010000

// Pre logon scenario like PLAP
#define EAP_FLAG_PRE_LOGON                  0x00020000

// User Authentication. For legacy Methods not setting EAP_FLAG_MACHINE_AUTH
// also means user authentication.
#define EAP_FLAG_USER_AUTH                  0x00040000

// Config can be viewed but not updated
#define EAP_FLAG_CONFG_READONLY             0x00080000

#define EAP_FLAG_Reserved8                  0x00100000

#define EAP_FLAG_Reserved9                  0x00400000

#define EAP_FLAG_VPN                        0x00800000

#define EAP_FLAG_ONLY_EAP_TLS               0x01000000

#define EAP_FLAG_SERVER_VALIDATION_REQUIRED 0x02000000

// the various states of the input fields
#define EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT                0X00000000
#define EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE        0X00000001

// This specifies that supplicant should cache this field data
#define EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST            0X00000002


// default property value for entries field to be shown in UI
#define EAP_UI_INPUT_FIELD_PROPS_DEFAULT          EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT
// To qualify certain entry data on UI is non-displayable (e.g. password)
#define EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE  EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE
// This specifies that supplicant should cache this field data
#define EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST            0X00000002
// This specifies that following field is read only (non-editable)
#define EAP_UI_INPUT_FIELD_PROPS_READ_ONLY              0X00000004


// the types of fields that eap can request to show
typedef enum _EAP_CONFIG_INPUT_FIELD_TYPE {
    EapConfigInputUsername,
    EapConfigInputPassword,
    EapConfigInputNetworkUsername,
    EapConfigInputNetworkPassword,
    EapConfigInputPin,
    EapConfigInputPSK,
    EapConfigInputEdit,
    EapConfigSmartCardUsername,
    EapConfigSmartCardError
} EAP_CONFIG_INPUT_FIELD_TYPE, *PEAP_CONFIG_INPUT_FIELD_TYPE;

#define EAP_CREDENTIAL_VERSION                     1
#define EAP_INTERACTIVE_UI_DATA_VERSION            1
#define EAPHOST_PEER_API_VERSION                   1
#define EAPHOST_METHOD_API_VERSION                 1
#define MAX_EAP_CONFIG_INPUT_FIELD_LENGTH          256
#define MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH    1024


// structure for carrying EAP input field
typedef struct _EAP_CONFIG_INPUT_FIELD_DATA {
    DWORD dwSize;
    EAP_CONFIG_INPUT_FIELD_TYPE Type;
    DWORD dwFlagProps;
    LPWSTR pwszLabel;
    LPWSTR pwszData;
    DWORD dwMinDataLength;
    DWORD dwMaxDataLength;
} EAP_CONFIG_INPUT_FIELD_DATA, *PEAP_CONFIG_INPUT_FIELD_DATA;


// List of EAP input fields
typedef struct _EAP_CONFIG_INPUT_FIELD_ARRAY {
    DWORD dwVersion; // used for versioning
    DWORD dwNumberOfFields;
#ifdef __midl
    [size_is(dwNumberOfFields)] EAP_CONFIG_INPUT_FIELD_DATA* pFields;
#else
    EAP_CONFIG_INPUT_FIELD_DATA* pFields;
#endif
} EAP_CONFIG_INPUT_FIELD_ARRAY, *PEAP_CONFIG_INPUT_FIELD_ARRAY;


// provides type of data, EAP_INTERACTIVE_UI_DATA can carry
typedef enum _EAP_INTERACTIVE_UI_DATA_TYPE {
    EapCredReq,             // Credential request during retry
    EapCredResp,            // Credential response during retry
    EapCredExpiryReq,       // Credential request during cred expiry
    EapCredExpiryResp,      // Credential response during cred expiry
    EapCredLogonReq,        // Initial credential request during auth
    EapCredLogonResp,       // Initial credential response during auth
} EAP_INTERACTIVE_UI_DATA_TYPE;


// following determine type of data blob that can be pointed by
// pbUiData member of EAP_INTERACTIVE_UI_DATA
typedef EAP_CONFIG_INPUT_FIELD_ARRAY EAP_CRED_REQ;
typedef EAP_CONFIG_INPUT_FIELD_ARRAY EAP_CRED_RESP;
typedef EAP_CONFIG_INPUT_FIELD_ARRAY EAP_CRED_LOGON_REQ;
typedef EAP_CONFIG_INPUT_FIELD_ARRAY EAP_CRED_LOGON_RESP;

// pNewCreds can be NULL in case of EAP_CRED_EXPIRY_REQ
// in case of REQ, pNewCreds can be empty (will not be looked at)
typedef struct _EAP_CRED_EXPIRY_REQ {
    EAP_CONFIG_INPUT_FIELD_ARRAY curCreds;
    EAP_CONFIG_INPUT_FIELD_ARRAY newCreds;
} EAP_CRED_EXPIRY_REQ, EAP_CRED_EXPIRY_RESP;

#ifdef __midl
    typedef [switch_type(EAP_INTERACTIVE_UI_DATA_TYPE)]
    union {
       [case(EapCredReq, EapCredResp)]      EAP_CRED_REQ*  credData;
       [case(EapCredExpiryReq,EapCredExpiryResp)] EAP_CRED_EXPIRY_REQ* credExpiryData;
       [case(EapCredLogonReq,EapCredLogonResp)] EAP_CRED_LOGON_REQ* credLogonData;
       [default]      ;
    } EAP_UI_DATA_FORMAT;
#else
    typedef union {
       EAP_CRED_REQ*  credData;
       EAP_CRED_EXPIRY_REQ* credExpiryData;
       EAP_CRED_LOGON_REQ* credLogonData;
    } EAP_UI_DATA_FORMAT;
#endif


// Data carried from EAP-method to supplicant for interactive UI to raised
typedef struct _EAP_INTERACTIVE_UI_DATA {
    //specifies version of data structure. It must be 0.
    DWORD dwVersion;
    //specifies size of the entire structure.
    DWORD dwSize;
    // this determines what kind of interactive data is passed.
    EAP_INTERACTIVE_UI_DATA_TYPE dwDataType;
    // specifies size of the data pointed by pbUiByte.
    DWORD cbUiData;
    // pointer to a data blob of a type determined by dwDataType above.
    // This can be of type EAP_CRED_REQ/EAP_CRED_RESP or
    // EAP_CRED_EXPIRY_REQ/EAP_CRED_EXPIRY_RESP or
    // EAP_CRED_LOGON_REQ/EAP_CRED_LOGON_RESP depending on uiDataType above
#ifdef __midl
    [switch_is (dwDataType)] EAP_UI_DATA_FORMAT pbUiData;
#else
    EAP_UI_DATA_FORMAT pbUiData;
#endif
} EAP_INTERACTIVE_UI_DATA ;

typedef
#ifdef __midl
    [v1_enum]
#endif
// Property types that EAP methods support
enum _EAP_METHOD_PROPERTY_TYPE
{
    emptPropCipherSuiteNegotiation = 0,     // value field is boolean
    emptPropMutualAuth,                     // value field is boolean
    emptPropIntegrity,                      // value field is boolean
    emptPropReplayProtection,               // value field is boolean
    emptPropConfidentiality,                // value field is boolean
    emptPropKeyDerivation,                  // value field is boolean
    emptPropKeyStrength64,                  // value field is boolean
    emptPropKeyStrength128,                 // value field is boolean
    emptPropKeyStrength256,                 // value field is boolean
    emptPropKeyStrength512,                 // value field is boolean
    emptPropKeyStrength1024,                // value field is boolean
    emptPropDictionaryAttackResistance,     // value field is boolean
    emptPropFastReconnect,                  // value field is boolean
    emptPropCryptoBinding,                  // value field is boolean
    emptPropSessionIndependence,            // value field is boolean
    emptPropFragmentation,                  // value field is boolean
    emptPropChannelBinding,                 // value field is boolean
    emptPropNap,                            // value field is boolean
    emptPropStandalone,                     // value field is boolean
    emptPropMppeEncryption,                 // value field is boolean
    emptPropTunnelMethod,                   // value field is boolean
    emptPropSupportsConfig,                 // value field is boolean
    emptPropCertifiedMethod,                // value field is boolean
    emptPropHiddenMethod,                   // value field is boolean
    emptPropMachineAuth,                    // value field is boolean
    emptPropUserAuth,                       // value field is boolean
    emptPropIdentityPrivacy,                // value field is boolean
    emptPropMethodChaining,                 // value field is boolean
    emptPropSharedStateEquivalence,         // value field is boolean
    emptLegacyMethodPropertyFlag = 31,      // value field is dword
    emptPropVendorSpecific = 255            // value field is string
} EAP_METHOD_PROPERTY_TYPE;

typedef
#ifdef __midl
    [v1_enum]
#endif
// Method property value types that EAP methods support
enum _EAP_METHOD_PROPERTY_VALUE_TYPE
{
    empvtBool = 0,      // value type is boolean
    empvtDword,         // value type is dword
    empvtString         // value type is string
} EAP_METHOD_PROPERTY_VALUE_TYPE;

typedef struct _EAP_METHOD_PROPERTY_VALUE_BOOL
{
    DWORD length;
    BOOL value;
} EAP_METHOD_PROPERTY_VALUE_BOOL;

typedef struct _EAP_METHOD_PROPERTY_VALUE_DWORD
{
    DWORD length;
    DWORD value;
} EAP_METHOD_PROPERTY_VALUE_DWORD;

typedef struct _EAP_METHOD_PROPERTY_VALUE_STRING
{
    DWORD length;
#ifdef __midl
    [size_is(length)] BYTE* value;
#else
    BYTE* value;
#endif
} EAP_METHOD_PROPERTY_VALUE_STRING;

#ifdef __midl
typedef [switch_type(EAP_METHOD_PROPERTY_VALUE_TYPE)]
union {
    [case(empvtBool)]
        EAP_METHOD_PROPERTY_VALUE_BOOL empvBool;      // value is boolean type

    [case(empvtDword)]
        EAP_METHOD_PROPERTY_VALUE_DWORD empvDword;    // value is dword type

    [case(empvtString)]
        EAP_METHOD_PROPERTY_VALUE_STRING empvString;   // value is string type
} EAP_METHOD_PROPERTY_VALUE;
#else
typedef union _EAP_METHOD_PROPERTY_VALUE {
    EAP_METHOD_PROPERTY_VALUE_BOOL empvBool;      // value is boolean type
    EAP_METHOD_PROPERTY_VALUE_DWORD empvDword;    // value is dword type
    EAP_METHOD_PROPERTY_VALUE_STRING empvString;   // value is string type
} EAP_METHOD_PROPERTY_VALUE;
#endif

// Structure to represent a method property
typedef struct _EAP_METHOD_PROPERTY {
    // specifies the method property type
    EAP_METHOD_PROPERTY_TYPE eapMethodPropertyType;

    // specifies the data type of value
    EAP_METHOD_PROPERTY_VALUE_TYPE eapMethodPropertyValueType;
#ifdef __midl
    [switch_is(eapMethodPropertyValueType)] EAP_METHOD_PROPERTY_VALUE eapMethodPropertyValue;
#else
    // specifies the value of the method property
    EAP_METHOD_PROPERTY_VALUE eapMethodPropertyValue;
#endif
} EAP_METHOD_PROPERTY;

// Structure to represent an array of method properties
typedef struct _EAP_METHOD_PROPERTY_ARRAY {
    // specifies the number of method properties in array
    DWORD dwNumberOfProperties;

    // specifies the array of method property
#ifdef __midl
    [size_is(dwNumberOfProperties)] EAP_METHOD_PROPERTY* pFields;
#else
    EAP_METHOD_PROPERTY* pMethodProperty;
#endif
} EAP_METHOD_PROPERTY_ARRAY;

//
// structure representing the parameters that needs to be passed 
// to EAPHost for raising identity ui
//
typedef struct _EAPHOST_IDENTITY_UI_PARAMS {
    // The following parameters are passed by supplicant to eaphost
    EAP_METHOD_TYPE eapMethodType;
    DWORD dwFlags;
    DWORD dwSizeofConnectionData;
#ifdef __midl
    [size_is(dwSizeofConnectionData)] BYTE* pConnectionData;
#else
    BYTE* pConnectionData;
#endif
    DWORD dwSizeofUserData;
#ifdef __midl
    [size_is(dwSizeofUserData)] BYTE* pUserData;
#else
    BYTE* pUserData;
#endif
    // The following parameters are returned by eaphost to supplicant
    DWORD dwSizeofUserDataOut;
#ifdef __midl
    [size_is(dwSizeofUserDataOut)] BYTE* pUserDataOut;
#else
    BYTE* pUserDataOut;
#endif
    LPWSTR pwszIdentity;
    DWORD  dwError;
    EAP_ERROR* pEapError;
} EAPHOST_IDENTITY_UI_PARAMS;

//
// structure representing the parameters that needs to be passed 
// to EAPHost for raising interactive ui
//
typedef struct _EAPHOST_INTERACTIVE_UI_PARAMS {
    // The following parameters are passed by supplicant to eaphost
    DWORD dwSizeofContextData;
#ifdef __midl
    [size_is(dwSizeofContextData)] BYTE* pContextData;
#else
    BYTE* pContextData;
#endif
    // The following parameters are returned by eaphost to supplicant
    DWORD dwSizeofInteractiveUIData;
#ifdef __midl
    [size_is(dwSizeofInteractiveUIData)] BYTE* pInteractiveUIData;
#else
    BYTE* pInteractiveUIData;
#endif
    DWORD dwError;
    EAP_ERROR* pEapError;
} EAPHOST_INTERACTIVE_UI_PARAMS;

typedef enum _EapCredentialType
{
    EAP_EMPTY_CREDENTIAL=0,
    EAP_USERNAME_PASSWORD_CREDENTIAL,
    EAP_WINLOGON_CREDENTIAL,
    EAP_CERTIFICATE_CREDENTIAL,
    EAP_SIM_CREDENTIAL
} EapCredentialType;

typedef struct _EapUsernamePasswordCredential
{
    LPWSTR username;
    LPWSTR password;
} EapUsernamePasswordCredential;

#define CERTIFICATE_HASH_LENGTH 20

typedef struct _EapCertificateCredential
{
    BYTE   certHash[CERTIFICATE_HASH_LENGTH];
    LPWSTR password;
} EapCertificateCredential;

typedef struct _EapSimCredential
{
    LPWSTR iccID;
} EapSimCredential;

#ifdef __midl
    typedef [switch_type(EapCredentialType)]
    union {
        [case(EAP_USERNAME_PASSWORD_CREDENTIAL)] EapUsernamePasswordCredential username_password;
        [case(EAP_CERTIFICATE_CREDENTIAL)] EapCertificateCredential certificate;
        [case(EAP_SIM_CREDENTIAL)] EapSimCredential sim;
        [default]   ;
    } EapCredentialTypeData;
#else
    typedef union {
         EapUsernamePasswordCredential username_password;
         EapCertificateCredential certificate;
         EapSimCredential sim;
    } EapCredentialTypeData;
#endif

typedef struct _EapCredential
{
    EapCredentialType credType;
#ifdef __midl
    [switch_is(credType)] EapCredentialTypeData credData;
#else
    EapCredentialTypeData credData;
#endif
} EapCredential;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //EAPTYPES_H
