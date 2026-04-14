/********************************************************************************
*                                                                               *
* Pathcch.h -- ApiSet Contract for api-ms-win-core-Path-l1                      *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#pragma once

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef enum PATHCCH_OPTIONS
{
    PATHCCH_NONE = 0x0,

    // This option allows applications to gain access to long paths. It has two
    // different behaviors. For process configured to enable long paths it will allow
    // the returned path to be longer than the max path limit that is normally imposed.
    // For process that are not this option will convert long paths into the extended
    // length DOS device form (with \\?\ prefix) when the path is longer than the limit.
    // This form is not length limited by the Win32 file system API on all versions of Windows.
    // This second behavior is the same behavior for OSes that don't have the long path feature.
    // This can not be specified with PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH.
    PATHCCH_ALLOW_LONG_PATHS = 0x01,

    // Can only be used when PATHCCH_ALLOW_LONG_PATHS is specified. This
    // Forces the API to treat the caller as long path enabled, independent of the
    // process's long name enabled state. Cannot be used with PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS.
    PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS = 0x02,

    // Can only be used when PATHCCH_ALLOW_LONG_PATHS is specified. This
    // Forces the API to treat the caller as long path disabled, independent of the
    // process's long name enabled state. Cannot be used with PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS.
    PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS = 0x04,

    // Disable the normalization of path segments that includes removing trailing dots and spaces.
    // This enables access to paths that win32 path normalization will block.
    PATHCCH_DO_NOT_NORMALIZE_SEGMENTS = 0x08,

    // Convert the input path into the extended length DOS device path form (with the \\?\ prefix)
    // if not already in that form. This enables access to paths that are otherwise not addressable
    // due to Win32 normalization rules (that can strip trailing dots and spaces) and path
    // length limitations. This option implies the same behavior of PATHCCH_DO_NOT_NORMALIZE_SEGMENTS.
    // This can not be specified with PATHCCH_ALLOW_LONG_PATHS.
    PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH = 0x10,

    // When combining or normalizing a path ensure there is a trailing backslash.
    PATHCCH_ENSURE_TRAILING_SLASH = 0x020,

    // Convert forward slashes to back slashes and collapse multiple slashes.
    // This is needed to to support sub-path or identity comparisons.
    PATHCCH_CANONICALIZE_SLASHES = 0x040,

} PATHCCH_OPTIONS;
DEFINE_ENUM_FLAG_OPERATORS(PATHCCH_OPTIONS)

#define VOLUME_PREFIX           L"\\\\?\\Volume"
#define VOLUME_PREFIX_LEN       (ARRAYSIZE(VOLUME_PREFIX) - 1)

// max # of characters we support using the "\\?\" syntax
// (0x7FFF + 1 for NULL terminator)
#define PATHCCH_MAX_CCH             0x8000

WINPATHCCHAPI
BOOL
APIENTRY
PathIsUNCEx(
    _In_ PCWSTR pszPath,
    _Outptr_opt_ PCWSTR* ppszServer
    );

WINPATHCCHAPI
BOOL
APIENTRY
PathCchIsRoot(
    _In_opt_ PCWSTR pszPath
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchAddBackslashEx(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath,
    _Outptr_opt_result_buffer_(*pcchRemaining) PWSTR* ppszEnd,
    _Out_opt_ size_t* pcchRemaining
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchAddBackslash(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchRemoveBackslashEx(
    _Inout_updates_(_Inexpressible_(cchPath)) PWSTR pszPath,
    _In_ size_t cchPath,
    _Outptr_opt_result_buffer_(*pcchRemaining) PWSTR* ppszEnd,
    _Out_opt_ size_t* pcchRemaining
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchRemoveBackslash(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchSkipRoot(
    _In_ PCWSTR pszPath,
    _Outptr_ PCWSTR* ppszRootEnd
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchStripToRoot(
    _Inout_updates_(_Inexpressible_(cchPath)) PWSTR pszPath,
    _In_ size_t cchPath
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchRemoveFileSpec(
    _Inout_updates_(_Inexpressible_(cchPath)) PWSTR pszPath,
    _In_ size_t cchPath
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchFindExtension(
    _In_reads_(_Inexpressible_(cchPath)) PCWSTR pszPath,
    _In_ size_t cchPath,
    _Outptr_ PCWSTR* ppszExt
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchAddExtension(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath,
    _In_ PCWSTR pszExt
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchRenameExtension(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath,
    _In_ PCWSTR pszExt
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchRemoveExtension(
    _Inout_updates_(_Inexpressible_(cchPath)) PWSTR pszPath,
    _In_ size_t cchPath
    );

/* PATHCCH_OPTIONS */
WINPATHCCHAPI
HRESULT
APIENTRY
PathCchCanonicalizeEx(
    _Out_writes_(cchPathOut) PWSTR pszPathOut,
    _In_ size_t cchPathOut,
    _In_ PCWSTR pszPathIn,
    _In_ ULONG dwFlags
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchCanonicalize(
    _Out_writes_(cchPathOut) PWSTR pszPathOut,
    _In_ size_t cchPathOut,
    _In_ PCWSTR pszPathIn
    );

/* PATHCCH_OPTIONS */
WINPATHCCHAPI
HRESULT
APIENTRY
PathCchCombineEx(
    _Out_writes_(cchPathOut) PWSTR pszPathOut,
    _In_ size_t cchPathOut,
    _In_opt_ PCWSTR pszPathIn,
    _In_opt_ PCWSTR pszMore,
    _In_ ULONG dwFlags
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchCombine(
    _Out_writes_(cchPathOut) PWSTR pszPathOut,
    _In_ size_t cchPathOut,
    _In_opt_ PCWSTR pszPathIn,
    _In_opt_ PCWSTR pszMore
    );

/* PATHCCH_OPTIONS */
WINPATHCCHAPI
HRESULT
APIENTRY
PathCchAppendEx(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath,
    _In_opt_ PCWSTR pszMore,
    _In_ ULONG dwFlags
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchAppend(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath,
    _In_opt_ PCWSTR pszMore
    );

WINPATHCCHAPI
HRESULT
APIENTRY
PathCchStripPrefix(
    _Inout_updates_(cchPath) PWSTR pszPath,
    _In_ size_t cchPath
    );

/* PATHCCH_OPTIONS */
WINPATHCCHAPI
HRESULT
APIENTRY
PathAllocCombine(
    _In_opt_ PCWSTR pszPathIn,
    _In_opt_ PCWSTR pszMore,
    _In_ ULONG dwFlags,
    _Outptr_ PWSTR* ppszPathOut
    );

/* PATHCCH_OPTIONS */
WINPATHCCHAPI
HRESULT
APIENTRY
PathAllocCanonicalize(
    _In_ PCWSTR pszPathIn,
    _In_ ULONG dwFlags,
    _Outptr_ PWSTR* ppszPathOut
    );

#ifndef PATHCCH_NO_DEPRECATE
// Deprecate the old path functions that do not take a buffer size (and assume MAX_PATH) to generate compile time errors.
// #define PATHCCH_NO_DEPRECATE before including this file to disable these deprecations.
#ifdef DEPRECATE_SUPPORTED

#pragma deprecated(PathAddBackslashA)
#pragma deprecated(PathAddBackslashW)
#pragma deprecated(PathAddExtensionA)
#pragma deprecated(PathAddExtensionW)
#pragma deprecated(PathAppendA)
#pragma deprecated(PathAppendW)
#pragma deprecated(PathCanonicalizeA)
#pragma deprecated(PathCanonicalizeW)
#pragma deprecated(PathCombineA)
#pragma deprecated(PathCombineW)
#pragma deprecated(PathRenameExtensionA)
#pragma deprecated(PathRenameExtensionW)

#endif  // !DEPRECATE_SUPPORTED
#endif  // !PATHCCH_NO_DEPRECATE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// non-const overload (C++ only)

_Success_(return != 0)
__inline BOOL PathIsUNCEx(_In_ PWSTR pszPath, _Outptr_opt_ PWSTR *ppszServer)
{
    return PathIsUNCEx(const_cast<PCWSTR>(pszPath), const_cast<PCWSTR*>(ppszServer));
}

__inline HRESULT
PathCchSkipRoot(
    _In_ PWSTR pszPath,
    _Outptr_ PWSTR *ppszRootEnd)
{
    return PathCchSkipRoot(const_cast<PCWSTR>(pszPath), const_cast<PCWSTR*>(ppszRootEnd));
}

__inline HRESULT
PathCchFindExtension(
    _In_reads_(_Inexpressible_(cchPath)) PWSTR pszPath,
    _In_ size_t cchPath,
    _Outptr_ PWSTR *ppszExt)
{
    return PathCchFindExtension(const_cast<PCWSTR>(pszPath), cchPath, const_cast<PCWSTR*>(ppszExt));
}

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

