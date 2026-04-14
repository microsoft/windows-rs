//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       sspi.h
//
//  Contents:   Security Support Provider Interface
//              Prototypes and structure definitions
//
//  Functions:  Security Support Provider API
//
//
//----------------------------------------------------------------------------

#include <sdkddkver.h>
#include <winapifamily.h>

// begin_ntifs
#ifndef __SSPI_H__
#define __SSPI_H__
// end_ntifs

#if _MSC_VER > 1000
#pragma once
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Determine environment:
//

#ifdef SECURITY_WIN32
#define ISSP_LEVEL  32
#define ISSP_MODE   1
#endif // SECURITY_WIN32

#ifdef SECURITY_KERNEL
#define ISSP_LEVEL  32          // ntifs

//
// SECURITY_KERNEL trumps SECURITY_WIN32.  Undefine ISSP_MODE so that
// we don't get redefine errors.
//
#ifdef ISSP_MODE
#undef ISSP_MODE
#endif
#define ISSP_MODE   0           // ntifs
#endif // SECURITY_KERNEL

#ifdef SECURITY_MAC
#define ISSP_LEVEL  32
#define ISSP_MODE   1
#endif // SECURITY_MAC


#ifndef ISSP_LEVEL
#error  You must define one of SECURITY_WIN32, SECURITY_KERNEL, or
#error  SECURITY_MAC
#endif // !ISSP_LEVEL


//
// Now, define platform specific mappings:
//


// begin_ntifs

#if defined(_NO_KSECDD_IMPORT_)

#define KSECDDDECLSPEC

#else

#define KSECDDDECLSPEC __declspec(dllimport)

#endif

typedef WCHAR SEC_WCHAR;
typedef CHAR SEC_CHAR;

#ifndef __SECSTATUS_DEFINED__
typedef LONG SECURITY_STATUS;
#define __SECSTATUS_DEFINED__
#endif

#define SEC_TEXT TEXT
#define SEC_FAR
#define SEC_ENTRY __stdcall

// end_ntifs

//
// Decide what a string - 32 bits only since for 16 bits it is clear.
//


#ifdef UNICODE
typedef SEC_WCHAR * SECURITY_PSTR;
typedef CONST SEC_WCHAR * SECURITY_PCSTR;
#else // UNICODE
typedef SEC_CHAR * SECURITY_PSTR;
typedef CONST SEC_CHAR * SECURITY_PCSTR;
#endif // UNICODE



//
// Equivalent string for rpcrt:
//

#define __SEC_FAR SEC_FAR


//
// Okay, security specific types:
//


// begin_ntifs

#ifndef __SECHANDLE_DEFINED__
typedef struct _SecHandle
{
    ULONG_PTR dwLower ;
    ULONG_PTR dwUpper ;
} SecHandle, * PSecHandle ;

#define __SECHANDLE_DEFINED__
#endif // __SECHANDLE_DEFINED__

#define SecInvalidateHandle( x )    \
            ((PSecHandle) (x))->dwLower = ((PSecHandle) (x))->dwUpper = ((ULONG_PTR) ((INT_PTR)-1)) ;

#define SecIsValidHandle( x ) \
            ( ( ((PSecHandle) (x))->dwLower != ((ULONG_PTR) ((INT_PTR) -1 ))) && \
              ( ((PSecHandle) (x))->dwUpper != ((ULONG_PTR) ((INT_PTR) -1 ))) )

//
// pseudo handle value: the handle has already been deleted
//

#define SEC_DELETED_HANDLE  ((ULONG_PTR) (-2))

typedef SecHandle CredHandle;
typedef PSecHandle PCredHandle;

typedef SecHandle CtxtHandle;
typedef PSecHandle PCtxtHandle;

// end_ntifs


#  ifdef WIN32_CHICAGO

typedef unsigned __int64 QWORD;
typedef QWORD SECURITY_INTEGER, *PSECURITY_INTEGER;
#define SEC_SUCCESS(Status) ((Status) >= 0)

#  elif defined(_NTDEF_) || defined(_WINNT_)

typedef LARGE_INTEGER _SECURITY_INTEGER, SECURITY_INTEGER, *PSECURITY_INTEGER; // ntifs

#  else // _NTDEF_ || _WINNT_

typedef struct _SECURITY_INTEGER
{
    unsigned long LowPart;
    long HighPart;
} SECURITY_INTEGER, *PSECURITY_INTEGER;

#  endif // _NTDEF_ || _WINNT_

#  ifndef SECURITY_MAC

typedef SECURITY_INTEGER TimeStamp;                 // ntifs
typedef SECURITY_INTEGER * PTimeStamp;      // ntifs

#  else // SECURITY_MAC
typedef unsigned long TimeStamp;
typedef unsigned long * PTimeStamp;
#  endif // SECUIRT_MAC



//
// If we are in 32 bit mode, define the SECURITY_STRING structure,
// as a clone of the base UNICODE_STRING structure.  This is used
// internally in security components, an as the string interface
// for kernel components (e.g. FSPs)
//

#  ifndef _NTDEF_
typedef struct _SECURITY_STRING {
    unsigned short      Length;
    unsigned short      MaximumLength;
#    ifdef MIDL_PASS
    [size_is(MaximumLength / 2), length_is(Length / 2)]
#    endif // MIDL_PASS
    unsigned short *    Buffer;
} SECURITY_STRING, * PSECURITY_STRING;
#  else // _NTDEF_
typedef UNICODE_STRING SECURITY_STRING, *PSECURITY_STRING;  // ntifs
#  endif // _NTDEF_


// begin_ntifs

//
// SecPkgInfo structure
//
//  Provides general information about a security provider
//

typedef struct _SecPkgInfoW
{
    unsigned long fCapabilities;        // Capability bitmask
    unsigned short wVersion;            // Version of driver
    unsigned short wRPCID;              // ID for RPC Runtime
    unsigned long cbMaxToken;           // Size of authentication token (max)
#ifdef MIDL_PASS
    [string]
#endif
    SEC_WCHAR * Name;           // Text name

#ifdef MIDL_PASS
    [string]
#endif
    SEC_WCHAR * Comment;        // Comment
} SecPkgInfoW, * PSecPkgInfoW;

// end_ntifs

typedef struct _SecPkgInfoA
{
    unsigned long fCapabilities;        // Capability bitmask
    unsigned short wVersion;            // Version of driver
    unsigned short wRPCID;              // ID for RPC Runtime
    unsigned long cbMaxToken;           // Size of authentication token (max)
#ifdef MIDL_PASS
    [string]
#endif
    SEC_CHAR * Name;            // Text name

#ifdef MIDL_PASS
    [string]
#endif
    SEC_CHAR * Comment;         // Comment
} SecPkgInfoA, * PSecPkgInfoA;

#ifdef UNICODE
#  define SecPkgInfo SecPkgInfoW        // ntifs
#  define PSecPkgInfo PSecPkgInfoW      // ntifs
#else
#  define SecPkgInfo SecPkgInfoA
#  define PSecPkgInfo PSecPkgInfoA
#endif // !UNICODE

// begin_ntifs

//
//  Security Package Capabilities
//
#define SECPKG_FLAG_INTEGRITY                    0x00000001  // Supports integrity on messages
#define SECPKG_FLAG_PRIVACY                      0x00000002  // Supports privacy (confidentiality)
#define SECPKG_FLAG_TOKEN_ONLY                   0x00000004  // Only security token needed
#define SECPKG_FLAG_DATAGRAM                     0x00000008  // Datagram RPC support
#define SECPKG_FLAG_CONNECTION                   0x00000010  // Connection oriented RPC support
#define SECPKG_FLAG_MULTI_REQUIRED               0x00000020  // Full 3-leg required for re-auth.
#define SECPKG_FLAG_CLIENT_ONLY                  0x00000040  // Server side functionality not available
#define SECPKG_FLAG_EXTENDED_ERROR               0x00000080  // Supports extended error msgs
#define SECPKG_FLAG_IMPERSONATION                0x00000100  // Supports impersonation
#define SECPKG_FLAG_ACCEPT_WIN32_NAME            0x00000200  // Accepts Win32 names
#define SECPKG_FLAG_STREAM                       0x00000400  // Supports stream semantics
#define SECPKG_FLAG_NEGOTIABLE                   0x00000800  // Can be used by the negotiate package
#define SECPKG_FLAG_GSS_COMPATIBLE               0x00001000  // GSS Compatibility Available
#define SECPKG_FLAG_LOGON                        0x00002000  // Supports common LsaLogonUser
#define SECPKG_FLAG_ASCII_BUFFERS                0x00004000  // Token Buffers are in ASCII
#define SECPKG_FLAG_FRAGMENT                     0x00008000  // Package can fragment to fit
#define SECPKG_FLAG_MUTUAL_AUTH                  0x00010000  // Package can perform mutual authentication
#define SECPKG_FLAG_DELEGATION                   0x00020000  // Package can delegate
#define SECPKG_FLAG_READONLY_WITH_CHECKSUM       0x00040000  // Package can delegate
#define SECPKG_FLAG_RESTRICTED_TOKENS            0x00080000  // Package supports restricted callers
#define SECPKG_FLAG_NEGO_EXTENDER                0x00100000  // this package extends SPNEGO, there is at most one
#define SECPKG_FLAG_NEGOTIABLE2                  0x00200000  // this package is negotiated under the NegoExtender
#define SECPKG_FLAG_APPCONTAINER_PASSTHROUGH     0x00400000  // this package receives all calls from appcontainer apps
#define SECPKG_FLAG_APPCONTAINER_CHECKS          0x00800000  // this package receives calls from appcontainer apps
                                                             // if the following checks succeed
                                                             // 1. Caller has domain auth capability or
                                                             // 2. Target is a proxy server or
                                                             // 3. The caller has supplied creds
#define SECPKG_FLAG_CREDENTIAL_ISOLATION_ENABLED 0x01000000  // this package is running with Credential Guard enabled
#define SECPKG_FLAG_APPLY_LOOPBACK               0x02000000  // this package supports reliable detection of loopback
                                                             // 1.) The client and server see the same sequence of tokens
                                                             // 2.) The server enforces a unique exchange for each
                                                             //     non-anonymous authentication. (Replay detection)

#define SECPKG_ID_NONE      0xFFFF

//
// Extended Call Flags that currently contains
// Appcontainer related information about the caller.
// Packages can query for these
// via an LsaFunction GetExtendedCallFlags
//

#define SECPKG_CALLFLAGS_APPCONTAINER                   0x00000001
#define SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE       0x00000002
#define SECPKG_CALLFLAGS_FORCE_SUPPLIED                 0x00000004
#define SECPKG_CALLFLAGS_APPCONTAINER_UPNCAPABLE        0x00000008

//
// SecBuffer
//
//  Generic memory descriptors for buffers passed in to the security
//  API
//

typedef struct _SecBuffer {
    unsigned long cbBuffer;             // Size of the buffer, in bytes
    unsigned long BufferType;           // Type of the buffer (below)
#ifdef MIDL_PASS
    [size_is(cbBuffer)] char * pvBuffer;                         // Pointer to the buffer
#else
    _Field_size_bytes_(cbBuffer) void SEC_FAR * pvBuffer;            // Pointer to the buffer
#endif
} SecBuffer, * PSecBuffer;

typedef struct _SecBufferDesc {
    unsigned long ulVersion;            // Version number
    unsigned long cBuffers;             // Number of buffers
#ifdef MIDL_PASS
    [size_is(cBuffers)]
#endif
    _Field_size_(cBuffers) PSecBuffer pBuffers;                // Pointer to array of buffers
} SecBufferDesc, SEC_FAR * PSecBufferDesc;

#define SECBUFFER_VERSION           0

#define SECBUFFER_EMPTY             0   // Undefined, replaced by provider
#define SECBUFFER_DATA              1   // Packet data
#define SECBUFFER_TOKEN             2   // Security token
#define SECBUFFER_PKG_PARAMS        3   // Package specific parameters
#define SECBUFFER_MISSING           4   // Missing Data indicator
#define SECBUFFER_EXTRA             5   // Extra data
#define SECBUFFER_STREAM_TRAILER    6   // Security Trailer
#define SECBUFFER_STREAM_HEADER     7   // Security Header
#define SECBUFFER_NEGOTIATION_INFO  8   // Hints from the negotiation pkg
#define SECBUFFER_PADDING           9   // non-data padding
#define SECBUFFER_STREAM            10  // whole encrypted message
#define SECBUFFER_MECHLIST          11
#define SECBUFFER_MECHLIST_SIGNATURE 12
#define SECBUFFER_TARGET            13  // obsolete
#define SECBUFFER_CHANNEL_BINDINGS  14
#define SECBUFFER_CHANGE_PASS_RESPONSE 15
#define SECBUFFER_TARGET_HOST       16
#define SECBUFFER_ALERT             17
#define SECBUFFER_APPLICATION_PROTOCOLS 18  // Lists of application protocol IDs, one per negotiation extension
#define SECBUFFER_SRTP_PROTECTION_PROFILES      19  // List of SRTP protection profiles, in descending order of preference
#define SECBUFFER_SRTP_MASTER_KEY_IDENTIFIER    20  // SRTP master key identifier
#define SECBUFFER_TOKEN_BINDING                 21  // Supported Token Binding protocol version and key parameters
#define SECBUFFER_PRESHARED_KEY                 22  // Preshared key
#define SECBUFFER_PRESHARED_KEY_IDENTITY        23  // Preshared key identity
#define SECBUFFER_DTLS_MTU                      24  // DTLS path MTU setting
#define SECBUFFER_SEND_GENERIC_TLS_EXTENSION    25  // Buffer for sending generic TLS extensions.
#define SECBUFFER_SUBSCRIBE_GENERIC_TLS_EXTENSION 26 // Buffer for subscribing to generic TLS extensions.
#define SECBUFFER_FLAGS                         27  // ISC/ASC REQ Flags
#define SECBUFFER_TRAFFIC_SECRETS               28  // Message sequence lengths and corresponding traffic secrets.
#define SECBUFFER_CERTIFICATE_REQUEST_CONTEXT   29  // TLS 1.3 certificate request context.
#define SECBUFFER_CHANNEL_BINDINGS_RESULT       30  // Output buffer for Channel Bindings Audit
#define SECBUFFER_APP_SESSION_STATE             31  // Application state associated with the TLS 1.3+ session ticket. Server only.
#define SECBUFFER_SESSION_TICKET                32  // TLS 1.3+ session ticket. Client only.

#define SECBUFFER_ATTRMASK                      0xF0000000
#define SECBUFFER_READONLY                      0x80000000  // Buffer is read-only, no checksum
#define SECBUFFER_READONLY_WITH_CHECKSUM        0x10000000  // Buffer is read-only, and checksummed
#define SECBUFFER_RESERVED                      0x60000000  // Flags reserved to security system


typedef struct _SEC_NEGOTIATION_INFO {
    unsigned long       Size;           // Size of this structure
    unsigned long       NameLength;     // Length of name hint
    SEC_WCHAR * Name;           // Name hint
    void *      Reserved;       // Reserved
} SEC_NEGOTIATION_INFO, * PSEC_NEGOTIATION_INFO ;

typedef struct _SEC_CHANNEL_BINDINGS {
    unsigned long  dwInitiatorAddrType;
    unsigned long  cbInitiatorLength;
    unsigned long  dwInitiatorOffset;
    unsigned long  dwAcceptorAddrType;
    unsigned long  cbAcceptorLength;
    unsigned long  dwAcceptorOffset;
    unsigned long  cbApplicationDataLength;
    unsigned long  dwApplicationDataOffset;
} SEC_CHANNEL_BINDINGS, * PSEC_CHANNEL_BINDINGS ;

#define SEC_CHANNEL_BINDINGS_EX_MAGIC 'XEBC'

typedef struct _SEC_CHANNEL_BINDINGS_EX {
    unsigned long magicNumber; // contains SEC_CHANNEL_BINDINGS_VERSION_2.  distinguish ex buffer from normal channel bindings buffer. Shouldn't collide with any of assigned dwInitiatorAddrType values
    unsigned long flags; // audit flag is set to indicate if audit is needed 
    unsigned long cbHeaderLength;
    unsigned long cbStructureLength;
    unsigned long dwInitiatorAddrType;
    unsigned long cbInitiatorLength;
    unsigned long dwInitiatorOffset;
    unsigned long dwAcceptorAddrType;
    unsigned long cbAcceptorLength;
    unsigned long dwAcceptorOffset;
    unsigned long cbApplicationDataLength;
    unsigned long dwApplicationDataOffset;
} SEC_CHANNEL_BINDINGS_EX, * PSEC_CHANNEL_BINDINGS_EX ;

#define SEC_CHANNEL_BINDINGS_AUDIT_BINDINGS 0x1

#define SEC_CHANNEL_BINDINGS_VALID_FLAGS SEC_CHANNEL_BINDINGS_AUDIT_BINDINGS

typedef struct _SEC_CHANNEL_BINDINGS_RESULT {
    unsigned long flags;
}SEC_CHANNEL_BINDINGS_RESULT, *PSEC_CHANNEL_BINDINGS_RESULT;

#define SEC_CHANNEL_BINDINGS_RESULT_CLIENT_SUPPORT 0x1 // Auth package indicates client versions should support bindings
#define SEC_CHANNEL_BINDINGS_RESULT_ABSENT 0x2 // The bindings are omitted or all zeroes
#define SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISMATCH 0x4 // The channel binding hash was incorrect
#define SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISSING 0x8 // Missing channel bindings are not allowed for this client
#define SEC_CHANNEL_BINDINGS_RESULT_VALID_MATCHED 0x10 // The client and server bindings match
#define SEC_CHANNEL_BINDINGS_RESULT_VALID_PROXY 0x20 // ASC_REQ_PROXY_BINDINGS required the client to provide a binding
#define SEC_CHANNEL_BINDINGS_RESULT_VALID_MISSING 0x40 // Client permitted by ASC_REQ_ALLOW_MISSING_BINDINGS

#define SEC_CHANNEL_BINDINGS_RESULT_VALID (SEC_CHANNEL_BINDINGS_RESULT_VALID_MATCHED | SEC_CHANNEL_BINDINGS_RESULT_VALID_PROXY | \
    SEC_CHANNEL_BINDINGS_RESULT_VALID_MISSING)
    
#define SEC_CHANNEL_BINDINGS_RESULT_NOTVALID (SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISMATCH | SEC_CHANNEL_BINDINGS_RESULT_NOTVALID_MISSING)

typedef enum _SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT
{
    SecApplicationProtocolNegotiationExt_None,
    SecApplicationProtocolNegotiationExt_NPN,
    SecApplicationProtocolNegotiationExt_ALPN
} SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT, *PSEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT;

