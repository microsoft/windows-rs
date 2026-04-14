//***************************************************************************
//
//  iscsidsc.h
// 
//  Module: Public iScsi Discovery header
//
//  Purpose: 
//
//  Copyright (c) 2002 Microsoft Corporation
//
//***************************************************************************

#ifndef _ISCSI_ISCSIDSC_
#define _ISCSI_ISCSIDSC_

#ifndef MIDL_PASS
#ifndef MOFCOMP_PASS
#include <ntddscsi.h>
#include <winioctl.h>
#endif
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ISCSI)

//
// Maxiumum length of a Initiator Name
//
#define MAX_ISCSI_HBANAME_LEN 256


//
// Maximum length of an iscsi name
//
#define MAX_ISCSI_NAME_LEN  223
#define MAX_ISCSI_ALIAS_LEN  255


//
// Maxiumum length of a portal names
//
#define MAX_ISCSI_PORTAL_NAME_LEN 256
#define MAX_ISCSI_PORTAL_ALIAS_LEN 256

//
// Maximum length of a text address
//
#define MAX_ISCSI_TEXT_ADDRESS_LEN 256

//
// Maxiumum length of a text port address. It can be a DNS name or a .
// name
//
#define MAX_ISCSI_PORTAL_ADDRESS_LEN MAX_ISCSI_TEXT_ADDRESS_LEN

//
// Maximum length of a discovery domain name
//
#define MAX_ISCSI_DISCOVERY_DOMAIN_LEN 256


//
// For apis that take a port number, this specifies that any port can
// be used
//
#define ISCSI_ANY_INITIATOR_PORT         ((ULONG)-1)

//
// For apis that take a port number this specifies that all ports
// should be used
//
#define ISCSI_ALL_INITIATOR_PORTS        ((ULONG)-1)

//
// Maximum length of a RADIUS server address +
// two terminating characters
//
#define MAX_RADIUS_ADDRESS_LEN 41


#ifdef MOFCOMP_PASS
//
// Definitions for iscsi security flags. These flags provide
// information about the security expectations of a target portal and
// are needed to insure a successful IKE/IPSEC negotiation. Note that
// the flags and values are taken directly from the iSNS spec
//
#define ISCSI_SECURITY_FLAGS uint64

    // 1 = Tunnel Mode Preferred; 0 = No Preference
#define ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED    "0x00000040"

    // 1 = Transport Mode Preferred; 0 = No Preference
#define ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED "0x00000020"
                
    // 1 = PFS Enabled; 0 = PFS Disabled
#define ISCSI_SECURITY_FLAG_PFS_ENABLED              "0x00000010"
                
    // 1 = Aggressive Mode Enabled; 0 = Disabled
#define ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED  "0x00000008"
                
    // 1 = Main Mode Enabled; 0 = MM Disabled
#define ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED        "0x00000004"
                
    // 1 = IKE/IPSec Enabled; 0 = IKE/IPSec Disabled
#define ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED        "0x00000002"

    // If set then all other ISCSI_SECURITY_FLAGS are valid             
#define ISCSI_SECURITY_FLAG_VALID                    "0x00000001"

#define SECURITY_FLAG_QUALIFIERS \
     description("Security flags") : amended, \
     BitMap{ \
            ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED, \
            ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED, \
            ISCSI_SECURITY_FLAG_PFS_ENABLED, \
            ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED, \
            ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED, \
            ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED, \
            ISCSI_SECURITY_FLAG_VALID \
           }, \
     BitValues{ \
            "Tunnel mode preferred", \
            "Transport mode preferred", \
            "PFS Enabled", \
            "Aggressive mode enabled", \
            "Main Mode Enabled", \
            "IKE/IPSec Enabled", \
            "ISCSI_SECURITY_FLAGS are valid" \
              } : amended
                                

#define ISCSI_SECURITY_FLAGS_CPPQUOTE \
"//\n" \
"// Definitions for iscsi security flags. These flags provide\n" \
"// information about the security expectations of a target portal and\n" \
"// are needed to insure a successful IKE/IPSEC negotiation. Note that\n" \
"// the flags and values are taken directly from the iSNS spec\n" \
"//\n" \
"\n" \
"    // 1 = Tunnel Mode Preferred; 0 = No Preference\n" \
"#define ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED    0x00000040\n" \
"\n" \
"    // 1 = Transport Mode Preferred; 0 = No Preference\n" \
"#define ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED 0x00000020\n" \
"               \n" \
"    // 1 = PFS Enabled; 0 = PFS Disabled\n" \
"#define ISCSI_SECURITY_FLAG_PFS_ENABLED              0x00000010\n" \
"               \n" \
"    // 1 = Aggressive Mode Enabled; 0 = Disabled\n" \
"#define ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED  0x00000008\n" \
"               \n" \
"    // 1 = Main Mode Enabled; 0 = MM Disabled\n" \
"#define ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED        0x00000004\n" \
"               \n" \
"    // 1 = IKE/IPSec Enabled; 0 = IKE/IPSec Disabled\n" \
"#define ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED        0x00000002\n" \
"\n" \
"    // If set then all other ISCSI_SECURITY_FLAGS are valid                \n" \
"#define ISCSI_SECURITY_FLAG_VALID                    0x00000001                \n" \
"\n"

//
// definitions for ISCSI_LOGIN_OPTIONS. This structure is used to pass
// information that affects the login negotiation of session
//

#define ISCSI_DIGEST_TYPE_NONE    "0"
#define ISCSI_DIGEST_TYPE_CRC32C  "1"

#define ISCSI_DIGEST_CPPQUOTE \
"#ifndef _ISCSI_ISCSIDSC_\n" \
"typedef enum\n" \
"{\n" \
"   ISCSI_DIGEST_TYPE_NONE = 0,\n" \
"   ISCSI_DIGEST_TYPE_CRC32C = 1\n" \
"} ISCSI_DIGEST_TYPES, *PISCSI_DIGEST_TYPES;\n" \
"\n" \
"typedef enum\n" \
"{\n" \
"   ISCSI_NO_AUTH_TYPE = 0,\n" \
"   ISCSI_CHAP_AUTH_TYPE = 1,\n" \
"   ISCSI_MUTUAL_CHAP_AUTH_TYPE = 2 \n" \
"} ISCSI_AUTH_TYPES, *PISCSI_AUTH_TYPES;\n" \
"#endif\n"   

#define ISCSI_LOGIN_FLAGS_CPPQUOTE \
"//\n" \
"// bit flags for ISCSI_LOGIN_FLAGS\n" \
"//\n" \
"#ifndef _ISCSI_ISCSIDSC_\n" \
"#define ISCSI_LOGIN_FLAGS ULONG\n\n" \
"#define ISCSI_LOGIN_FLAG_REQUIRE_IPSEC                0x00000001\n" \
"#define ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED            0x00000002\n" \
"#define ISCSI_LOGIN_FLAG_RESERVED1                    0x00000004\n" \
"#define ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING         0x00000008\n" \
"#define ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE          0x00000010\n" \
"#define ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION          0x00000020\n" \
"\n" \
"#endif\n"   

