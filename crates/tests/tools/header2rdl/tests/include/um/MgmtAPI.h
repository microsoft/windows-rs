/*++

Copyright (c) 1992-1999 Microsoft Corporation

Module Name:

    mgmtapi.h

Abstract:

    Definitions for SNMP Management API Development.

--*/

#ifndef _INC_MGMTAPI
#define _INC_MGMTAPI

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Additional header files                                                   //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

#include <snmp.h>
#include <winsock.h>

#ifdef __cplusplus
extern "C" {
#endif

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// MGMT API error code definitions                                           //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

#define SNMP_MGMTAPI_TIMEOUT                40
#define SNMP_MGMTAPI_SELECT_FDERRORS        41
#define SNMP_MGMTAPI_TRAP_ERRORS            42
#define SNMP_MGMTAPI_TRAP_DUPINIT           43
#define SNMP_MGMTAPI_NOTRAPS                44
#define SNMP_MGMTAPI_AGAIN                  45
#define SNMP_MGMTAPI_INVALID_CTL            46
#define SNMP_MGMTAPI_INVALID_SESSION        47
#define SNMP_MGMTAPI_INVALID_BUFFER         48

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// MGMT API control codes
//                                                                           //
///////////////////////////////////////////////////////////////////////////////
#define MGMCTL_SETAGENTPORT             0x01

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// MGMT API type definitions                                                 //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

typedef PVOID LPSNMP_MGR_SESSION;

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// MGMT API prototypes                                                       //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

LPSNMP_MGR_SESSION
SNMP_FUNC_TYPE
SnmpMgrOpen(
    _In_opt_ LPSTR lpAgentAddress,          // Name/address of target agent
    _In_opt_ LPSTR lpAgentCommunity,        // Community for target agent
    IN INT   nTimeOut,                      // Comm time-out in milliseconds
    IN INT   nRetries                       // Comm time-out/retry count
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrCtl(
    LPSNMP_MGR_SESSION session,             // pointer to the MGMTAPI session
    DWORD              dwCtlCode,           // control code for the command requested
    LPVOID             lpvInBuffer,         // buffer with the input parameters for the operation
    DWORD              cbInBuffer,          // size of lpvInBuffer in bytes
    LPVOID             lpvOUTBuffer,        // buffer for all the output parameters of the command
    DWORD              cbOUTBuffer,         // size of lpvOUTBuffer
    LPDWORD            lpcbBytesReturned    // space used from lpvOutBuffer
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrClose(
    IN LPSNMP_MGR_SESSION session           // SNMP session pointer
    );

SNMPAPI
SNMP_FUNC_TYPE
SnmpMgrRequest(
    IN     LPSNMP_MGR_SESSION session,           // SNMP session pointer
    IN     BYTE               requestType,       // Get, GetNext, or Set
    IN OUT RFC1157VarBindList *variableBindings, // Varible bindings
       OUT AsnInteger         *errorStatus,      // Result error status
       OUT AsnInteger         *errorIndex        // Result error index
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrStrToOid(
    _In_opt_  LPSTR         string,         // OID string to be converted
    OUT AsnObjectIdentifier *oid            // OID internal representation
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrOidToStr(
    IN  AsnObjectIdentifier *oid,           // OID to be converted
    _Outptr_opt_ LPSTR   *string         // OID string representation
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrTrapListen(
    OUT HANDLE *phTrapAvailable             // Event indicating trap available
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrGetTrap(
    OUT AsnObjectIdentifier *enterprise,         // Generating enterprise
    OUT AsnNetworkAddress   *IPAddress,          // Generating IP address
    OUT AsnInteger          *genericTrap,        // Generic trap type
    OUT AsnInteger          *specificTrap,       // Enterprise specific type
    OUT AsnTimeticks        *timeStamp,          // Time stamp
    OUT RFC1157VarBindList  *variableBindings    // Variable bindings
    );

BOOL
SNMP_FUNC_TYPE
SnmpMgrGetTrapEx(
    OUT AsnObjectIdentifier *enterprise,       // Generating enterprise
    OUT AsnNetworkAddress   *agentAddress,     // Generating agent addr
    OUT AsnNetworkAddress   *sourceAddress,    // Generating network addr
    OUT AsnInteger          *genericTrap,      // Generic trap type
    OUT AsnInteger          *specificTrap,     // Enterprise specific type
    OUT AsnOctetString      *community,        // Generating community
    OUT AsnTimeticks        *timeStamp,        // Time stamp
    OUT RFC1157VarBindList  *variableBindings  // Variable bindings
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_MGMTAPI