typedef struct _SEC_APPLICATION_PROTOCOL_LIST {
    SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT ProtoNegoExt; // Protocol negotiation extension type to use with this list of protocols
    unsigned short ProtocolListSize;                       // Size in bytes of the protocol ID list
    unsigned char ProtocolList[ANYSIZE_ARRAY];             // 8-bit length-prefixed application protocol IDs, most preferred first
} SEC_APPLICATION_PROTOCOL_LIST, *PSEC_APPLICATION_PROTOCOL_LIST;

typedef struct _SEC_APPLICATION_PROTOCOLS {
    unsigned long ProtocolListsSize;                            // Size in bytes of the protocol ID lists array
    SEC_APPLICATION_PROTOCOL_LIST ProtocolLists[ANYSIZE_ARRAY]; // Array of protocol ID lists
} SEC_APPLICATION_PROTOCOLS, *PSEC_APPLICATION_PROTOCOLS;


typedef struct _SEC_SRTP_PROTECTION_PROFILES {
    unsigned short ProfilesSize;                // Size in bytes of the SRTP protection profiles array
    unsigned short ProfilesList[ANYSIZE_ARRAY]; // Array of SRTP protection profiles
} SEC_SRTP_PROTECTION_PROFILES, *PSEC_SRTP_PROTECTION_PROFILES;

typedef struct _SEC_SRTP_MASTER_KEY_IDENTIFIER {
    unsigned char MasterKeyIdentifierSize;             // Size in bytes of the SRTP master key identifier
    unsigned char MasterKeyIdentifier[ANYSIZE_ARRAY];  // SRTP master key identifier
} SEC_SRTP_MASTER_KEY_IDENTIFIER, *PSEC_SRTP_MASTER_KEY_IDENTIFIER;


typedef struct _SEC_TOKEN_BINDING {
    unsigned char MajorVersion;                 // Supported major version of the Token Binding protocol
    unsigned char MinorVersion;                 // Supported minor version of the Token Binding protocol
    unsigned short KeyParametersSize;           // Size in bytes of the Token Binding key parameter IDs array
    unsigned char KeyParameters[ANYSIZE_ARRAY]; // Token Binding key parameter IDs, most preferred first
} SEC_TOKEN_BINDING, *PSEC_TOKEN_BINDING;

typedef struct _SEC_PRESHAREDKEY {
    unsigned short KeySize;                     // Size in bytes of the PSK
    unsigned char  Key[ANYSIZE_ARRAY];          // PSK
} SEC_PRESHAREDKEY, *PSEC_PRESHAREDKEY;

typedef struct _SEC_PRESHAREDKEY_IDENTITY {
    unsigned short KeyIdentitySize;             // Size in bytes of the PSK Identity
    unsigned char  KeyIdentity[ANYSIZE_ARRAY];  // PSK Identity
} SEC_PRESHAREDKEY_IDENTITY, *PSEC_PRESHAREDKEY_IDENTITY;

typedef struct _SEC_DTLS_MTU {
    unsigned short PathMTU;                     // Path MTU for the connection
} SEC_DTLS_MTU, *PSEC_DTLS_MTU;

typedef struct _SEC_FLAGS {
    unsigned long long Flags; // The caller sets ISC/ASC REQ flags; the lower 32 bits are reserved, must be set to 0.
} SEC_FLAGS, *PSEC_FLAGS;

typedef struct _SEC_CERTIFICATE_REQUEST_CONTEXT {
    unsigned char cbCertificateRequestContext; // Size in bytes of the rgCertificateRequestContext array.
    unsigned char rgCertificateRequestContext[ANYSIZE_ARRAY]; // The TLS 1.3 certificate request context.
} SEC_CERTIFICATE_REQUEST_CONTEXT, *PSEC_CERTIFICATE_REQUEST_CONTEXT;

typedef struct _SEC_APP_SESSION_STATE {
    unsigned short AppSessionStateSize; // Size in bytes of the application state, up to 2048 bytes.
    unsigned char  AppSessionState[ANYSIZE_ARRAY]; // Application state to be associated with the session ticket.
} SEC_APP_SESSION_STATE, *PSEC_APP_SESSION_STATE;

typedef struct _SEC_SESSION_TICKET {
    unsigned short SessionTicketSize; // Size in bytes of the session ticket.
    unsigned char  SessionTicket[ANYSIZE_ARRAY]; // TLS 1.3+ session ticket.
} SEC_SESSION_TICKET, *PSEC_SESSION_TICKET;


//
//  Traffic secret types:
//
typedef enum _SEC_TRAFFIC_SECRET_TYPE
{
    SecTrafficSecret_None,
    SecTrafficSecret_Client,
    SecTrafficSecret_Server
} SEC_TRAFFIC_SECRET_TYPE, *PSEC_TRAFFIC_SECRET_TYPE;

#define SZ_ALG_MAX_SIZE 64

typedef struct _SEC_TRAFFIC_SECRETS {
    wchar_t SymmetricAlgId[SZ_ALG_MAX_SIZE];     // Negotiated symmetric key algorithm. e.g. BCRYPT_AES_ALGORITHM.
    wchar_t ChainingMode[SZ_ALG_MAX_SIZE];       // Negotiated symmetric key algorithm chaining mode. e.g. BCRYPT_CHAIN_MODE_GCM or BCRYPT_CHAIN_MODE_CCM.
    wchar_t HashAlgId[SZ_ALG_MAX_SIZE];          // Negotiated hash algorithm. e.g. BCRYPT_SHA256_ALGORITHM or BCRYPT_SHA384_ALGORITHM.
    unsigned short KeySize;                      // Size in bytes of the symmetric key to derive from this traffic secret.
    unsigned short IvSize;                       // Size in bytes of the IV to derive from this traffic secret.
    unsigned short MsgSequenceStart;             // Offset of the first byte of the TLS message sequence to be protected with a key derived from TrafficSecret. Zero to indicate the first byte of the buffer.
    unsigned short MsgSequenceEnd;               // Offset of the last byte of the TLS message sequence to be protected with a key derived from TrafficSecret. Zero if the secret is for the encryption of application data or decryption of incoming records.
    SEC_TRAFFIC_SECRET_TYPE TrafficSecretType;   // Type of traffic secret from the TRAFFIC_SECRET_TYPE enumeration.
    unsigned short TrafficSecretSize;            // Size in bytes of the traffic secret.
    unsigned char  TrafficSecret[ANYSIZE_ARRAY]; // Traffic secret of type TrafficSecretType, TrafficSecretSize bytes long, used to derive write key and IV for message protection.
} SEC_TRAFFIC_SECRETS, *PSEC_TRAFFIC_SECRETS;


//
//  Data Representation Constant:
//
#define SECURITY_NATIVE_DREP        0x00000010
#define SECURITY_NETWORK_DREP       0x00000000

//
//  Credential Use Flags
//
#define SECPKG_CRED_INBOUND         0x00000001
#define SECPKG_CRED_OUTBOUND        0x00000002
#define SECPKG_CRED_BOTH            0x00000003
#define SECPKG_CRED_DEFAULT         0x00000004
#define SECPKG_CRED_RESERVED        0xF0000000

//
//  SSP SHOULD prompt the user for credentials/consent, independent
//  of whether credentials to be used are the 'logged on' credentials
//  or retrieved from credman.
//
//  An SSP may choose not to prompt, however, in circumstances determined
//  by the SSP.
//

#define SECPKG_CRED_AUTOLOGON_RESTRICTED    0x00000010

//
// auth will always fail, ISC() is called to process policy data only
//

#define SECPKG_CRED_PROCESS_POLICY_ONLY     0x00000020

// Enables configuring Kerberos to only request tickets from a specific DC version

#define SECPKG_CRED_KERB_ANCHOR_DS_VERSION  0x00000040


//
//  InitializeSecurityContext Requirement and return flags:
//

#define ISC_REQ_DELEGATE                0x00000001
#define ISC_REQ_MUTUAL_AUTH             0x00000002
#define ISC_REQ_REPLAY_DETECT           0x00000004
#define ISC_REQ_SEQUENCE_DETECT         0x00000008
#define ISC_REQ_CONFIDENTIALITY         0x00000010
#define ISC_REQ_USE_SESSION_KEY         0x00000020
#define ISC_REQ_PROMPT_FOR_CREDS        0x00000040
#define ISC_REQ_USE_SUPPLIED_CREDS      0x00000080
#define ISC_REQ_ALLOCATE_MEMORY         0x00000100
#define ISC_REQ_USE_DCE_STYLE           0x00000200
#define ISC_REQ_DATAGRAM                0x00000400
#define ISC_REQ_CONNECTION              0x00000800
#define ISC_REQ_CALL_LEVEL              0x00001000
#define ISC_REQ_FRAGMENT_SUPPLIED       0x00002000
#define ISC_REQ_EXTENDED_ERROR          0x00004000
#define ISC_REQ_STREAM                  0x00008000
#define ISC_REQ_INTEGRITY               0x00010000
#define ISC_REQ_IDENTIFY                0x00020000
#define ISC_REQ_NULL_SESSION            0x00040000
#define ISC_REQ_MANUAL_CRED_VALIDATION  0x00080000
#define ISC_REQ_RESERVED1               0x00100000
#define ISC_REQ_FRAGMENT_TO_FIT         0x00200000
// This exists only in Windows Vista and greater
#define ISC_REQ_FORWARD_CREDENTIALS     0x00400000
#define ISC_REQ_NO_INTEGRITY            0x00800000 // honored only by SPNEGO
#define ISC_REQ_USE_HTTP_STYLE          0x01000000
#define ISC_REQ_UNVERIFIED_TARGET_NAME  0x20000000
#define ISC_REQ_CONFIDENTIALITY_ONLY    0x40000000 // honored by SPNEGO/Kerberos
#define ISC_REQ_MESSAGES                 0x0000000100000000 // Disables the TLS 1.3+ record layer and causes the security context to consume and produce cleartext TLS messages, rather than records.
// Request that schannel perform server cert chain validation without failing the handshake on errors (deferred),
// same as SCH_CRED_DEFERRED_CRED_VALIDATION except applies to context not credential handle.
#define ISC_REQ_DEFERRED_CRED_VALIDATION 0x0000000200000000
// Prevents the client sending the post_handshake_auth extension in the TLS 1.3 Client Hello.
#define ISC_REQ_NO_POST_HANDSHAKE_AUTH   0x0000000400000000
// Request TLS 1.3+ session ticket reuse. Passive observers may be able to track the TLS client across networks.
#define ISC_REQ_REUSE_SESSION_TICKETS    0x0000000800000000
// Request explicit TLS 1.3+ session management. Received TLS 1.3+ session tickets will be returned to the SSPI caller.
// The SSPI caller will specify the session ticket to send in each resumption attempt.
#define ISC_REQ_EXPLICIT_SESSION         0x0000001000000000

#define ISC_RET_DELEGATE                0x00000001
#define ISC_RET_MUTUAL_AUTH             0x00000002
#define ISC_RET_REPLAY_DETECT           0x00000004
#define ISC_RET_SEQUENCE_DETECT         0x00000008
#define ISC_RET_CONFIDENTIALITY         0x00000010
#define ISC_RET_USE_SESSION_KEY         0x00000020
#define ISC_RET_USED_COLLECTED_CREDS    0x00000040
#define ISC_RET_USED_SUPPLIED_CREDS     0x00000080
#define ISC_RET_ALLOCATED_MEMORY        0x00000100
#define ISC_RET_USED_DCE_STYLE          0x00000200
#define ISC_RET_DATAGRAM                0x00000400
#define ISC_RET_CONNECTION              0x00000800
#define ISC_RET_INTERMEDIATE_RETURN     0x00001000
#define ISC_RET_CALL_LEVEL              0x00002000
#define ISC_RET_EXTENDED_ERROR          0x00004000
#define ISC_RET_STREAM                  0x00008000
#define ISC_RET_INTEGRITY               0x00010000
#define ISC_RET_IDENTIFY                0x00020000
#define ISC_RET_NULL_SESSION            0x00040000
#define ISC_RET_MANUAL_CRED_VALIDATION  0x00080000
#define ISC_RET_RESERVED1               0x00100000
#define ISC_RET_FRAGMENT_ONLY           0x00200000
// This exists only in Windows Vista and greater
#define ISC_RET_FORWARD_CREDENTIALS     0x00400000

#define ISC_RET_USED_HTTP_STYLE         0x01000000
#define ISC_RET_NO_ADDITIONAL_TOKEN     0x02000000 // *INTERNAL*
#define ISC_RET_REAUTHENTICATION        0x08000000 // *INTERNAL*
#define ISC_RET_CONFIDENTIALITY_ONLY    0x40000000 // honored by SPNEGO/Kerberos
#define ISC_RET_MESSAGES                 0x0000000100000000 // Indicates that the TLS 1.3+ record layer is disabled, and the security context consumes and produces cleartext TLS messages, rather than records.
#define ISC_RET_DEFERRED_CRED_VALIDATION 0x0000000200000000 // Indicates that SCH_CRED_DEFERRED_CRED_VALIDATION/ISC_REQ_DEFERRED_CRED_VALIDATION request will be honored.
#define ISC_RET_NO_POST_HANDSHAKE_AUTH   0x0000000400000000 // Indicates that the TLS 1.3 Client Hello will not contain the post_handshake_auth extension.
#define ISC_RET_REUSE_SESSION_TICKETS    0x0000000800000000 // Indicates that the TLS 1.3+ client may reuse session tickets.
#define ISC_RET_EXPLICIT_SESSION         0x0000001000000000 // Indicates that explicit TLS 1.3+ session management is enabled.

#define ASC_REQ_DELEGATE                0x00000001
#define ASC_REQ_MUTUAL_AUTH             0x00000002
#define ASC_REQ_REPLAY_DETECT           0x00000004
#define ASC_REQ_SEQUENCE_DETECT         0x00000008
#define ASC_REQ_CONFIDENTIALITY         0x00000010
#define ASC_REQ_USE_SESSION_KEY         0x00000020
#define ASC_REQ_SESSION_TICKET          0x00000040
#define ASC_REQ_ALLOCATE_MEMORY         0x00000100
#define ASC_REQ_USE_DCE_STYLE           0x00000200
#define ASC_REQ_DATAGRAM                0x00000400
#define ASC_REQ_CONNECTION              0x00000800
#define ASC_REQ_CALL_LEVEL              0x00001000
#define ASC_REQ_FRAGMENT_SUPPLIED       0x00002000
#define ASC_REQ_EXTENDED_ERROR          0x00008000
#define ASC_REQ_STREAM                  0x00010000
#define ASC_REQ_INTEGRITY               0x00020000
#define ASC_REQ_LICENSING               0x00040000
#define ASC_REQ_IDENTIFY                0x00080000
#define ASC_REQ_ALLOW_NULL_SESSION      0x00100000
#define ASC_REQ_ALLOW_NON_USER_LOGONS   0x00200000
#define ASC_REQ_ALLOW_CONTEXT_REPLAY    0x00400000
#define ASC_REQ_FRAGMENT_TO_FIT         0x00800000

#define ASC_REQ_NO_TOKEN                0x01000000
#define ASC_REQ_PROXY_BINDINGS          0x04000000
//      SSP_RET_REAUTHENTICATION        0x08000000  // *INTERNAL*
#define ASC_REQ_ALLOW_MISSING_BINDINGS  0x10000000
#define ASC_REQ_MESSAGES                0x0000000100000000 // Disables the TLS 1.3+ record layer and causes the security context to consume and produce cleartext TLS messages, rather than records.
// Request explicit TLS 1.3+ session management. TLS 1.3+ session ticket will only be generated when requested by the SSPI caller.
#define ASC_REQ_EXPLICIT_SESSION        0x0000001000000000

#define ASC_RET_DELEGATE                0x00000001
#define ASC_RET_MUTUAL_AUTH             0x00000002
#define ASC_RET_REPLAY_DETECT           0x00000004
#define ASC_RET_SEQUENCE_DETECT         0x00000008
#define ASC_RET_CONFIDENTIALITY         0x00000010
#define ASC_RET_USE_SESSION_KEY         0x00000020
#define ASC_RET_SESSION_TICKET          0x00000040
#define ASC_RET_ALLOCATED_MEMORY        0x00000100
#define ASC_RET_USED_DCE_STYLE          0x00000200
#define ASC_RET_DATAGRAM                0x00000400
#define ASC_RET_CONNECTION              0x00000800
#define ASC_RET_CALL_LEVEL              0x00002000 // skipped 1000 to be like ISC_
#define ASC_RET_THIRD_LEG_FAILED        0x00004000
#define ASC_RET_EXTENDED_ERROR          0x00008000
#define ASC_RET_STREAM                  0x00010000
#define ASC_RET_INTEGRITY               0x00020000
#define ASC_RET_LICENSING               0x00040000
#define ASC_RET_IDENTIFY                0x00080000
#define ASC_RET_NULL_SESSION            0x00100000
#define ASC_RET_ALLOW_NON_USER_LOGONS   0x00200000
#define ASC_RET_ALLOW_CONTEXT_REPLAY    0x00400000  // deprecated - don't use this flag!!!
#define ASC_RET_FRAGMENT_ONLY           0x00800000
#define ASC_RET_NO_TOKEN                0x01000000
#define ASC_RET_NO_ADDITIONAL_TOKEN     0x02000000  // *INTERNAL*
//      SSP_RET_REAUTHENTICATION        0x08000000  // *INTERNAL*
#define ASC_RET_MESSAGES                0x0000000100000000 // Indicates that the TLS 1.3+ record layer is disabled, and the security context consumes and produces cleartext TLS messages, rather than records.
#define ASC_RET_REUSE_SESSION_TICKETS   0x0000000800000000 // Indicates that the TLS 1.3+ server will allow session ticket reuse.
#define ASC_RET_EXPLICIT_SESSION        0x0000001000000000 // Indicates that explicit TLS 1.3+ session management is enabled.

//
//  Security Credentials Attributes:
//

#define SECPKG_CRED_ATTR_NAMES        1
#define SECPKG_CRED_ATTR_SSI_PROVIDER 2
#define SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS     3 // aliases SECPKG_CRED_ATTR_KDC_NETWORK_SETTINGS
#define SECPKG_CRED_ATTR_KDC_NETWORK_SETTINGS   3 // aliases SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS
#define SECPKG_CRED_ATTR_CERT         4
#define SECPKG_CRED_ATTR_PAC_BYPASS   5

typedef struct _SecPkgCredentials_NamesW
{
#ifdef MIDL_PASS
    [string]
#endif
    SEC_WCHAR * sUserName;

} SecPkgCredentials_NamesW, * PSecPkgCredentials_NamesW;

// end_ntifs

typedef struct _SecPkgCredentials_NamesA
{
#ifdef MIDL_PASS
    [string]
#endif
    SEC_CHAR * sUserName;

} SecPkgCredentials_NamesA, * PSecPkgCredentials_NamesA;

#ifdef UNICODE
#  define SecPkgCredentials_Names SecPkgCredentials_NamesW      // ntifs
#  define PSecPkgCredentials_Names PSecPkgCredentials_NamesW    // ntifs
#else
#  define SecPkgCredentials_Names SecPkgCredentials_NamesA
#  define PSecPkgCredentials_Names PSecPkgCredentials_NamesA
#endif // !UNICODE