#define ISCSI_LOGIN_FLAGS_QUALIFIERS \
    BitValues{"Require IPSEC", "Multipath Enabled", "Reserved1", "Allow Portal Hopping"}, \
    BitMap{ "0x00000001", "0x00000002", "0x00000004", "0x00000008" }

#define ISCSI_LOGIN_FLAGS uint32

#define ISCSI_AUTH_TYPES_QUALIFIERS \
    Values{"No Authentication", \
           "CHAP", \
           "Mutual CHAP" \
          } : amended, \
    ValueMap{"0", "1", "2"}

#define ISCSI_AUTH_TYPES_CPPQUOTE \

#define ISCSI_AUTH_TYPES uint32

//
// Login options flags
//
#define ISCSI_LOGIN_OPTIONS_INFO_SPECIFIED uint32

#define ISCSI_LOGIN_OPTIONS_HEADER_DIGEST               "0x00000001"
#define ISCSI_LOGIN_OPTIONS_DATA_DIGEST                 "0x00000002"
#define ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS         "0x00000004"
#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT         "0x00000008"
#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN       "0x00000010"
#define ISCSI_LOGIN_OPTIONS_USERNAME                    "0x00000020"
#define ISCSI_LOGIN_OPTIONS_PASSWORD                    "0x00000040"
#define ISCSI_LOGIN_OPTIONS_AUTH_TYPE                   "0x00000080"

#define ISCSI_LOGIN_OPTIONS_INFO_CPPQUOTE \
"//\n" \
"// Bit flags for InformationSpecifies\n" \
"//\n" \
"#define ISCSI_LOGIN_OPTIONS_HEADER_DIGEST               0x00000001\n" \
"#define ISCSI_LOGIN_OPTIONS_DATA_DIGEST                 0x00000002\n" \
"#define ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS         0x00000004\n" \
"#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT         0x00000008\n" \
"#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN       0x00000010\n" \
"#define ISCSI_LOGIN_OPTIONS_USERNAME                    0x00000020\n" \
"#define ISCSI_LOGIN_OPTIONS_PASSWORD                    0x00000040\n" \
"#define ISCSI_LOGIN_OPTIONS_AUTH_TYPE                   0x00000080\n" \
"\n"

#define ISCSI_LOGIN_OPTIONS_INFO_QUALIFIERS \
     BitMap{ ISCSI_LOGIN_OPTIONS_HEADER_DIGEST, \
             ISCSI_LOGIN_OPTIONS_DATA_DIGEST, \
             ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS, \
             ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT, \
             ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN \
           }, \
     BitValues{"HeaderDigest", "DataDigest", "MaxConnections", \
               "DefaultTime2Wait", "DefaultTime2Retain" } : amended


//
// IKE Identification payload types
//
#define       ID_IPV4_ADDR                        "1"
#define       ID_FQDN                             "2"
#define       ID_USER_FQDN                        "3"
#define       ID_IPV6_ADDR                        "5"

#define IKE_IDENTIFICATION_TYPE_QUALIFIERS \
             ValueMap{ ID_IPV4_ADDR, \
                       ID_FQDN, \
                       ID_USER_FQDN, \
                       ID_IPV6_ADDR \
                       }, \
              Values{  "ID_IPV4_ADDR", \
                       "ID_FQDN", \
                       "ID_USER_FQDN", \
                       "ID_IPV6_ADDR" \
                        }

#define IKE_AUTHENTICATION_PAYLOAD_TYPE_CPPQUOTE \
"//\n" \
"// IKE Identification payload types (from RFC 2407)\n" \
"//\n" \
"#define       ID_IPV4_ADDR                        1\n" \
"#define       ID_FQDN                             2\n" \
"#define       ID_USER_FQDN                        3\n" \
"#define       ID_IPV6_ADDR                        5\n" \
"\n"
                   
    
#else

//
// Definitions for iscsi security flags. These flags provide
// information about the security expectations of a target portal and
// are needed to insure a successful IKE/IPSEC negotiation. Note that
// the flags and values are taken directly from the iSNS spec
//
typedef ULONGLONG ISCSI_SECURITY_FLAGS;
typedef ISCSI_SECURITY_FLAGS *PISCSI_SECURITY_FLAGS;


    // 1 = Tunnel Mode Preferred; 0 = No Preference
#define ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED    0x00000040

    // 1 = Transport Mode Preferred; 0 = No Preference
#define ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED 0x00000020
                
    // 1 = PFS Enabled; 0 = PFS Disabled
#define ISCSI_SECURITY_FLAG_PFS_ENABLED              0x00000010
                
    // 1 = Aggressive Mode Enabled; 0 = Disabled
#define ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED  0x00000008
                
    // 1 = Main Mode Enabled; 0 = MM Disabled
#define ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED        0x00000004
                
    // 1 = IKE/IPSec Enabled; 0 = IKE/IPSec Disabled
#define ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED        0x00000002

    // If set then all other ISCSI_SECURITY_FLAGS are valid             
#define ISCSI_SECURITY_FLAG_VALID                    0x00000001             

//
// definitions for ISCSI_LOGIN_OPTIONS. This structure is used to pass
// information that affects the login negotiation of session
//

typedef enum
{
    ISCSI_DIGEST_TYPE_NONE = 0,
    ISCSI_DIGEST_TYPE_CRC32C = 1
} ISCSI_DIGEST_TYPES, *PISCSI_DIGEST_TYPES;

//
// bit flags for ISCSI_LOGIN_FLAGS
//
typedef ULONG ISCSI_LOGIN_FLAGS, *PISCSI_LOGIN_FLAGS;

#define ISCSI_LOGIN_FLAG_REQUIRE_IPSEC                0x00000001
#define ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED            0x00000002
#define ISCSI_LOGIN_FLAG_RESERVED1                    0x00000004
#define ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING         0x00000008
#define ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE          0x00000010
#define ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION          0x00000020


//
// Bit flags for InformationSpecifies
//

typedef ULONG ISCSI_LOGIN_OPTIONS_INFO_SPECIFIED;
typedef ISCSI_LOGIN_OPTIONS_INFO_SPECIFIED *PISCSI_LOGIN_OPTIONS_INFO_SPECIFIED;

#define ISCSI_LOGIN_OPTIONS_HEADER_DIGEST               0x00000001
#define ISCSI_LOGIN_OPTIONS_DATA_DIGEST                 0x00000002
#define ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS         0x00000004
#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT         0x00000008
#define ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN       0x00000010
#define ISCSI_LOGIN_OPTIONS_USERNAME                    0x00000020
#define ISCSI_LOGIN_OPTIONS_PASSWORD                    0x00000040
#define ISCSI_LOGIN_OPTIONS_AUTH_TYPE                   0x00000080


#define ISCSI_LOGIN_OPTIONS_VERSION 0

typedef enum
{
    ISCSI_NO_AUTH_TYPE = 0,
    ISCSI_CHAP_AUTH_TYPE = 1,
    ISCSI_MUTUAL_CHAP_AUTH_TYPE = 2
} ISCSI_AUTH_TYPES, *PISCSI_AUTH_TYPES;

