//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef _HTTPCACH_H_
#define _HTTPCACH_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Key and data for uri-cache
//

#define URI_CACHE_NAME             L"URI"

class __declspec(uuid("f1bc4f8c-6bf8-42c0-b745-4fbe1a67e5a7"))
IUriKey : public IHttpCacheKey
{ 
public:

    PCWSTR
    GetCacheName(
        VOID
    ) const
    {
        return URI_CACHE_NAME;
    }

    virtual
    PCWSTR
    GetUrl(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetSiteName(
        VOID
    ) const = 0;

    virtual
    DWORD
    GetSiteId(
        VOID
    ) const = 0;

    virtual
    VOID
    UpdateFrequentlyHitTickCount(
        DWORD   dwTicks
    ) = 0;

    virtual
    PCWSTR
    GetConfigPath(
        VOID
    ) const = 0;
};

//
// Key and data for file-cache
//

#define FILE_CACHE_NAME             L"FILE"

class __declspec(uuid("7377f6a4-266c-4043-b62a-9ea955a5e97a"))
IFileKey : public IHttpCacheKey
{
public:

    PCWSTR
    GetCacheName(
        VOID
    ) const
    {
        return FILE_CACHE_NAME;
    }

    virtual
    PCWSTR
    GetPath(
        VOID
    ) const = 0;
};

//
// Key and data for token-cache
//

#define TOKEN_CACHE_NAME            L"TOKEN"

class __declspec(uuid("1d3dc8cb-fc52-42bc-97e1-1bf02136e8ba"))
IHttpTokenKey : public IHttpCacheKey
{
public:

    PCWSTR
    GetCacheName(
        VOID
    ) const
    {
        return TOKEN_CACHE_NAME;
    }

    virtual
    PCWSTR
    GetUserName(
        VOID
    ) const = 0;

    virtual
    PCWSTR
    GetPasswordHash(
        VOID
    ) const = 0;

    //
    // Return the LOGON32_LOGON_* method used when logging on the user
    //

    virtual
    DWORD
    GetLogonMethod(
        VOID
    ) const = 0;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