// begin_ntifs

#if NTDDI_VERSION > NTDDI_WS03
typedef struct _SecPkgCredentials_SSIProviderW
{
    SEC_WCHAR * sProviderName;
    unsigned long       ProviderInfoLength;
    char *      ProviderInfo;
} SecPkgCredentials_SSIProviderW, * PSecPkgCredentials_SSIProviderW;
#endif // End W2k3SP1 and greater
// end_ntifs

typedef struct _SecPkgCredentials_SSIProviderA
{
    SEC_CHAR  * sProviderName;
    unsigned long       ProviderInfoLength;
    char *      ProviderInfo;
} SecPkgCredentials_SSIProviderA, * PSecPkgCredentials_SSIProviderA;

#ifdef UNICODE
#  define SecPkgCredentials_SSIProvider SecPkgCredentials_SSIProviderW      // ntifs
#  define PSecPkgCredentials_SSIProvider PSecPkgCredentials_SSIProviderW    // ntifs
#else
#  define SecPkgCredentials_SSIProvider SecPkgCredentials_SSIProviderA
#  define PSecPkgCredentials_SSIProvider PSecPkgCredentials_SSIProviderA
#endif // !UNICODE

// begin_ntifs

#define KDC_PROXY_SETTINGS_V1                 1
#define KDC_NETWORK_SETTINGS_V2               2

#define KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY     0x1

#define KDC_NETWORK_SETTINGS_FLAGS_FORCEPROXY             0x1
#define KDC_NETWORK_SETTINGS_FLAGS_CONFIGURE_PROXY        0x80000000
#define KDC_NETWORK_SETTINGS_FLAGS_CONFIGURE_DISCOVERY    0x40000000

#define KDC_NETWORK_DISCOVERY_FLAGS_DS13_REQUIRED      0x80000000

typedef struct _SecPkgCredentials_KdcProxySettingsW
{
    ULONG  Version;             // KDC_PROXY_SETTINGS_V1
    ULONG  Flags;               // KDC_PROXY_SETTINGS_FLAGS_*
    USHORT ProxyServerOffset;   // ProxyServer, optional
    USHORT ProxyServerLength;
    USHORT ClientTlsCredOffset; // ClientTlsCred, optional
    USHORT ClientTlsCredLength;
} SecPkgCredentials_KdcProxySettingsW, *PSecPkgCredentials_KdcProxySettingsW;

typedef struct _SecPkgCredentials_KdcNetworkSettingsW
{
    ULONG   Version;             // KDC_NETWORK_SETTINGS_V2
    ULONG   Flags;               // KDC_NETWORK_SETTINGS_FLAGS_*
    USHORT  ProxyServerOffset;   // ProxyServer, optional
    USHORT  ProxyServerLength;
    USHORT  ClientTlsCredOffset; // ClientTlsCred, optional
    USHORT  ClientTlsCredLength;
    ULONG   DcDiscoveryFlags;    // KDC_NETWORK_DISCOVERY_*
} SecPkgCredentials_KdcNetworkSettingsW, *PSecPkgCredentials_KdcNetworkSettingsW;

// end_ntifs

// begin_ntifs

typedef struct _SecPkgCredentials_Cert
{
    unsigned long  EncodedCertSize;
    unsigned char* EncodedCert;
} SecPkgCredentials_Cert, * PSecPkgCredentials_Cert;

// end_ntifs

// begin_ntifs

//
//  Security Context Attributes:
//

#define SECPKG_ATTR_SIZES           0
#define SECPKG_ATTR_NAMES           1
#define SECPKG_ATTR_LIFESPAN        2
#define SECPKG_ATTR_DCE_INFO        3
#define SECPKG_ATTR_STREAM_SIZES    4
#define SECPKG_ATTR_KEY_INFO        5
#define SECPKG_ATTR_AUTHORITY       6
#define SECPKG_ATTR_PROTO_INFO      7
#define SECPKG_ATTR_PASSWORD_EXPIRY 8
#define SECPKG_ATTR_SESSION_KEY     9
#define SECPKG_ATTR_PACKAGE_INFO    10
#define SECPKG_ATTR_USER_FLAGS      11
#define SECPKG_ATTR_NEGOTIATION_INFO 12
#define SECPKG_ATTR_NATIVE_NAMES    13
#define SECPKG_ATTR_FLAGS           14
// These attributes exist only in Win XP and greater
#define SECPKG_ATTR_USE_VALIDATED   15
#define SECPKG_ATTR_CREDENTIAL_NAME 16
#define SECPKG_ATTR_TARGET_INFORMATION 17
#define SECPKG_ATTR_ACCESS_TOKEN    18
// These attributes exist only in Win2K3 and greater
#define SECPKG_ATTR_TARGET          19
#define SECPKG_ATTR_AUTHENTICATION_ID  20
// These attributes exist only in Win2K3SP1 and greater
#define SECPKG_ATTR_LOGOFF_TIME     21
//
// win7 or greater
//
#define SECPKG_ATTR_NEGO_KEYS         22
#define SECPKG_ATTR_PROMPTING_NEEDED  24
#define SECPKG_ATTR_UNIQUE_BINDINGS   25
#define SECPKG_ATTR_ENDPOINT_BINDINGS 26
#define SECPKG_ATTR_CLIENT_SPECIFIED_TARGET 27

#define SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS 30
#define SECPKG_ATTR_NEGO_PKG_INFO        31 // contains nego info of packages
#define SECPKG_ATTR_NEGO_STATUS          32 // contains the last error
#define SECPKG_ATTR_CONTEXT_DELETED      33 // a context has been deleted

//
// win8 or greater
//
#define SECPKG_ATTR_DTLS_MTU        34
#define SECPKG_ATTR_DATAGRAM_SIZES  SECPKG_ATTR_STREAM_SIZES

#define SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES 128

//
// win8.1 or greater
//
#define SECPKG_ATTR_APPLICATION_PROTOCOL 35

//
// win10 or greater
//
#define SECPKG_ATTR_NEGOTIATED_TLS_EXTENSIONS 36
#define SECPKG_ATTR_IS_LOOPBACK 37  // indicates authentication to localhost

typedef struct _SecPkgContext_SubjectAttributes {
    void* AttributeInfo; // contains a PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION structure
} SecPkgContext_SubjectAttributes, *PSecPkgContext_SubjectAttributes;

#define SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS 0x1
#define SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM     0x2

//
// types of credentials, used by SECPKG_ATTR_PROMPTING_NEEDED
//

typedef enum _SECPKG_CRED_CLASS {
    SecPkgCredClass_None = 0,  // no creds
    SecPkgCredClass_Ephemeral = 10,  // logon creds
    SecPkgCredClass_PersistedGeneric = 20, // saved creds, not target specific
    SecPkgCredClass_PersistedSpecific = 30, // saved creds, target specific
    SecPkgCredClass_Explicit = 40, // explicitly supplied creds
} SECPKG_CRED_CLASS, * PSECPKG_CRED_CLASS;

typedef struct _SecPkgContext_CredInfo {
    SECPKG_CRED_CLASS CredClass;
    unsigned long IsPromptingNeeded;
} SecPkgContext_CredInfo, *PSecPkgContext_CredInfo;

typedef struct _SecPkgContext_NegoPackageInfo
{
    unsigned long PackageMask;
} SecPkgContext_NegoPackageInfo, * PSecPkgContext_NegoPackageInfo;

typedef struct _SecPkgContext_NegoStatus
{
    unsigned long LastStatus;
} SecPkgContext_NegoStatus, * PSecPkgContext_NegoStatus;

typedef struct _SecPkgContext_Sizes
{
    unsigned long cbMaxToken;
    unsigned long cbMaxSignature;
    unsigned long cbBlockSize;
    unsigned long cbSecurityTrailer;
} SecPkgContext_Sizes, * PSecPkgContext_Sizes;

typedef struct _SecPkgContext_StreamSizes
{
    unsigned long   cbHeader;
    unsigned long   cbTrailer;
    unsigned long   cbMaximumMessage;
    unsigned long   cBuffers;
    unsigned long   cbBlockSize;
} SecPkgContext_StreamSizes, * PSecPkgContext_StreamSizes;

typedef SecPkgContext_StreamSizes SecPkgContext_DatagramSizes;
typedef PSecPkgContext_StreamSizes PSecPkgContext_DatagramSizes;

typedef struct _SecPkgContext_NamesW
{
    SEC_WCHAR * sUserName;
} SecPkgContext_NamesW, * PSecPkgContext_NamesW;

// end_ntifs

typedef enum _SECPKG_ATTR_LCT_STATUS {
    SecPkgAttrLastClientTokenYes,
    SecPkgAttrLastClientTokenNo,
    SecPkgAttrLastClientTokenMaybe
} SECPKG_ATTR_LCT_STATUS, * PSECPKG_ATTR_LCT_STATUS;


typedef struct _SecPkgContext_LastClientTokenStatus {
    SECPKG_ATTR_LCT_STATUS LastClientTokenStatus;
} SecPkgContext_LastClientTokenStatus, * PSecPkgContext_LastClientTokenStatus;

typedef struct _SecPkgContext_NamesA
{
    SEC_CHAR * sUserName;
} SecPkgContext_NamesA, * PSecPkgContext_NamesA;

#ifdef UNICODE
#  define SecPkgContext_Names SecPkgContext_NamesW          // ntifs
#  define PSecPkgContext_Names PSecPkgContext_NamesW        // ntifs
#else
#  define SecPkgContext_Names SecPkgContext_NamesA
#  define PSecPkgContext_Names PSecPkgContext_NamesA
#endif // !UNICODE

// begin_ntifs

typedef struct _SecPkgContext_Lifespan
{
    TimeStamp tsStart;
    TimeStamp tsExpiry;
} SecPkgContext_Lifespan, * PSecPkgContext_Lifespan;

typedef struct _SecPkgContext_DceInfo
{
    unsigned long AuthzSvc;
    void * pPac;
} SecPkgContext_DceInfo, * PSecPkgContext_DceInfo;

// end_ntifs

typedef struct _SecPkgContext_KeyInfoA
{
    SEC_CHAR *  sSignatureAlgorithmName;
    SEC_CHAR *  sEncryptAlgorithmName;
    unsigned long       KeySize;
    unsigned long       SignatureAlgorithm;
    unsigned long       EncryptAlgorithm;
} SecPkgContext_KeyInfoA, * PSecPkgContext_KeyInfoA;

// begin_ntifs

typedef struct _SecPkgContext_KeyInfoW
{
    SEC_WCHAR * sSignatureAlgorithmName;
    SEC_WCHAR * sEncryptAlgorithmName;
    unsigned long       KeySize;
    unsigned long       SignatureAlgorithm;
    unsigned long       EncryptAlgorithm;
} SecPkgContext_KeyInfoW, * PSecPkgContext_KeyInfoW;

// end_ntifs

#ifdef UNICODE
#define SecPkgContext_KeyInfo   SecPkgContext_KeyInfoW      // ntifs
#define PSecPkgContext_KeyInfo  PSecPkgContext_KeyInfoW     // ntifs
#else
#define SecPkgContext_KeyInfo   SecPkgContext_KeyInfoA
#define PSecPkgContext_KeyInfo  PSecPkgContext_KeyInfoA
#endif

typedef struct _SecPkgContext_AuthorityA
{
    SEC_CHAR *  sAuthorityName;
} SecPkgContext_AuthorityA, * PSecPkgContext_AuthorityA;

// begin_ntifs

typedef struct _SecPkgContext_AuthorityW
{
    SEC_WCHAR * sAuthorityName;
} SecPkgContext_AuthorityW, * PSecPkgContext_AuthorityW;

// end_ntifs

#ifdef UNICODE
#define SecPkgContext_Authority SecPkgContext_AuthorityW        // ntifs
#define PSecPkgContext_Authority    PSecPkgContext_AuthorityW   // ntifs
#else
#define SecPkgContext_Authority SecPkgContext_AuthorityA
#define PSecPkgContext_Authority    PSecPkgContext_AuthorityA
#endif

typedef struct _SecPkgContext_ProtoInfoA
{
    SEC_CHAR *  sProtocolName;
    unsigned long       majorVersion;
    unsigned long       minorVersion;
} SecPkgContext_ProtoInfoA, * PSecPkgContext_ProtoInfoA;

// begin_ntifs

typedef struct _SecPkgContext_ProtoInfoW
{
    SEC_WCHAR * sProtocolName;
    unsigned long majorVersion;
    unsigned long minorVersion;
} SecPkgContext_ProtoInfoW, * PSecPkgContext_ProtoInfoW;

// end_ntifs

#ifdef UNICODE
#define SecPkgContext_ProtoInfo   SecPkgContext_ProtoInfoW      // ntifs
#define PSecPkgContext_ProtoInfo  PSecPkgContext_ProtoInfoW     // ntifs
#else
#define SecPkgContext_ProtoInfo   SecPkgContext_ProtoInfoA
#define PSecPkgContext_ProtoInfo  PSecPkgContext_ProtoInfoA
#endif

// begin_ntifs

typedef struct _SecPkgContext_PasswordExpiry
{
    TimeStamp tsPasswordExpires;
} SecPkgContext_PasswordExpiry, * PSecPkgContext_PasswordExpiry;

#if NTDDI_VERSION > NTDDI_WS03
typedef struct _SecPkgContext_LogoffTime
{
    TimeStamp tsLogoffTime;
} SecPkgContext_LogoffTime, * PSecPkgContext_LogoffTime;
#endif // Greater than Windows Server 2003 RTM (SP1 and greater contains this)

typedef struct _SecPkgContext_SessionKey
{
    unsigned long SessionKeyLength;
    _Field_size_bytes_(SessionKeyLength) unsigned char * SessionKey;
} SecPkgContext_SessionKey, *PSecPkgContext_SessionKey;

// used by nego2
typedef struct _SecPkgContext_NegoKeys
{
  unsigned long KeyType;
  unsigned short KeyLength;
  _Field_size_bytes_(KeyLength) unsigned char* KeyValue;
  unsigned long  VerifyKeyType;
  unsigned short VerifyKeyLength;
  _Field_size_bytes_(VerifyKeyLength) unsigned char* VerifyKeyValue;
} SecPkgContext_NegoKeys, * PSecPkgContext_NegoKeys;

typedef struct _SecPkgContext_PackageInfoW
{
    PSecPkgInfoW PackageInfo;
} SecPkgContext_PackageInfoW, * PSecPkgContext_PackageInfoW;

// end_ntifs

typedef struct _SecPkgContext_PackageInfoA
{
    PSecPkgInfoA PackageInfo;
} SecPkgContext_PackageInfoA, * PSecPkgContext_PackageInfoA;

// begin_ntifs

typedef struct _SecPkgContext_UserFlags
{
    unsigned long UserFlags;
} SecPkgContext_UserFlags, * PSecPkgContext_UserFlags;

typedef struct _SecPkgContext_Flags
{
    unsigned long Flags;
} SecPkgContext_Flags, * PSecPkgContext_Flags;

// end_ntifs

#ifdef UNICODE
#define SecPkgContext_PackageInfo   SecPkgContext_PackageInfoW      // ntifs
#define PSecPkgContext_PackageInfo  PSecPkgContext_PackageInfoW     // ntifs
#else
#define SecPkgContext_PackageInfo   SecPkgContext_PackageInfoA
#define PSecPkgContext_PackageInfo  PSecPkgContext_PackageInfoA
#endif


typedef struct _SecPkgContext_NegotiationInfoA
{
    PSecPkgInfoA    PackageInfo ;
    unsigned long   NegotiationState ;
} SecPkgContext_NegotiationInfoA, * PSecPkgContext_NegotiationInfoA ;

// begin_ntifs
typedef struct _SecPkgContext_NegotiationInfoW
{
    PSecPkgInfoW    PackageInfo ;
    unsigned long   NegotiationState ;
} SecPkgContext_NegotiationInfoW, * PSecPkgContext_NegotiationInfoW ;

// end_ntifs

#ifdef UNICODE
#define SecPkgContext_NegotiationInfo   SecPkgContext_NegotiationInfoW
#define PSecPkgContext_NegotiationInfo  PSecPkgContext_NegotiationInfoW
#else
#define SecPkgContext_NegotiationInfo   SecPkgContext_NegotiationInfoA
#define PSecPkgContext_NegotiationInfo  PSecPkgContext_NegotiationInfoA
#endif

#define SECPKG_NEGOTIATION_COMPLETE             0
#define SECPKG_NEGOTIATION_OPTIMISTIC           1
#define SECPKG_NEGOTIATION_IN_PROGRESS          2
#define SECPKG_NEGOTIATION_DIRECT               3
#define SECPKG_NEGOTIATION_TRY_MULTICRED        4


typedef struct _SecPkgContext_NativeNamesW
{
    SEC_WCHAR * sClientName;
    SEC_WCHAR * sServerName;
} SecPkgContext_NativeNamesW, * PSecPkgContext_NativeNamesW;

typedef struct _SecPkgContext_NativeNamesA
{
    SEC_CHAR * sClientName;
    SEC_CHAR * sServerName;
} SecPkgContext_NativeNamesA, * PSecPkgContext_NativeNamesA;


#ifdef UNICODE
#  define SecPkgContext_NativeNames SecPkgContext_NativeNamesW          // ntifs
#  define PSecPkgContext_NativeNames PSecPkgContext_NativeNamesW        // ntifs
#else
#  define SecPkgContext_NativeNames SecPkgContext_NativeNamesA
#  define PSecPkgContext_NativeNames PSecPkgContext_NativeNamesA
#endif // !UNICODE

// begin_ntifs

#if OSVER(NTDDI_VERSION) > NTDDI_WIN2K

typedef struct _SecPkgContext_CredentialNameW
{
    unsigned long CredentialType;
    SEC_WCHAR *sCredentialName;
} SecPkgContext_CredentialNameW, * PSecPkgContext_CredentialNameW;

#endif // Later than win2k
// end_ntifs

typedef struct _SecPkgContext_CredentialNameA
{
    unsigned long CredentialType;
    SEC_CHAR *sCredentialName;
} SecPkgContext_CredentialNameA, * PSecPkgContext_CredentialNameA;

#ifdef UNICODE
#  define SecPkgContext_CredentialName SecPkgContext_CredentialNameW          // ntifs
#  define PSecPkgContext_CredentialName PSecPkgContext_CredentialNameW        // ntifs
#else
#  define SecPkgContext_CredentialName SecPkgContext_CredentialNameA
#  define PSecPkgContext_CredentialName PSecPkgContext_CredentialNameA
#endif // !UNICODE

typedef struct _SecPkgContext_AccessToken
{
    void * AccessToken;
} SecPkgContext_AccessToken, * PSecPkgContext_AccessToken;

typedef struct _SecPkgContext_TargetInformation
{
    unsigned long MarshalledTargetInfoLength;
    unsigned char * MarshalledTargetInfo;

} SecPkgContext_TargetInformation, * PSecPkgContext_TargetInformation;