typedef struct
{
    ULONG Version;                    // ISCSI_LOGIN_OPTIONS_VERSION

                       // Bit field indicating which information is specified   
    ISCSI_LOGIN_OPTIONS_INFO_SPECIFIED InformationSpecified;

    ISCSI_LOGIN_FLAGS LoginFlags;

    ISCSI_AUTH_TYPES AuthType;
    
    ISCSI_DIGEST_TYPES HeaderDigest;
    ISCSI_DIGEST_TYPES DataDigest;
    ULONG MaximumConnections;
    ULONG DefaultTime2Wait;
    ULONG DefaultTime2Retain;

    //
    // Username and passwords are used for iscsi authentication and are
    // defined as defined as binary blobs. They have different meanings
    // based upon the different iSCSI authentication method used.
    //     For CHAP the username is the CHAP Name (CHAP_N) 
    //     and the password is the shared secret of the target. If the
    //     Username is not specified then the service will use the
    //     initiator node name as the CHAP Name
    //
    // If they are strings then they are expected
    // to be ANSI strings. 
    //
    ULONG UsernameLength;
    ULONG PasswordLength;
#ifdef MIDL_PASS
    [size_is(UsernameLength)]
#endif
    PUCHAR Username;
#ifdef MIDL_PASS
    [size_is(PasswordLength)]
#endif
    PUCHAR Password;
} ISCSI_LOGIN_OPTIONS, *PISCSI_LOGIN_OPTIONS;


//
// This defines flags that affect how a target is managed and its
// information stored
//
typedef ULONG ISCSI_TARGET_FLAGS;
typedef ISCSI_TARGET_FLAGS *PISCSI_TARGET_FLAGS;

//
// if this flag is set then the target will never be reported unless it
// is also discovered dynamically.
//
#define ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET            0x00000002

//
// If this flag is set then the target information passed will be
// merged with any target information already statically configured for
// the target
//
#define ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION       0x00000004


//
// IKE Identification payload types (from RFC 2407)
//
typedef UCHAR IKE_IDENTIFICATION_PAYLOAD_TYPE, *PIKE_IDENTIFICATION_PAYLOAD_TYPE;

#define       ID_IPV4_ADDR                        1
#define       ID_FQDN                             2
#define       ID_USER_FQDN                        3
#define       ID_IPV6_ADDR                        5



//
// Methods and data structures for those methods that can be used
// for IKE authentication in the SetIScsiIKEInfo api
//
typedef enum
{
    IKE_AUTHENTICATION_PRESHARED_KEY_METHOD = 1
} IKE_AUTHENTICATION_METHOD, *PIKE_AUTHENTICATION_METHOD;

typedef struct
{
    ISCSI_SECURITY_FLAGS SecurityFlags;
    IKE_IDENTIFICATION_PAYLOAD_TYPE IdType;
    ULONG IdLengthInBytes;
    PUCHAR Id;
    ULONG KeyLengthInBytes;
    PUCHAR Key;
} IKE_AUTHENTICATION_PRESHARED_KEY, *PIKE_AUTHENTICATION_PRESHARED_KEY;

typedef struct
{
    IKE_AUTHENTICATION_METHOD AuthMethod;
    union
    {
        IKE_AUTHENTICATION_PRESHARED_KEY PsKey;
    };
} IKE_AUTHENTICATION_INFORMATION, *PIKE_AUTHENTICATION_INFORMATION;

#endif

#ifndef MOFCOMP_PASS

#ifdef _ISDSCP_
#define ISDSC_API WINAPI
#else
#define ISDSC_API DECLSPEC_IMPORT WINAPI
#endif


//
// Each iscsi session and connection has a unique session or connection
// id that is used to reference the session. It it not related to the
// actual ISID
//
typedef struct _ISCSI_UNIQUE_SESSION_ID
{   
   ULONGLONG AdapterUnique;
   ULONGLONG AdapterSpecific;
} ISCSI_UNIQUE_SESSION_ID, *PISCSI_UNIQUE_SESSION_ID,
  ISCSI_UNIQUE_CONNECTION_ID, *PISCSI_UNIQUE_CONNECTION_ID;

//
// This specifies a mapping from a target LUN to a OS LUN
//

typedef struct
{
    ULONG OSLUN;
    ULONGLONG TargetLUN;
} SCSI_LUN_LIST, *PSCSI_LUN_LIST;

typedef struct
{
    WCHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    WCHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    WCHAR OSDeviceName[MAX_PATH]; /* \device\ScsiPort3 */
    ISCSI_UNIQUE_SESSION_ID SessionId;
    ULONG OSBusNumber;
    ULONG OSTargetNumber;
    ULONG LUNCount;
#ifdef MIDL_PASS
    [size_is(LUNCount)]
#endif
    PSCSI_LUN_LIST LUNList;
} ISCSI_TARGET_MAPPINGW, *PISCSI_TARGET_MAPPINGW;

typedef struct
{
    CHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    CHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    CHAR OSDeviceName[MAX_PATH]; /* \device\ScsiPort3 */
    ISCSI_UNIQUE_SESSION_ID SessionId;
    ULONG OSBusNumber;
    ULONG OSTargetNumber;
    ULONG LUNCount;
#ifdef MIDL_PASS
    [size_is(LUNCount)]
#endif
    PSCSI_LUN_LIST LUNList;
} ISCSI_TARGET_MAPPINGA, *PISCSI_TARGET_MAPPINGA;

#ifdef UNICODE
typedef ISCSI_TARGET_MAPPINGW  ISCSI_TARGET_MAPPING;
typedef PISCSI_TARGET_MAPPINGW  PISCSI_TARGET_MAPPING;
#else
typedef ISCSI_TARGET_MAPPINGA  ISCSI_TARGET_MAPPING;
typedef PISCSI_TARGET_MAPPINGA  PISCSI_TARGET_MAPPING;
#endif // UNICODE
                   
typedef struct
{
    WCHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    WCHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
} ISCSI_TARGET_PORTALW, *PISCSI_TARGET_PORTALW;

typedef struct
{
    CHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    CHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
} ISCSI_TARGET_PORTALA, *PISCSI_TARGET_PORTALA;

#ifdef UNICODE
typedef ISCSI_TARGET_PORTALW  ISCSI_TARGET_PORTAL;
typedef PISCSI_TARGET_PORTALW  PISCSI_TARGET_PORTAL;
#else
typedef ISCSI_TARGET_PORTALA  ISCSI_TARGET_PORTAL;
typedef PISCSI_TARGET_PORTALA  PISCSI_TARGET_PORTAL;
#endif // UNICODE

typedef struct
{
    WCHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    WCHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    WCHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
} ISCSI_TARGET_PORTAL_INFOW, *PISCSI_TARGET_PORTAL_INFOW;

typedef struct
{
    CHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    CHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    CHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
} ISCSI_TARGET_PORTAL_INFOA, *PISCSI_TARGET_PORTAL_INFOA;

