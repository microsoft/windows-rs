/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    sddl.w

Abstract:

    This module defines the support and conversions routines necessary for SDDL.

Revision History:

--*/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef __SDDL_H__
#define __SDDL_H__

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <minwindef.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// SDDL Version information
//
#define SDDL_REVISION_1     1
#define SDDL_REVISION       SDDL_REVISION_1

//
// SDDL Component tags
//
#define SDDL_OWNER                          TEXT("O")       // Owner tag
#define SDDL_GROUP                          TEXT("G")       // Group tag
#define SDDL_DACL                           TEXT("D")       // DACL tag
#define SDDL_SACL                           TEXT("S")       // SACL tag

//
// SDDL Security descriptor controls
//
#define SDDL_PROTECTED                      TEXT("P")       // DACL or SACL Protected
#define SDDL_AUTO_INHERIT_REQ               TEXT("AR")      // Auto inherit request
#define SDDL_AUTO_INHERITED                 TEXT("AI")      // DACL/SACL are auto inherited
#define SDDL_NULL_ACL                       TEXT("NO_ACCESS_CONTROL")    // Null ACL

//
// SDDL Ace types
//
#define SDDL_ACCESS_ALLOWED                 TEXT("A")   // Access allowed
#define SDDL_ACCESS_DENIED                  TEXT("D")   // Access denied
#define SDDL_OBJECT_ACCESS_ALLOWED          TEXT("OA")  // Object access allowed
#define SDDL_OBJECT_ACCESS_DENIED           TEXT("OD")  // Object access denied
#define SDDL_AUDIT                          TEXT("AU")  // Audit
#define SDDL_ALARM                          TEXT("AL")  // Alarm
#define SDDL_OBJECT_AUDIT                   TEXT("OU")  // Object audit
#define SDDL_OBJECT_ALARM                   TEXT("OL")  // Object alarm
#define SDDL_MANDATORY_LABEL                TEXT("ML")  // Integrity label
#define SDDL_PROCESS_TRUST_LABEL            TEXT("TL")  // Process trust label
#define SDDL_CALLBACK_ACCESS_ALLOWED        TEXT("XA")  // Callback access allowed
#define SDDL_CALLBACK_ACCESS_DENIED         TEXT("XD")  // Callback access denied
#define SDDL_RESOURCE_ATTRIBUTE             TEXT("RA")  // Resource attribute
#define SDDL_SCOPED_POLICY_ID               TEXT("SP")  // Scoped policy
#define SDDL_CALLBACK_AUDIT                 TEXT("XU")  // Callback audit
#define SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED TEXT("ZA")  // Callback object access allowed
#define SDDL_ACCESS_FILTER                  TEXT("FL")  // Access Filter

//
// SDDL Resource attribute ace data types
//

#define SDDL_INT                            TEXT("TI")   // Signed integer
#define SDDL_UINT                           TEXT("TU")   // Unsigned integer
#define SDDL_WSTRING                        TEXT("TS")   // Wide string
#define SDDL_SID                            TEXT("TD")   // SID
#define SDDL_BLOB                           TEXT("TX")   // Octet String
#define SDDL_BOOLEAN                        TEXT("TB")   // Boolean

//
// SDDL Ace flags
//
#define SDDL_CONTAINER_INHERIT              TEXT("CI")  // Container inherit
#define SDDL_OBJECT_INHERIT                 TEXT("OI")  // Object inherit
#define SDDL_NO_PROPAGATE                   TEXT("NP")  // Inherit no propagate
#define SDDL_INHERIT_ONLY                   TEXT("IO")  // Inherit only
#define SDDL_INHERITED                      TEXT("ID")  // Inherited
#define SDDL_CRITICAL                       TEXT("CR")  // Critical
#define SDDL_TRUST_PROTECTED_FILTER         TEXT("TP")  // Trust Protected Filter
#define SDDL_AUDIT_SUCCESS                  TEXT("SA")  // Audit success
#define SDDL_AUDIT_FAILURE                  TEXT("FA")  // Audit failure

