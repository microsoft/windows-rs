/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:
    wspiapi.h

Abstract:
    The file contains protocol independent API functions.

Revision History:
    Wed Jul 12 10:50:31 2000, Created

--*/

#ifndef _WSPIAPI_H_
#define _WSPIAPI_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (NTDDI_VERSION >= NTDDI_WIN2K)

#include <stdio.h>              // sprintf()
#include <stdlib.h>             // calloc(), strtoul()
#include <malloc.h>             // calloc()
#include <string.h>             // strlen(), strcmp(), strstr()

#if defined(__GOT_SECURE_LIB__) && __GOT_SECURE_LIB__ >= 200402L

#define _WSPIAPI_STRCPY_S strcpy_s
#define _WSPIAPI_STRCAT_S strcat_s
#define _WSPIAPI_STRNCPY_S strncpy_s
#define _WSPIAPI_SPRINTF_S_1 sprintf_s

#else

#define _WSPIAPI_STRCPY_S(_Dst, _Size, _Src) strcpy((_Dst), (_Src))
#define _WSPIAPI_STRCAT_S(_Dst, _Size, _Src) strcat((_Dst), (_Src))
#define _WSPIAPI_STRNCPY_S(_Dst, _Size, _Src, _Count) strncpy((_Dst), (_Src), (_Count)); (_Dst)[(_Size) - 1] = 0
#define _WSPIAPI_SPRINTF_S_1(_Dst, _Size, _Format, _Arg1) sprintf((_Dst), (_Format), (_Arg1))

#endif // defined(__GOT_SECURE_LIB__) && __GOT_SECURE_LIB__ >= 200402L

#if !defined(_WSPIAPI_COUNTOF)
#if !defined(__cplusplus)
#define _WSPIAPI_COUNTOF(_Array) (sizeof(_Array) / sizeof(_Array[0]))
#else
template <typename __CountofType, size_t _N>
char (&__wspiapi_countof_helper(__CountofType (&_Array)[_N]))[_N];
#define _WSPIAPI_COUNTOF(_Array) sizeof(__wspiapi_countof_helper(_Array))
#endif
#endif

#define WspiapiMalloc(tSize)    calloc(1, (tSize))
#define WspiapiFree(p)          free(p)
#define WspiapiSwap(a, b, c)    { (c) = (a); (a) = (b); (b) = (c); }
#define getaddrinfo             WspiapiGetAddrInfo
#define getnameinfo             WspiapiGetNameInfo
#define freeaddrinfo            WspiapiFreeAddrInfo

//
// These function pointers are also within the #if (NTDDI_VERSION >= WIN2K)
// because they are used by the other functions defined in this file available
// only on win2k and above.
//
typedef int (WINAPI *WSPIAPI_PGETADDRINFO) (
    _In_  const char                      *nodename,
    _In_  const char                      *servname,
    _In_  const struct addrinfo           *hints,
    _Out_ struct addrinfo                 **res);

typedef int (WINAPI *WSPIAPI_PGETNAMEINFO) (
    _In_reads_bytes_(salen)  const struct sockaddr    *sa,
    _In_  socklen_t                              salen,
    _Out_writes_bytes_(hostlen) char                   *host,
    _In_  size_t                                 hostlen,
    _Out_writes_bytes_(servlen) char                   *serv,
    _In_  size_t                                 servlen,
    _In_  int                                    flags);

typedef void (WINAPI *WSPIAPI_PFREEADDRINFO) (
    _In_  struct addrinfo                 *ai);