#ifdef UNICODE
typedef ISCSI_TARGET_PORTAL_INFOW  ISCSI_TARGET_PORTAL_INFO;
typedef PISCSI_TARGET_PORTAL_INFOW  PISCSI_TARGET_PORTAL_INFO;
#else
typedef ISCSI_TARGET_PORTAL_INFOA  ISCSI_TARGET_PORTAL_INFO;
typedef PISCSI_TARGET_PORTAL_INFOA  PISCSI_TARGET_PORTAL_INFO;
#endif // UNICODE

typedef struct
{
    WCHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    WCHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    WCHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
    ISCSI_SECURITY_FLAGS SecurityFlags;
    ISCSI_LOGIN_OPTIONS LoginOptions;
} ISCSI_TARGET_PORTAL_INFO_EXW, *PISCSI_TARGET_PORTAL_INFO_EXW;

typedef struct
{
    CHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    CHAR SymbolicName[MAX_ISCSI_PORTAL_NAME_LEN];
    CHAR Address[MAX_ISCSI_PORTAL_ADDRESS_LEN];
    USHORT Socket;
    ISCSI_SECURITY_FLAGS SecurityFlags;
    ISCSI_LOGIN_OPTIONS LoginOptions;
} ISCSI_TARGET_PORTAL_INFO_EXA, *PISCSI_TARGET_PORTAL_INFO_EXA;

#ifdef UNICODE
typedef ISCSI_TARGET_PORTAL_INFO_EXW  ISCSI_TARGET_PORTAL_INFO_EX;
typedef PISCSI_TARGET_PORTAL_INFO_EXW  PISCSI_TARGET_PORTAL_INFO_EX;
#else
typedef ISCSI_TARGET_PORTAL_INFO_EXA  ISCSI_TARGET_PORTAL_INFO_EX;
typedef PISCSI_TARGET_PORTAL_INFO_EXA  PISCSI_TARGET_PORTAL_INFO_EX;
#endif // UNICODE

typedef struct
{
    ULONG Count;
    ISCSI_TARGET_PORTALW Portals[1];
} ISCSI_TARGET_PORTAL_GROUPW, *PISCSI_TARGET_PORTAL_GROUPW;

typedef struct
{
    ULONG Count;
    ISCSI_TARGET_PORTALA Portals[1];
} ISCSI_TARGET_PORTAL_GROUPA, *PISCSI_TARGET_PORTAL_GROUPA;

#ifdef UNICODE
typedef ISCSI_TARGET_PORTAL_GROUPW ISCSI_TARGET_PORTAL_GROUP;
typedef PISCSI_TARGET_PORTAL_GROUPW PISCSI_TARGET_PORTAL_GROUP;
#else
typedef ISCSI_TARGET_PORTAL_GROUPA ISCSI_TARGET_PORTAL_GROUP;
typedef PISCSI_TARGET_PORTAL_GROUPA PISCSI_TARGET_PORTAL_GROUP;
#endif // UNICODE


typedef struct
{
    ISCSI_UNIQUE_CONNECTION_ID ConnectionId;  // TODO: add connectionid to mof
    PWCHAR InitiatorAddress;
    PWCHAR TargetAddress;
    USHORT InitiatorSocket;
    USHORT TargetSocket;
    UCHAR CID[2];
} ISCSI_CONNECTION_INFOW, *PISCSI_CONNECTION_INFOW;

typedef struct
{
    ISCSI_UNIQUE_SESSION_ID SessionId;
    PWCHAR InitiatorName;
    PWCHAR TargetNodeName;
    PWCHAR TargetName;
    UCHAR ISID[6];
    UCHAR TSID[2];
    ULONG ConnectionCount;
    PISCSI_CONNECTION_INFOW Connections;    
} ISCSI_SESSION_INFOW, *PISCSI_SESSION_INFOW;


typedef struct
{
    ISCSI_UNIQUE_CONNECTION_ID ConnectionId;  // TODO: add connectionid to mof
    PCHAR InitiatorAddress;
    PCHAR TargetAddress;
    USHORT InitiatorSocket;
    USHORT TargetSocket;
    UCHAR CID[2];
} ISCSI_CONNECTION_INFOA, *PISCSI_CONNECTION_INFOA;

typedef struct
{
    ISCSI_UNIQUE_SESSION_ID SessionId;
    PCHAR InitiatorName;
    PCHAR TargetNodeName;
    PCHAR TargetName;
    UCHAR ISID[6];
    UCHAR TSID[2];
    ULONG ConnectionCount;
    PISCSI_CONNECTION_INFOA Connections;    
} ISCSI_SESSION_INFOA, *PISCSI_SESSION_INFOA;


#ifdef UNICODE
typedef ISCSI_SESSION_INFOW ISCSI_SESSION_INFO;
typedef PISCSI_SESSION_INFOW PISCSI_SESSION_INFO;

typedef ISCSI_CONNECTION_INFOW ISCSI_CONNECTION_INFO;
typedef PISCSI_CONNECTION_INFOW PISCSI_CONNECTION_INFO;
#else
typedef ISCSI_SESSION_INFOA ISCSI_SESSION_INFO;
typedef PISCSI_SESSION_INFOA PISCSI_SESSION_INFO;

typedef ISCSI_CONNECTION_INFOA ISCSI_CONNECTION_INFO;
typedef PISCSI_CONNECTION_INFOA PISCSI_CONNECTION_INFO;
#endif

typedef struct
{
    ISCSI_UNIQUE_CONNECTION_ID ConnectionId;
    UCHAR State;
    UCHAR Protocol;
    UCHAR HeaderDigest;
    UCHAR DataDigest;
    ULONG MaxRecvDataSegmentLength;
    ISCSI_AUTH_TYPES AuthType;
    ULONGLONG EstimatedThroughput;
    ULONG MaxDatagramSize;
} ISCSI_CONNECTION_INFO_EX, *PISCSI_CONNECTION_INFO_EX;

typedef struct
{
    ISCSI_UNIQUE_SESSION_ID SessionId;
    BOOLEAN InitialR2t;
    BOOLEAN ImmediateData;
    UCHAR Type;
    BOOLEAN DataSequenceInOrder;
    BOOLEAN DataPduInOrder;
    UCHAR ErrorRecoveryLevel;
    ULONG MaxOutstandingR2t;
    ULONG FirstBurstLength;
    ULONG MaxBurstLength;
    ULONG MaximumConnections;
    ULONG ConnectionCount;
    PISCSI_CONNECTION_INFO_EX Connections;
} ISCSI_SESSION_INFO_EX, *PISCSI_SESSION_INFO_EX;

#ifndef MIDL_PASS
typedef struct
{
    WCHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    WCHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    SCSI_ADDRESS ScsiAddress;
    GUID DeviceInterfaceType;
    WCHAR DeviceInterfaceName[MAX_PATH];
    WCHAR LegacyName[MAX_PATH];
    STORAGE_DEVICE_NUMBER StorageDeviceNumber;
    DWORD /* DEVINST */ DeviceInstance;
} ISCSI_DEVICE_ON_SESSIONW, *PISCSI_DEVICE_ON_SESSIONW;