//
// SDDL Rights
//
#define SDDL_READ_PROPERTY                  TEXT("RP")
#define SDDL_WRITE_PROPERTY                 TEXT("WP")
#define SDDL_CREATE_CHILD                   TEXT("CC")
#define SDDL_DELETE_CHILD                   TEXT("DC")
#define SDDL_LIST_CHILDREN                  TEXT("LC")
#define SDDL_SELF_WRITE                     TEXT("SW")
#define SDDL_LIST_OBJECT                    TEXT("LO")
#define SDDL_DELETE_TREE                    TEXT("DT")
#define SDDL_CONTROL_ACCESS                 TEXT("CR")
#define SDDL_READ_CONTROL                   TEXT("RC")
#define SDDL_WRITE_DAC                      TEXT("WD")
#define SDDL_WRITE_OWNER                    TEXT("WO")
#define SDDL_STANDARD_DELETE                TEXT("SD")
#define SDDL_GENERIC_ALL                    TEXT("GA")
#define SDDL_GENERIC_READ                   TEXT("GR")
#define SDDL_GENERIC_WRITE                  TEXT("GW")
#define SDDL_GENERIC_EXECUTE                TEXT("GX")
#define SDDL_FILE_ALL                       TEXT("FA")
#define SDDL_FILE_READ                      TEXT("FR")
#define SDDL_FILE_WRITE                     TEXT("FW")
#define SDDL_FILE_EXECUTE                   TEXT("FX")
#define SDDL_KEY_ALL                        TEXT("KA")
#define SDDL_KEY_READ                       TEXT("KR")
#define SDDL_KEY_WRITE                      TEXT("KW")
#define SDDL_KEY_EXECUTE                    TEXT("KX")
#define SDDL_NO_WRITE_UP                    TEXT("NW")
#define SDDL_NO_READ_UP                     TEXT("NR")
#define SDDL_NO_EXECUTE_UP                  TEXT("NX")

//
// SDDL User alias max size
//      - currently, upto two supported eg. "DA"
//      - modify this if more WCHARs need to be there in future e.g. "DAX"
//

#define SDDL_ALIAS_SIZE                     2

