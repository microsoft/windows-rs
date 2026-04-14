//
// Copyright (C) Microsoft Corporation
// All rights reserved.
//
// Code in details namespace is for internal usage within the library code
//

#ifndef _WRL_DEF_H_
#define _WRL_DEF_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <sdkddkver.h>
#include <sal.h>

// Warnings outside of the push/pop sequence will be disabled for all user
// projects.  The only warnings that should be disabled outside the push/pop
// are warnings that are a) benign and b) will show up in user projects
// without being directly caused by the user
#pragma region disable warnings

#pragma warning(disable: 4505) // unreferenced local function has been removed
#pragma warning(disable: 4503) // decorated name length exceeded, name was truncated
#pragma warning(disable: 4482) // nonstandard extension used: enum 'enum::enumvalue' used in qualified name

#pragma endregion // disable warnings


// Minimal requirements for compiler
#if _MSC_VER < 1600
#error WRL requires compiler version 16.00 or greater
#endif

// Requires to be compiled by C++ compiler
#ifndef __cplusplus
#error WRL requires C++ compilation (use a .cpp suffix)
#endif

// Don't allow to compile sources with /clr
#ifdef _MANAGED
#error WRL cannot be compiled with /clr option enabled
#endif

// Minimal requirement set to Windows Vista
#if NTDDI_VERSION < NTDDI_VISTA 
#error WRL requires NTDDI_VERSION to be #defined at least to NTDDI_VISTA or greater
#endif

// IA64 architecture is not supported by WRL
#ifdef _M_IA64
#error WRL doesn't support IA64 architecture
#endif

#if ((!defined(_PREFAST_) || (_MSC_FULL_VER >= 160021202)) ) /* BUG ESC:849 - disabling under prefast */
#else /* BUG ESC:849 - disabling under prefast */
 // PLACEHOLDER CODE:
#define nullptr  0
#endif /* BUG ESC:849 - disabling under prefast */

#endif // _WRL_DEF_H_