typedef struct
{
    CHAR InitiatorName[MAX_ISCSI_HBANAME_LEN];
    CHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    SCSI_ADDRESS ScsiAddress;
    GUID DeviceInterfaceType;
    CHAR DeviceInterfaceName[MAX_PATH];
    CHAR LegacyName[MAX_PATH];
    STORAGE_DEVICE_NUMBER StorageDeviceNumber;
    DWORD /* DEVINST */ DeviceInstance;
} ISCSI_DEVICE_ON_SESSIONA, *PISCSI_DEVICE_ON_SESSIONA;

#ifdef UNICODE
typedef ISCSI_DEVICE_ON_SESSIONW ISCSI_DEVICE_ON_SESSION;
typedef PISCSI_DEVICE_ON_SESSIONW PISCSI_DEVICE_ON_SESSION;
#else
typedef ISCSI_DEVICE_ON_SESSIONA ISCSI_DEVICE_ON_SESSION;
typedef PISCSI_DEVICE_ON_SESSIONA PISCSI_DEVICE_ON_SESSION;
#endif

#endif

typedef struct
{
    WCHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    BOOLEAN IsInformationalSession;
    WCHAR InitiatorInstance[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    ISCSI_TARGET_PORTALW TargetPortal;
    ISCSI_SECURITY_FLAGS SecurityFlags;
    PISCSI_TARGET_MAPPINGW Mappings;
    ISCSI_LOGIN_OPTIONS LoginOptions;
} PERSISTENT_ISCSI_LOGIN_INFOW, *PPERSISTENT_ISCSI_LOGIN_INFOW;

typedef struct
{
    CHAR TargetName[MAX_ISCSI_NAME_LEN + 1];
    BOOLEAN IsInformationalSession;
    CHAR InitiatorInstance[MAX_ISCSI_HBANAME_LEN];
    ULONG InitiatorPortNumber;
    ISCSI_TARGET_PORTALA TargetPortal;
    ISCSI_SECURITY_FLAGS SecurityFlags;
    PISCSI_TARGET_MAPPINGA Mappings;
    ISCSI_LOGIN_OPTIONS LoginOptions;
} PERSISTENT_ISCSI_LOGIN_INFOA, *PPERSISTENT_ISCSI_LOGIN_INFOA;

#ifdef UNICODE
typedef PERSISTENT_ISCSI_LOGIN_INFOW PERSISTENT_ISCSI_LOGIN_INFO;
typedef PPERSISTENT_ISCSI_LOGIN_INFOW PPERSISTENT_ISCSI_LOGIN_INFO;
#else
typedef PERSISTENT_ISCSI_LOGIN_INFOA PERSISTENT_ISCSI_LOGIN_INFO;
typedef PPERSISTENT_ISCSI_LOGIN_INFOA PPERSISTENT_ISCSI_LOGIN_INFO;
#endif


//
// definitions of information that can be queried and set via the
// GetIScsiTargetInformation and SetIScsiTargetInformation apis
//
typedef enum
{
    ISCSI_TCP_PROTOCOL_TYPE
} TARGETPROTOCOLTYPE, *PTARGETPROTOCOLTYPE;

typedef enum
{
                        // Requires DiscoveryMechansim parameter
    ProtocolType,       // Protocol used to communicate with TARGET - TARGETPROTOCOLTYPE

                        // Requires DiscoveryMechansim parameter
    TargetAlias,        // Alias Name - WCHAR Alias[MAX_ISCSI_TARGET_ALIAS_LEN]

                        // 
    DiscoveryMechanisms, // Discovery mechanisms - strings

                        // Requires DiscoveryMechansim parameter
    PortalGroups,       // Portal Groups - array of portal groups, preceeded by a ULONG count. Each Portal Group must be aliagned on a 4 byte boundry

                        // Requires DiscoveryMechansim parameter
                        // ISCSI_TARGET_MAPPING
    PersistentTargetMappings,
    
                        // Requires DiscoveryMechansim parameter
    InitiatorName,      // Name of initiator that can connect to target
    
                        // Requires DiscoveryMechansim parameter
    TargetFlags,        // TargetFlags

    LoginOptions        // Requires DiscoveryMechansim parameter
                        // ISCSI_LOGIN_OPTIONS
               
} TARGET_INFORMATION_CLASS, *PTARGET_INFORMATION_CLASS;


typedef struct
{
    ULONG MajorVersion;
    ULONG MinorVersion;
    ULONG BuildNumber;
} ISCSI_VERSION_INFO, *PISCSI_VERSION_INFO;



#if ! (defined(MIDL_PASS))

ISDSC_STATUS ISDSC_API GetIScsiVersionInformation(
    PISCSI_VERSION_INFO VersionInfo
    );

ISDSC_STATUS ISDSC_API GetIScsiTargetInformationW(
    IN _In_ PWSTR TargetName,
    IN _In_opt_ PWSTR DiscoveryMechanism,
    IN TARGET_INFORMATION_CLASS InfoClass,
    IN OUT PULONG BufferSize,
    OUT PVOID Buffer
    );

ISDSC_STATUS ISDSC_API GetIScsiTargetInformationA(
    IN _In_ PSTR TargetName,
    IN _In_opt_ PSTR DiscoveryMechanism,
    IN TARGET_INFORMATION_CLASS InfoClass,
    IN OUT PULONG BufferSize,
    OUT PVOID Buffer
    );

#ifdef UNICODE
#define GetIScsiTargetInformation GetIScsiTargetInformationW
#else
#define GetIScsiTargetInformation GetIScsiTargetInformationA
#endif // UNICODE

ISDSC_STATUS ISDSC_API AddIScsiConnectionW(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN PVOID Reserved,
    IN ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALW TargetPortal,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ULONG KeySize,
    IN OPTIONAL _In_reads_opt_(KeySize) PCHAR Key,
    OUT OPTIONAL PISCSI_UNIQUE_CONNECTION_ID ConnectionId
    );
    
ISDSC_STATUS ISDSC_API AddIScsiConnectionA(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN PVOID Reserved,
    IN ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALA TargetPortal,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ULONG KeySize,
    IN OPTIONAL _In_reads_opt_(KeySize) PCHAR Key,
    OUT OPTIONAL PISCSI_UNIQUE_CONNECTION_ID ConnectionId
    );
    
#ifdef UNICODE
#define AddIScsiConnection AddIScsiConnectionW
#else
#define AddIScsiConnection AddIScsiConnectionA
#endif // UNICODE


ISDSC_STATUS ISDSC_API RemoveIScsiConnection(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN PISCSI_UNIQUE_CONNECTION_ID ConnectionId
    );

ISDSC_STATUS ISDSC_API ReportIScsiTargetsW(
    IN BOOLEAN ForceUpdate,
    IN OUT PULONG BufferSize,
    OUT _Inout_updates_opt_(*BufferSize) PWCHAR Buffer
    );

ISDSC_STATUS ISDSC_API ReportIScsiTargetsA(
    IN BOOLEAN ForceUpdate,
    IN OUT PULONG BufferSize,
    OUT _Inout_updates_opt_(*BufferSize) PCHAR Buffer
    );

#ifdef UNICODE
#define ReportIScsiTargets ReportIScsiTargetsW
#else
#define ReportIScsiTargets ReportIScsiTargetsA
#endif


ISDSC_STATUS ISDSC_API AddIScsiStaticTargetW(
    IN _In_ PWSTR TargetName,
    IN OPTIONAL _In_opt_ PWSTR TargetAlias,
    IN ISCSI_TARGET_FLAGS TargetFlags,
    IN BOOLEAN Persist,
    IN OPTIONAL PISCSI_TARGET_MAPPINGW Mappings,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL PISCSI_TARGET_PORTAL_GROUPW PortalGroup
    );

ISDSC_STATUS ISDSC_API AddIScsiStaticTargetA(
    IN _In_ PSTR TargetName,
    IN OPTIONAL _In_opt_ PSTR TargetAlias,
    IN ISCSI_TARGET_FLAGS TargetFlags,
    IN BOOLEAN Persist,
    IN OPTIONAL PISCSI_TARGET_MAPPINGA Mappings,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL PISCSI_TARGET_PORTAL_GROUPA PortalGroup
    );

#ifdef UNICODE
#define AddIScsiStaticTarget AddIScsiStaticTargetW
#else
#define AddIScsiStaticTarget AddIScsiStaticTargetA
#endif

ISDSC_STATUS ISDSC_API RemoveIScsiStaticTargetW(
    IN _In_ PWSTR TargetName
    );

ISDSC_STATUS ISDSC_API RemoveIScsiStaticTargetA(
    IN _In_ PSTR TargetName
    );

#ifdef UNICODE
#define RemoveIScsiStaticTarget RemoveIScsiStaticTargetW
#else
#define RemoveIScsiStaticTarget RemoveIScsiStaticTargetA
#endif

ISDSC_STATUS ISDSC_API AddIScsiSendTargetPortalW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags, 
    IN PISCSI_TARGET_PORTALW Portal
    );