//
// SDDL User aliases
//
#define SDDL_DOMAIN_ADMINISTRATORS          TEXT("DA")      // Domain admins
#define SDDL_DOMAIN_GUESTS                  TEXT("DG")      // Domain guests
#define SDDL_DOMAIN_USERS                   TEXT("DU")      // Domain users
#define SDDL_ENTERPRISE_DOMAIN_CONTROLLERS  TEXT("ED")      // Enterprise domain controllers
#define SDDL_DOMAIN_DOMAIN_CONTROLLERS      TEXT("DD")      // Domain domain controllers
#define SDDL_DOMAIN_COMPUTERS               TEXT("DC")      // Domain computers
#define SDDL_BUILTIN_ADMINISTRATORS         TEXT("BA")      // Builtin (local ) administrators
#define SDDL_BUILTIN_GUESTS                 TEXT("BG")      // Builtin (local ) guests
#define SDDL_BUILTIN_USERS                  TEXT("BU")      // Builtin (local ) users
#define SDDL_LOCAL_ADMIN                    TEXT("LA")      // Local administrator account
#define SDDL_LOCAL_GUEST                    TEXT("LG")      // Local group account
#define SDDL_ACCOUNT_OPERATORS              TEXT("AO")      // Account operators
#define SDDL_BACKUP_OPERATORS               TEXT("BO")      // Backup operators
#define SDDL_PRINTER_OPERATORS              TEXT("PO")      // Printer operators
#define SDDL_SERVER_OPERATORS               TEXT("SO")      // Server operators
#define SDDL_AUTHENTICATED_USERS            TEXT("AU")      // Authenticated users
#define SDDL_PERSONAL_SELF                  TEXT("PS")      // Personal self
#define SDDL_CREATOR_OWNER                  TEXT("CO")      // Creator owner
#define SDDL_CREATOR_GROUP                  TEXT("CG")      // Creator group
#define SDDL_LOCAL_SYSTEM                   TEXT("SY")      // Local system
#define SDDL_POWER_USERS                    TEXT("PU")      // Power users
#define SDDL_EVERYONE                       TEXT("WD")      // Everyone ( World )
#define SDDL_REPLICATOR                     TEXT("RE")      // Replicator
#define SDDL_INTERACTIVE                    TEXT("IU")      // Interactive logon user
#define SDDL_NETWORK                        TEXT("NU")      // Nework logon user
#define SDDL_SERVICE                        TEXT("SU")      // Service logon user
#define SDDL_RESTRICTED_CODE                TEXT("RC")      // Restricted code
#define SDDL_WRITE_RESTRICTED_CODE          TEXT("WR")      // Write Restricted code
#define SDDL_ANONYMOUS                      TEXT("AN")      // Anonymous Logon
#define SDDL_SCHEMA_ADMINISTRATORS          TEXT("SA")      // Schema Administrators
#define SDDL_CERT_SERV_ADMINISTRATORS       TEXT("CA")      // Certificate Server Administrators
#define SDDL_RAS_SERVERS                    TEXT("RS")      // RAS servers group
#define SDDL_ENTERPRISE_ADMINS              TEXT("EA")      // Enterprise administrators
#define SDDL_GROUP_POLICY_ADMINS            TEXT("PA")      // Group Policy administrators
#define SDDL_ALIAS_PREW2KCOMPACC            TEXT("RU")      // alias to allow previous windows 2000
#define SDDL_LOCAL_SERVICE                  TEXT("LS")      // Local service account (for services)
#define SDDL_NETWORK_SERVICE                TEXT("NS")      // Network service account (for services)
#define SDDL_REMOTE_DESKTOP                 TEXT("RD")      // Remote desktop users (for terminal server)
#define SDDL_NETWORK_CONFIGURATION_OPS      TEXT("NO")      // Network configuration operators ( to manage configuration of networking features)
#define SDDL_PERFMON_USERS                  TEXT("MU")      // Performance Monitor Users
#define SDDL_PERFLOG_USERS                  TEXT("LU")      // Performance Log Users
#define SDDL_IIS_USERS                      TEXT("IS")      // Anonymous Internet Users
#define SDDL_CRYPTO_OPERATORS               TEXT("CY")      // Crypto Operators
#define SDDL_OWNER_RIGHTS                   TEXT("OW")      // Owner Rights SID
#define SDDL_EVENT_LOG_READERS              TEXT("ER")      // Event log readers
#define SDDL_ENTERPRISE_RO_DCs              TEXT("RO")      // Enterprise Read-only domain controllers
#define SDDL_CERTSVC_DCOM_ACCESS            TEXT("CD")      // Users who can connect to certification authorities using DCOM
#define SDDL_ALL_APP_PACKAGES               TEXT("AC")      // All applications running in an app package context
#define SDDL_RDS_REMOTE_ACCESS_SERVERS      TEXT("RA")      // Servers in this group enable users of RemoteApp programs and personal virtual desktops access to these resources.
#define SDDL_RDS_ENDPOINT_SERVERS           TEXT("ES")      // Servers in this group run virtual machines and host sessions where users RemoteApp programs and personal virtual desktops run.
#define SDDL_RDS_MANAGEMENT_SERVERS         TEXT("MS")      // Servers in this group can perform routine administrative actions on servers running Remote Desktop Services.
#define SDDL_USER_MODE_DRIVERS              TEXT("UD")      // UserMode driver
#define SDDL_HYPER_V_ADMINS                 TEXT("HA")      // Members of this group have complete and unrestricted access to all features of Hyper-V.
#define SDDL_CLONEABLE_CONTROLLERS          TEXT("CN")      // Members of this group that are domain controllers may be cloned.
#define SDDL_ACCESS_CONTROL_ASSISTANCE_OPS  TEXT("AA")      // Members of this group can remotely query authorization attributes and permissions for resources on this computer.
#define SDDL_REMOTE_MANAGEMENT_USERS        TEXT("RM")      // Members of this group can access WMI resources over management protocols (such as WS-Management via the Windows Remote Management service). This applies only to WMI namespaces that grant access to the user.
#define SDDL_AUTHORITY_ASSERTED             TEXT("AS")      // Authentication Authority Asserted
#define SDDL_SERVICE_ASSERTED               TEXT("SS")      // Authentication Service Asserted
#define SDDL_PROTECTED_USERS                TEXT("AP")      // Members of this group are afforded additional protections against authentication security threats.
#define SDDL_KEY_ADMINS                     TEXT("KA")      // Members of this group have full control over all key credential objects in the domain
#define SDDL_ENTERPRISE_KEY_ADMINS          TEXT("EK")      // Members of this group have full control over all key credential objects in the forest
#define SDDL_USER_MODE_HARDWARE_OPERATORS   TEXT("HO")      // Members of this group may operate hardware from user mode
#define SDDL_OPENSSH_USERS                  TEXT("SH")      // Members of this group may connect to this computer using SSH