typedef struct _SecPkgContext_AuthzID
{
    unsigned long AuthzIDLength;
    char * AuthzID;

} SecPkgContext_AuthzID, * PSecPkgContext_AuthzID;

typedef struct _SecPkgContext_Target
{
    unsigned long TargetLength;
    char * Target;

} SecPkgContext_Target, * PSecPkgContext_Target;


typedef struct _SecPkgContext_ClientSpecifiedTarget
{
    SEC_WCHAR * sTargetName;
} SecPkgContext_ClientSpecifiedTarget, * PSecPkgContext_ClientSpecifiedTarget;

typedef struct _SecPkgContext_Bindings
{
    unsigned long BindingsLength;
    _Field_size_bytes_(BindingsLength) SEC_CHANNEL_BINDINGS * Bindings;
} SecPkgContext_Bindings, * PSecPkgContext_Bindings;

typedef enum _SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS
{
    SecApplicationProtocolNegotiationStatus_None,
    SecApplicationProtocolNegotiationStatus_Success,
    SecApplicationProtocolNegotiationStatus_SelectedClientOnly
} SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS, *PSEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS;

#define MAX_PROTOCOL_ID_SIZE 0xff

typedef struct _SecPkgContext_ApplicationProtocol
{
    SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS ProtoNegoStatus; // Application  protocol negotiation status
    SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT ProtoNegoExt;       // Protocol negotiation extension type corresponding to this protocol ID
    unsigned char ProtocolIdSize;                                // Size in bytes of the application protocol ID
    unsigned char ProtocolId[MAX_PROTOCOL_ID_SIZE];              // Byte string representing the negotiated application protocol ID
} SecPkgContext_ApplicationProtocol, *PSecPkgContext_ApplicationProtocol;

typedef struct _SecPkgContext_NegotiatedTlsExtensions
{
    unsigned long ExtensionsCount;                               // Number of negotiated TLS extensions.
    _Field_size_(ExtensionsCount) unsigned short * Extensions;   // Pointer to array of 2-byte TLS extension IDs (allocated by IANA).
} SecPkgContext_NegotiatedTlsExtensions, * PSecPkgContext_NegotiatedTlsExtensions;

typedef struct _SECPKG_APP_MODE_INFO {
    ULONG     UserFunction;
    ULONG_PTR Argument1;
    ULONG_PTR Argument2;
    SecBuffer UserData;
    BOOLEAN   ReturnToLsa;
} SECPKG_APP_MODE_INFO, *PSECPKG_APP_MODE_INFO;


// begin_ntifs

typedef void
(SEC_ENTRY * SEC_GET_KEY_FN) (
    void * Arg,                 // Argument passed in
    void * Principal,           // Principal ID
    unsigned long KeyVer,               // Key Version
    void * * Key,       // Returned ptr to key
    SECURITY_STATUS * Status    // returned status
    );

//
// Flags for ExportSecurityContext
//

#define SECPKG_CONTEXT_EXPORT_RESET_NEW         0x00000001      // New context is reset to initial state
#define SECPKG_CONTEXT_EXPORT_DELETE_OLD        0x00000002      // Old context is deleted during export
// This is only valid in W2K3SP1 and greater
#define SECPKG_CONTEXT_EXPORT_TO_KERNEL         0x00000004      // Context is to be transferred to the kernel


KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
AcquireCredentialsHandleW(
#if ISSP_MODE == 0     // For Kernel mode
    _In_opt_  PSECURITY_STRING pPrincipal,
    _In_      PSECURITY_STRING pPackage,
#else
    _In_opt_  LPWSTR pszPrincipal,                // Name of principal
    _In_      LPWSTR pszPackage,                  // Name of package
#endif
    _In_      unsigned long fCredentialUse,       // Flags indicating use
    _In_opt_  void * pvLogonId,                   // Pointer to logon ID
    _In_opt_  void * pAuthData,                   // Package specific data
    _In_opt_  SEC_GET_KEY_FN pGetKeyFn,           // Pointer to GetKey() func
    _In_opt_  void * pvGetKeyArgument,            // Value to pass to GetKey()
    _Out_     PCredHandle phCredential,           // (out) Cred Handle
    _Out_opt_ PTimeStamp ptsExpiry                // (out) Lifetime (optional)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ACQUIRE_CREDENTIALS_HANDLE_FN_W)(
#if ISSP_MODE == 0
    PSECURITY_STRING,
    PSECURITY_STRING,
#else
    SEC_WCHAR *,
    SEC_WCHAR *,
#endif
    unsigned long,
    void *,
    void *,
    SEC_GET_KEY_FN,
    void *,
    PCredHandle,
    PTimeStamp);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
AcquireCredentialsHandleA(
    _In_opt_  LPSTR pszPrincipal,                 // Name of principal
    _In_      LPSTR pszPackage,                   // Name of package
    _In_      unsigned long fCredentialUse,       // Flags indicating use
    _In_opt_  void * pvLogonId,                   // Pointer to logon ID
    _In_opt_  void * pAuthData,                   // Package specific data
    _In_opt_  SEC_GET_KEY_FN pGetKeyFn,           // Pointer to GetKey() func
    _In_opt_  void * pvGetKeyArgument,            // Value to pass to GetKey()
    _Out_     PCredHandle phCredential,           // (out) Cred Handle
    _Out_opt_ PTimeStamp ptsExpiry                // (out) Lifetime (optional)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ACQUIRE_CREDENTIALS_HANDLE_FN_A)(
    SEC_CHAR *,
    SEC_CHAR *,
    unsigned long,
    void *,
    void *,
    SEC_GET_KEY_FN,
    void *,
    PCredHandle,
    PTimeStamp);

#ifdef UNICODE
#  define AcquireCredentialsHandle AcquireCredentialsHandleW                  // ntifs
#  define ACQUIRE_CREDENTIALS_HANDLE_FN ACQUIRE_CREDENTIALS_HANDLE_FN_W       // ntifs
#else
#  define AcquireCredentialsHandle AcquireCredentialsHandleA
#  define ACQUIRE_CREDENTIALS_HANDLE_FN ACQUIRE_CREDENTIALS_HANDLE_FN_A
#endif // !UNICODE

// begin_ntifs

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
FreeCredentialsHandle(
    _In_ PCredHandle phCredential            // Handle to free
    );

typedef SECURITY_STATUS
(SEC_ENTRY * FREE_CREDENTIALS_HANDLE_FN)(
    PCredHandle );

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
AddCredentialsW(
    _In_      PCredHandle hCredentials,
#if ISSP_MODE == 0      // For Kernel mode
    _In_opt_  PSECURITY_STRING pPrincipal,
    _In_      PSECURITY_STRING pPackage,
#else
    _In_opt_  LPWSTR pszPrincipal,                // Name of principal
    _In_      LPWSTR pszPackage,                  // Name of package
#endif
    _In_      unsigned long fCredentialUse,       // Flags indicating use
    _In_opt_  void * pAuthData,           // Package specific data
    _In_opt_  SEC_GET_KEY_FN pGetKeyFn,           // Pointer to GetKey() func
    _In_opt_  void * pvGetKeyArgument,    // Value to pass to GetKey()
    _Out_opt_ PTimeStamp ptsExpiry                // (out) Lifetime (optional)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ADD_CREDENTIALS_FN_W)(
    PCredHandle,
#if ISSP_MODE == 0
    PSECURITY_STRING,
    PSECURITY_STRING,
#else
    SEC_WCHAR *,
    SEC_WCHAR *,
#endif
    unsigned long,
    void *,
    SEC_GET_KEY_FN,
    void *,
    PTimeStamp);

SECURITY_STATUS SEC_ENTRY
AddCredentialsA(
    _In_ PCredHandle hCredentials,
    _In_opt_ LPSTR pszPrincipal,             // Name of principal
    _In_ LPSTR pszPackage,                   // Name of package
    _In_ unsigned long fCredentialUse,       // Flags indicating use
    _In_opt_ void * pAuthData,           // Package specific data
    _In_opt_ SEC_GET_KEY_FN pGetKeyFn,           // Pointer to GetKey() func
    _In_opt_ void * pvGetKeyArgument,    // Value to pass to GetKey()
    _Out_opt_ PTimeStamp ptsExpiry                // (out) Lifetime (optional)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ADD_CREDENTIALS_FN_A)(
    PCredHandle,
    SEC_CHAR *,
    SEC_CHAR *,
    unsigned long,
    void *,
    SEC_GET_KEY_FN,
    void *,
    PTimeStamp);

#ifdef UNICODE
#define AddCredentials  AddCredentialsW
#define ADD_CREDENTIALS_FN  ADD_CREDENTIALS_FN_W
#else
#define AddCredentials  AddCredentialsA
#define ADD_CREDENTIALS_FN ADD_CREDENTIALS_FN_A
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

////////////////////////////////////////////////////////////////////////
///
/// Asynchronous interface. Kernel-only (for now).
///
////////////////////////////////////////////////////////////////////////

#if ISSP_MODE == 0      // For Kernel mode

typedef struct _SspiAsyncContext SspiAsyncContext;

//
// Callback used for notifying completion of an async SSPI call.
//
typedef void (*SspiAsyncNotifyCallback)(
    _In_     SspiAsyncContext* Handle,
    _In_opt_ PVOID CallbackData
);

//
// Return a newly initialized context.
//
SspiAsyncContext* SspiCreateAsyncContext();

//
// Free up a context.
//
void SspiFreeAsyncContext(
    _In_opt_ SspiAsyncContext* Handle
);

//
// Mark an async context for reuse. Only the context state is altered.
// Client notification info, such as callback, is left alone
//
// If the context is invalid or currently in use, an error will be returned.
//
NTSTATUS SspiReinitAsyncContext(
    _Inout_ SspiAsyncContext* Handle
);

//
// Installs a callback which will be notified on async call completion.
//
SECURITY_STATUS SspiSetAsyncNotifyCallback(
    _In_ SspiAsyncContext* Context,
    _In_ SspiAsyncNotifyCallback Callback,
    _In_opt_ void* CallbackData
);

//
// Determines if the given async context requires notification on completion
// of the call.
//
BOOLEAN SspiAsyncContextRequiresNotify(
    _In_ SspiAsyncContext* AsyncContext
);

//
// Gets the current status of an async call. Until the call is completed,
// status will be SEC_I_ASYNC_CALL_PENDING.
//
SECURITY_STATUS SspiGetAsyncCallStatus(
    _In_ SspiAsyncContext* Handle
);

SECURITY_STATUS SspiAcquireCredentialsHandleAsyncW(
    _Inout_   SspiAsyncContext* AsyncContext,
#if ISSP_MODE == 0
    _In_opt_  PSECURITY_STRING pszPrincipal,            // Name of principal
    _In_      PSECURITY_STRING pszPackage,              // Name of package
#else
    _In_opt_  LPWSTR pszPrincipal,                      // Name of principal
    _In_      LPWSTR pszPackage,                        // Name of package
#endif
    _In_      unsigned long fCredentialUse,             // Flags indicating use
    _In_opt_  void * pvLogonId,                         // Pointer to logon ID
    _In_opt_  void * pAuthData,                         // Package specific data
    _In_opt_  SEC_GET_KEY_FN pGetKeyFn,                 // Pointer to GetKey() func
    _In_opt_  void * pvGetKeyArgument,                  // Value to pass to GetKey()
    _In_      PCredHandle phCredential,                 // (out) Cred Handle
    _In_opt_  PTimeStamp ptsExpiry                      // (out) Lifetime (optional)
);

SECURITY_STATUS SspiAcquireCredentialsHandleAsyncA(
    _Inout_   SspiAsyncContext* AsyncContext,
    _In_opt_  LPSTR pszPrincipal,                       // Name of principal
    _In_      LPSTR pszPackage,                         // Name of package
    _In_      unsigned long fCredentialUse,             // Flags indicating use
    _In_opt_  void * pvLogonId,                         // Pointer to logon ID
    _In_opt_  void * pAuthData,                         // Package specific data
    _In_opt_  SEC_GET_KEY_FN pGetKeyFn,                 // Pointer to GetKey() func
    _In_opt_  void * pvGetKeyArgument,                  // Value to pass to GetKey()
    _In_      PCredHandle phCredential,                 // (out) Cred Handle
    _In_opt_  PTimeStamp ptsExpiry                      // (out) Lifetime (optional)
);

SECURITY_STATUS SspiInitializeSecurityContextAsyncW(
    _Inout_     SspiAsyncContext* AsyncContext,
    _In_opt_    PCredHandle phCredential,               // Cred to base context
    _In_opt_    PCtxtHandle phContext,                  // Existing context (OPT)
#if ISSP_MODE == 0
    _In_opt_    PSECURITY_STRING pszTargetName,         // Name of target
#else
    _In_opt_    LPWSTR pszTargetName,                   // Name of target
#endif
    _In_        unsigned long fContextReq,              // Context Requirements
    _In_        unsigned long Reserved1,                // Reserved, MBZ
    _In_        unsigned long TargetDataRep,            // Data rep of target
    _In_opt_    PSecBufferDesc pInput,                  // Input Buffers
    _In_        unsigned long Reserved2,                // Reserved, MBZ
    _In_opt_    PCtxtHandle phNewContext,               // (out) New Context handle
    _In_opt_    PSecBufferDesc pOutput,                 // (inout) Output Buffers
    _In_        unsigned long * pfContextAttr,          // (out) Context attrs
    _In_opt_    PTimeStamp ptsExpiry                    // (out) Life span (OPT)
);

SECURITY_STATUS SspiInitializeSecurityContextAsyncA(
    _Inout_     SspiAsyncContext* AsyncContext,
    _In_opt_    PCredHandle phCredential,               // Cred to base context
    _In_opt_    PCtxtHandle phContext,                  // Existing context (OPT)
    _In_opt_    LPSTR pszTargetName,                    // Name of target
    _In_        unsigned long fContextReq,              // Context Requirements
    _In_        unsigned long Reserved1,                // Reserved, MBZ
    _In_        unsigned long TargetDataRep,            // Data rep of target
    _In_opt_    PSecBufferDesc pInput,                  // Input Buffers
    _In_        unsigned long Reserved2,                // Reserved, MBZ
    _In_opt_    PCtxtHandle phNewContext,               // (out) New Context handle
    _In_opt_    PSecBufferDesc pOutput,                 // (inout) Output Buffers
    _In_        unsigned long * pfContextAttr,          // (out) Context attrs
    _In_opt_    PTimeStamp ptsExpiry                    // (out) Life span (OPT)
);

SECURITY_STATUS SspiAcceptSecurityContextAsync(
    _Inout_   SspiAsyncContext* AsyncContext,
    _In_opt_  PCredHandle phCredential,               // Cred to base context
    _In_opt_  PCtxtHandle phContext,                  // Existing context (OPT)
    _In_opt_  PSecBufferDesc pInput,                  // Input buffer
    _In_      unsigned long fContextReq,              // Context Requirements
    _In_      unsigned long TargetDataRep,            // Target Data Rep
    _In_opt_  PCtxtHandle phNewContext,               // (out) New context handle
    _In_opt_  PSecBufferDesc pOutput,                 // (inout) Output buffers
    _In_      unsigned long* pfContextAttr,           // (out) Context attributes
    _In_opt_  PTimeStamp ptsExpiry                    // (out) Life span (OPT)
);

SECURITY_STATUS SspiFreeCredentialsHandleAsync(
    _Inout_   SspiAsyncContext* AsyncContext,
    _In_      PCredHandle phCredential                  // Handle to free
);

SECURITY_STATUS SspiDeleteSecurityContextAsync(
    _Inout_   SspiAsyncContext* AsyncContext,
    _In_      PCtxtHandle phContext                     // Context to delete
);

#ifdef UNICODE
#    define SspiAcquireCredentialsHandleAsync SspiAcquireCredentialsHandleAsyncW
#    define SspiInitializeSecurityContextAsync SspiInitializeSecurityContextAsyncW
#else
#    define SspiAcquireCredentialsHandleAsync SspiAcquireCredentialsHandleAsyncA
#    define SspiInitializeSecurityContextAsync SspiInitializeSecurityContextAsync
#endif

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

////////////////////////////////////////////////////////////////////////
///
/// Password Change Functions
///
////////////////////////////////////////////////////////////////////////

#if ISSP_MODE != 0

SECURITY_STATUS SEC_ENTRY
ChangeAccountPasswordW(
    _In_    SEC_WCHAR *  pszPackageName,
    _In_    SEC_WCHAR *  pszDomainName,
    _In_    SEC_WCHAR *  pszAccountName,
    _In_    SEC_WCHAR *  pszOldPassword,
    _In_    SEC_WCHAR *  pszNewPassword,
    _In_    BOOLEAN              bImpersonating,
    _In_    unsigned long        dwReserved,
    _Inout_ PSecBufferDesc       pOutput
    );

typedef SECURITY_STATUS
(SEC_ENTRY * CHANGE_PASSWORD_FN_W)(
    SEC_WCHAR *,
    SEC_WCHAR *,
    SEC_WCHAR *,
    SEC_WCHAR *,
    SEC_WCHAR *,
    BOOLEAN,
    unsigned long,
    PSecBufferDesc
    );



SECURITY_STATUS SEC_ENTRY
ChangeAccountPasswordA(
    _In_    SEC_CHAR *  pszPackageName,
    _In_    SEC_CHAR *  pszDomainName,
    _In_    SEC_CHAR *  pszAccountName,
    _In_    SEC_CHAR *  pszOldPassword,
    _In_    SEC_CHAR *  pszNewPassword,
    _In_    BOOLEAN             bImpersonating,
    _In_    unsigned long       dwReserved,
    _Inout_ PSecBufferDesc      pOutput
    );

typedef SECURITY_STATUS
(SEC_ENTRY * CHANGE_PASSWORD_FN_A)(
    SEC_CHAR *,
    SEC_CHAR *,
    SEC_CHAR *,
    SEC_CHAR *,
    SEC_CHAR *,
    BOOLEAN,
    unsigned long,
    PSecBufferDesc
    );

#ifdef UNICODE
#  define ChangeAccountPassword ChangeAccountPasswordW
#  define CHANGE_PASSWORD_FN CHANGE_PASSWORD_FN_W
#else
#  define ChangeAccountPassword ChangeAccountPasswordA
#  define CHANGE_PASSWORD_FN CHANGE_PASSWORD_FN_A
#endif // !UNICODE

#endif // ISSP_MODE


