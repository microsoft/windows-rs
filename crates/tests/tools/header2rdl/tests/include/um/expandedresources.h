//
// Copyright (c) Microsoft Corporation. All rights reserved.
//

//
// API Set Contract:
//
//    api-ms-win-gaming-expandedresources-l1-1-*
//
// Abstract:
//
//    This header file provides API function signatures for expanded resources / GameMode apps.
//

#ifdef MSC_VER
#pragma once
#endif

#ifndef _APISET_EXPANDEDRESOURCES_
#define _APISET_EXPANDEDRESOURCES_

#include <apiset.h>
#include <apisetcconv.h>
#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#endif

#if defined(__cplusplus)
extern "C" {
#endif

HRESULT
WINAPI
HasExpandedResources(
    _Out_ BOOL* hasExpandedResources
    );

HRESULT
WINAPI
GetExpandedResourceExclusiveCpuCount(
    _Out_ ULONG* exclusiveCpuCount
    );

HRESULT
WINAPI
ReleaseExclusiveCpuSets(
    );

#if defined(__cplusplus)
} // end extern "C"
#endif // defined(__cplusplus)

#endif // _APISET_EXPANDEDRESOURCES_

