//+---------------------------------------------------------------------------
//
// Microsoft Windows
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
// File:     nsemail.h
//
// Abstract: NS_EMAIL namespace related definitions
//
//----------------------------------------------------------------------------

#ifndef _NSEMAIL_H_
#define _NSEMAIL_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef enum napi_provider_type_tag
{ 
    ProviderType_Application = 1, 
    ProviderType_Service, 
} NAPI_PROVIDER_TYPE;

typedef enum napi_provider_level_tag
{ 
    ProviderLevel_None = 0,
    ProviderLevel_Secondary, 
    ProviderLevel_Primary
} NAPI_PROVIDER_LEVEL;

//-----------------------------------------------------------------------------
// 
// Layout of Installation Blob for Email Naming NSPv2 Providers:
//
// +-------------------------------------------------------------------------+
// |                     NAPI_PROVIDER_INSTALLATION_BLOB                     |
// | - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
// | dwVersion                                                               |
// | dwProviderType                                                          |
// | fSupportsWildCard                                                       |
// | cDomains (equal to N in this example)                                   |
// | OffsetFirstDomain (aligned offset of NAPI_DOMAIN_DESCRIPTION_BLOB # 1)  |
// | +---------------------------------------------------------------------+ |
// | |                   NAPI_DOMAIN_DESCRIPTION_BLOB # 1                  | |
// | | - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - + |
// | | AuthLevel                                                           | |
// | | cchDomainName                                                       | |
// | | OffsetNextDomainDescription (aligned offset of Domain Blob # 2)     | |
// | | OffsetThisDomainName (aligned offset of Domain Name 1)              | |
// | +---------------------------------------------------------------------+ |
// | +---------------------------------------------------------------------+ |
// | |                   NAPI_DOMAIN_DESCRIPTION_BLOB # 2                  | |
// | | - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - + |
// | | AuthLevel                                                           | |
// | | cchDomainName                                                       | |
// | | OffsetNextDomainDescription (aligned offset of Domain Blob # 3)     | |
// | | OffsetThisDomainName (aligned offset of Domain Name 2)              | |
// | +---------------------------------------------------------------------+ |
// | ... ... ...                                                             |
// | ... ... ...                                                             |
// | ... ... ...                                                             |
// | +---------------------------------------------------------------------+ |
// | |                   NAPI_DOMAIN_DESCRIPTION_BLOB # N                  | |
// | | - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - + |
// | | AuthLevel                                                           | |
// | | cchDomainName                                                       | |
// | | OffsetNextDomainDescription (should be 0 if N is the last domain)   | |
// | | OffsetThisDomainName (aligned offset of Domain Name N)              | |
// | +---------------------------------------------------------------------+ |
// | Domain Name 1                                                           |
// | Domain Name 2                                                           |
// | ... ... ...                                                             |
// | Domain Name N                                                           |
// +-------------------------------------------------------------------------+
//
// Notes: - The above NAPI_PROVIDER_INSTALLATION_BLOB must be passed to 
//          WSCInstallNameSpaceEx encapsulated in a winsock2.h's BLOB 
//          (typedef struct _BLOB { ULONG  cbSize; BYTE * pBlobData;} BLOB;)
//          by storing its address in pBlobData, and its total size in 
//          cbSize.
//        - The NAPI_PROVIDER_INSTALLATION_BLOB must be at an address at
//          least 4-bytes aligned, better if machine-pointer-size aligned.
//        - Domain Blob (NAPI_DOMAIN_DESCRIPTION_BLOB) offsets need to be 
//          4-bytes aligned. 
//        - Domain Name offsets need to be 2-bytes aligned.
//        - Domain Names are strings of UNICODE chars and they must be at
//          least cchDomainName UNICODE chars long. Zero-termination is 
//          recommended but not required.
//        - Domain Blobs (NAPI_DOMAIN_DESCRIPTION_BLOBs) do not need to be 
//          contiguous and Domain Name strings do not need to be in order 
//          or grouped after the Domain Blobs, as in the example above.
//          They may appear in any order and also interleaved with Domain 
//          Blobs, as long as the alignment rules are respected.
//       
//-----------------------------------------------------------------------------

typedef struct napi_domain_description_blob_tag
{
    DWORD   AuthLevel;
    DWORD   cchDomainName;
    DWORD   OffsetNextDomainDescription;
    DWORD   OffsetThisDomainName;
} NAPI_DOMAIN_DESCRIPTION_BLOB;

typedef struct napi_provider_installation_blob_tag
{
    DWORD   dwVersion;
    DWORD   dwProviderType;
    DWORD   fSupportsWildCard;
    ULONG   cDomains;
    ULONG   OffsetFirstDomain;
} NAPI_PROVIDER_INSTALLATION_BLOB;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