////////////////////////////////////////////////////////////////////////
///
/// Context Management Functions
///
////////////////////////////////////////////////////////////////////////

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
InitializeSecurityContextW(
    _In_opt_    PCredHandle phCredential,               // Cred to base context
    _In_opt_    PCtxtHandle phContext,                  // Existing context (OPT)
#if ISSP_MODE == 0
    _In_opt_ PSECURITY_STRING pTargetName,
#else
    _In_opt_ SEC_WCHAR * pszTargetName,         // Name of target
#endif
    _In_        unsigned long fContextReq,              // Context Requirements
    _In_        unsigned long Reserved1,                // Reserved, MBZ
    _In_        unsigned long TargetDataRep,            // Data rep of target
    _In_opt_    PSecBufferDesc pInput,                  // Input Buffers
    _In_        unsigned long Reserved2,                // Reserved, MBZ
    _Inout_opt_ PCtxtHandle phNewContext,               // (out) New Context handle
    _Inout_opt_ PSecBufferDesc pOutput,                 // (inout) Output Buffers
    _Out_       unsigned long * pfContextAttr,  // (out) Context attrs
    _Out_opt_   PTimeStamp ptsExpiry                    // (out) Life span (OPT)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * INITIALIZE_SECURITY_CONTEXT_FN_W)(
    PCredHandle,
    PCtxtHandle,
#if ISSP_MODE == 0
    PSECURITY_STRING,
#else
    SEC_WCHAR *,
#endif
    unsigned long,
    unsigned long,
    unsigned long,
    PSecBufferDesc,
    unsigned long,
    PCtxtHandle,
    PSecBufferDesc,
    unsigned long *,
    PTimeStamp);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
InitializeSecurityContextA(
    _In_opt_    PCredHandle phCredential,               // Cred to base context
    _In_opt_    PCtxtHandle phContext,                  // Existing context (OPT)
    _In_opt_    SEC_CHAR * pszTargetName,       // Name of target
    _In_        unsigned long fContextReq,              // Context Requirements
    _In_        unsigned long Reserved1,                // Reserved, MBZ
    _In_        unsigned long TargetDataRep,            // Data rep of target
    _In_opt_    PSecBufferDesc pInput,                  // Input Buffers
    _In_        unsigned long Reserved2,                // Reserved, MBZ
    _Inout_opt_ PCtxtHandle phNewContext,               // (out) New Context handle
    _Inout_opt_ PSecBufferDesc pOutput,                 // (inout) Output Buffers
    _Out_       unsigned long * pfContextAttr,  // (out) Context attrs
    _Out_opt_   PTimeStamp ptsExpiry                    // (out) Life span (OPT)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * INITIALIZE_SECURITY_CONTEXT_FN_A)(
    PCredHandle,
    PCtxtHandle,
    SEC_CHAR *,
    unsigned long,
    unsigned long,
    unsigned long,
    PSecBufferDesc,
    unsigned long,
    PCtxtHandle,
    PSecBufferDesc,
    unsigned long *,
    PTimeStamp);

#ifdef UNICODE
#  define InitializeSecurityContext InitializeSecurityContextW              // ntifs
#  define INITIALIZE_SECURITY_CONTEXT_FN INITIALIZE_SECURITY_CONTEXT_FN_W   // ntifs
#else
#  define InitializeSecurityContext InitializeSecurityContextA
#  define INITIALIZE_SECURITY_CONTEXT_FN INITIALIZE_SECURITY_CONTEXT_FN_A
#endif // !UNICODE

// begin_ntifs

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
AcceptSecurityContext(
    _In_opt_    PCredHandle phCredential,               // Cred to base context
    _In_opt_    PCtxtHandle phContext,                  // Existing context (OPT)
    _In_opt_    PSecBufferDesc pInput,                  // Input buffer
    _In_        unsigned long fContextReq,              // Context Requirements
    _In_        unsigned long TargetDataRep,            // Target Data Rep
    _Inout_opt_ PCtxtHandle phNewContext,               // (out) New context handle
    _Inout_opt_ PSecBufferDesc pOutput,                 // (inout) Output buffers
    _Out_       unsigned long * pfContextAttr,  // (out) Context attributes
    _Out_opt_   PTimeStamp ptsExpiry                    // (out) Life span (OPT)
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ACCEPT_SECURITY_CONTEXT_FN)(
    PCredHandle,
    PCtxtHandle,
    PSecBufferDesc,
    unsigned long,
    unsigned long,
    PCtxtHandle,
    PSecBufferDesc,
    unsigned long *,
    PTimeStamp);

SECURITY_STATUS SEC_ENTRY
CompleteAuthToken(
    _In_ PCtxtHandle phContext,              // Context to complete
    _In_ PSecBufferDesc pToken               // Token to complete
    );

typedef SECURITY_STATUS
(SEC_ENTRY * COMPLETE_AUTH_TOKEN_FN)(
    PCtxtHandle,
    PSecBufferDesc);

KSECDDDECLSPEC
_Check_return_
SECURITY_STATUS SEC_ENTRY
ImpersonateSecurityContext(
    _In_ PCtxtHandle phContext               // Context to impersonate
    );

typedef SECURITY_STATUS
(SEC_ENTRY * IMPERSONATE_SECURITY_CONTEXT_FN)(
    PCtxtHandle);


KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
RevertSecurityContext(
    _In_ PCtxtHandle phContext               // Context from which to re
    );

typedef SECURITY_STATUS
(SEC_ENTRY * REVERT_SECURITY_CONTEXT_FN)(
    PCtxtHandle);


KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
QuerySecurityContextToken(
    _In_  PCtxtHandle phContext,
    _Out_ void * * Token
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_SECURITY_CONTEXT_TOKEN_FN)(
    PCtxtHandle, void * *);

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
DeleteSecurityContext(
    _In_ PCtxtHandle phContext               // Context to delete
    );

typedef SECURITY_STATUS
(SEC_ENTRY * DELETE_SECURITY_CONTEXT_FN)(
    PCtxtHandle);

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
ApplyControlToken(
    _In_ PCtxtHandle phContext,              // Context to modify
    _In_ PSecBufferDesc pInput               // Input token to apply
    );

typedef SECURITY_STATUS
(SEC_ENTRY * APPLY_CONTROL_TOKEN_FN)(
    PCtxtHandle, PSecBufferDesc);

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
QueryContextAttributesW(
    _In_  PCtxtHandle phContext,              // Context to query
    _In_  unsigned long ulAttribute,          // Attribute to query
    _Out_ void * pBuffer              // Buffer for attributes
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CONTEXT_ATTRIBUTES_FN_W)(
    PCtxtHandle,
    unsigned long,
    void *);

SECURITY_STATUS SEC_ENTRY
QueryContextAttributesExW(
    _In_  PCtxtHandle phContext,              // Context to query
    _In_  unsigned long ulAttribute,          // Attribute to query
    _Out_writes_bytes_(cbBuffer) void * pBuffer,    // Buffer for attributes
    _In_  unsigned long cbBuffer              // Length of buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CONTEXT_ATTRIBUTES_EX_FN_W)(
    PCtxtHandle,
    unsigned long,
    void *,
    unsigned long);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
QueryContextAttributesA(
    _In_  PCtxtHandle phContext,              // Context to query
    _In_  unsigned long ulAttribute,          // Attribute to query
    _Out_ void * pBuffer              // Buffer for attributes
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CONTEXT_ATTRIBUTES_FN_A)(
    PCtxtHandle,
    unsigned long,
    void *);

SECURITY_STATUS SEC_ENTRY
QueryContextAttributesExA(
    _In_  PCtxtHandle phContext,              // Context to query
    _In_  unsigned long ulAttribute,          // Attribute to query
    _Out_writes_bytes_(cbBuffer) void * pBuffer,    // Buffer for attributes
    _In_  unsigned long cbBuffer              // Length of buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CONTEXT_ATTRIBUTES_EX_FN_A)(
    PCtxtHandle,
    unsigned long,
    void *,
    unsigned long);

#ifdef UNICODE
#  define QueryContextAttributes QueryContextAttributesW            // ntifs
#  define QUERY_CONTEXT_ATTRIBUTES_FN QUERY_CONTEXT_ATTRIBUTES_FN_W // ntifs
#  define QueryContextAttributesEx QueryContextAttributesExW        // ntifs
#  define QUERY_CONTEXT_ATTRIBUTES_EX_FN QUERY_CONTEXT_ATTRIBUTES_EX_FN_W // ntifs
#else
#  define QueryContextAttributes QueryContextAttributesA
#  define QUERY_CONTEXT_ATTRIBUTES_FN QUERY_CONTEXT_ATTRIBUTES_FN_A
#  define QueryContextAttributesEx QueryContextAttributesExA
#  define QUERY_CONTEXT_ATTRIBUTES_EX_FN QUERY_CONTEXT_ATTRIBUTES_EX_FN_A
#endif // !UNICODE

// begin_ntifs

#if (OSVER(NTDDI_VERSION) > NTDDI_WIN2K)

SECURITY_STATUS SEC_ENTRY
SetContextAttributesW(
    _In_ PCtxtHandle phContext,                   // Context to Set
    _In_ unsigned long ulAttribute,               // Attribute to Set
    _In_reads_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_ unsigned long cbBuffer                   // Size (in bytes) of Buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * SET_CONTEXT_ATTRIBUTES_FN_W)(
    PCtxtHandle,
    unsigned long,
    void *,
    unsigned long );

#endif // Greater than w2k

// end_ntifs

SECURITY_STATUS SEC_ENTRY
SetContextAttributesA(
    _In_ PCtxtHandle phContext,                   // Context to Set
    _In_ unsigned long ulAttribute,               // Attribute to Set
    _In_reads_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_ unsigned long cbBuffer                   // Size (in bytes) of Buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * SET_CONTEXT_ATTRIBUTES_FN_A)(
    PCtxtHandle,
    unsigned long,
    void *,
    unsigned long );

#ifdef UNICODE
#  define SetContextAttributes SetContextAttributesW            // ntifs
#  define SET_CONTEXT_ATTRIBUTES_FN SET_CONTEXT_ATTRIBUTES_FN_W // ntifs
#else
#  define SetContextAttributes SetContextAttributesA
#  define SET_CONTEXT_ATTRIBUTES_FN SET_CONTEXT_ATTRIBUTES_FN_A
#endif // !UNICODE

// begin_ntifs

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
QueryCredentialsAttributesW(
    _In_    PCredHandle phCredential,           // Credential to query
    _In_    unsigned long ulAttribute,          // Attribute to query
    _Inout_ void * pBuffer              // Buffer for attributes
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CREDENTIALS_ATTRIBUTES_FN_W)(
    PCredHandle,
    unsigned long,
    void *);

SECURITY_STATUS SEC_ENTRY
QueryCredentialsAttributesExW(
    _In_    PCredHandle phCredential,           // Credential to query
    _In_    unsigned long ulAttribute,          // Attribute to query
    _Inout_updates_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_    unsigned long cbBuffer              // Length of buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W)(
    PCredHandle,
    unsigned long,
    void *,
    unsigned long);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
QueryCredentialsAttributesA(
    _In_    PCredHandle phCredential,           // Credential to query
    _In_    unsigned long ulAttribute,          // Attribute to query
    _Inout_ void * pBuffer              // Buffer for attributes
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CREDENTIALS_ATTRIBUTES_FN_A)(
    PCredHandle,
    unsigned long,
    void *);

SECURITY_STATUS SEC_ENTRY
QueryCredentialsAttributesExA(
    _In_    PCredHandle phCredential,           // Credential to query
    _In_    unsigned long ulAttribute,          // Attribute to query
    _Inout_updates_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_    unsigned long cbBuffer              // Length of buffer
    );


typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A)(
    PCredHandle,
    unsigned long,
    void *,
    unsigned long);

#ifdef UNICODE
#  define QueryCredentialsAttributes QueryCredentialsAttributesW            // ntifs
#  define QUERY_CREDENTIALS_ATTRIBUTES_FN QUERY_CREDENTIALS_ATTRIBUTES_FN_W // ntifs
#  define QueryCredentialsAttributesEx QueryCredentialsAttributesExW        // ntifs
#  define QUERY_CREDENTIALS_ATTRIBUTES_EX_FN QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W // ntifs
#else
#  define QueryCredentialsAttributes QueryCredentialsAttributesA
#  define QUERY_CREDENTIALS_ATTRIBUTES_FN QUERY_CREDENTIALS_ATTRIBUTES_FN_A
#  define QueryCredentialsAttributesEx QueryCredentialsAttributesExA
#  define QUERY_CREDENTIALS_ATTRIBUTES_EX_FN QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A
#endif // !UNICODE

// begin_ntifs

#if NTDDI_VERSION > NTDDI_WS03

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
SetCredentialsAttributesW(
    _In_ PCredHandle phCredential,                // Credential to Set
    _In_ unsigned long ulAttribute,               // Attribute to Set
    _In_reads_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_ unsigned long cbBuffer                   // Size (in bytes) of Buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * SET_CREDENTIALS_ATTRIBUTES_FN_W)(
    PCredHandle,
    unsigned long,
    void *,
    unsigned long );

#endif // For W2k3SP1 and greater

// end_ntifs

SECURITY_STATUS SEC_ENTRY
SetCredentialsAttributesA(
    _In_ PCredHandle phCredential,                // Credential to Set
    _In_ unsigned long ulAttribute,               // Attribute to Set
    _In_reads_bytes_(cbBuffer) void * pBuffer, // Buffer for attributes
    _In_ unsigned long cbBuffer                   // Size (in bytes) of Buffer
    );

typedef SECURITY_STATUS
(SEC_ENTRY * SET_CREDENTIALS_ATTRIBUTES_FN_A)(
    PCredHandle,
    unsigned long,
    void *,
    unsigned long );

#ifdef UNICODE
#  define SetCredentialsAttributes SetCredentialsAttributesW            // ntifs
#  define SET_CREDENTIALS_ATTRIBUTES_FN SET_CREDENTIALS_ATTRIBUTES_FN_W // ntifs
#else
#  define SetCredentialsAttributes SetCredentialsAttributesA
#  define SET_CREDENTIALS_ATTRIBUTES_FN SET_CREDENTIALS_ATTRIBUTES_FN_A
#endif // !UNICODE

// begin_ntifs

SECURITY_STATUS SEC_ENTRY
FreeContextBuffer(
    _Inout_ PVOID pvContextBuffer      // buffer to free
    );

typedef SECURITY_STATUS
(SEC_ENTRY * FREE_CONTEXT_BUFFER_FN)(
    _Inout_ PVOID
    );
    
SECURITY_STATUS
SEC_ENTRY
SecAllocateAndSetIPAddress(
    _In_reads_bytes_(cchIpAddress)  PUCHAR lpIpAddress,
    _In_  ULONG  cchIpAddress,
    _Out_ int* FreeCallContext // Avoid creating a dependence on minwindef.h by replacing PBOOL with its definition int*
    );

SECURITY_STATUS
SEC_ENTRY
SecAllocateAndSetCallTarget(
    _In_reads_bytes_opt_(cchIpAddress)  PUCHAR lpIpAddress,
    _In_  ULONG  cchIpAddress,
    _In_opt_ LPWSTR TargetName,
    _Out_ int* FreeCallContext // Avoid creating a dependence on minwindef.h by replacing PBOOL with its definition int*
    );

VOID
SEC_ENTRY
SecFreeCallContext(
    VOID
    );

// end_ntifs


// begin_ntifs
///////////////////////////////////////////////////////////////////
////
////    Message Support API
////
//////////////////////////////////////////////////////////////////

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
MakeSignature(
    _In_ PCtxtHandle phContext,              // Context to use
    _In_ unsigned long fQOP,                 // Quality of Protection
    _In_ PSecBufferDesc pMessage,            // Message to sign
    _In_ unsigned long MessageSeqNo          // Message Sequence Num.
    );

typedef SECURITY_STATUS
(SEC_ENTRY * MAKE_SIGNATURE_FN)(
    PCtxtHandle,
    unsigned long,
    PSecBufferDesc,
    unsigned long);


KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
VerifySignature(
    _In_  PCtxtHandle phContext,              // Context to use
    _In_  PSecBufferDesc pMessage,            // Message to verify
    _In_  unsigned long MessageSeqNo,         // Sequence Num.
    _Out_ unsigned long * pfQOP       // QOP used
    );

typedef SECURITY_STATUS
(SEC_ENTRY * VERIFY_SIGNATURE_FN)(
    PCtxtHandle,
    PSecBufferDesc,
    unsigned long,
    unsigned long *);

// This only exists win Win2k3 and Greater
#define SECQOP_WRAP_NO_ENCRYPT      0x80000001
#define SECQOP_WRAP_OOB_DATA        0x40000000

SECURITY_STATUS SEC_ENTRY
EncryptMessage( _In_    PCtxtHandle         phContext,
                _In_    unsigned long       fQOP,
                _In_    PSecBufferDesc      pMessage,
                _In_    unsigned long       MessageSeqNo);

typedef SECURITY_STATUS
(SEC_ENTRY * ENCRYPT_MESSAGE_FN)(
    PCtxtHandle, unsigned long, PSecBufferDesc, unsigned long);


SECURITY_STATUS SEC_ENTRY
DecryptMessage( _In_      PCtxtHandle         phContext,
                _In_      PSecBufferDesc      pMessage,
                _In_      unsigned long       MessageSeqNo,
                _Out_opt_ unsigned long *     pfQOP);


typedef SECURITY_STATUS
(SEC_ENTRY * DECRYPT_MESSAGE_FN)(
    PCtxtHandle, PSecBufferDesc, unsigned long,
    unsigned long *);


// end_ntifs

// begin_ntifs
///////////////////////////////////////////////////////////////////////////
////
////    Misc.
////
///////////////////////////////////////////////////////////////////////////

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
EnumerateSecurityPackagesW(
    _Out_       unsigned long * pcPackages,     // Receives num. packages
    _Outptr_ PSecPkgInfoW  * ppPackageInfo    // Receives array of info
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ENUMERATE_SECURITY_PACKAGES_FN_W)(
    unsigned long *,
    PSecPkgInfoW *);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
EnumerateSecurityPackagesA(
    _Out_       unsigned long * pcPackages,     // Receives num. packages
    _Outptr_ PSecPkgInfoA  * ppPackageInfo    // Receives array of info
    );

typedef SECURITY_STATUS
(SEC_ENTRY * ENUMERATE_SECURITY_PACKAGES_FN_A)(
    unsigned long *,
    PSecPkgInfoA *);

#ifdef UNICODE
#  define EnumerateSecurityPackages EnumerateSecurityPackagesW              // ntifs
#  define ENUMERATE_SECURITY_PACKAGES_FN ENUMERATE_SECURITY_PACKAGES_FN_W   // ntifs
#else
#  define EnumerateSecurityPackages EnumerateSecurityPackagesA
#  define ENUMERATE_SECURITY_PACKAGES_FN ENUMERATE_SECURITY_PACKAGES_FN_A
#endif // !UNICODE

// begin_ntifs

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
QuerySecurityPackageInfoW(
#if ISSP_MODE == 0
    _In_        PSECURITY_STRING pPackageName,
#else
    _In_        LPWSTR pszPackageName,          // Name of package
#endif
    _Outptr_ PSecPkgInfoW *ppPackageInfo     // Receives package info
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_SECURITY_PACKAGE_INFO_FN_W)(
#if ISSP_MODE == 0
    PSECURITY_STRING,
#else
    SEC_WCHAR *,
#endif
    PSecPkgInfoW *);