ISDSC_STATUS ISDSC_API AddIScsiSendTargetPortalA(
    IN OPTIONAL _In_opt_ PSTR InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags, 
    IN PISCSI_TARGET_PORTALA Portal
    );

#ifdef UNICODE
#define AddIScsiSendTargetPortal AddIScsiSendTargetPortalW
#else
#define AddIScsiSendTargetPortal AddIScsiSendTargetPortalA
#endif

ISDSC_STATUS ISDSC_API RemoveIScsiSendTargetPortalW(
    IN OPTIONAL PWSTR _In_opt_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALW Portal
    );

ISDSC_STATUS ISDSC_API RemoveIScsiSendTargetPortalA(
    IN OPTIONAL PSTR _In_opt_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALA Portal
    );

#ifdef UNICODE
#define RemoveIScsiSendTargetPortal RemoveIScsiSendTargetPortalW
#else
#define RemoveIScsiSendTargetPortal RemoveIScsiSendTargetPortalA
#endif

ISDSC_STATUS ISDSC_API RefreshIScsiSendTargetPortalW(
    IN OPTIONAL PWSTR _In_opt_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALW Portal
    );

ISDSC_STATUS ISDSC_API RefreshIScsiSendTargetPortalA(
    IN OPTIONAL PSTR _In_opt_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PISCSI_TARGET_PORTALA Portal
    );

#ifdef UNICODE
#define RefreshIScsiSendTargetPortal RefreshIScsiSendTargetPortalW
#else
#define RefreshIScsiSendTargetPortal RefreshIScsiSendTargetPortalA
#endif


ISDSC_STATUS ISDSC_API ReportIScsiSendTargetPortalsW(
    IN OUT PULONG PortalCount,
    IN OUT PISCSI_TARGET_PORTAL_INFOW PortalInfo
    );

ISDSC_STATUS ISDSC_API ReportIScsiSendTargetPortalsA(
    IN OUT PULONG PortalCount,
    IN OUT PISCSI_TARGET_PORTAL_INFOA PortalInfo
    );
#ifdef UNICODE
#define ReportIScsiSendTargetPortals ReportIScsiSendTargetPortalsW
#else
#define ReportIScsiSendTargetPortals ReportIScsiSendTargetPortalsA
#endif

ISDSC_STATUS ISDSC_API ReportIScsiSendTargetPortalsExW(
    OUT PULONG PortalCount,
    IN OUT PULONG PortalInfoSize,
    IN OUT PISCSI_TARGET_PORTAL_INFO_EXW PortalInfo
    );

ISDSC_STATUS ISDSC_API ReportIScsiSendTargetPortalsExA(
    OUT PULONG PortalCount,
    IN OUT PULONG PortalInfoSize,
    IN OUT PISCSI_TARGET_PORTAL_INFO_EXA PortalInfo
    );
#ifdef UNICODE
#define ReportIScsiSendTargetPortalsEx ReportIScsiSendTargetPortalsExW
#else
#define ReportIScsiSendTargetPortalsEx ReportIScsiSendTargetPortalsExA
#endif


ISDSC_STATUS ISDSC_API LoginIScsiTargetW(
    IN _In_ PWSTR TargetName,
    IN BOOLEAN IsInformationalSession,
    IN OPTIONAL _In_opt_ PWSTR InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN OPTIONAL PISCSI_TARGET_PORTALW TargetPortal,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags,
    IN OPTIONAL PISCSI_TARGET_MAPPINGW Mappings,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ULONG KeySize,
    IN OPTIONAL _In_reads_opt_(KeySize) PCHAR Key,
    IN BOOLEAN IsPersistent,
    OUT PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    OUT PISCSI_UNIQUE_CONNECTION_ID UniqueConnectionId
    );

ISDSC_STATUS ISDSC_API LoginIScsiTargetA(
    IN PSTR _In_ TargetName,
    IN BOOLEAN IsInformationalSession,
    IN OPTIONAL _In_opt_ PSTR InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN OPTIONAL PISCSI_TARGET_PORTALA TargetPortal,
    IN OPTIONAL ISCSI_SECURITY_FLAGS SecurityFlags,
    IN OPTIONAL PISCSI_TARGET_MAPPINGA Mappings,
    IN OPTIONAL PISCSI_LOGIN_OPTIONS LoginOptions,
    IN OPTIONAL ULONG KeySize,
    IN OPTIONAL _In_reads_opt_(KeySize) PCHAR Key,
    IN BOOLEAN IsPersistent,
    OUT PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    OUT PISCSI_UNIQUE_CONNECTION_ID UniqueConnectionId
    );

#ifdef UNICODE
#define LoginIScsiTarget LoginIScsiTargetW
#else
#define LoginIScsiTarget LoginIScsiTargetA
#endif


ISDSC_STATUS ISDSC_API ReportIScsiPersistentLoginsW(
    OUT ULONG *Count,
    IN OUT PPERSISTENT_ISCSI_LOGIN_INFOW PersistentLoginInfo,
    IN OUT PULONG BufferSizeInBytes
    );

ISDSC_STATUS ISDSC_API ReportIScsiPersistentLoginsA(
    OUT ULONG *Count,
    IN OUT PPERSISTENT_ISCSI_LOGIN_INFOA PersistentLoginInfo,
    IN OUT PULONG BufferSizeInBytes
    );

#ifdef UNICODE
#define ReportIScsiPersistentLogins ReportIScsiPersistentLoginsW
#else
#define ReportIScsiPersistentLogins ReportIScsiPersistentLoginsA
#endif


                                    
ISDSC_STATUS ISDSC_API LogoutIScsiTarget(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId
    );