#ifdef __cplusplus
extern "C" {
#endif
    
////////////////////////////////////////////////////////////
// v4 only versions of getaddrinfo and friends.
// NOTE: gai_strerror is inlined in ws2tcpip.h
////////////////////////////////////////////////////////////

__inline    
char *
WINAPI
WspiapiStrdup (
        _In_z_ const char *                   pszString)
/*++

Routine Description
    allocates enough storage via calloc() for a copy of the string,
    copies the string into the new memory, and returns a pointer to it.

Arguments
    pszString       string to copy into new memory

Return Value
    a pointer to the newly allocated storage with the string in it.
    NULL if enough memory could not be allocated, or string was NULL.

--*/    
{
    char    *pszMemory;
    size_t  cchMemory;

    if (!pszString)
        return(NULL);

    cchMemory = strlen(pszString) + 1;
    pszMemory = (char *) WspiapiMalloc(cchMemory);
    if (!pszMemory)
        return(NULL);

    _WSPIAPI_STRCPY_S(pszMemory, cchMemory, pszString);
    return pszMemory;
}

    
    
__inline
BOOL
WINAPI
WspiapiParseV4Address (
    _In_z_ const char *                   pszAddress,
    _Out_  PDWORD                         pdwAddress)
/*++

Routine Description
    get the IPv4 address (in network byte order) from its string
    representation.  the syntax should be a.b.c.d.
    
Arguments
    pszArgument         string representation of the IPv4 address
    ptAddress           pointer to the resulting IPv4 address

Return Value
    Returns FALSE if there is an error, TRUE for success.
    
--*/
{
    DWORD       dwAddress   = 0;
    const char  *pcNext     = NULL;
    int         iCount      = 0;

    // ensure there are 3 '.' (periods)
    for (pcNext = pszAddress; *pcNext != '\0'; pcNext++)
        if (*pcNext == '.')
            iCount++;
    if (iCount != 3)
        return FALSE;

    // return an error if dwAddress is INADDR_NONE (255.255.255.255)
    // since this is never a valid argument to getaddrinfo.
#pragma warning(suppress: 4996)
    dwAddress = inet_addr(pszAddress);
    if (dwAddress == INADDR_NONE)
        return FALSE;

    *pdwAddress = dwAddress;
    return TRUE;
}



__inline
struct addrinfo *
WINAPI
WspiapiNewAddrInfo (
    _In_  int                             iSocketType,
    _In_  int                             iProtocol,
    _In_  WORD                            wPort,
    _In_  DWORD                           dwAddress)
/*++

Routine Description
    allocate an addrinfo structure and populate fields.
    IPv4 specific internal function, not exported.
    
Arguments
    iSocketType         SOCK_*.  can be wildcarded (zero).
    iProtocol           IPPROTO_*.  can be wildcarded (zero).
    wPort               port number of service (in network order).
    dwAddress           IPv4 address (in network order).
    
Return Value
    returns an addrinfo struct, or NULL if out of memory.

--*/    
{
    struct addrinfo     *ptNew;
    struct sockaddr_in  *ptAddress;

    // allocate a new addrinfo structure.
    ptNew       =
        (struct addrinfo *) WspiapiMalloc(sizeof(struct addrinfo));
    if (!ptNew)
        return NULL;

    ptAddress   =
        (struct sockaddr_in *) WspiapiMalloc(sizeof(struct sockaddr_in));
    if (!ptAddress)
    {
#pragma warning(suppress: 6280)
        WspiapiFree(ptNew);
        return NULL;
    }
    ptAddress->sin_family       = AF_INET;
    ptAddress->sin_port         = wPort;
    ptAddress->sin_addr.s_addr  = dwAddress;
    
    // fill in the fields...
    ptNew->ai_family            = PF_INET;
    ptNew->ai_socktype          = iSocketType;
    ptNew->ai_protocol          = iProtocol;
    ptNew->ai_addrlen           = sizeof(struct sockaddr_in);
    ptNew->ai_addr              = (struct sockaddr *) ptAddress;

    return ptNew;
}



__inline
int
WINAPI
WspiapiQueryDNS(
    _In_z_ const char                     *pszNodeName,
    _In_   int                            iSocketType,
    _In_   int                            iProtocol,  
    _In_   WORD                           wPort,      
    _Out_writes_bytes_(NI_MAXHOST) char   pszAlias[NI_MAXHOST],
    _Outptr_ struct addrinfo           **pptResult)
/*++

Routine Description
    helper routine for WspiapiLookupNode.
    performs name resolution by querying the DNS for A records.
    *pptResult would need to be freed if an error is returned.
    
Arguments
    pszNodeName         name of node to resolve.
    iSocketType         SOCK_*.  can be wildcarded (zero).
    iProtocol           IPPROTO_*.  can be wildcarded (zero).
    wPort               port number of service (in network order).
    pszAlias            where to return the alias.  must be of size NI_MAXHOST.
    pptResult           where to return the result.
    
Return Value
    Returns 0 on success, an EAI_* style error value otherwise.

--*/    
{
    struct addrinfo **pptNext   = pptResult;
    struct hostent  *ptHost     = NULL;
    char            **ppAddresses;

    *pptNext    = NULL;
    pszAlias[0] = '\0';

#pragma warning(suppress: 4996 38026)
    ptHost = gethostbyname(pszNodeName);
    if (ptHost)
    {
        if ((ptHost->h_addrtype == AF_INET)     &&
            (ptHost->h_length   == sizeof(struct in_addr)))
        {
            for (ppAddresses    = ptHost->h_addr_list;
                 *ppAddresses   != NULL;
                 ppAddresses++)
            {
                // create an addrinfo structure...
                *pptNext = WspiapiNewAddrInfo(
                    iSocketType,
                    iProtocol,
                    wPort,
                    ((struct in_addr *) *ppAddresses)->s_addr);
                if (!*pptNext)
                    return EAI_MEMORY;

                pptNext = &((*pptNext)->ai_next);
            }
        }

        // pick up the canonical name.
        _WSPIAPI_STRNCPY_S(pszAlias, NI_MAXHOST, ptHost->h_name, NI_MAXHOST - 1);
        
        return 0;
    }
    
    switch (WSAGetLastError())
    {
        case WSAHOST_NOT_FOUND: return EAI_NONAME;
        case WSATRY_AGAIN:      return EAI_AGAIN;
        case WSANO_RECOVERY:    return EAI_FAIL;
        case WSANO_DATA:        return EAI_NODATA;
        default:                return EAI_NONAME;
    }
}



__inline
int
WINAPI
WspiapiLookupNode(
    _In_z_ const char                     *pszNodeName,
    _In_   int                            iSocketType,
    _In_   int                            iProtocol,
    _In_   WORD                           wPort,
    _In_   BOOL                           bAI_CANONNAME,
    _Outptr_ struct addrinfo           **pptResult)
/*++

Routine Description
    resolve a nodename and return a list of addrinfo structures.
    IPv4 specific internal function, not exported.
    *pptResult would need to be freed if an error is returned.
    
    NOTE: if bAI_CANONNAME is true, the canonical name should be
          returned in the first addrinfo structure.
    
Arguments
    pszNodeName         name of node to resolve.
    iSocketType         SOCK_*.  can be wildcarded (zero).
    iProtocol           IPPROTO_*.  can be wildcarded (zero).
    wPort               port number of service (in network order).
    bAI_CANONNAME       whether the AI_CANONNAME flag is set.
    pptResult           where to return result.
    
Return Value
    Returns 0 on success, an EAI_* style error value otherwise.

--*/
{
    int     iError              = 0;
    int     iAliasCount         = 0;

    char    szFQDN1[NI_MAXHOST] = "";
    char    szFQDN2[NI_MAXHOST] = "";
    char    *pszName            = szFQDN1;
    char    *pszAlias           = szFQDN2;
    char    *pszScratch         = NULL;
    _WSPIAPI_STRNCPY_S(pszName, NI_MAXHOST, pszNodeName, NI_MAXHOST - 1);
    
    for (;;)
    {
        iError = WspiapiQueryDNS(pszNodeName,
                                 iSocketType,
                                 iProtocol,
                                 wPort,
                                 pszAlias,
                                 pptResult);
        if (iError)
            break;

        // if we found addresses, then we are done.
        if (*pptResult)
            break;

        // stop infinite loops due to DNS misconfiguration.  there appears
        // to be no particular recommended limit in RFCs 1034 and 1035.
        if ((!strlen(pszAlias))             ||
            (!strcmp(pszName, pszAlias))    ||
            (++iAliasCount == 16))
        {
            iError = EAI_FAIL;
            break;
        }

        // there was a new CNAME, look again.
        WspiapiSwap(pszName, pszAlias, pszScratch);
    }

    if (!iError && bAI_CANONNAME)
    {
        (*pptResult)->ai_canonname = WspiapiStrdup(pszAlias);
        if (!(*pptResult)->ai_canonname)
            iError = EAI_MEMORY;
    }

    return iError;
}



__inline
int
WINAPI
WspiapiClone (
    _In_  WORD                            wPort,      
    _In_  struct addrinfo                 *ptResult)
/*++

Routine Description
    clone every addrinfo structure in ptResult for the UDP service.
    ptResult would need to be freed if an error is returned.
    
Arguments
    wPort               port number of UDP service.
    ptResult            list of addrinfo structures, each
                        of whose node needs to be cloned.

Return Value
    Returns 0 on success, an EAI_MEMORY on allocation failure.

--*/
{
    struct addrinfo *ptNext = NULL;
    struct addrinfo *ptNew  = NULL;

    for (ptNext = ptResult; ptNext != NULL; )
    {
        // create an addrinfo structure...
        ptNew = WspiapiNewAddrInfo(
            SOCK_DGRAM,
            ptNext->ai_protocol,
            wPort,
            ((struct sockaddr_in *) ptNext->ai_addr)->sin_addr.s_addr);
        if (!ptNew)
            break;

        // link the cloned addrinfo
        ptNew->ai_next  = ptNext->ai_next;
        ptNext->ai_next = ptNew;
        ptNext          = ptNew->ai_next;
    }

    if (ptNext != NULL)
        return EAI_MEMORY;
    
    return 0;
}



__inline
void
WINAPI
WspiapiLegacyFreeAddrInfo (
    _In_  struct addrinfo                 *ptHead)
/*++

Routine Description
    Free an addrinfo structure (or chain of structures).
    As specified in RFC 2553, Section 6.4.
    
Arguments
    ptHead              structure (chain) to free
    
--*/    
{
    struct addrinfo *ptNext;    // next strcture to free

    for (ptNext = ptHead; ptNext != NULL; ptNext = ptHead)
    {
        if (ptNext->ai_canonname)
            WspiapiFree(ptNext->ai_canonname);
        
        if (ptNext->ai_addr)
            WspiapiFree(ptNext->ai_addr);

        ptHead = ptNext->ai_next;
        WspiapiFree(ptNext);
    }
}



__inline
int
WINAPI
WspiapiLegacyGetAddrInfo(
    _In_ const char                       *pszNodeName,
    _In_ const char                       *pszServiceName,
    _In_ const struct addrinfo            *ptHints,
    _Outptr_ struct addrinfo           **pptResult)
/*++

Routine Description
    Protocol-independent name-to-address translation.
    As specified in RFC 2553, Section 6.4.
    This is the hacked version that only supports IPv4.
    
Arguments
    pszNodeName         node name to lookup.
    pszServiceName      service name to lookup.
    ptHints             hints about how to process request.
    pptResult           where to return result.
    
Return Value
    returns zero if successful, an EAI_* error code if not.

--*/    
{
    int                 iError      = 0;
    int                 iFlags      = 0;
    int                 iFamily     = PF_UNSPEC;
    int                 iSocketType = 0;
    int                 iProtocol   = 0;
    WORD                wPort       = 0;
    DWORD               dwAddress   = 0;

    struct servent      *ptService  = NULL;
    char                *pc         = NULL;
    BOOL                bClone      = FALSE;
    WORD                wTcpPort    = 0;
    WORD                wUdpPort    = 0;
    
    
    // initialize pptResult with default return value.
    *pptResult  = NULL;


    ////////////////////////////////////////
    // validate arguments...
    //
    
    // both the node name and the service name can't be NULL.
    if ((!pszNodeName) && (!pszServiceName))
        return EAI_NONAME;

    // validate hints.
    if (ptHints)
    {
        // all members other than ai_flags, ai_family, ai_socktype
        // and ai_protocol must be zero or a null pointer.
        if ((ptHints->ai_addrlen    != 0)       ||
            (ptHints->ai_canonname  != NULL)    ||
            (ptHints->ai_addr       != NULL)    ||
            (ptHints->ai_next       != NULL))
        {
            return EAI_FAIL;
        }
        
        // the spec has the "bad flags" error code, so presumably we
        // should check something here.  insisting that there aren't
        // any unspecified flags set would break forward compatibility,
        // however.  so we just check for non-sensical combinations.
        //
        // we cannot come up with a canonical name given a null node name.
        iFlags      = ptHints->ai_flags;
        if ((iFlags & AI_CANONNAME) && !pszNodeName)
            return EAI_BADFLAGS;

        // we only support a limited number of protocol families.
        iFamily     = ptHints->ai_family;
        if ((iFamily != PF_UNSPEC) && (iFamily != PF_INET))
            return EAI_FAMILY;

        // we only support only these socket types.
        iSocketType = ptHints->ai_socktype;
        if ((iSocketType != 0)                  &&
            (iSocketType != SOCK_STREAM)        &&
            (iSocketType != SOCK_DGRAM)         &&
            (iSocketType != SOCK_RAW))
            return EAI_SOCKTYPE;

        // REVIEW: What if ai_socktype and ai_protocol are at odds?
        iProtocol   = ptHints->ai_protocol;
    }


    ////////////////////////////////////////
    // do service lookup...

    if (pszServiceName)
    {
        wPort = (WORD) strtoul(pszServiceName, &pc, 10);
        if (*pc == '\0')        // numeric port string
        {
            wPort = wTcpPort = wUdpPort = htons(wPort);
            if (iSocketType == 0)
            {
                bClone      = TRUE;
                iSocketType = SOCK_STREAM;
            }
        }
        else                    // non numeric port string
        {
            if ((iSocketType == 0) || (iSocketType == SOCK_DGRAM))
            {
                ptService = getservbyname(pszServiceName, "udp");
                if (ptService)
                    wPort = wUdpPort = ptService->s_port;
            }

            if ((iSocketType == 0) || (iSocketType == SOCK_STREAM))
            {
                ptService = getservbyname(pszServiceName, "tcp");
                if (ptService)
                    wPort = wTcpPort = ptService->s_port;
            }
            
            // assumes 0 is an invalid service port...
            if (wPort == 0)     // no service exists
                return (iSocketType ? EAI_SERVICE : EAI_NONAME);

            if (iSocketType == 0)
            {
                // if both tcp and udp, process tcp now & clone udp later.
                iSocketType = (wTcpPort) ? SOCK_STREAM : SOCK_DGRAM;
                bClone      = (wTcpPort && wUdpPort); 
            }
        }
    }
    


    ////////////////////////////////////////
    // do node name lookup...

    // if we weren't given a node name,
    // return the wildcard or loopback address (depending on AI_PASSIVE).
    //
    // if we have a numeric host address string,
    // return the binary address.
    //
    if ((!pszNodeName) || (WspiapiParseV4Address(pszNodeName, &dwAddress)))
    {
        if (!pszNodeName)
        {
            dwAddress = htonl((iFlags & AI_PASSIVE)
                              ? INADDR_ANY
                              : INADDR_LOOPBACK);
        }
        
        // create an addrinfo structure...
        *pptResult =
            WspiapiNewAddrInfo(iSocketType, iProtocol, wPort, dwAddress);
        if (!(*pptResult))
            iError = EAI_MEMORY;
        
        if (!iError && pszNodeName)
        {
            // implementation specific behavior: set AI_NUMERICHOST
            // to indicate that we got a numeric host address string.
            (*pptResult)->ai_flags |= AI_NUMERICHOST;
            
            // return the numeric address string as the canonical name
            if (iFlags & AI_CANONNAME)
            {
                (*pptResult)->ai_canonname =
#pragma warning(suppress: 4996)
                    WspiapiStrdup(inet_ntoa(*((struct in_addr *) &dwAddress)));
                if (!(*pptResult)->ai_canonname)        
                    iError = EAI_MEMORY;
            }
        }
    }


    // if we do not have a numeric host address string and
    // AI_NUMERICHOST flag is set, return an error!
    else if (iFlags & AI_NUMERICHOST)
    {
        iError = EAI_NONAME;
    }
    

    // since we have a non-numeric node name,
    // we have to do a regular node name lookup.
    else
    {
        iError = WspiapiLookupNode(pszNodeName,
                                   iSocketType,
                                   iProtocol,
                                   wPort,
                                   (iFlags & AI_CANONNAME),
                                   pptResult);
    }

    if (!iError && bClone)
    {
        iError = WspiapiClone(wUdpPort, *pptResult);
    }

    if (iError)
    {
        WspiapiLegacyFreeAddrInfo(*pptResult);
        *pptResult  = NULL;        
    }

    return (iError);
}



__inline
int
WINAPI
WspiapiLegacyGetNameInfo(
    _In_reads_bytes_(tSocketLength) const struct sockaddr        *ptSocketAddress,
    _In_                                    socklen_t       tSocketLength,
    _Out_writes_bytes_(tNodeLength)               char            *pszNodeName,
    _In_                                    size_t          tNodeLength,
    _Out_writes_bytes_(tServiceLength)            char            *pszServiceName,
    _In_                                    size_t          tServiceLength,
    _In_                                    int             iFlags)
/*++

Routine Description
    protocol-independent address-to-name translation.
    as specified in RFC 2553, Section 6.5.
    this is the hacked version that only supports IPv4.
    
Arguments
    ptSocketAddress     socket address to translate.
    tSocketLength       length of above socket address.
    pszNodeName         where to return the node name.
    tNodeLength         size of above buffer.
    pszServiceName      where to return the service name.
    tServiceLength      size of above buffer.
    iFlags              flags of type NI_*.
    
Return Value
    returns zero if successful, an EAI_* error code if not.

--*/    
{
    struct servent  *ptService;
    WORD            wPort;    
    char            szBuffer[]  = "65535";
    char            *pszService = szBuffer;

    struct hostent  *ptHost;
    struct in_addr  tAddress;
    char            *pszNode    = NULL;
    char            *pc         = NULL;
    

    // sanity check ptSocketAddress and tSocketLength.
    if ((!ptSocketAddress) || (tSocketLength < (socklen_t)sizeof(struct sockaddr)))
        return EAI_FAIL;
    
    if (ptSocketAddress->sa_family != AF_INET)
        return EAI_FAMILY;

    if (tSocketLength < (socklen_t)sizeof(struct sockaddr_in))
        return EAI_FAIL;
    
    if (!(pszNodeName && tNodeLength) &&
        !(pszServiceName && tServiceLength))
    {
        return EAI_NONAME;    
    }

    // the draft has the "bad flags" error code, so presumably we
    // should check something here.  insisting that there aren't
    // any unspecified flags set would break forward compatibility,
    // however.  so we just check for non-sensical combinations.
    if ((iFlags & NI_NUMERICHOST) && (iFlags & NI_NAMEREQD))
    {                                                                       
        return EAI_BADFLAGS;
    }
        
    // translate the port to a service name (if requested).
    if (pszServiceName && tServiceLength)
    {
        wPort = ((struct sockaddr_in *) ptSocketAddress)->sin_port;
        
        if (iFlags & NI_NUMERICSERV)
        {
            // return numeric form of the address.
            _WSPIAPI_SPRINTF_S_1(szBuffer, _WSPIAPI_COUNTOF(szBuffer), "%u", ntohs(wPort));
        }
        else
        {
            // return service name corresponding to port.
            ptService = getservbyport(wPort,
                                      (iFlags & NI_DGRAM) ? "udp" : NULL);
            if (ptService && ptService->s_name)
            {
                // lookup successful.
                pszService = ptService->s_name;
            }
            else
            {
                // DRAFT: return numeric form of the port!
                _WSPIAPI_SPRINTF_S_1(szBuffer, _WSPIAPI_COUNTOF(szBuffer), "%u", ntohs(wPort));
            }
        }
        
        
        if (tServiceLength > strlen(pszService))
            _WSPIAPI_STRCPY_S(pszServiceName, tServiceLength, pszService);
        else
            return EAI_FAIL;
    }

    
    // translate the address to a node name (if requested).
    if (pszNodeName && tNodeLength)
    {    
        // this is the IPv4-only version, so we have an IPv4 address.
        tAddress = ((struct sockaddr_in *) ptSocketAddress)->sin_addr;

        if (iFlags & NI_NUMERICHOST)
        {
            // return numeric form of the address.
#pragma warning(suppress: 4996)
            pszNode  = inet_ntoa(tAddress);
        }
        else
        {
            // return node name corresponding to address.
#pragma warning(suppress: 4996 38026)
            ptHost = gethostbyaddr((char *) &tAddress, sizeof(struct in_addr), AF_INET);
            if (ptHost && ptHost->h_name)
            {
                // DNS lookup successful.
                // stop copying at a "." if NI_NOFQDN is specified.
                pszNode = ptHost->h_name;
                if ((iFlags & NI_NOFQDN) &&
                    ((pc = strchr(pszNode, '.')) != NULL))
                    *pc = '\0';
            }
            else
            {
                // DNS lookup failed.  return numeric form of the address.
                if (iFlags & NI_NAMEREQD)
                {
                    switch (WSAGetLastError())
                    {
                        case WSAHOST_NOT_FOUND: return EAI_NONAME;
                        case WSATRY_AGAIN:      return EAI_AGAIN;
                        case WSANO_RECOVERY:    return EAI_FAIL;
                        default:                return EAI_NONAME;
                    }
                }
                else
#pragma warning(suppress: 4996)
                    pszNode  = inet_ntoa(tAddress);
            }
        }

        if (tNodeLength > strlen(pszNode))
            _WSPIAPI_STRCPY_S(pszNodeName, tNodeLength, pszNode);
        else
            return EAI_FAIL;
    }

    return 0;
}



typedef struct 
{
    char const          *pszName;
    FARPROC             pfAddress;
} WSPIAPI_FUNCTION;

#define WSPIAPI_FUNCTION_ARRAY                                  \
{                                                               \
    "getaddrinfo",      (FARPROC) WspiapiLegacyGetAddrInfo,     \
    "getnameinfo",      (FARPROC) WspiapiLegacyGetNameInfo,     \
    "freeaddrinfo",     (FARPROC) WspiapiLegacyFreeAddrInfo,    \
}



__inline
FARPROC
WINAPI
WspiapiLoad(
    _In_  WORD                            wFunction)
/*++

Routine Description
    try to locate the address family independent name resolution routines
    (i.e. getaddrinfo, getnameinfo, freeaddrinfo, gai_strerror).
    
Locks
    this function call is not synchronized.  hence the library containing
    the routines might be loaded multiple times.  another option is to
    synchronize through a spin lock using a static local variable and the
    InterlockedExchange operation.  

    
Arguments
    wFunction           ordinal # of the function to get the pointer to
                        0   getaddrinfo
                        1   getnameinfo
                        2   freeaddrinfo
    
Return Value
    address of the library/legacy routine

--*/
{
    HMODULE                 hLibrary        = NULL;

    // these static variables store state across calls, across threads.
    static BOOL             bInitialized    = FALSE;
    static WSPIAPI_FUNCTION rgtGlobal[]     = WSPIAPI_FUNCTION_ARRAY;
    static const int        iNumGlobal      = (sizeof(rgtGlobal) /
                                               sizeof(WSPIAPI_FUNCTION));
    
    // we overwrite rgtGlobal only if all routines exist in library.
    WSPIAPI_FUNCTION        rgtLocal[]      = WSPIAPI_FUNCTION_ARRAY;
    __analysis_assume((sizeof(rgtLocal)/sizeof(WSPIAPI_FUNCTION)) == iNumGlobal);
    FARPROC                 fScratch        = NULL;
    int                     i               = 0;
    
    
    if (bInitialized)           // WspiapiLoad has already been called once
        return (rgtGlobal[wFunction].pfAddress);

    for (;;)                    // breakout loop
    {
        CHAR SystemDir[MAX_PATH + 1];
        CHAR Path[MAX_PATH + 8];

        if (GetSystemDirectoryA(SystemDir, MAX_PATH) == 0) 
        {
            break;
        }

        // in Whistler and beyond...
        // the routines are present in the WinSock 2 library (ws2_32.dll).
        // printf("Looking in ws2_32 for getaddrinfo...\n");
        _WSPIAPI_STRCPY_S(Path, _WSPIAPI_COUNTOF(Path), SystemDir);
        _WSPIAPI_STRCAT_S(Path, _WSPIAPI_COUNTOF(Path), "\\ws2_32");
        hLibrary = LoadLibraryA(Path);
        if (hLibrary != NULL)
        {
            fScratch = GetProcAddress(hLibrary, "getaddrinfo");
            if (fScratch == NULL)
            {
                FreeLibrary(hLibrary);
                hLibrary = NULL;
            }
        }
        if (hLibrary != NULL)
            break;
        

        // in the IPv6 Technology Preview...        
        // the routines are present in the IPv6 WinSock library (wship6.dll).
        // printf("Looking in wship6 for getaddrinfo...\n");
        _WSPIAPI_STRCPY_S(Path, _WSPIAPI_COUNTOF(Path), SystemDir);
        _WSPIAPI_STRCAT_S(Path, _WSPIAPI_COUNTOF(Path), "\\wship6");
        hLibrary = LoadLibraryA(Path);
        if (hLibrary != NULL)
        {
            fScratch = GetProcAddress(hLibrary, "getaddrinfo");
            if (fScratch == NULL)
            {
                FreeLibrary(hLibrary);
                hLibrary = NULL;
            }
        }

        break;
    }


    if (hLibrary != NULL)
    {
        // use routines from this library...
        // since getaddrinfo is here, we expect all routines to be here,
        // but will fall back to IPv4-only if any of them is missing.
        for (i = 0; i < iNumGlobal; i++)
        {
            rgtLocal[i].pfAddress
                = GetProcAddress(hLibrary, rgtLocal[i].pszName);
            if (rgtLocal[i].pfAddress == NULL)
            {
                FreeLibrary(hLibrary);
                hLibrary = NULL;
                break;
            }
        }

        if (hLibrary != NULL)
        {
            // printf("found!\n");
            for (i = 0; i < iNumGlobal; i++)
                rgtGlobal[i].pfAddress = rgtLocal[i].pfAddress;
        }
    }
    
    bInitialized = TRUE;
    return (rgtGlobal[wFunction].pfAddress);
}



__inline
int
WINAPI
WspiapiGetAddrInfo(
    _In_opt_ const char                       *nodename,
    _In_opt_ const char                       *servname,
    _In_opt_ const struct addrinfo            *hints,
    _Outptr_ struct addrinfo                 **res)
{
    int                             iError;
    static WSPIAPI_PGETADDRINFO     pfGetAddrInfo   = NULL;
    
    if (!pfGetAddrInfo)
        pfGetAddrInfo   = (WSPIAPI_PGETADDRINFO) WspiapiLoad(0);

    iError = (*pfGetAddrInfo)(nodename, servname, hints, res);
    WSASetLastError(iError);
    return iError;
}



__inline
int
WINAPI
WspiapiGetNameInfo (
    _In_reads_bytes_(salen) const struct sockaddr           *sa,
    _In_  socklen_t                                    salen,
    _Out_writes_bytes_(hostlen) char                         *host,
    _In_  size_t                                       hostlen,
    _Out_writes_bytes_(servlen) char                         *serv,
    _In_  size_t                                       servlen,
    _In_  int                                          flags)
{
    int                             iError;
    static WSPIAPI_PGETNAMEINFO     pfGetNameInfo   = NULL;
    
    if (!pfGetNameInfo)
        pfGetNameInfo   = (WSPIAPI_PGETNAMEINFO) WspiapiLoad(1);

    iError = (*pfGetNameInfo)(sa, salen, host, hostlen, serv, servlen, flags);
    WSASetLastError(iError);
    return iError;
}



__inline
void
WINAPI
WspiapiFreeAddrInfo (
    _In_  struct addrinfo                 *ai)
{
    static WSPIAPI_PFREEADDRINFO    pfFreeAddrInfo   = NULL;

    if (!pfFreeAddrInfo)
        pfFreeAddrInfo  = (WSPIAPI_PFREEADDRINFO) WspiapiLoad(2);
    (*pfFreeAddrInfo)(ai);
}

#ifdef  __cplusplus
}
#endif

#endif // if (NTDDI_VERSION >= WIN2K)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _WSPIAPI_H_












