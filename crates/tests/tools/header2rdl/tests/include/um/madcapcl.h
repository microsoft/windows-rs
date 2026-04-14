// Copyright (c) 1990-1999  Microsoft Corporation
#ifndef _MADCAPCL_H_
#define _MADCAPCL_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <winapifamily.h>
#include <time.h>

#if _MSC_VER > 1000
#pragma once
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define MCAST_CLIENT_ID_LEN 17

enum {
    MCAST_API_VERSION_0 = 0,
    MCAST_API_VERSION_1
};

#define MCAST_API_CURRENT_VERSION MCAST_API_VERSION_1

typedef unsigned short IP_ADDR_FAMILY;

typedef union _IPNG_ADDRESS {
    DWORD   IpAddrV4;
    BYTE    IpAddrV6[16];
} IPNG_ADDRESS, *PIPNG_ADDRESS;


/*++
Description:

    This union is used to pass both IPv4 and IPv6 style address.

Members:

    IpAddrV4 - IPv4 style address

    IpAddrV6 - IPv6 style address

--*/

typedef struct _MCAST_CLIENT_UID {
    LPBYTE ClientUID;
    DWORD ClientUIDLength;
} MCAST_CLIENT_UID, *LPMCAST_CLIENT_UID;

/*++
Description:

    This describes the unique clientID for each request.

Members:

    ClientUID - Buffer containing the clientID

    ClientUIDLength - The size of the above buffer in bytes.

--*/


typedef struct _MCAST_SCOPE_CTX {
    IPNG_ADDRESS      ScopeID;
    IPNG_ADDRESS      Interface;
    IPNG_ADDRESS      ServerID;
} MCAST_SCOPE_CTX, *PMCAST_SCOPE_CTX;

/*++
Description:

    This defines the handle of the scope from which the address
    is to be allocated/renewed/released.

Members:

    ScopeID - Scope ID is essentially first ip of the scope

    Interface - Interface on which this scope was found

    ServerID - IPAddress of the MADCAP server

--*/


typedef struct _MCAST_SCOPE_ENTRY {
    MCAST_SCOPE_CTX ScopeCtx;
    IPNG_ADDRESS      LastAddr;
    DWORD       TTL;
    UNICODE_STRING  ScopeDesc;
} MCAST_SCOPE_ENTRY, *PMCAST_SCOPE_ENTRY;

/*++
Description:

    This structure contains all the info pertaining to a given multicast
    scope.

Members:

    ScopeCtx - the handle for this scope

    LastAddr - last addr of the scope

    TTL - TTL value of this scope.

    ScopeDesc - user friendly description of scope

--*/



typedef struct _MCAST_LEASE_REQUEST {
    LONG        LeaseStartTime;
    LONG        MaxLeaseStartTime;
    DWORD       LeaseDuration;
    DWORD       MinLeaseDuration;
    IPNG_ADDRESS  ServerAddress;
    WORD        MinAddrCount;
    WORD        AddrCount;
    PBYTE       pAddrBuf;
} MCAST_LEASE_REQUEST, *PMCAST_LEASE_REQUEST;

/*++
Description:

    This structure is used to describe the request parameters for
    requesting/renewing/releasing multicast addresses

Members:

    LeaseStartTime - desired start time of the lease, pass 0 if desired start time
                     is current time. The desired time is specified in the number of seconds elapsed
                     since midnight (00:00:00), January 1, 1970, coordinated universal time.

    MaxLeaseStartTime - the maximum start time that the client is willing to accept.
                        Where time is the number of seconds elapsed since midnight (00:00:00),
                        January 1, 1970, coordinated universal time.

    LeaseDuration - desired lease time for the request, pass 0 if default
                    lease time is requested.

    MinLeaseDuration - the minimum lease time that the client is willing
                       to accept

    ServerAddress - server's ip address where this lease whas renewed/requested.
                    pass 0 if unknown (e.g in McastRequestAddress)

    MinAddrCount - minimum number of addresses that the client is willing
                   to accept

    AddrCount - the desired number of addresses requested/allocated/renewed.
                This also specifies the size of the array specified by Addr.

    pAddrBuf - buffer containing specific addresses being requested/renewed/released.
                For IPv4 it is a pointer to 4 byte addresses and for IPv6 it
                points to 16 byte chunks. Pass NULL if no specific addresses
                are requested.

Remarks:

    In MCAST_API_VERSION_1 version, the MaxLeaseStartTime, MinLeaseDuration and
    MinAddrCount are ignored by the API implementation. However, the clients should
    set appropriate desired values for these members so as when the OS update brings
    new implementation of the APIs then the clients can take advantage of it.

--*/


typedef struct _MCAST_LEASE_RESPONSE {
    LONG        LeaseStartTime;
    LONG        LeaseEndTime;
    IPNG_ADDRESS  ServerAddress;
    WORD        AddrCount;
    PBYTE       pAddrBuf;
} MCAST_LEASE_RESPONSE, *PMCAST_LEASE_RESPONSE;

/*++

Description:

    This structure is used to pass the response of the operation of
    requesting/renewing/releasing multicast addresses.

Members:

    LeaseStartTime - start time of the lease in number of seconds elapsed since
                     midnight (00:00:00), January 1, 1970, coordinated universal time.

    LeaseEndTime - time when lease ends, where time is the number of seconds elapsed
                   since midnight (00:00:00), January 1, 1970, coordinated universal time.

    ServerAddress - server's ip address where this lease is renewed/requested.

    AddrCount - number of addresses requested/allocated/renewed.
                This also specifies the size of the array specified by Addr.

    Addr - buffer containing addresses being requested/renewed/released. For IPv4
            it is a pointer to 4 byte addresses and for IPv6 it points to 16 byte chunks

--*/

DWORD
APIENTRY
McastApiStartup(
    IN  OUT  PDWORD   Version
    );

VOID
APIENTRY
McastApiCleanup(
    VOID
    );

DWORD
APIENTRY
McastGenUID(
    IN OUT LPMCAST_CLIENT_UID    pRequestID
    );

DWORD
APIENTRY
McastEnumerateScopes(
    IN     IP_ADDR_FAMILY       AddrFamily,
    IN     BOOL                 ReQuery,
    IN OUT PMCAST_SCOPE_ENTRY   pScopeList,
    IN OUT PDWORD               pScopeLen,
    OUT    PDWORD               pScopeCount
    );

DWORD
APIENTRY
McastRequestAddress(
    IN     IP_ADDR_FAMILY           AddrFamily,
    IN     LPMCAST_CLIENT_UID       pRequestID,
    IN     PMCAST_SCOPE_CTX         pScopeCtx,
    IN     PMCAST_LEASE_REQUEST     pAddrRequest,
    IN OUT PMCAST_LEASE_RESPONSE    pAddrResponse
    );

DWORD
APIENTRY
McastRenewAddress(
    IN     IP_ADDR_FAMILY           AddrFamily,
    IN     LPMCAST_CLIENT_UID       pRequestID,
    IN     PMCAST_LEASE_REQUEST     pRenewRequest,
    IN OUT PMCAST_LEASE_RESPONSE    pRenewResponse
    );

DWORD
APIENTRY
McastReleaseAddress(
    IN     IP_ADDR_FAMILY          AddrFamily,
    IN     LPMCAST_CLIENT_UID      pRequestID,
    IN     PMCAST_LEASE_REQUEST    pReleaseRequest
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif 
#endif // _MADCAPCL_H_