ISDSC_STATUS ISDSC_API RemoveIScsiPersistentTargetW(
    IN PWSTR _In_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PWSTR _In_ TargetName,
    IN PISCSI_TARGET_PORTALW Portal
    );

ISDSC_STATUS ISDSC_API RemoveIScsiPersistentTargetA(
    IN PSTR _In_ InitiatorInstance,
    IN OPTIONAL ULONG InitiatorPortNumber,
    IN PSTR _In_ TargetName,
    IN PISCSI_TARGET_PORTALA Portal
    );

#ifdef UNICODE
#define RemoveIScsiPersistentTarget RemoveIScsiPersistentTargetW
#else
#define RemoveIScsiPersistentTarget RemoveIScsiPersistentTargetA
#endif

ISDSC_STATUS ISDSC_API SendScsiInquiry(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN ULONGLONG Lun,
    IN UCHAR EvpdCmddt,
    IN UCHAR PageCode,
    OUT PUCHAR ScsiStatus,
    IN OUT PULONG ResponseSize,
    OUT PUCHAR ResponseBuffer,
    IN OUT PULONG SenseSize,
    OUT PUCHAR SenseBuffer
    );

ISDSC_STATUS ISDSC_API SendScsiReadCapacity(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN ULONGLONG Lun,
    OUT PUCHAR ScsiStatus,
    IN OUT PULONG ResponseSize,
    OUT PUCHAR ResponseBuffer,
    IN OUT PULONG SenseSize,
    OUT PUCHAR SenseBuffer
    );

ISDSC_STATUS ISDSC_API SendScsiReportLuns(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    OUT PUCHAR ScsiStatus,
    IN OUT PULONG ResponseSize,
    OUT PUCHAR ResponseBuffer,
    IN OUT PULONG SenseSize,
    OUT PUCHAR SenseBuffer
    );

ISDSC_STATUS ISDSC_API ReportIScsiInitiatorListW(
    IN OUT ULONG *BufferSize,
    OUT _Inout_updates_opt_(*BufferSize) PWCHAR Buffer
    );

ISDSC_STATUS ISDSC_API ReportIScsiInitiatorListA(
    IN OUT PULONG BufferSize,
    OUT _Inout_updates_opt_(*BufferSize) PCHAR Buffer
    );

#ifdef UNICODE
#define ReportIScsiInitiatorList ReportIScsiInitiatorListW
#else
#define ReportIScsiInitiatorList ReportIScsiInitiatorListA
#endif


ISDSC_STATUS ISDSC_API ReportActiveIScsiTargetMappingsW(
    IN OUT PULONG BufferSize,
    OUT PULONG MappingCount,
    OUT PISCSI_TARGET_MAPPINGW Mappings
    );

ISDSC_STATUS ISDSC_API ReportActiveIScsiTargetMappingsA(
    IN OUT PULONG BufferSize,
    OUT PULONG MappingCount,
    OUT PISCSI_TARGET_MAPPINGA Mappings
    );

#ifdef UNICODE
#define ReportActiveIScsiTargetMappings ReportActiveIScsiTargetMappingsW
#else
#define ReportActiveIScsiTargetMappings ReportActiveIScsiTargetMappingsA
#endif

ISDSC_STATUS ISDSC_API SetIScsiTunnelModeOuterAddressW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PWSTR _In_opt_ DestinationAddress,
    IN PWSTR _In_opt_ OuterModeAddress,
    IN BOOLEAN Persist
    );
    
ISDSC_STATUS ISDSC_API SetIScsiTunnelModeOuterAddressA(
    IN OPTIONAL _In_opt_ PSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PSTR _In_opt_ DestinationAddress,
    IN PSTR _In_opt_ OuterModeAddress,
    IN BOOLEAN Persist
    );

#ifdef UNICODE
#define SetIScsiTunnelModeOuterAddress SetIScsiTunnelModeOuterAddressW
#else
#define SetIScsiTunnelModeOuterAddress SetIScsiTunnelModeOuterAddressA
#endif


ISDSC_STATUS ISDSC_API SetIScsiIKEInfoW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PIKE_AUTHENTICATION_INFORMATION AuthInfo,
    IN BOOLEAN Persist
    );

ISDSC_STATUS ISDSC_API SetIScsiIKEInfoA(
    IN OPTIONAL _In_opt_ PSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PIKE_AUTHENTICATION_INFORMATION AuthInfo,
    IN BOOLEAN Persist
    );

#ifdef UNICODE
#define SetIScsiIKEInfo SetIScsiIKEInfoW
#else
#define SetIScsiIKEInfo SetIScsiIKEInfoA
#endif

ISDSC_STATUS ISDSC_API GetIScsiIKEInfoW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PULONG Reserved,                                        
    IN OUT PIKE_AUTHENTICATION_INFORMATION AuthInfo
    );

ISDSC_STATUS ISDSC_API GetIScsiIKEInfoA(
    IN OPTIONAL _In_opt_ PSTR InitiatorName,
    IN ULONG InitiatorPortNumber,
    IN PULONG Reserved,                                        
    IN OUT PIKE_AUTHENTICATION_INFORMATION AuthInfo
    );

#ifdef UNICODE
#define GetIScsiIKEInfo GetIScsiIKEInfoW
#else
#define GetIScsiIKEInfo GetIScsiIKEInfoA
#endif

ISDSC_STATUS ISDSC_API SetIScsiGroupPresharedKey(
    IN ULONG KeyLength,
    IN PUCHAR Key,
    BOOLEAN Persist
    );

ISDSC_STATUS ISDSC_API SetIScsiInitiatorCHAPSharedSecret(
    IN ULONG SharedSecretLength,
    IN PUCHAR SharedSecret
    );

ISDSC_STATUS ISDSC_API SetIScsiInitiatorRADIUSSharedSecret(
    IN ULONG SharedSecretLength,
    IN PUCHAR SharedSecret
    );

ISDSC_STATUS ISDSC_API SetIScsiInitiatorNodeNameW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorNodeName
    );

ISDSC_STATUS ISDSC_API SetIScsiInitiatorNodeNameA(
    IN OPTIONAL _In_opt_ PSTR InitiatorNodeName
    );

#ifdef UNICODE
#define SetIScsiInitiatorNodeName SetIScsiInitiatorNodeNameW
#else
#define SetIScsiInitiatorNodeName SetIScsiInitiatorNodeNameA
#endif

ISDSC_STATUS ISDSC_API GetIScsiInitiatorNodeNameW(
     _Out_writes_(MAX_ISCSI_NAME_LEN+1) PWCHAR  InitiatorNodeName
    );

ISDSC_STATUS ISDSC_API GetIScsiInitiatorNodeNameA(
     _Out_writes_(MAX_ISCSI_NAME_LEN+1) PCHAR  InitiatorNodeName
    );

#ifdef UNICODE
#define GetIScsiInitiatorNodeName GetIScsiInitiatorNodeNameW
#else
#define GetIScsiInitiatorNodeName GetIScsiInitiatorNodeNameA
#endif

