//
// Copyright (c) Microsoft Corporation. All rights reserved.
//

//
// API Set Contract:
//
//    api-ms-win-gaming-experience-l1-1-*
//
// Abstract:
//
//    This header file provides API function signatures for gaming experience apps.
//

#ifdef MSC_VER
#pragma once
#endif

#ifndef _APISET_GAMING_EXPERIENCE_
#define _APISET_GAMING_EXPERIENCE_

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

BOOL
WINAPI
IsGamingFullScreenExperienceSupported(
    );

BOOL
WINAPI
IsGamingFullScreenExperienceActive(
    );

BOOL
WINAPI
CanSetGamingFullScreenExperience(
    );

HRESULT
WINAPI
SetGamingFullScreenExperience(
    _In_ BOOL active
    );

typedef
VOID (CALLBACK *PGAMING_FULL_SCREEN_EXPERIENCE_CHANGE_ROUTINE) (
    _In_ PVOID context
    );

DECLARE_HANDLE(GAMING_FULL_SCREEN_EXPERIENCE_REGISTRATION);

HRESULT
WINAPI
RegisterGamingFullScreenExperienceChangeNotification(
    _In_ PGAMING_FULL_SCREEN_EXPERIENCE_CHANGE_ROUTINE routine,
    _In_opt_ PVOID context,
    _Out_ GAMING_FULL_SCREEN_EXPERIENCE_REGISTRATION* registration
    );

VOID
WINAPI
UnregisterGamingFullScreenExperienceChangeNotification(
    _Inout_ GAMING_FULL_SCREEN_EXPERIENCE_REGISTRATION registration
    );

#if defined(__cplusplus)
} // end extern "C"
#endif // defined(__cplusplus)

#endif // _APISET_GAMING_EXPERIENCE_