//
// Note !! While making the above changes check if ScepReplaceNewAcronymsInSDDL
// needs to be updated to allow the new SIDs to be translated on downlevel OS.
//

//
// Integrity Labels
//
#define SDDL_ML_LOW                         TEXT("LW")      // Low mandatory level
#define SDDL_ML_MEDIUM                      TEXT("ME")      // Medium mandatory level
#define SDDL_ML_MEDIUM_PLUS                 TEXT("MP")      // Medium Plus mandatory level
#define SDDL_ML_HIGH                        TEXT("HI")      // High mandatory level
#define SDDL_ML_SYSTEM                      TEXT("SI")      // System mandatory level

//
// SDDL Seperators - character version
//
#define SDDL_SEPERATORC                           TEXT(';')
#define SDDL_DELIMINATORC                         TEXT(':')
#define SDDL_ACE_BEGINC                           TEXT('(')
#define SDDL_ACE_ENDC                             TEXT(')')
#define SDDL_SPACEC                               TEXT(' ')
#define SDDL_ACE_COND_BEGINC                      TEXT('(')
#define SDDL_ACE_COND_ENDC                        TEXT(')')
#define SDDL_ACE_COND_STRING_BEGINC               TEXT('"')
#define SDDL_ACE_COND_STRING_ENDC                 TEXT('"')
#define SDDL_ACE_COND_COMPOSITEVALUE_BEGINC       TEXT('{')
#define SDDL_ACE_COND_COMPOSITEVALUE_ENDC         TEXT('}')
#define SDDL_ACE_COND_COMPOSITEVALUE_SEPERATORC   TEXT(',')
#define SDDL_ACE_COND_BLOB_PREFIXC                TEXT('#')
#define SDDL_ACE_COND_SID_BEGINC                  TEXT('(')
#define SDDL_ACE_COND_SID_ENDC                    TEXT(')')

//
// SDDL Seperators - string version
//
#define SDDL_SEPERATOR                            TEXT(";")
#define SDDL_DELIMINATOR                          TEXT(":")
#define SDDL_ACE_BEGIN                            TEXT("(")
#define SDDL_ACE_END                              TEXT(")")
#define SDDL_ACE_COND_BEGIN                       TEXT("(")
#define SDDL_ACE_COND_END                         TEXT(")")
#define SDDL_SPACE                                TEXT(" ")
#define SDDL_ACE_COND_BLOB_PREFIX                 TEXT("#")
#define SDDL_ACE_COND_SID_PREFIX                  TEXT("SID")
#define SDDL_ACE_COND_ATTRIBUTE_PREFIX            TEXT("@")
#define SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX       TEXT("@USER.")
#define SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX   TEXT("@RESOURCE.")
#define SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX     TEXT("@DEVICE.")
#define SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX      TEXT("@TOKEN.")