ISDSC_STATUS ISDSC_API AddISNSServerW(
    IN _In_ PWSTR Address
    );

ISDSC_STATUS ISDSC_API AddISNSServerA(
    IN _In_ PSTR Address
    );

#ifdef UNICODE
#define AddISNSServer AddISNSServerW
#else
#define AddISNSServer AddISNSServerA
#endif


ISDSC_STATUS ISDSC_API RemoveISNSServerW(
    IN _In_ PWSTR Address
    );

ISDSC_STATUS ISDSC_API RemoveISNSServerA(
    IN _In_ PSTR Address
    );
#ifdef UNICODE
#define RemoveISNSServer RemoveISNSServerW
#else
#define RemoveISNSServer RemoveISNSServerA
#endif

ISDSC_STATUS ISDSC_API RefreshISNSServerW(
    IN _In_ PWSTR Address
    );

ISDSC_STATUS ISDSC_API RefreshISNSServerA(
    IN _In_ PSTR Address
    );
#ifdef UNICODE
#define RefreshISNSServer RefreshISNSServerW
#else
#define RefreshISNSServer RefreshISNSServerA
#endif

ISDSC_STATUS ISDSC_API ReportISNSServerListW(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PWCHAR Buffer
    );

ISDSC_STATUS ISDSC_API ReportISNSServerListA(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PCHAR Buffer
    );
#ifdef UNICODE
#define ReportISNSServerList ReportISNSServerListW
#else
#define ReportISNSServerList ReportISNSServerListA
#endif



ISDSC_STATUS ISDSC_API GetIScsiSessionListW(
    IN OUT ULONG *BufferSize,
    OUT ULONG *SessionCount,
    OUT OPTIONAL PISCSI_SESSION_INFOW SessionInfo
    );

ISDSC_STATUS ISDSC_API GetIScsiSessionListA(
    IN OUT ULONG *BufferSize,
    OUT ULONG *SessionCount,
    OUT OPTIONAL PISCSI_SESSION_INFOA SessionInfo
    );
#ifdef UNICODE
#define GetIScsiSessionList GetIScsiSessionListW
#else
#define GetIScsiSessionList GetIScsiSessionListA
#endif

ISDSC_STATUS ISDSC_API GetIScsiSessionListEx (
    IN OUT ULONG  *BufferSize,
    OUT ULONG  *SessionCountPtr,
    OUT OPTIONAL PISCSI_SESSION_INFO_EX  SessionInfo
);

ISDSC_STATUS ISDSC_API GetDevicesForIScsiSessionW(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN OUT ULONG *DeviceCount,
    OUT PISCSI_DEVICE_ON_SESSIONW Devices
);

ISDSC_STATUS ISDSC_API GetDevicesForIScsiSessionA(
    IN PISCSI_UNIQUE_SESSION_ID UniqueSessionId,
    IN OUT ULONG *DeviceCount,
    OUT PISCSI_DEVICE_ON_SESSIONA Devices
);

#ifdef UNICODE
#define GetDevicesForIScsiSession GetDevicesForIScsiSessionW
#else
#define GetDevicesForIScsiSession GetDevicesForIScsiSessionA
#endif

ISDSC_STATUS ISDSC_API SetupPersistentIScsiVolumes(
    );

ISDSC_STATUS ISDSC_API SetupPersistentIScsiDevices(
    );

ISDSC_STATUS ISDSC_API AddPersistentIScsiDeviceW(
    IN _In_ PWSTR DevicePath
);

ISDSC_STATUS ISDSC_API AddPersistentIScsiDeviceA(
    IN _In_ PSTR DevicePath
);

#ifdef UNICODE
#define AddPersistentIScsiDevice AddPersistentIScsiDeviceW
#else
#define AddPersistentIScsiDevice AddPersistentIScsiDeviceA
#endif

ISDSC_STATUS ISDSC_API RemovePersistentIScsiDeviceW(
    IN _In_ PWSTR DevicePath
);

ISDSC_STATUS ISDSC_API RemovePersistentIScsiDeviceA(
    IN _In_ PSTR DevicePath
);

#ifdef UNICODE
#define RemovePersistentIScsiDevice RemovePersistentIScsiDeviceW
#else
#define RemovePersistentIScsiDevice RemovePersistentIScsiDeviceA
#endif

ISDSC_STATUS ISDSC_API ClearPersistentIScsiDevices(
);

ISDSC_STATUS ISDSC_API ReportPersistentIScsiDevicesW(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PWCHAR Buffer
);

ISDSC_STATUS ISDSC_API ReportPersistentIScsiDevicesA(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PCHAR Buffer
);

#ifdef UNICODE
#define ReportPersistentIScsiDevices ReportPersistentIScsiDevicesW
#else
#define ReportPersistentIScsiDevices ReportPersistentIScsiDevicesA
#endif

ISDSC_STATUS ISDSC_API ReportIScsiTargetPortalsW(
    IN OPTIONAL _In_opt_ PWSTR InitiatorName,
    IN PWSTR _In_ TargetName,
    IN OPTIONAL PUSHORT TargetPortalTag,
    IN OUT PULONG ElementCount,
    OUT PISCSI_TARGET_PORTALW Portals
    );

ISDSC_STATUS ISDSC_API ReportIScsiTargetPortalsA(
    IN OPTIONAL _In_opt_ PSTR InitiatorName,
    IN PSTR _In_ TargetName,
    IN OPTIONAL PUSHORT TargetPortalTag,
    IN OUT PULONG ElementCount,
    OUT PISCSI_TARGET_PORTALA Portals
    );

#ifdef UNICODE
#define ReportIScsiTargetPortals ReportIScsiTargetPortalsW
#else
#define ReportIScsiTargetPortals ReportIScsiTargetPortalsA
#endif

ISDSC_STATUS ISDSC_API AddRadiusServerW(
    IN _In_ PWSTR Address
    );

ISDSC_STATUS ISDSC_API AddRadiusServerA(
    IN _In_ PSTR Address
    );

#ifdef UNICODE
#define AddRadiusServer AddRadiusServerW
#else
#define AddRadiusServer AddRadiusServerA
#endif


ISDSC_STATUS ISDSC_API RemoveRadiusServerW(
    IN _In_ PWSTR Address
    );

ISDSC_STATUS ISDSC_API RemoveRadiusServerA(
    IN _In_ PSTR Address
    );
#ifdef UNICODE
#define RemoveRadiusServer RemoveRadiusServerW
#else
#define RemoveRadiusServer RemoveRadiusServerA
#endif


ISDSC_STATUS ISDSC_API ReportRadiusServerListW(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PWCHAR Buffer
    );

ISDSC_STATUS ISDSC_API ReportRadiusServerListA(
    IN OUT PULONG BufferSizeInChar,
    OUT _Out_writes_opt_(*BufferSizeInChar) PCHAR Buffer
    );
#ifdef UNICODE
#define ReportRadiusServerList ReportRadiusServerListW
#else
#define ReportRadiusServerList ReportRadiusServerListA
#endif

#endif // MIDL_PASS

#endif // ! MOFCOMP_PASS

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_ISCSI)

#ifdef __cplusplus
}
#endif

#endif