// end_ntifs

SECURITY_STATUS SEC_ENTRY
QuerySecurityPackageInfoA(
    _In_        LPSTR pszPackageName,           // Name of package
    _Outptr_ PSecPkgInfoA *ppPackageInfo     // Receives package info
    );

typedef SECURITY_STATUS
(SEC_ENTRY * QUERY_SECURITY_PACKAGE_INFO_FN_A)(
    SEC_CHAR *,
    PSecPkgInfoA *);

#ifdef UNICODE
#  define QuerySecurityPackageInfo QuerySecurityPackageInfoW                // ntifs
#  define QUERY_SECURITY_PACKAGE_INFO_FN QUERY_SECURITY_PACKAGE_INFO_FN_W   // ntifs
#else
#  define QuerySecurityPackageInfo QuerySecurityPackageInfoA
#  define QUERY_SECURITY_PACKAGE_INFO_FN QUERY_SECURITY_PACKAGE_INFO_FN_A
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef enum _SecDelegationType {
    SecFull,
    SecService,
    SecTree,
    SecDirectory,
    SecObject
} SecDelegationType, * PSecDelegationType;

SECURITY_STATUS SEC_ENTRY
DelegateSecurityContext(
    PCtxtHandle         phContext,          // IN Active context to delegate
#if ISSP_MODE == 0
    PSECURITY_STRING    pTarget,            // IN Target path
#else
    _In_ LPSTR          pszTarget,
#endif
    SecDelegationType   DelegationType,     // IN Type of delegation
    PTimeStamp          pExpiry,            // IN OPTIONAL time limit
    PSecBuffer          pPackageParameters, // IN OPTIONAL package specific
    PSecBufferDesc      pOutput);           // OUT Token for applycontroltoken.

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


///////////////////////////////////////////////////////////////////////////
////
////    Proxies
////
///////////////////////////////////////////////////////////////////////////


//
// Proxies are only available on NT platforms
//

// begin_ntifs

///////////////////////////////////////////////////////////////////////////
////
////    Context export/import
////
///////////////////////////////////////////////////////////////////////////


KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
ExportSecurityContext(
    _In_  PCtxtHandle          phContext,             // (in) context to export
    _In_  ULONG                fFlags,                // (in) option flags
    _Out_ PSecBuffer           pPackedContext,        // (out) marshalled context
    _Out_ void * * pToken             // (out, optional) token handle for impersonation
    );

typedef SECURITY_STATUS
(SEC_ENTRY * EXPORT_SECURITY_CONTEXT_FN)(
    PCtxtHandle,
    ULONG,
    PSecBuffer,
    void * *
    );

KSECDDDECLSPEC
SECURITY_STATUS SEC_ENTRY
ImportSecurityContextW(
#if ISSP_MODE == 0
    _In_  PSECURITY_STRING     pszPackage,
#else
    _In_  LPWSTR               pszPackage,
#endif
    _In_  PSecBuffer           pPackedContext,        // (in) marshalled context
    _In_  void *               Token,                 // (in, optional) handle to token for context
    _Out_ PCtxtHandle          phContext              // (out) new context handle
    );

typedef SECURITY_STATUS
(SEC_ENTRY * IMPORT_SECURITY_CONTEXT_FN_W)(
#if ISSP_MODE == 0
    PSECURITY_STRING,
#else
    SEC_WCHAR *,
#endif
    PSecBuffer,
    VOID *,
    PCtxtHandle
    );

// end_ntifs
SECURITY_STATUS SEC_ENTRY
ImportSecurityContextA(
    _In_  LPSTR                pszPackage,
    _In_  PSecBuffer           pPackedContext,        // (in) marshalled context
    _In_  VOID *               Token,                 // (in, optional) handle to token for context
    _Out_ PCtxtHandle          phContext              // (out) new context handle
    );

typedef SECURITY_STATUS
(SEC_ENTRY * IMPORT_SECURITY_CONTEXT_FN_A)(
    SEC_CHAR *,
    PSecBuffer,
    void *,
    PCtxtHandle
    );

#ifdef UNICODE
#  define ImportSecurityContext ImportSecurityContextW              // ntifs
#  define IMPORT_SECURITY_CONTEXT_FN IMPORT_SECURITY_CONTEXT_FN_W   // ntifs
#else
#  define ImportSecurityContext ImportSecurityContextA
#  define IMPORT_SECURITY_CONTEXT_FN IMPORT_SECURITY_CONTEXT_FN_A
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// begin_ntifs

#if ISSP_MODE == 0
KSECDDDECLSPEC
NTSTATUS
NTAPI
SecMakeSPN(
    IN PUNICODE_STRING ServiceClass,
    IN PUNICODE_STRING ServiceName,
    IN PUNICODE_STRING InstanceName OPTIONAL,
    IN USHORT InstancePort OPTIONAL,
    IN PUNICODE_STRING Referrer OPTIONAL,
    IN OUT PUNICODE_STRING Spn,
    OUT PULONG Length OPTIONAL,
    IN BOOLEAN Allocate
    );

#if OSVER(NTDDI_VERSION) > NTDD_WIN2K

KSECDDDECLSPEC
NTSTATUS
NTAPI
SecMakeSPNEx(
    IN PUNICODE_STRING ServiceClass,
    IN PUNICODE_STRING ServiceName,
    IN PUNICODE_STRING InstanceName OPTIONAL,
    IN USHORT InstancePort OPTIONAL,
    IN PUNICODE_STRING Referrer OPTIONAL,
    IN PUNICODE_STRING TargetInfo OPTIONAL,
    IN OUT PUNICODE_STRING Spn,
    OUT PULONG Length OPTIONAL,
    IN BOOLEAN Allocate
    );

#if OSVER(NTDDI_VERSION) > NTDDI_WS03

KSECDDDECLSPEC
NTSTATUS
NTAPI
SecMakeSPNEx2(
    IN PUNICODE_STRING ServiceClass,
    IN PUNICODE_STRING ServiceName,
    IN PUNICODE_STRING InstanceName OPTIONAL,
    IN USHORT InstancePort OPTIONAL,
    IN PUNICODE_STRING Referrer OPTIONAL,
    IN PUNICODE_STRING InTargetInfo OPTIONAL,
    IN OUT PUNICODE_STRING Spn,
    OUT PULONG TotalSize OPTIONAL,
    IN BOOLEAN Allocate,
    IN BOOLEAN IsTargetInfoMarshaled
    );

#endif // Windows Vista and greater

KSECDDDECLSPEC
NTSTATUS
SEC_ENTRY
SecLookupAccountSid(
    _In_      PSID Sid,
    _Out_     PULONG NameSize,
    _Inout_   PUNICODE_STRING NameBuffer,
    _Out_     PULONG DomainSize OPTIONAL,
    _Out_opt_ PUNICODE_STRING DomainBuffer OPTIONAL,
    _Out_     PSID_NAME_USE NameUse
    );

KSECDDDECLSPEC
NTSTATUS
SEC_ENTRY
SecLookupAccountName(
    _In_        PUNICODE_STRING Name,
    _Inout_     PULONG SidSize,
    _Out_       PSID Sid,
    _Out_       PSID_NAME_USE NameUse,
    _Out_       PULONG DomainSize OPTIONAL,
    _Inout_opt_ PUNICODE_STRING ReferencedDomain OPTIONAL
    );

#endif // Greater than W2k

#if OSVER(NTDDI_VERSION) > NTDDI_WINXP

KSECDDDECLSPEC
NTSTATUS
SEC_ENTRY
SecLookupWellKnownSid(
    _In_        WELL_KNOWN_SID_TYPE SidType,
    _Out_       PSID Sid,
    _In_        ULONG SidBufferSize,
    _Inout_opt_ PULONG SidSize OPTIONAL
    );

#endif // Greater than XP


#endif

// end_ntifs

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

///////////////////////////////////////////////////////////////////////////////
////
////  Fast access for RPC:
////
///////////////////////////////////////////////////////////////////////////////

#define SECURITY_ENTRYPOINT_ANSIW "InitSecurityInterfaceW"
#define SECURITY_ENTRYPOINT_ANSIA "InitSecurityInterfaceA"
#define SECURITY_ENTRYPOINTW SEC_TEXT("InitSecurityInterfaceW")     // ntifs
#define SECURITY_ENTRYPOINTA SEC_TEXT("InitSecurityInterfaceA")
#define SECURITY_ENTRYPOINT16 "INITSECURITYINTERFACEA"

#ifdef SECURITY_WIN32
#  ifdef UNICODE
#    define SECURITY_ENTRYPOINT SECURITY_ENTRYPOINTW                // ntifs
#    define SECURITY_ENTRYPOINT_ANSI SECURITY_ENTRYPOINT_ANSIW
#  else // UNICODE
#    define SECURITY_ENTRYPOINT SECURITY_ENTRYPOINTA
#    define SECURITY_ENTRYPOINT_ANSI SECURITY_ENTRYPOINT_ANSIA
#  endif // UNICODE
#else // SECURITY_WIN32
#  define SECURITY_ENTRYPOINT SECURITY_ENTRYPOINT16
#  define SECURITY_ENTRYPOINT_ANSI SECURITY_ENTRYPOINT16
#endif // SECURITY_WIN32

// begin_ntifs

#define FreeCredentialHandle FreeCredentialsHandle

typedef struct _SECURITY_FUNCTION_TABLE_W {
    unsigned long                       dwVersion;
    ENUMERATE_SECURITY_PACKAGES_FN_W    EnumerateSecurityPackagesW;
    QUERY_CREDENTIALS_ATTRIBUTES_FN_W   QueryCredentialsAttributesW;
    ACQUIRE_CREDENTIALS_HANDLE_FN_W     AcquireCredentialsHandleW;
    FREE_CREDENTIALS_HANDLE_FN          FreeCredentialsHandle;
    void *                      Reserved2;
    INITIALIZE_SECURITY_CONTEXT_FN_W    InitializeSecurityContextW;
    ACCEPT_SECURITY_CONTEXT_FN          AcceptSecurityContext;
    COMPLETE_AUTH_TOKEN_FN              CompleteAuthToken;
    DELETE_SECURITY_CONTEXT_FN          DeleteSecurityContext;
    APPLY_CONTROL_TOKEN_FN              ApplyControlToken;
    QUERY_CONTEXT_ATTRIBUTES_FN_W       QueryContextAttributesW;
    IMPERSONATE_SECURITY_CONTEXT_FN     ImpersonateSecurityContext;
    REVERT_SECURITY_CONTEXT_FN          RevertSecurityContext;
    MAKE_SIGNATURE_FN                   MakeSignature;
    VERIFY_SIGNATURE_FN                 VerifySignature;
    FREE_CONTEXT_BUFFER_FN              FreeContextBuffer;
    QUERY_SECURITY_PACKAGE_INFO_FN_W    QuerySecurityPackageInfoW;
    void *                      Reserved3;
    void *                      Reserved4;
    EXPORT_SECURITY_CONTEXT_FN          ExportSecurityContext;
    IMPORT_SECURITY_CONTEXT_FN_W        ImportSecurityContextW;
    ADD_CREDENTIALS_FN_W                AddCredentialsW ;
    void *                      Reserved8;
    QUERY_SECURITY_CONTEXT_TOKEN_FN     QuerySecurityContextToken;
    ENCRYPT_MESSAGE_FN                  EncryptMessage;
    DECRYPT_MESSAGE_FN                  DecryptMessage;
#if OSVER(NTDDI_VERSION) > NTDDI_WIN2K
    // Fields below this are available in OSes after w2k
    SET_CONTEXT_ATTRIBUTES_FN_W         SetContextAttributesW;
#endif // greater thean 2K

#if NTDDI_VERSION > NTDDI_WS03SP1
    // Fields below this are available in OSes after W2k3SP1
    SET_CREDENTIALS_ATTRIBUTES_FN_W     SetCredentialsAttributesW;
#endif
#if ISSP_MODE != 0
    CHANGE_PASSWORD_FN_W                ChangeAccountPasswordW;
#else
    void *                      Reserved9;
#endif
#if NTDDI_VERSION > NTDDI_WINBLUE
    // Fields below this are available in OSes after Windows 8.1
    QUERY_CONTEXT_ATTRIBUTES_EX_FN_W    QueryContextAttributesExW;
    QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_W QueryCredentialsAttributesExW;
#endif
} SecurityFunctionTableW, * PSecurityFunctionTableW;

// end_ntifs

typedef struct _SECURITY_FUNCTION_TABLE_A {
    unsigned long                       dwVersion;
    ENUMERATE_SECURITY_PACKAGES_FN_A    EnumerateSecurityPackagesA;
    QUERY_CREDENTIALS_ATTRIBUTES_FN_A   QueryCredentialsAttributesA;
    ACQUIRE_CREDENTIALS_HANDLE_FN_A     AcquireCredentialsHandleA;
    FREE_CREDENTIALS_HANDLE_FN          FreeCredentialHandle;
    void *                      Reserved2;
    INITIALIZE_SECURITY_CONTEXT_FN_A    InitializeSecurityContextA;
    ACCEPT_SECURITY_CONTEXT_FN          AcceptSecurityContext;
    COMPLETE_AUTH_TOKEN_FN              CompleteAuthToken;
    DELETE_SECURITY_CONTEXT_FN          DeleteSecurityContext;
    APPLY_CONTROL_TOKEN_FN              ApplyControlToken;
    QUERY_CONTEXT_ATTRIBUTES_FN_A       QueryContextAttributesA;
    IMPERSONATE_SECURITY_CONTEXT_FN     ImpersonateSecurityContext;
    REVERT_SECURITY_CONTEXT_FN          RevertSecurityContext;
    MAKE_SIGNATURE_FN                   MakeSignature;
    VERIFY_SIGNATURE_FN                 VerifySignature;
    FREE_CONTEXT_BUFFER_FN              FreeContextBuffer;
    QUERY_SECURITY_PACKAGE_INFO_FN_A    QuerySecurityPackageInfoA;
    void *                      Reserved3;
    void *                      Reserved4;
    EXPORT_SECURITY_CONTEXT_FN          ExportSecurityContext;
    IMPORT_SECURITY_CONTEXT_FN_A        ImportSecurityContextA;
    ADD_CREDENTIALS_FN_A                AddCredentialsA ;
    void *                      Reserved8;
    QUERY_SECURITY_CONTEXT_TOKEN_FN     QuerySecurityContextToken;
    ENCRYPT_MESSAGE_FN                  EncryptMessage;
    DECRYPT_MESSAGE_FN                  DecryptMessage;
    SET_CONTEXT_ATTRIBUTES_FN_A         SetContextAttributesA;
    SET_CREDENTIALS_ATTRIBUTES_FN_A     SetCredentialsAttributesA;
#if ISSP_MODE != 0
    CHANGE_PASSWORD_FN_A                ChangeAccountPasswordA;
#else
    void *                      Reserved9;
#endif
#if NTDDI_VERSION > NTDDI_WINBLUE
    // Fields below this are available in OSes after Windows 8.1
    QUERY_CONTEXT_ATTRIBUTES_EX_FN_A    QueryContextAttributesExA;
    QUERY_CREDENTIALS_ATTRIBUTES_EX_FN_A QueryCredentialsAttributesExA;
#endif
} SecurityFunctionTableA, * PSecurityFunctionTableA;

#ifdef UNICODE
#  define SecurityFunctionTable SecurityFunctionTableW      // ntifs
#  define PSecurityFunctionTable PSecurityFunctionTableW    // ntifs
#else
#  define SecurityFunctionTable SecurityFunctionTableA
#  define PSecurityFunctionTable PSecurityFunctionTableA
#endif // !UNICODE

#define SECURITY_

// Function table has all routines through DecryptMessage
#define SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION     1   // ntifs

// Function table has all routines through SetContextAttributes
#define SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_2   2   // ntifs

// Function table has all routines through SetCredentialsAttributes
#define SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_3   3   // ntifs

// Function table has all routines through ChangeAccountPassword
#define SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_4   4   // ntifs

// Function table has all routines through QueryCredentialsAttributesEx
#define SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION_5   5   // ntifs

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

PSecurityFunctionTableA SEC_ENTRY
InitSecurityInterfaceA(
    void
    );

typedef PSecurityFunctionTableA
(SEC_ENTRY * INIT_SECURITY_INTERFACE_A)(void);

// begin_ntifs

KSECDDDECLSPEC
PSecurityFunctionTableW SEC_ENTRY
InitSecurityInterfaceW(
    void
    );

typedef PSecurityFunctionTableW
(SEC_ENTRY * INIT_SECURITY_INTERFACE_W)(void);

// end_ntifs

#ifdef UNICODE
#  define InitSecurityInterface InitSecurityInterfaceW          // ntifs
#  define INIT_SECURITY_INTERFACE INIT_SECURITY_INTERFACE_W     // ntifs
#else
#  define InitSecurityInterface InitSecurityInterfaceA
#  define INIT_SECURITY_INTERFACE INIT_SECURITY_INTERFACE_A
#endif // !UNICODE


#ifdef SECURITY_WIN32

//
// SASL Profile Support
//


SECURITY_STATUS
SEC_ENTRY
SaslEnumerateProfilesA(
    _Outptr_ LPSTR * ProfileList,
    _Out_       ULONG * ProfileCount
    );

SECURITY_STATUS
SEC_ENTRY
SaslEnumerateProfilesW(
    _Outptr_ LPWSTR * ProfileList,
    _Out_       ULONG * ProfileCount
    );

#ifdef UNICODE
#define SaslEnumerateProfiles   SaslEnumerateProfilesW
#else
#define SaslEnumerateProfiles   SaslEnumerateProfilesA
#endif


SECURITY_STATUS
SEC_ENTRY
SaslGetProfilePackageA(
    _In_        LPSTR ProfileName,
    _Outptr_ PSecPkgInfoA * PackageInfo
    );


SECURITY_STATUS
SEC_ENTRY
SaslGetProfilePackageW(
    _In_        LPWSTR ProfileName,
    _Outptr_ PSecPkgInfoW * PackageInfo
    );

#ifdef UNICODE
#define SaslGetProfilePackage   SaslGetProfilePackageW
#else
#define SaslGetProfilePackage   SaslGetProfilePackageA
#endif

SECURITY_STATUS
SEC_ENTRY
SaslIdentifyPackageA(
    _In_        PSecBufferDesc pInput,
    _Outptr_ PSecPkgInfoA * PackageInfo
    );

SECURITY_STATUS
SEC_ENTRY
SaslIdentifyPackageW(
    _In_        PSecBufferDesc pInput,
    _Outptr_ PSecPkgInfoW * PackageInfo
    );

#ifdef UNICODE
#define SaslIdentifyPackage SaslIdentifyPackageW
#else
#define SaslIdentifyPackage SaslIdentifyPackageA
#endif