#if !defined(_NTDDK_)

#if (_WIN32_WINNT >= 0x0500)
_Success_(return != FALSE)
BOOL
WINAPI
ConvertSidToStringSidA(
    _In_  PSID     Sid,
    _Outptr_ LPSTR  *StringSid
    );

_Success_(return != FALSE)
BOOL
WINAPI
ConvertSidToStringSidW(
    _In_ PSID Sid,
    _Outptr_ LPWSTR* StringSid
    );

#ifdef UNICODE
#define ConvertSidToStringSid  ConvertSidToStringSidW
#else
#define ConvertSidToStringSid  ConvertSidToStringSidA
#endif // !UNICODE

_Success_(return != FALSE)
BOOL
WINAPI
ConvertStringSidToSidA(
    _In_ LPCSTR   StringSid,
    _Outptr_ PSID   *Sid
    );

_Success_(return != FALSE)
BOOL
WINAPI
ConvertStringSidToSidW(
    _In_ LPCWSTR StringSid,
    _Outptr_ PSID* Sid
    );

#ifdef UNICODE
#define ConvertStringSidToSid  ConvertStringSidToSidW
#else
#define ConvertStringSidToSid  ConvertStringSidToSidA
#endif // !UNICODE

_Success_(return != FALSE)
BOOL
WINAPI
ConvertStringSecurityDescriptorToSecurityDescriptorA(
    _In_  LPCSTR StringSecurityDescriptor,
    _In_  DWORD StringSDRevision,
    _Outptr_ PSECURITY_DESCRIPTOR  *SecurityDescriptor,
    _Out_opt_ PULONG  SecurityDescriptorSize
    );

_Success_(return != FALSE)
BOOL
WINAPI
ConvertStringSecurityDescriptorToSecurityDescriptorW(
    _In_ LPCWSTR StringSecurityDescriptor,
    _In_ DWORD StringSDRevision,
    _Outptr_ PSECURITY_DESCRIPTOR* SecurityDescriptor,
    _Out_opt_ PULONG SecurityDescriptorSize
    );

#ifdef UNICODE
#define ConvertStringSecurityDescriptorToSecurityDescriptor  ConvertStringSecurityDescriptorToSecurityDescriptorW
#else
#define ConvertStringSecurityDescriptorToSecurityDescriptor  ConvertStringSecurityDescriptorToSecurityDescriptorA
#endif // !UNICODE

_Success_(return != FALSE)
BOOL
WINAPI
ConvertSecurityDescriptorToStringSecurityDescriptorA(
    _In_  PSECURITY_DESCRIPTOR  SecurityDescriptor,
    _In_  DWORD RequestedStringSDRevision,
    _In_  SECURITY_INFORMATION SecurityInformation,
    _Outptr_ LPSTR  *StringSecurityDescriptor,
    _Out_opt_ PULONG StringSecurityDescriptorLen
    );

_Success_(return != FALSE)
BOOL
WINAPI
ConvertSecurityDescriptorToStringSecurityDescriptorW(
    _In_ PSECURITY_DESCRIPTOR SecurityDescriptor,
    _In_ DWORD RequestedStringSDRevision,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Outptr_ LPWSTR* StringSecurityDescriptor,
    _Out_opt_ PULONG StringSecurityDescriptorLen
    );

#ifdef UNICODE
#define ConvertSecurityDescriptorToStringSecurityDescriptor  ConvertSecurityDescriptorToStringSecurityDescriptorW
#else
#define ConvertSecurityDescriptorToStringSecurityDescriptor  ConvertSecurityDescriptorToStringSecurityDescriptorA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif /* _WIN32_WINNT >=  0x0500 */

#endif /* !defined(_NTDDK_) */

#ifdef __cplusplus
}
#endif

#endif  // endif __SDDL_H__

