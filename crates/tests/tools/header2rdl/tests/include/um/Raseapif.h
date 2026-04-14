/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    raseapif.h

Abstract:

    Defines interface between a third party authentication module
    and the Remote Access Service PPP engine.

--*/

#ifndef _RASEAPIF_
#define _RASEAPIF_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C"
{
#endif

#if (WINVER >= 0x0500)

    //
    // Defines used for installtion of EAP DLL
    //
    // Custom EAP DLL (ex. Name=Sample.dll, Type=(decimal 40) regsitry installation)
    //
    // HKEY_LOCAL_MACHINE\System\CurrentControlSet\Services\Rasman\PPP\EAP\40)
    //      Path                (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      ConfigUIPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      InteractiveUIPath   (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      IdentityPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      FriendlyName        (REG_SZ) Sample EAP Protocol
    //      RequireConfigUI     (REG_DWORD)     1
    //      ConfigCLSID         (REG_SZ)        {0000031A-0000-0000-C000-000000000046}
    //      StandaloneSupported (REG_DWORD)     1
    //

#define RAS_EAP_REGISTRY_LOCATION TEXT("System\\CurrentControlSet\\Services\\Rasman\\PPP\\EAP")

#define RAS_EAP_VALUENAME_PATH TEXT("Path")
#define RAS_EAP_VALUENAME_CONFIGUI TEXT("ConfigUIPath")
#define RAS_EAP_VALUENAME_INTERACTIVEUI TEXT("InteractiveUIPath")
#define RAS_EAP_VALUENAME_IDENTITY TEXT("IdentityPath")
#define RAS_EAP_VALUENAME_FRIENDLY_NAME TEXT("FriendlyName")
#define RAS_EAP_VALUENAME_DEFAULT_DATA TEXT("ConfigData")
#define RAS_EAP_VALUENAME_REQUIRE_CONFIGUI TEXT("RequireConfigUI")
#define RAS_EAP_VALUENAME_ENCRYPTION TEXT("MPPEEncryptionSupported")
#define RAS_EAP_VALUENAME_INVOKE_NAMEDLG TEXT("InvokeUsernameDialog")
#define RAS_EAP_VALUENAME_INVOKE_PWDDLG TEXT("InvokePasswordDialog")
#define RAS_EAP_VALUENAME_CONFIG_CLSID TEXT("ConfigCLSID")
#define RAS_EAP_VALUENAME_STANDALONE_SUPPORTED TEXT("StandaloneSupported")
#define RAS_EAP_VALUENAME_ROLES_SUPPORTED TEXT("RolesSupported")
#define RAS_EAP_VALUENAME_PER_POLICY_CONFIG TEXT("PerPolicyConfig")
#define RAS_EAP_VALUENAME_ISTUNNEL_METHOD TEXT("IsTunnelMethod")
#define RAS_EAP_VALUENAME_FILTER_INNERMETHODS TEXT("FilterInnerMethods")

// Flags for RolesSupported.
#define RAS_EAP_ROLE_AUTHENTICATOR 0x00000001
#define RAS_EAP_ROLE_AUTHENTICATEE 0x00000002

//
// Following flags describe the hosting of EAP
// methods in PEAP or in EAP.  If this role is
// missing the default behavior is to show the
// EAP method in PEAP and in EAP.
//
#define RAS_EAP_ROLE_EXCLUDE_IN_EAP 0x00000004
#define RAS_EAP_ROLE_EXCLUDE_IN_PEAP 0x00000008
#define RAS_EAP_ROLE_EXCLUDE_IN_VPN 0x00000010

    typedef enum _RAS_AUTH_ATTRIBUTE_TYPE_
    {
        raatMinimum = 0,                // Undefined
        raatUserName,                   // Value field is a Pointer
        raatUserPassword,               // Value field is a Pointer
        raatMD5CHAPPassword,            // Value field is a Pointer
        raatNASIPAddress,               // Value field is a 32 bit integral value
        raatNASPort,                    // Value field is a 32 bit integral value
        raatServiceType,                // Value field is a 32 bit integral value
        raatFramedProtocol,             // Value field is a 32 bit integral value
        raatFramedIPAddress,            // Value field is a 32 bit integral value
        raatFramedIPNetmask,            // Value field is a 32 bit integral value
        raatFramedRouting = 10,         // Value field is a 32 bit integral value
        raatFilterId,                   // Value field is a Pointer
        raatFramedMTU,                  // Value field is a 32 bit integral value
        raatFramedCompression,          // Value field is a 32 bit integral value
        raatLoginIPHost,                // Value field is a 32 bit integral value
        raatLoginService,               // Value field is a 32 bit integral value
        raatLoginTCPPort,               // Value field is a 32 bit integral value
        raatUnassigned17,               // Undefined
        raatReplyMessage,               // Value field is a Pointer
        raatCallbackNumber,             // Value field is a Pointer
        raatCallbackId = 20,            // Value field is a Pointer
        raatUnassigned21,               // Undefined
        raatFramedRoute,                // Value field is a Pointer
        raatFramedIPXNetwork,           // Value field is a 32 bit integral value
        raatState,                      // Value field is a Pointer
        raatClass,                      // Value field is a Pointer
        raatVendorSpecific,             // Value field is a Pointer
        raatSessionTimeout,             // Value field is a 32 bit integral value
        raatIdleTimeout,                // Value field is a 32 bit integral value
        raatTerminationAction,          // Value field is a 32 bit integral value
        raatCalledStationId = 30,       // Value field is a Pointer
        raatCallingStationId,           // Value field is a Pointer
        raatNASIdentifier,              // Value field is a Pointer
        raatProxyState,                 // Value field is a Pointer
        raatLoginLATService,            // Value field is a Pointer
        raatLoginLATNode,               // Value field is a Pointer
        raatLoginLATGroup,              // Value field is a Pointer
        raatFramedAppleTalkLink,        // Value field is a 32 bit integral value
        raatFramedAppleTalkNetwork,     // Value field is a 32 bit integral value
        raatFramedAppleTalkZone,        // Value field is a Pointer
        raatAcctStatusType = 40,        // Value field is a 32 bit integral value
        raatAcctDelayTime,              // Value field is a 32 bit integral value
        raatAcctInputOctets,            // Value field is a 32 bit integral value
        raatAcctOutputOctets,           // Value field is a 32 bit integral value
        raatAcctSessionId,              // Value field is a Pointer
        raatAcctAuthentic,              // Value field is a 32 bit integral value
        raatAcctSessionTime,            // Value field is a 32 bit integral value
        raatAcctInputPackets,           // Value field is a 32 bit integral value
        raatAcctOutputPackets,          // Value field is a 32 bit integral value
        raatAcctTerminateCause,         // Value field is a 32 bit integral value
        raatAcctMultiSessionId = 50,    // Value field is a Pointer
        raatAcctLinkCount,              // Value field is a 32 bit integral value
        raatAcctEventTimeStamp = 55,    // Value field is a 32 bit integral value
        raatMD5CHAPChallenge = 60,      // Value field is a Pointer
        raatNASPortType,                // Value field is a 32 bit integral value
        raatPortLimit,                  // Value field is a 32 bit integral value
        raatLoginLATPort,               // Value field is a Pointer
        raatTunnelType,                 // Value field is a 32 bit integral value
        raatTunnelMediumType,           // Value field is a 32 bit integral value
        raatTunnelClientEndpoint,       // Value field is a Pointer
        raatTunnelServerEndpoint,       // Value field is a Pointer
        raatARAPPassword = 70,          // Value field is a Pointer
        raatARAPFeatures,               // Value field is a Pointer
        raatARAPZoneAccess,             // Value field is a 32 bit integral value
        raatARAPSecurity,               // Value field is a 32 bit integral value
        raatARAPSecurityData,           // Value field is a Pointer
        raatPasswordRetry,              // Value field is a 32 bit integral value
        raatPrompt,                     // Value field is a 32 bit integral value
        raatConnectInfo,                // Value field is a Pointer
        raatConfigurationToken,         // Value field is a Pointer
        raatEAPMessage,                 // Value field is a Pointer
        raatSignature = 80,             // Value field is a Pointer
        raatARAPChallengeResponse = 84, // Value field is a Pointer
        raatAcctInterimInterval = 85,   // Value field is a 32 bit integral value
        raatNASIPv6Address = 95,
        raatFramedInterfaceId,
        raatFramedIPv6Prefix,
        raatLoginIPv6Host,
        raatFramedIPv6Route,
        raatFramedIPv6Pool,
        raatARAPGuestLogon = 8096,        // Value field is a 32 bit integral value
        raatCertificateOID,               // Value field is a Pointer
        raatEAPConfiguration,             // Value field is a Pointer
        raatPEAPEmbeddedEAPTypeId = 8099, // Value field is a 32 bit integral value
        raatInnerEAPTypeId = 8099,        // Value field is a 32 bit integral value
        raatPEAPFastRoamedSession = 8100, // Value field is a 32 bit integral value
        raatFastRoamedSession = 8100,     // Value field is a 32 bit integral value
        raatEAPTLV = 8102,                // Value field is a Pointer
        raatCredentialsChanged,           // Value field is a Integer with boolean semantics
        raatCertificateThumbprint = 8250, // Value field is a Pointer
        raatPeerId = 9000,                // Value field is a pointer
        raatServerId,                     // Value field is a pointer
        raatMethodId,                     // Value field is a pointer
        raatEMSK,                         // Value field is a pointer
        raatSessionId,                    // Value field is a pointer
        raatReserved = 0xFFFFFFFF         // Undefined

    } RAS_AUTH_ATTRIBUTE_TYPE;

//
// VSA attribute ids for ARAP
//
#define raatARAPChallenge 33
#define raatARAPOldPassword 19
#define raatARAPNewPassword 20
#define raatARAPPasswordChangeReason 21

#ifndef _NGCTICKETCONTEXT_
#define _NGCTICKETCONTEXT_

// The NCRYPT_PIN_CACHE_PIN_PROPERTY and NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY properties
// return a 32 byte random unique ID encoded as a null terminated base64 Unicode string. The string length
// is 32 * 4/3 + 1 characters = 45 characters, 90 bytes
#define NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH 90

#define NGC_TICKET_PROPERTY_STRING_LENGTH (NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH / sizeof(WCHAR))

    typedef ULONG_PTR NCRYPT_KEY_HANDLE;

    typedef struct _NgcTicketContext
    {
        WCHAR wszTicket[NGC_TICKET_PROPERTY_STRING_LENGTH];
        NCRYPT_KEY_HANDLE hKey;
        HANDLE hImpersonateToken;
    } NgcTicketContext;

#endif

    //
    // Value is set to the 32 bit integral value or a pointer to data.
    // 32 bit integral values should be in host format, not network format.
    // Length for a 32 bit integral value can be 1, 2 or 4. The array of
    // attributes must be terminated with an attribute of type raatMinimum.
    //

    typedef struct _RAS_AUTH_ATTRIBUTE
    {
        RAS_AUTH_ATTRIBUTE_TYPE raaType;
        DWORD dwLength;
        _Field_size_(dwLength) PVOID Value;

    } RAS_AUTH_ATTRIBUTE, *PRAS_AUTH_ATTRIBUTE;

    //
    // EAP packet codes from EAP spec.
    //

#define EAPCODE_Request 1
#define EAPCODE_Response 2
#define EAPCODE_Success 3
#define EAPCODE_Failure 4

#define MAXEAPCODE 4

    //
    // Values of the fFlags field in PPP_EAP_INPUT
    // These have the same values as the RASEAPF_ flags in ras.h
    //

#define RAS_EAP_FLAG_ROUTER 0x00000001          // This is a router
#define RAS_EAP_FLAG_NON_INTERACTIVE 0x00000002 // No UI should be displayed
#define RAS_EAP_FLAG_LOGON \
    0x00000004 // The user data was
               // obtained from Winlogon
#define RAS_EAP_FLAG_PREVIEW \
    0x00000008                             // User has checked
                                           // "Prompt for information
                                           // before dialing"
#define RAS_EAP_FLAG_FIRST_LINK 0x00000010 // This is the first link
#define RAS_EAP_FLAG_MACHINE_AUTH \
    0x00000020 // Use the default machine cert
               // or user cert based on the
               // application logon context
#define RAS_EAP_FLAG_GUEST_ACCESS \
    0x00000040 // Request to provide guest
               // access.
#define RAS_EAP_FLAG_8021X_AUTH \
    0x00000080 // Anything specific to 8021x
               // to be done in TLS
#define RAS_EAP_FLAG_HOSTED_IN_PEAP \
    0x00000100 // This EAP Method is hosted
               // in PEAP
#define RAS_EAP_FLAG_RESUME_FROM_HIBERNATE \
    0x00000200 // Indicates this is the
               // first call after m/c
               // resumed from hibernation.
#define RAS_EAP_FLAG_PEAP_UPFRONT \
    0x00000400 // Indicate peap is enabled
               // at the beginning of IAS pipeline.
#define RAS_EAP_FLAG_ALTERNATIVE_USER_DB \
    0x00000800 // The user database is
               // not active directory
#define RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH \
    0x00001000 // Indicate peap should not
               // fast reconnect

#define RAS_EAP_FLAG_PRE_LOGON 0x00020000 // Pre logon scenario like PLAP

#define RAS_EAP_FLAG_CONFG_READONLY 0x00080000 // connection settings are read-only

#define RAS_EAP_FLAG_RESERVED 0x00100000 // PEAP specific flag

#define RAS_EAP_FLAG_SAVE_CREDMAN 0x00200000 // Save credentials to CredMan

#define RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED 0x02000000 // Certificate server validation must be performed

    typedef struct _PPP_EAP_PACKET
    {
        BYTE Code; // 1-Request, 2-Response, 3-Success, 4-Failure

        BYTE Id; // Id of this packet

        BYTE Length[2]; // Length of this packet

        BYTE Data[1]; // Data - First byte is Type for Request/Response

    } PPP_EAP_PACKET, *PPPP_EAP_PACKET;

#define PPP_EAP_PACKET_HDR_LEN (sizeof(PPP_EAP_PACKET) - 1)

    //
    // Interface structure between the engine and APs. This is passed to the
    // AP's via the RasCpBegin call.
    //

    typedef struct _PPP_EAP_INPUT
    {
        //
        // Size of this structure
        //

        DWORD dwSizeInBytes;

        //
        // The following five fields are valid only in RasEapBegin call
        //

        DWORD fFlags; // See RAS_EAP_FLAG_*

        BOOL fAuthenticator; // Act as authenticator or authenticatee

        WCHAR* pwszIdentity; // Users's identity

        WCHAR* pwszPassword; // Client's account password. Only valid when
                             // fAuthenticator is FALSE.

        BYTE bInitialId; // Initial packet identifier. Must be used for
                         // the first EAP packet sent by the DLL and
                         // incremented by one for each subsequent
                         // request packet.

        //
        // During the RasEapBegin call on the authenticator side, pUserAttributes
        // contains the set of attributes for the currently dialed in user, e.g.,
        // the port used, NAS IP Address, etc.
        //
        // When the fAuthenticationComplete flag is TRUE, pUserAttributes contains
        // attributes (if any) returned by the authentication provider.
        //
        // This memory is not owned by the EAP DLL and should be treated as
        // read-only.
        //

        RAS_AUTH_ATTRIBUTE* pUserAttributes;

        //
        // The next two fields are used only if the EAP DLL is using the
        // currently configured authentication provider ex: RADIUS or Windows NT
        // domain authentication, and the fAuthenticator field above is set to
        // TRUE.
        //

        //
        // Indicates that the authenticator has completed authentication.
        // Ignore this field if an authentication provider is not being used.
        //

        BOOL fAuthenticationComplete;

        //
        // Result of the authentication process by the authentication provider.
        // NO_ERROR indicates success, otherwise it is a value from winerror.h,
        // raserror.h or mprerror.h indicating failure reason.
        //

        DWORD dwAuthResultCode;

        //
        // Valid only on the authenticatee side. This may be used on the
        // authenticatee side to impersonate the user being authenticated.
        //

        OPTIONAL HANDLE hTokenImpersonateUser;

        //
        // This variable should be examined only by the authenticatee side.
        // The EAP specification states that the success packet may be lost and
        // since it is a non-acknowledged packet, reception of an NCP packet should
        // be interpreted as a success packet. This varable is set to TRUE in this
        // case only on the authenticatee side
        //

        BOOL fSuccessPacketReceived;

        //
        // Will be set to TRUE only when the user dismissed the interactive
        // UI that was invoked by the EAP dll
        //

        BOOL fDataReceivedFromInteractiveUI;

        //
        // Data received from the Interactive UI. Will be set to
        // non-NULL when fDataReceivedFromInteractiveUI is set to TRUE and
        // RasEapInvokeInteractiveUI returned non-NULL data. This buffer will be
        // freed by the PPP engine on return from the RasEapMakeMessage call. A
        // copy of this data should be made in the EAP Dll's memory space.
        //

        _Field_size_bytes_opt_(dwSizeOfDataFromInteractiveUI) PBYTE pDataFromInteractiveUI;

        //
        // Size in bytes of data pointed to by pInteractiveConnectionData. This may
        // be 0 if there was no data passed back by RasEapInvokeInteractiveUI.
        //

        DWORD dwSizeOfDataFromInteractiveUI;

        //
        // Connection data received from the Config UI. Will be set to non-NULL
        // when the RasEapBegin call is made and the RasEapInvokeConfigUI
        // returned non-NULL data. This buffer will be freed by the PPP engine
        // on return from the RasEapBegin call. A copy of this data should
        // be made in the EAP Dll's memory space.
        //

        _Field_size_bytes_opt_(dwSizeOfConnectionData) PBYTE pConnectionData;

        //
        // Size in bytes of data pointed to by pConnectionData. This may be
        // 0 if there was no data passed back by the RasEapInvokeConfigUI call.
        //

        DWORD dwSizeOfConnectionData;

        //
        // User data received from the Identity UI or Interactive UI. Will be set
        // to non-NULL when the RasEapBegin call is made if such data exists.
        // This buffer will be freed by the PPP engine on return from the
        // RasEapBegin call. A copy of this data should be made in the EAP Dll's
        // memory space.
        //

        _Field_size_bytes_opt_(dwSizeOfUserData) PBYTE pUserData;

        //
        // Size in bytes of data pointed to by pUserData. This may be 0 if there
        // is no data.
        //

        DWORD dwSizeOfUserData;

        //
        // Reserved.
        //

        HANDLE hReserved;

        //
        // Connection identifier passed to EapHostPeerBeginSession by supplicant.
        //

        GUID guidConnectionId;

        // Is the Eap connection request a Vpn request
        BOOL isVpn;

    } PPP_EAP_INPUT, *PPPP_EAP_INPUT;

    typedef enum _PPP_EAP_ACTION
    {
        //
        // These actions are provided by the EAP DLL as output from the
        // RasEapMakeMessage API.  They tell the PPP engine what action (if any) to
        // take on the EAP DLL's behalf, and eventually inform the engine that the
        // EAP DLL has finished authentication.
        //

        EAPACTION_NoAction,                   // Be passive, i.e. listen without timeout (default)
        EAPACTION_Authenticate,               // Invoke the back-end authenticator.
        EAPACTION_Done,                       // End auth session, dwAuthResultCode is set
        EAPACTION_SendAndDone,                // As above but send message without timeout first
        EAPACTION_Send,                       // Send message, don't timeout waiting for reply
        EAPACTION_SendWithTimeout,            // Send message, timeout if reply not received
        EAPACTION_SendWithTimeoutInteractive, // As above, but don't increment
                                              // retry count

        EAPACTION_IndicateTLV,     // Do not use
        EAPACTION_IndicateIdentity // Do not use
    } PPP_EAP_ACTION;

    typedef struct _PPP_EAP_OUTPUT
    {
        //
        // Size of this structure
        //

        DWORD dwSizeInBytes;

        //
        // Action that the PPP engine should take
        //

        PPP_EAP_ACTION Action;

        //
        // dwAuthResultCode is valid only with an Action code of Done or
        // SendAndDone. Zero value indicates succesful authentication.
        // Non-zero indicates unsuccessful authentication with the value
        // indicating the reason for authentication failure.
        // Non-zero return codes should be only from winerror.h, raserror.h and
        // mprerror.h
        //

        DWORD dwAuthResultCode;

        //
        // When Action is EAPACTION_Authenticate, pUserAttributes may contain
        // additional attributes necessary to authenticate the user, e.g.,
        // User-Password. If no credentials are presented, the back-end
        // authenticator will assume the user is authentic and only retrieve
        // authorizations.
        //
        // When Action is EAPACTION_Done, EAPACTION_SendAndDone, or EAPACTION_Send,
        // pUserAttributes may contain additional attributes for the user. These
        // attributes will overwrite any attributes of the same type returned by
        // the back-end authenticator.
        //
        // It is up to the EAP DLL to free this memory in RasEapEnd call.
        //

        OPTIONAL RAS_AUTH_ATTRIBUTE* pUserAttributes;
        //
        // Flag set to true will cause the RasEapInvokeInteractiveUI call to be
        // made.
        //

        BOOL fInvokeInteractiveUI;

        //
        // Pointer to context data, if any, to be sent to the UI. The EAP dll
        // is responsible for freeing this buffer in the RasEapEnd call or when
        // a response from the user for this invocation is obtained.
        //

        OPTIONAL PBYTE pUIContextData;

        //
        // Size in bytes of the data pointed to by pUIContextData. Ignored if
        // pUIContextData is NULL
        //

        DWORD dwSizeOfUIContextData;

        //
        // When set to TRUE, indicates that the information pointed to by
        // pConnectionData should be saved in the phonebook. Only valid on
        // the authenticatee side.
        //

        BOOL fSaveConnectionData;

        //
        // If fSaveConnectionData above is true, the data pointed to by
        // pConnectionData will be saved in the phonebook. This data
        // must be freed by the DLL when the RasEapEnd call is made.
        //

        OPTIONAL PBYTE pConnectionData;

        //
        // Size, in bytes, of the data pointed to by pConnectionData
        //

        DWORD dwSizeOfConnectionData;

        //
        // When set to TRUE, indicates that the information pointed to by
        // pUserData should be saved in the registry for this user. Only valid
        // on the authenticatee side.
        //

        BOOL fSaveUserData;

        //
        // If fSaveUserData above is true, the data pointed to by pUserData will be
        // saved in the registry for this user. This data must be freed by the DLL
        // when the RasEapEnd call is made.
        //

        OPTIONAL PBYTE pUserData;

        //
        // Size, in bytes, of the data pointed to by pUserData
        //

        DWORD dwSizeOfUserData;

        NgcTicketContext* pNgcKerbTicket;

        BOOL fSaveToCredMan;

    } PPP_EAP_OUTPUT, *PPPP_EAP_OUTPUT;

    typedef struct _PPP_EAP_INFO
    {
        //
        // Size of this structure
        //

        DWORD dwSizeInBytes;

        DWORD dwEapTypeId;

        //
        // Called to initialize/uninitialize this module. This will be called before
        // any other call is made. fInitialize will be TRUE iff the module has to be
        // initialized. Must return errorcodes only from winerror.h, raserror.h or
        // mprerror.h
        //

        DWORD(APIENTRY* RasEapInitialize)(_In_ BOOL fInitialize);

        //
        // Called to get a context buffer for this EAP session and pass
        // initialization information. This will be called before any other
        // call is made, except RasEapInitialize. Must return errorcodes only from
        // winerror.h, raserror.h or mprerror.h
        //

        DWORD(APIENTRY* RasEapBegin)(_Outptr_ VOID** ppWorkBuffer, _In_ PPP_EAP_INPUT* pPppEapInput);

        //
        // Called to free the context buffer for this EAP session.
        // Called after this session is completed successfully or not, provided
        // the RasEapBegin call for this EAP session returned successfully.
        // Must return errorcodes only from winerror.h, raserror.h or mprerror.h
        //

        DWORD(APIENTRY* RasEapEnd)(_In_ VOID* pWorkBuffer);

        //
        // Called to process an incomming packet and/or send a packet.
        // cbSendPacket is the size in bytes of the buffer pointed to by
        // pSendPacket. Must return errorcodes only from winerror.h, raserror.h or
        // mprerror.h. Error return code indicates an error occurance during the
        // authentication process.
        //

        DWORD(APIENTRY* RasEapMakeMessage)(
            _In_ VOID* pWorkBuf,
            _In_ PPP_EAP_PACKET* pReceivePacket,
            _Inout_updates_(cbSendPacket) PPP_EAP_PACKET* pSendPacket,
            _In_ DWORD cbSendPacket,
            _Out_ PPP_EAP_OUTPUT* pEapOutput,
            _In_opt_ PPP_EAP_INPUT* pEapInput);

    } PPP_EAP_INFO, *PPPP_EAP_INFO;

    //
    // structure representing the parameters that needs to be passed
    // to the eap method for raising identity ui
    //
    typedef struct _LEGACY_IDENTITY_UI_PARAMS
    {
        DWORD eapType;
        DWORD dwFlags;
        DWORD dwSizeofConnectionData;
        _Field_size_(dwSizeofConnectionData) BYTE* pConnectionData;
        DWORD dwSizeofUserData;
        _Field_size_(dwSizeofUserData) BYTE* pUserData;
        DWORD dwSizeofUserDataOut;
        _Field_size_(dwSizeofUserDataOut) BYTE* pUserDataOut;
        LPWSTR pwszIdentity;
        DWORD dwError;
    } LEGACY_IDENTITY_UI_PARAMS;

    //
    // structure representing the parameters that needs to be passed
    // to the eap method for raising interactive ui
    //
    typedef struct _LEGACY_INTERACTIVE_UI_PARAMS
    {
        DWORD eapType;
        DWORD dwSizeofContextData;
        _Field_size_(dwSizeofContextData) BYTE* pContextData;
        DWORD dwSizeofInteractiveUIData;
        _Field_size_(dwSizeofInteractiveUIData) BYTE* pInteractiveUIData;
        DWORD dwError;
    } LEGACY_INTERACTIVE_UI_PARAMS;

    //
    // RasEapGetInfo should be exported by the 3rd party EAP dll installed in the
    // registry via the Path value.
    //

    DWORD APIENTRY RasEapGetInfo(_In_ DWORD dwEapTypeId, _Out_ PPP_EAP_INFO* pEapInfo);

    //
    // RasEapFreeMemory should be exported by the 3rd party EAP dlls installed in
    // the registry via the InteractiveUIPath, ConfigUIPath, and IdentityPath
    // values.
    //

    DWORD APIENTRY RasEapFreeMemory(_In_ BYTE* pMemory);

    //
    // RasEapInvokeInteractiveUI and RasEapFreeMemory should be exported by the
    // 3rd party EAP dll installed in the registry via the InteractiveUIPath
    // value.
    //

    DWORD APIENTRY RasEapInvokeInteractiveUI(
        _In_ DWORD dwEapTypeId,
        _In_ HWND hwndParent,
        _In_reads_(dwSizeOfUIContextData) BYTE* pUIContextData,
        _In_ DWORD dwSizeOfUIContextData,
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppDataFromInteractiveUI,
        _Out_ DWORD* pdwSizeOfDataFromInteractiveUI);

    //
    // RasEapInvokeConfigUI and RasEapFreeMemory should be exported by the
    // 3rd party EAP dll installed in the registry via the ConfigUIPath value.
    //

    DWORD APIENTRY RasEapInvokeConfigUI(
        _In_ DWORD dwEapTypeId,
        _In_ HWND hwndParent,
        _In_ DWORD dwFlags,
        _In_reads_(dwSizeOfConnectionDataIn) BYTE* pConnectionDataIn,
        _In_ DWORD dwSizeOfConnectionDataIn,
        _Outptr_result_buffer_(*pdwSizeOfConnectionDataOut) BYTE** ppConnectionDataOut,
        _Out_ DWORD* pdwSizeOfConnectionDataOut);

    //
    // RasEapGetIdentity and RasEapFreeMemory should be exported by the
    // 3rd party EAP dll installed in the registry via the IdentityPath value.
    //

    DWORD APIENTRY RasEapGetIdentity(
        _In_ DWORD dwEapTypeId,
        _In_ HWND hwndParent,
        _In_ DWORD dwFlags,
        _In_ LPCWSTR pwszPhonebook,
        _In_ LPCWSTR pwszEntry,
        _In_reads_(dwSizeOfConnectionDataIn) BYTE* pConnectionDataIn,
        _In_ DWORD dwSizeOfConnectionDataIn,
        _In_reads_(dwSizeOfUserDataIn) BYTE* pUserDataIn,
        _In_ DWORD dwSizeOfUserDataIn,
        _Outptr_result_buffer_(*pdwSizeOfUserDataOut) BYTE** ppUserDataOut,
        _Out_ DWORD* pdwSizeOfUserDataOut,
        _Outptr_ LPWSTR* ppwszIdentityOut);

#endif /* WINVER >= 0x0500 */

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _RASEAPIF_