SECURITY_STATUS
SEC_ENTRY
SaslInitializeSecurityContextW(
    _In_opt_    PCredHandle                 phCredential,       // Cred to base context
    _In_opt_    PCtxtHandle                 phContext,          // Existing context (OPT)
    _In_opt_    LPWSTR                      pszTargetName,      // Name of target
    _In_        unsigned long               fContextReq,        // Context Requirements
    _In_        unsigned long               Reserved1,          // Reserved, MBZ
    _In_        unsigned long               TargetDataRep,      // Data rep of target
    _In_opt_    PSecBufferDesc              pInput,             // Input Buffers
    _In_        unsigned long               Reserved2,          // Reserved, MBZ
    _Inout_opt_ PCtxtHandle                 phNewContext,       // (out) New Context handle
    _Inout_opt_ PSecBufferDesc              pOutput,            // (inout) Output Buffers
    _Out_       unsigned long *     pfContextAttr,      // (out) Context attrs
    _Out_opt_   PTimeStamp                  ptsExpiry           // (out) Life span (OPT)
    );

SECURITY_STATUS
SEC_ENTRY
SaslInitializeSecurityContextA(
    _In_opt_    PCredHandle                 phCredential,       // Cred to base context
    _In_opt_    PCtxtHandle                 phContext,          // Existing context (OPT)
    _In_opt_    LPSTR                       pszTargetName,      // Name of target
    _In_        unsigned long               fContextReq,        // Context Requirements
    _In_        unsigned long               Reserved1,          // Reserved, MBZ
    _In_        unsigned long               TargetDataRep,      // Data rep of target
    _In_opt_    PSecBufferDesc              pInput,             // Input Buffers
    _In_        unsigned long               Reserved2,          // Reserved, MBZ
    _Inout_opt_ PCtxtHandle                 phNewContext,       // (out) New Context handle
    _Inout_opt_ PSecBufferDesc              pOutput,            // (inout) Output Buffers
    _Out_       unsigned long *     pfContextAttr,      // (out) Context attrs
    _Out_opt_   PTimeStamp                  ptsExpiry           // (out) Life span (OPT)
    );

#ifdef UNICODE
#define SaslInitializeSecurityContext   SaslInitializeSecurityContextW
#else
#define SaslInitializeSecurityContext   SaslInitializeSecurityContextA
#endif


SECURITY_STATUS
SEC_ENTRY
SaslAcceptSecurityContext(
    _In_opt_    PCredHandle                 phCredential,       // Cred to base context
    _In_opt_    PCtxtHandle                 phContext,          // Existing context (OPT)
    _In_opt_    PSecBufferDesc              pInput,             // Input buffer
    _In_        unsigned long               fContextReq,        // Context Requirements
    _In_        unsigned long               TargetDataRep,      // Target Data Rep
    _Inout_opt_ PCtxtHandle                 phNewContext,       // (out) New context handle
    _Inout_opt_ PSecBufferDesc              pOutput,            // (inout) Output buffers
    _Out_       unsigned long *     pfContextAttr,      // (out) Context attributes
    _Out_opt_   PTimeStamp                  ptsExpiry           // (out) Life span (OPT)
    );


#define SASL_OPTION_SEND_SIZE       1       // Maximum size to send to peer
#define SASL_OPTION_RECV_SIZE       2       // Maximum size willing to receive
#define SASL_OPTION_AUTHZ_STRING    3       // Authorization string
#define SASL_OPTION_AUTHZ_PROCESSING    4       // Authorization string processing

typedef enum _SASL_AUTHZID_STATE {
    Sasl_AuthZIDForbidden,             // allow no AuthZID strings to be specified - error out (default)
    Sasl_AuthZIDProcessed             // AuthZID Strings processed by Application or SSP
} SASL_AUTHZID_STATE ;

SECURITY_STATUS
SEC_ENTRY
SaslSetContextOption(
    _In_ PCtxtHandle ContextHandle,
    _In_ ULONG Option,
    _In_ PVOID Value,
    _In_ ULONG Size
    );


SECURITY_STATUS
SEC_ENTRY
SaslGetContextOption(
    _In_      PCtxtHandle ContextHandle,
    _In_      ULONG Option,
    _Out_     PVOID Value,
    _In_      ULONG Size,
    _Out_opt_ PULONG Needed OPTIONAL
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef SECURITY_DOS
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4147)
#endif
#endif

//
// This is the legacy credentials structure.
// The EX version below is preferred.

// begin_ntifs

#ifndef _AUTH_IDENTITY_EX2_DEFINED
#define _AUTH_IDENTITY_EX2_DEFINED

#define SEC_WINNT_AUTH_IDENTITY_VERSION_2 0x201

typedef struct _SEC_WINNT_AUTH_IDENTITY_EX2 {
   unsigned long Version; // contains SEC_WINNT_AUTH_IDENTITY_VERSION_2
   unsigned short cbHeaderLength;
   unsigned long cbStructureLength;
   unsigned long UserOffset;                // Non-NULL terminated string, unicode only
   unsigned short UserLength;               // # of bytes (NOT WCHARs), not including NULL.
   unsigned long DomainOffset;              // Non-NULL terminated string, unicode only
   unsigned short DomainLength;             // # of bytes (NOT WCHARs), not including NULL.
   unsigned long PackedCredentialsOffset;   // Non-NULL terminated string, unicode only
   unsigned short PackedCredentialsLength;  // # of bytes (NOT WCHARs), not including NULL.
   unsigned long Flags;
   unsigned long PackageListOffset;         // Non-NULL terminated string, unicode only
   unsigned short PackageListLength;
} SEC_WINNT_AUTH_IDENTITY_EX2, *PSEC_WINNT_AUTH_IDENTITY_EX2;

#endif // _AUTH_IDENTITY_EX2_DEFINED

#ifndef _AUTH_IDENTITY_DEFINED
#define _AUTH_IDENTITY_DEFINED

//
// This was not defined in NTIFS.h for windows 2000 however
// this struct has always been there and are safe to use
// in windows 2000 and above.
//

#define SEC_WINNT_AUTH_IDENTITY_ANSI    0x1
#define SEC_WINNT_AUTH_IDENTITY_UNICODE 0x2

typedef struct _SEC_WINNT_AUTH_IDENTITY_W {
  unsigned short *User;         //  Non-NULL terminated string.
  unsigned long UserLength;     //  # of characters (NOT bytes), not including NULL.
  unsigned short *Domain;       //  Non-NULL terminated string.
  unsigned long DomainLength;   //  # of characters (NOT bytes), not including NULL.
  unsigned short *Password;     //  Non-NULL terminated string.
  unsigned long PasswordLength; //  # of characters (NOT bytes), not including NULL.
  unsigned long Flags;
} SEC_WINNT_AUTH_IDENTITY_W, *PSEC_WINNT_AUTH_IDENTITY_W;

// end_ntifs

#define _AUTH_IDENTITY_A_DEFINED

typedef struct _SEC_WINNT_AUTH_IDENTITY_A {
  unsigned char *User;          //  Non-NULL terminated string.
  unsigned long UserLength;     //  # of characters (NOT bytes), not including NULL.
  unsigned char *Domain;        //  Non-NULL terminated string.
  unsigned long DomainLength;   //  # of characters (NOT bytes), not including NULL.
  unsigned char *Password;      //  Non-NULL terminated string.
  unsigned long PasswordLength; //  # of characters (NOT bytes), not including NULL.
  unsigned long Flags;
} SEC_WINNT_AUTH_IDENTITY_A, *PSEC_WINNT_AUTH_IDENTITY_A;


#ifdef UNICODE
#define SEC_WINNT_AUTH_IDENTITY SEC_WINNT_AUTH_IDENTITY_W       // ntifs
#define PSEC_WINNT_AUTH_IDENTITY PSEC_WINNT_AUTH_IDENTITY_W     // ntifs
#define _SEC_WINNT_AUTH_IDENTITY _SEC_WINNT_AUTH_IDENTITY_W     // ntifs
#else // UNICODE
#define SEC_WINNT_AUTH_IDENTITY SEC_WINNT_AUTH_IDENTITY_A
#define PSEC_WINNT_AUTH_IDENTITY PSEC_WINNT_AUTH_IDENTITY_A
#define _SEC_WINNT_AUTH_IDENTITY _SEC_WINNT_AUTH_IDENTITY_A
#endif // UNICODE

// begin_ntifs

#endif //_AUTH_IDENTITY_DEFINED                                 // ntifs

//
// This is the combined authentication identity structure that may be
// used with the negotiate package, NTLM, Kerberos, or SCHANNEL
//

#ifndef SEC_WINNT_AUTH_IDENTITY_VERSION
#define SEC_WINNT_AUTH_IDENTITY_VERSION 0x200

typedef struct _SEC_WINNT_AUTH_IDENTITY_EXW {
    unsigned long Version;
    unsigned long Length;
    unsigned short *User;           //  Non-NULL terminated string.
    unsigned long UserLength;       //  # of characters (NOT bytes), not including NULL.
    unsigned short *Domain;         //  Non-NULL terminated string.
    unsigned long DomainLength;     //  # of characters (NOT bytes), not including NULL.
    unsigned short *Password;       //  Non-NULL terminated string.
    unsigned long PasswordLength;   //  # of characters (NOT bytes), not including NULL.
    unsigned long Flags;
    unsigned short *PackageList;
    unsigned long PackageListLength;
} SEC_WINNT_AUTH_IDENTITY_EXW, *PSEC_WINNT_AUTH_IDENTITY_EXW;

// end_ntifs

typedef struct _SEC_WINNT_AUTH_IDENTITY_EXA {
    unsigned long Version;
    unsigned long Length;
    unsigned char *User;            //  Non-NULL terminated string.
    unsigned long UserLength;       //  # of characters (NOT bytes), not including NULL.
    unsigned char *Domain;          //  Non-NULL terminated string.
    unsigned long DomainLength;     //  # of characters (NOT bytes), not including NULL.
    unsigned char *Password;        //  Non-NULL terminated string.
    unsigned long PasswordLength;   //  # of characters (NOT bytes), not including NULL.
    unsigned long Flags;
    unsigned char * PackageList;
    unsigned long PackageListLength;
} SEC_WINNT_AUTH_IDENTITY_EXA, *PSEC_WINNT_AUTH_IDENTITY_EXA;

#ifdef UNICODE
#define SEC_WINNT_AUTH_IDENTITY_EX  SEC_WINNT_AUTH_IDENTITY_EXW    // ntifs
#define PSEC_WINNT_AUTH_IDENTITY_EX PSEC_WINNT_AUTH_IDENTITY_EXW   // ntifs
#else
#define SEC_WINNT_AUTH_IDENTITY_EX  SEC_WINNT_AUTH_IDENTITY_EXA
#endif

// begin_ntifs

#endif // SEC_WINNT_AUTH_IDENTITY_VERSION

// end_ntifs

#ifndef _AUTH_IDENTITY_INFO_DEFINED
#define _AUTH_IDENTITY_INFO_DEFINED

//
// the procedure for how to parse a SEC_WINNT_AUTH_IDENTITY_INFO structure:
//
// 1) First check the first DWORD of SEC_WINNT_AUTH_IDENTITY_INFO, if the first
//   DWORD is 0x200, it is either an AuthIdExw or AuthIdExA, otherwise if the first
//   DWORD is 0x201, the structure is an AuthIdEx2 structure. Otherwise the structure
//   is either an AuthId_a or an AuthId_w.
//
// 2) Secondly check the flags for SEC_WINNT_AUTH_IDENTITY_ANSI or
//   SEC_WINNT_AUTH_IDENTITY_UNICODE, the presence of the former means the structure
//   is an ANSI structure. Otherwise, the structure is the wide version.  Note that
//   AuthIdEx2 does not have an ANSI version so this check does not apply to it.
//

typedef union _SEC_WINNT_AUTH_IDENTITY_INFO {
    SEC_WINNT_AUTH_IDENTITY_EXW AuthIdExw;
    SEC_WINNT_AUTH_IDENTITY_EXA AuthIdExa;
    SEC_WINNT_AUTH_IDENTITY_A AuthId_a;
    SEC_WINNT_AUTH_IDENTITY_W AuthId_w;
    SEC_WINNT_AUTH_IDENTITY_EX2 AuthIdEx2;
} SEC_WINNT_AUTH_IDENTITY_INFO, *PSEC_WINNT_AUTH_IDENTITY_INFO;

// the credential structure is encrypted via
// RtlEncryptMemory(OptionFlags = 0)
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_PROCESS_ENCRYPTED 0x10

// the credential structure is protected by local system via
// RtlEncryptMemory(OptionFlags=RTL_ENCRYPT_OPTION_SAME_LOGON)
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_PROTECTED  0x20

// the credential structure is encrypted by a non-system context
// RtlEncryptMemory(OptionFlags=RTL_ENCRYPT_OPTION_SAME_LOGON)
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_USER_PROTECTED    0x40

// the credential structure is encrypted with
// RtlEncryptMemory(OptionFlags=RTL_ENCRYPT_OPTION_FOR_SYSTEM)
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SYSTEM_ENCRYPTED  0x80

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_RESERVED       0x10000
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_USER      0x20000
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_NULL_DOMAIN    0x40000
#define SEC_WINNT_AUTH_IDENTITY_FLAGS_ID_PROVIDER    0x80000


//
//  These bits are for communication between SspiPromptForCredentials()
//  and the credential providers. Do not use these bits for any other
//  purpose.
//

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_USE_MASK  0xFF000000

//
//  Instructs the credential provider to not save credentials itself
//  when caller selects the "Remember my credential" checkbox.
//


#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE  0x80000000

//
// Support the old name for this flag for callers that were built for earlier
// versions of the SDK.
//

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_BY_CALLER   SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE

//
//  State of the "Remember my credentials" checkbox.
//  When set, indicates checked; when cleared, indicates unchecked.
//

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED     0x40000000

//
// The "Save" checkbox is not displayed on the credential provider tiles
//

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX           0x20000000

//
// Credential providers will not attempt to prepopulate the CredUI dialog
// box with credentials retrieved from Cred Man.
//

#define SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD  0x10000000


#define SEC_WINNT_AUTH_IDENTITY_FLAGS_VALID_SSPIPFC_FLAGS   \
                (SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_SAVE | \
                 SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_SAVE_CRED_CHECKED | \
                 SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_NO_CHECKBOX | \
                 SEC_WINNT_AUTH_IDENTITY_FLAGS_SSPIPFC_CREDPROV_DO_NOT_LOAD)


#endif // _AUTH_IDENTITY_INFO_DEFINED

#ifndef _SSPIPFC_NONE_ // the public view

// begin_ntifs

typedef PVOID PSEC_WINNT_AUTH_IDENTITY_OPAQUE; // the credential structure is opaque

// end_ntifs

#else  // the internal view

typedef PSEC_WINNT_AUTH_IDENTITY_INFO PSEC_WINNT_AUTH_IDENTITY_OPAQUE;

#endif // _SSPIPFC_NONE_

//
//  dwFlags parameter of SspiPromptForCredentials():
//

//
//  Indicates that the credentials should not be saved if
//  the user selects the 'save' (or 'remember my password')
//  checkbox in the credential dialog box. The location pointed
//  to by the pfSave parameter indicates whether or not the user
//  selected the checkbox.
//
//  Note that some credential providers won't honour this flag and
//  may save the credentials in a persistent manner anyway if the
//  user selects the 'save' checbox.
//

#define SSPIPFC_CREDPROV_DO_NOT_SAVE    0x00000001

//
// Support the old name for this flag for callers that were built for earlier
// versions of the SDK.
//


#define SSPIPFC_SAVE_CRED_BY_CALLER     SSPIPFC_CREDPROV_DO_NOT_SAVE

//
// The password and smart card credential providers will not display the
// "Remember my credentials" check box in the provider tiles.
//

#define SSPIPFC_NO_CHECKBOX             0x00000002

//
// Credential providers will not attempt to prepopulate the CredUI dialog
// box with credentials retrieved from Cred Man.
//

#define SSPIPFC_CREDPROV_DO_NOT_LOAD    0x00000004

//
// Credential providers along with UI Dialog will be hosted in a separate
// broker process.
//
#define SSPIPFC_USE_CREDUIBROKER	0x00000008

#define SSPIPFC_VALID_FLAGS (SSPIPFC_CREDPROV_DO_NOT_SAVE | SSPIPFC_NO_CHECKBOX | SSPIPFC_CREDPROV_DO_NOT_LOAD | SSPIPFC_USE_CREDUIBROKER)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _SSPIPFC_NONE_ // the public view

// Use SspiFreeAuthIdentity() to free the buffer returned
// in ppAuthIdentity.

unsigned long
SEC_ENTRY
SspiPromptForCredentialsW(
    _In_ PCWSTR pszTargetName,
#ifdef _CREDUI_INFO_DEFINED
    _In_opt_ PCREDUI_INFOW pUiInfo,
#else
    _In_opt_ PVOID pUiInfo,
#endif // _CREDUI_INFO_DEFINED
    _In_ unsigned long dwAuthError,
    _In_ PCWSTR pszPackage,
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE pInputAuthIdentity,
    _Outptr_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE* ppAuthIdentity,
    _Inout_opt_ int* pfSave,
    _In_ unsigned long dwFlags
    );

// Use SspiFreeAuthIdentity() to free the buffer returned
// in ppAuthIdentity.

unsigned long
SEC_ENTRY
SspiPromptForCredentialsA(
    _In_ PCSTR pszTargetName,
#ifdef _CREDUI_INFO_DEFINED
    _In_opt_ PCREDUI_INFOA pUiInfo,
#else
    _In_opt_ PVOID pUiInfo,
#endif // _CREDUI_INFO_DEFINED
    _In_ unsigned long dwAuthError,
    _In_ PCSTR pszPackage,
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE pInputAuthIdentity,
    _Outptr_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE* ppAuthIdentity,
    _Inout_opt_ int* pfSave,
    _In_ unsigned long dwFlags
    );
#endif // _SSPIPFC_NONE_

#ifdef UNICODE
#define SspiPromptForCredentials   SspiPromptForCredentialsW
#else
#define SspiPromptForCredentials   SspiPromptForCredentialsA
#endif

#ifdef _SEC_WINNT_AUTH_TYPES

typedef struct _SEC_WINNT_AUTH_BYTE_VECTOR {
    unsigned long ByteArrayOffset; // each element is a byte
    unsigned short ByteArrayLength; //
} SEC_WINNT_AUTH_BYTE_VECTOR, *PSEC_WINNT_AUTH_BYTE_VECTOR;

typedef struct _SEC_WINNT_AUTH_DATA {
   GUID CredType;
   SEC_WINNT_AUTH_BYTE_VECTOR CredData;
} SEC_WINNT_AUTH_DATA, *PSEC_WINNT_AUTH_DATA;

typedef struct _SEC_WINNT_AUTH_PACKED_CREDENTIALS {
   unsigned short cbHeaderLength;    // the length of the header
   unsigned short cbStructureLength; // pay load length including the header
   SEC_WINNT_AUTH_DATA AuthData;
} SEC_WINNT_AUTH_PACKED_CREDENTIALS, *PSEC_WINNT_AUTH_PACKED_CREDENTIALS;

// {28BFC32F-10F6-4738-98D1-1AC061DF716A}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_PASSWORD =
   { 0x28bfc32f, 0x10f6, 0x4738, { 0x98, 0xd1, 0x1a, 0xc0, 0x61, 0xdf, 0x71, 0x6a } };

// {235F69AD-73FB-4dbc-8203-0629E739339B}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_CERT =
   { 0x235f69ad, 0x73fb, 0x4dbc, { 0x82, 0x3, 0x6, 0x29, 0xe7, 0x39, 0x33, 0x9b } };

// {7CB72412-1016-491A-8C87-4D2AA1B7DD3A}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_CREDMAN_CERT =
   { 0x7cb72412, 0x1016, 0x491a, { 0x8c, 0x87, 0x4d, 0x2a, 0xa1, 0xb7, 0xdd, 0x3a } };


// {10A47879-5EBF-4B85-BD8D-C21BB4F49C8A}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_NGC =
   { 0x10a47879, 0x5ebf, 0x4b85, { 0xbd, 0x8d, 0xc2, 0x1b, 0xb4, 0xf4, 0x9c, 0x8a } };

// {32E8F8D7-7871-4BCC-83C5-460F66C6135C}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_FIDO =
{ 0x32e8f8d7, 0x7871, 0x4bcc, { 0x83, 0xc5, 0x46, 0xf, 0x66, 0xc6, 0x13, 0x5c } };

// {D587AAE8-F78F-4455-A112-C934BEEE7CE1}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_KEYTAB =
{ 0xd587aae8, 0xf78f, 0x4455, { 0xa1, 0x12, 0xc9, 0x34, 0xbe, 0xee, 0x7c, 0xe1 } };

// {12E52E0F-6F9B-4F83-9020-9DE42B226267}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_DELEGATION_TOKEN =
{ 0x12e52e0f, 0x6f9b, 0x4f83, { 0x90, 0x20, 0x9d, 0xe4, 0x2b, 0x22, 0x62, 0x67 } };

typedef struct _SEC_WINNT_AUTH_DATA_PASSWORD {
   SEC_WINNT_AUTH_BYTE_VECTOR UnicodePassword;
} SEC_WINNT_AUTH_DATA_PASSWORD, PSEC_WINNT_AUTH_DATA_PASSWORD;

//
// smartcard cred data
//
// {68FD9879-079C-4dfe-8281-578AADC1C100}

EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_CSP_DATA =
   { 0x68fd9879, 0x79c, 0x4dfe, { 0x82, 0x81, 0x57, 0x8a, 0xad, 0xc1, 0xc1, 0x0 } };

// {B86C4FF3-49D7-4DC4-B560-B1163685B236}
EXTERN_C __declspec(selectany) const GUID SEC_WINNT_AUTH_DATA_TYPE_SMARTCARD_CONTEXTS =
   { 0xb86c4ff3, 0x49d7, 0x4dc4, { 0xb5, 0x60, 0xb1, 0x16, 0x36, 0x85, 0xb2, 0x36 } };

typedef struct _SEC_WINNT_AUTH_CERTIFICATE_DATA {
   unsigned short cbHeaderLength;
   unsigned short cbStructureLength;
   SEC_WINNT_AUTH_BYTE_VECTOR Certificate;
} SEC_WINNT_AUTH_CERTIFICATE_DATA, *PSEC_WINNT_AUTH_CERTIFICATE_DATA;

typedef struct _SEC_WINNT_AUTH_NGC_DATA {
   LUID LogonId;
   unsigned long Flags;
   SEC_WINNT_AUTH_BYTE_VECTOR CspInfo;
   SEC_WINNT_AUTH_BYTE_VECTOR UserIdKeyAuthTicket;
   SEC_WINNT_AUTH_BYTE_VECTOR DecryptionKeyName;
   SEC_WINNT_AUTH_BYTE_VECTOR DecryptionKeyAuthTicket;
} SEC_WINNT_AUTH_NGC_DATA, *PSEC_WINNT_AUTH_NGC_DATA;

#define NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES     (0x1) //corresponds to KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES
#define NGC_DATA_FLAG_KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO (0x2) //corresponds to  KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO
#define NGC_DATA_FLAG_IS_SMARTCARD_DATA                                (0x4)
#define NGC_DATA_FLAG_IS_CLOUD_TRUST_CRED                              (0x8) // credential should be treated as "cloud trust" - use Cloud TGTs instead of on-prem PKINIT

typedef struct _SEC_WINNT_AUTH_DATA_TYPE_SMARTCARD_CONTEXTS_DATA
{
    PVOID pcc;
    PVOID hProv;
    LPWSTR pwszECDHKeyName; // only optionally set for ECDSA smartcards
} SEC_WINNT_AUTH_DATA_TYPE_SMARTCARD_CONTEXTS_DATA, *PSEC_WINNT_AUTH_DATA_TYPE_SMARTCARD_CONTEXTS_DATA;

// FIDO cred data
typedef struct _SEC_WINNT_AUTH_FIDO_DATA {
   unsigned short cbHeaderLength;
   unsigned short cbStructureLength;
   SEC_WINNT_AUTH_BYTE_VECTOR Secret; // offsets are from the beginning of this structure
   SEC_WINNT_AUTH_BYTE_VECTOR NewSecret;
   SEC_WINNT_AUTH_BYTE_VECTOR EncryptedNewSecret; // For storage by cloud AP
   SEC_WINNT_AUTH_BYTE_VECTOR NetworkLogonBuffer; // Opaque data, understood by plugin, may contain signed Nonce and other data to perform a network logon
   ULONG64 ulSignatureCount; // signature count to be stored in public cached info, required for CredProv
} SEC_WINNT_AUTH_FIDO_DATA, *PSEC_WINNT_AUTH_FIDO_DATA;

typedef struct _SEC_WINNT_CREDUI_CONTEXT_VECTOR
{
   ULONG CredUIContextArrayOffset; // offset starts at the beginning of
   // this structure, and each element is a SEC_WINNT_AUTH_BYTE_VECTOR that
   // describes the flat CredUI context returned by SpGetCredUIContext()
   USHORT CredUIContextCount;
} SEC_WINNT_CREDUI_CONTEXT_VECTOR, *PSEC_WINNT_CREDUI_CONTEXT_VECTOR;

typedef struct _SEC_WINNT_AUTH_SHORT_VECTOR
{
    ULONG ShortArrayOffset; // each element is a short
    USHORT ShortArrayCount; // number of characters
} SEC_WINNT_AUTH_SHORT_VECTOR, *PSEC_WINNT_AUTH_SHORT_VECTOR;

// free the returned memory using SspiLocalFree

SECURITY_STATUS
SEC_ENTRY
SspiGetCredUIContext(
   _In_ HANDLE ContextHandle,
   _In_ GUID* CredType,
   _In_opt_ LUID* LogonId, // use this LogonId, the caller must be localsystem to supply a logon id
   _Outptr_ PSEC_WINNT_CREDUI_CONTEXT_VECTOR* CredUIContexts,
   _Out_opt_ HANDLE* TokenHandle
   );

SECURITY_STATUS
SEC_ENTRY
SspiUpdateCredentials(
   _In_ HANDLE ContextHandle,
   _In_ GUID* CredType,
   _In_ ULONG FlatCredUIContextLength,
   _In_reads_bytes_(FlatCredUIContextLength) PUCHAR FlatCredUIContext
   );

typedef struct _CREDUIWIN_MARSHALED_CONTEXT
{
    GUID StructureType;
    USHORT cbHeaderLength;
    LUID LogonId; // user's logon id
    GUID MarshaledDataType;
    ULONG MarshaledDataOffset;
    USHORT MarshaledDataLength;
} CREDUIWIN_MARSHALED_CONTEXT, *PCREDUIWIN_MARSHALED_CONTEXT;

typedef struct _SEC_WINNT_CREDUI_CONTEXT
{
    USHORT cbHeaderLength;
    HANDLE CredUIContextHandle; // the handle to call SspiGetCredUIContext()
#ifdef _CREDUI_INFO_DEFINED
    PCREDUI_INFOW UIInfo; // input from SspiPromptForCredentials()
#else
    PVOID UIInfo;
#endif // _CREDUI_INFO_DEFINED
    ULONG dwAuthError; // the authentication error
    PSEC_WINNT_AUTH_IDENTITY_OPAQUE pInputAuthIdentity;
    PUNICODE_STRING TargetName;
} SEC_WINNT_CREDUI_CONTEXT, *PSEC_WINNT_CREDUI_CONTEXT;

// {3C3E93D9-D96B-49b5-94A7-458592088337}
EXTERN_C __declspec(selectany) const GUID CREDUIWIN_STRUCTURE_TYPE_SSPIPFC  =
{ 0x3c3e93d9, 0xd96b, 0x49b5, { 0x94, 0xa7, 0x45, 0x85, 0x92, 0x8, 0x83, 0x37 } };

// {C2FFFE6F-503D-4c3d-A95E-BCE821213D44}
EXTERN_C __declspec(selectany) const GUID SSPIPFC_STRUCTURE_TYPE_CREDUI_CONTEXT =
{ 0xc2fffe6f, 0x503d, 0x4c3d, { 0xa9, 0x5e, 0xbc, 0xe8, 0x21, 0x21, 0x3d, 0x44 } };

typedef struct _SEC_WINNT_AUTH_PACKED_CREDENTIALS_EX {
   unsigned short cbHeaderLength;
   unsigned long Flags; // contains the Flags field in
                        // SEC_WINNT_AUTH_IDENTITY_EX
   SEC_WINNT_AUTH_BYTE_VECTOR PackedCredentials;
   SEC_WINNT_AUTH_SHORT_VECTOR PackageList;
} SEC_WINNT_AUTH_PACKED_CREDENTIALS_EX, *PSEC_WINNT_AUTH_PACKED_CREDENTIALS_EX;

//
// free the returned memory using SspiLocalFree
//

SECURITY_STATUS
SEC_ENTRY
SspiUnmarshalCredUIContext(
    _In_reads_bytes_(MarshaledCredUIContextLength) PUCHAR MarshaledCredUIContext,
    _In_ ULONG MarshaledCredUIContextLength,
    _Outptr_ PSEC_WINNT_CREDUI_CONTEXT* CredUIContext
    );

#endif // _SEC_WINNT_AUTH_TYPES


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

SECURITY_STATUS
SEC_ENTRY
SspiPrepareForCredRead(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity,
    _In_ PCWSTR pszTargetName,
    _Out_ PULONG pCredmanCredentialType,
    _Outptr_ PCWSTR* ppszCredmanTargetName
    );

SECURITY_STATUS
SEC_ENTRY
SspiPrepareForCredWrite(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity,
    _In_opt_ PCWSTR pszTargetName, // supply NULL for username-target credentials
    _Out_ PULONG pCredmanCredentialType,
    _Outptr_ PCWSTR* ppszCredmanTargetName,
    _Outptr_ PCWSTR* ppszCredmanUserName,
    _Outptr_result_bytebuffer_(*pCredentialBlobSize) PUCHAR *ppCredentialBlob,
    _Out_ PULONG pCredentialBlobSize
    );


//
// Input flags for SspiEncryptAuthIdentityEx and
// SspiDecryptAuthIdentityEx functions
//

#define SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_LOGON        0x1
#define SEC_WINNT_AUTH_IDENTITY_ENCRYPT_SAME_PROCESS      0x2
#define SEC_WINNT_AUTH_IDENTITY_ENCRYPT_FOR_SYSTEM        0x4

SECURITY_STATUS
SEC_ENTRY
SspiEncryptAuthIdentity(
    _Inout_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData
    );

SECURITY_STATUS
SEC_ENTRY
SspiEncryptAuthIdentityEx(
    _In_    ULONG   Options,
    _Inout_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData
    );

SECURITY_STATUS
SEC_ENTRY
SspiDecryptAuthIdentity(
    _Inout_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE EncryptedAuthData
    );

SECURITY_STATUS
SEC_ENTRY
SspiDecryptAuthIdentityEx(
    _In_    ULONG   Options,
    _Inout_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE EncryptedAuthData
    );

BOOLEAN
SEC_ENTRY
SspiIsAuthIdentityEncrypted(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE EncryptedAuthData
    );

// begin_ntifs

#if (NTDDI_VERSION >= NTDDI_WIN7)
//
//  Convert the _OPAQUE structure passed in to the
//  3 tuple <username, domainname, 'password'>.
//
//  Note: The 'strings' returned need not necessarily be
//  in user recognisable form. The purpose of this API
//  is to 'flatten' the _OPAQUE structure into the 3 tuple.
//  User recognisable <username, domainname> can always be
//  obtained by passing NULL to the pszPackedCredentialsString
//  parameter.
//
// zero out the pszPackedCredentialsString then
// free the returned memory using SspiLocalFree()
//

SECURITY_STATUS
SEC_ENTRY
SspiEncodeAuthIdentityAsStrings(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE pAuthIdentity,
    _Outptr_result_maybenull_ PCWSTR* ppszUserName,
    _Outptr_result_maybenull_ PCWSTR* ppszDomainName,
    _Outptr_opt_result_maybenull_ PCWSTR* ppszPackedCredentialsString
    );

SECURITY_STATUS
SEC_ENTRY
SspiValidateAuthIdentity(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData
    );

//
// free the returned memory using SspiFreeAuthIdentity()
//

SECURITY_STATUS
SEC_ENTRY
SspiCopyAuthIdentity(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSEC_WINNT_AUTH_IDENTITY_OPAQUE* AuthDataCopy
    );

//
// use only for the memory returned by SspiCopyAuthIdentity().
// Internally calls SspiZeroAuthIdentity().
//

VOID
SEC_ENTRY
SspiFreeAuthIdentity(
    _In_opt_ __drv_freesMem(Mem) PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData
    );

VOID
SEC_ENTRY
SspiZeroAuthIdentity(
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthData
    );

VOID
SEC_ENTRY
SspiLocalFree(
    _In_opt_ PVOID DataBuffer
    );

//
// call SspiFreeAuthIdentity to free the returned AuthIdentity
// which zeroes out the credentials blob before freeing it
//

SECURITY_STATUS
SEC_ENTRY
SspiEncodeStringsAsAuthIdentity(
    _In_opt_ PCWSTR pszUserName,
    _In_opt_ PCWSTR pszDomainName,
    _In_opt_ PCWSTR pszPackedCredentialsString,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSEC_WINNT_AUTH_IDENTITY_OPAQUE* ppAuthIdentity
    );

SECURITY_STATUS
SEC_ENTRY
SspiCompareAuthIdentities(
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity1,
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity2,
    _Out_opt_ PBOOLEAN SameSuppliedUser,
    _Out_opt_ PBOOLEAN SameSuppliedIdentity
    );

//
// zero out the returned AuthIdentityByteArray then
// free the returned memory using SspiLocalFree()
//

SECURITY_STATUS
SEC_ENTRY
SspiMarshalAuthIdentity(
    _In_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity,
    _Out_ unsigned long* AuthIdentityLength,
    _Outptr_result_bytebuffer_(*AuthIdentityLength) char** AuthIdentityByteArray
    );

//
// free the returned auth identity using SspiFreeAuthIdentity()
//

SECURITY_STATUS
SEC_ENTRY
SspiUnmarshalAuthIdentity(
    _In_ unsigned long AuthIdentityLength,
    _In_reads_bytes_(AuthIdentityLength) char* AuthIdentityByteArray,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSEC_WINNT_AUTH_IDENTITY_OPAQUE* ppAuthIdentity
    );

#endif // NTDDI_VERSION

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN7)

BOOLEAN
SEC_ENTRY
SspiIsPromptingNeeded(
    _In_ unsigned long ErrorOrNtStatus
    );

#endif // NTDDI_VERSION

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN7)

SECURITY_STATUS
SEC_ENTRY
SspiGetTargetHostName(
    _In_ PCWSTR pszTargetName,
    _Outptr_ PWSTR* pszHostName
    );

SECURITY_STATUS
SEC_ENTRY
SspiExcludePackage(
    _In_opt_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE AuthIdentity,
    _In_ PCWSTR pszPackageName,
    _Outptr_ PSEC_WINNT_AUTH_IDENTITY_OPAQUE* ppNewAuthIdentity
    );

#endif // NTDDI_VERSION

//
// Common types used by negotiable security packages
//
// These are defined after W2K
//

#define SEC_WINNT_AUTH_IDENTITY_MARSHALLED      0x4     // all data is in one buffer
#define SEC_WINNT_AUTH_IDENTITY_ONLY            0x8     // these credentials are for identity only - no PAC needed

// end_ntifs

// Set the requested flags in the channel bindings.  pBindings->Bindings may change if the structure gets
// larger, but the caller's obligation to call FreeContextBuffer is unchanged.
SECURITY_STATUS
SEC_ENTRY
SspiSetChannelBindingFlags(
    _Inout_ SecPkgContext_Bindings* pBindings,
    unsigned long flags
    );

//
// Routines for manipulating packages
//

typedef struct _SECURITY_PACKAGE_OPTIONS {
    unsigned long   Size;
    unsigned long   Type;
    unsigned long   Flags;
    unsigned long   SignatureSize;
    void *  Signature;
} SECURITY_PACKAGE_OPTIONS, * PSECURITY_PACKAGE_OPTIONS;

#define SECPKG_OPTIONS_TYPE_UNKNOWN 0
#define SECPKG_OPTIONS_TYPE_LSA     1
#define SECPKG_OPTIONS_TYPE_SSPI    2

#define SECPKG_OPTIONS_PERMANENT    0x00000001

SECURITY_STATUS
SEC_ENTRY
AddSecurityPackageA(
    _In_     LPSTR                     pszPackageName,
    _In_opt_ PSECURITY_PACKAGE_OPTIONS pOptions
    );

SECURITY_STATUS
SEC_ENTRY
AddSecurityPackageW(
    _In_     LPWSTR                    pszPackageName,
    _In_opt_ PSECURITY_PACKAGE_OPTIONS pOptions
    );

#ifdef UNICODE
#define AddSecurityPackage  AddSecurityPackageW
#else
#define AddSecurityPackage  AddSecurityPackageA
#endif

SECURITY_STATUS
SEC_ENTRY
DeleteSecurityPackageA(
    _In_ LPSTR pszPackageName
    );

SECURITY_STATUS
SEC_ENTRY
DeleteSecurityPackageW(
    _In_ LPWSTR pszPackageName
    );

#ifdef UNICODE
#define DeleteSecurityPackage   DeleteSecurityPackageW
#else
#define DeleteSecurityPackage   DeleteSecurityPackageA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}  // extern "C"
#endif

// begin_ntifs
#endif // __SSPI_H__
// end_ntifs
