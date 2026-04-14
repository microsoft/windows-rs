// Copyright (C) Microsoft Corporation. All rights reserved.

#ifndef _ROERROR_H
#define _ROERROR_H

#if defined(_MSC_VER)
#pragma once
#endif

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <hstring.h>
#include <restrictedErrorInfo.h>
#include <rpc.h>

#include <sdkddkver.h>

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef _OLE32_
#define WINOLEGLOBAL EXTERN_C
#else
#define WINOLEGLOBAL EXTERN_C DECLSPEC_IMPORT
#endif // _OLE32_

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef enum {
        RO_ERROR_REPORTING_NONE               = 0x00000000,
        RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS = 0x00000001,
        RO_ERROR_REPORTING_FORCEEXCEPTIONS    = 0x00000002,
        RO_ERROR_REPORTING_USESETERRORINFO    = 0x00000004,
        RO_ERROR_REPORTING_SUPPRESSSETERRORINFO = 0x00000008,
}RO_ERROR_REPORTING_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(RO_ERROR_REPORTING_FLAGS);

//not including the terminating null
#define MAX_ERROR_MESSAGE_CHARS 512

STDAPI
RoGetErrorReportingFlags(
    _Out_ UINT32* pflags
    );

STDAPI
RoSetErrorReportingFlags(
    _In_ UINT32 flags
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

STDAPI
RoResolveRestrictedErrorInfoReference(
    _In_ PCWSTR reference,
    _Outptr_ IRestrictedErrorInfo** ppRestrictedErrorInfo
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

STDAPI
SetRestrictedErrorInfo(
    _In_opt_ IRestrictedErrorInfo* pRestrictedErrorInfo
    );

STDAPI
GetRestrictedErrorInfo(
    _Outptr_result_maybenull_ IRestrictedErrorInfo** ppRestrictedErrorInfo
    );

STDAPI_(BOOL)
RoOriginateErrorW(
    _In_ HRESULT error,
    _In_ UINT cchMax,
    _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS) )
             _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
             _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) ) PCWSTR message
    );

STDAPI_(BOOL)
RoOriginateError(
    _In_ HRESULT error,
    _In_opt_ HSTRING message
    );

STDAPI_(BOOL)
RoTransformErrorW(
    _In_ HRESULT oldError,
    _In_ HRESULT newError,
    _In_ UINT cchMax,
    _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS))
             _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
             _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) ) PCWSTR message
    );

STDAPI_(BOOL)
RoTransformError(
    _In_ HRESULT oldError,
    _In_ HRESULT newError,
    _In_opt_ HSTRING message
    );

STDAPI
RoCaptureErrorContext(
    HRESULT hr
    );

STDAPI_(void)
RoFailFastWithErrorContext(
    HRESULT hrError
    );

#ifdef __cplusplus
namespace Windows
{
    namespace Foundation
    {
        namespace Diagnostics
        {
            typedef enum {
                None               = RO_ERROR_REPORTING_NONE,
                SuppressExceptions = RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS,
                ForceExceptions    = RO_ERROR_REPORTING_FORCEEXCEPTIONS,
                UseSetErrorInfo    = RO_ERROR_REPORTING_USESETERRORINFO,
                SuppressSetErrorInfo = RO_ERROR_REPORTING_SUPPRESSSETERRORINFO,
            } RoErrorReportingFlags;

            DEFINE_ENUM_FLAG_OPERATORS(RoErrorReportingFlags);

            inline HRESULT GetErrorReportingFlags(_Out_ UINT32 *pflags)
            {
                return ::RoGetErrorReportingFlags(pflags);
            }

            inline HRESULT SetErrorReportingFlags(_In_ UINT32 flags)
            {
                return ::RoSetErrorReportingFlags(flags);
            }

            inline BOOL OriginateError(_In_ HRESULT error, _In_ UINT cchMax,
                _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS) )
                _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
                _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) )
                 PCWSTR message)
            {
                if (SUCCEEDED(error))
                {
                    return FALSE;
                }

                return ::RoOriginateErrorW(error, cchMax, message);
            }

            inline BOOL OriginateError(_In_ HRESULT error, _In_ HSTRING message)
            {
                if (SUCCEEDED(error))
                {
                    return FALSE;
                }

                return ::RoOriginateError(error, message);
            }

            inline BOOL TransformError(_In_ HRESULT oldError, _In_ HRESULT newError, _In_ UINT cchMax,
             _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS))
             _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
             _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) ) PCWSTR message)
            {
                if ((oldError == newError) ||
                    (SUCCEEDED(oldError) && SUCCEEDED(newError)))
                {
                    return FALSE;
                }

                return ::RoTransformErrorW(oldError, newError, cchMax, message);
            }

            inline BOOL TransformError(_In_ HRESULT oldError, _In_ HRESULT newError, _In_ HSTRING message)
            {
                if ((oldError == newError) ||
                    (SUCCEEDED(oldError) && SUCCEEDED(newError)))
                {
                    return FALSE;
                }

                return ::RoTransformError(oldError, newError, message);
            }
        }
    }
}

namespace ABI
{
    namespace Windows
    {
        namespace Foundation
        {
            namespace Diagnostics
            {
                typedef enum {
                    None               = RO_ERROR_REPORTING_NONE,
                    SuppressExceptions = RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS,
                    ForceExceptions    = RO_ERROR_REPORTING_FORCEEXCEPTIONS,
                    UseSetErrorInfo    = RO_ERROR_REPORTING_USESETERRORINFO,
                    SuppressSetErrorInfo = RO_ERROR_REPORTING_SUPPRESSSETERRORINFO,
                } RoErrorReportingFlags;

                DEFINE_ENUM_FLAG_OPERATORS(RoErrorReportingFlags);

                inline HRESULT GetErrorReportingFlags(_Out_ UINT32 *pflags)
                {
                    return ::RoGetErrorReportingFlags(pflags);
                }

                inline HRESULT SetErrorReportingFlags(_In_ UINT32 flags)
                {
                    return ::RoSetErrorReportingFlags(flags);
                }

                inline BOOL OriginateError(_In_ HRESULT error, _In_ UINT cchMax,
                    _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS) )
                    _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
                    _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) )
                    PCWSTR message)
                {
                    return ::RoOriginateErrorW(error, cchMax, message);
                }

                inline BOOL OriginateError(_In_ HRESULT error, _In_ HSTRING message)
                {
                    return ::RoOriginateError(error, message);
                }

                inline BOOL TransformError(_In_ HRESULT oldError, _In_ HRESULT newError, _In_ UINT cchMax,
                    _When_(cchMax == 0, _In_reads_or_z_opt_(MAX_ERROR_MESSAGE_CHARS))
                    _When_(cchMax > 0 && cchMax < MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(cchMax) )
                    _When_(cchMax >= MAX_ERROR_MESSAGE_CHARS, _In_reads_or_z_(MAX_ERROR_MESSAGE_CHARS) ) PCWSTR message)
                {
                    return ::RoTransformErrorW(oldError, newError, cchMax, message);
                }

                inline BOOL TransformError(_In_ HRESULT oldError, _In_ HRESULT newError, _In_ HSTRING message)
                {
                    return ::RoTransformError(oldError, newError, message);
                }
            }
        }
    }
}

#endif  //__cplusplus

//  Exception Codes are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code

//0x40080201L
#define EXCEPTION_RO_ORIGINATEERROR  ((DWORD)((0x1 << 30) | (FACILITY_WINDOWS << 16) | 0x201))

//0x40080202L
#define EXCEPTION_RO_TRANSFORMERROR  ((DWORD)((0x1 << 30) | (FACILITY_WINDOWS << 16) | 0x202))

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

// The various versions of RoOriginateError will raise and catch a structured exception with an
// exception code of EXCEPTION_RO_ORIGINATEERROR and the following information in the
// ExceptionInformation array. The characterCount element is the number of characters in the message,
// not including the NULL terminator.
// ExceptionInformation[0] -- HRESULT error
// ExceptionInformation[1] -- UINT characterCount
// ExceptionInformation[2] -- PCWSTR message
//
// The various versions of RoTransformError will raise and catch a structured exception with an
// exception code of EXCEPTION_RO_TRANSFORMERROR and the following information in the
// ExceptionInformation array. The characterCountelement is the number of characters in the message,
// not including the NULL terminator.
// ExceptionInformation[0] -- HRESULT oldError
// ExceptionInformation[1] -- HRESULT newError
// ExceptionInformation[2] -- UINT characterCount
// ExceptionInformaiton[3] -- PCWSTR message
//
// As documented in MSDN, the ExceptionInformation can be retrieved in the filter expression
// of a frame-based exception handler by calling GetExceptionInformation. A debugger can
// retrieve these values by calling WaitForDebugEvent, keeping in mind that exception records have
// a different binary layout between 32bit and 64bit processes.

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

STDAPI_(BOOL)
RoOriginateLanguageException(
    _In_ HRESULT error,
    _In_opt_ HSTRING message,
    _In_opt_ IUnknown* languageException
    );

STDAPI_(void)
RoClearError(
    void
    );

STDAPI
RoReportUnhandledError(
    _In_ IRestrictedErrorInfo* pRestrictedErrorInfo
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef HRESULT (WINAPI *PINSPECT_MEMORY_CALLBACK)(
    _In_opt_ void* context,
    UINT_PTR readAddress,
    UINT32 length,
    _Out_writes_(length) BYTE* buffer);

STDAPI
RoInspectThreadErrorInfo(
    _In_ UINT_PTR targetTebAddress,
    USHORT machine,
    PINSPECT_MEMORY_CALLBACK readMemoryCallback,
    _In_opt_ PVOID context,
    _Out_ UINT_PTR* targetErrorInfoAddress
    );

STDAPI
RoInspectCapturedStackBackTrace(
    _In_ UINT_PTR targetErrorInfoAddress,
    USHORT machine,
    PINSPECT_MEMORY_CALLBACK readMemoryCallback,
    _In_opt_ PVOID context,
    _Out_ UINT32* frameCount,
    _Out_ UINT_PTR* targetBackTraceAddress
    );

STDAPI
RoGetMatchingRestrictedErrorInfo(
    _In_ HRESULT hrIn,
    _COM_Outptr_ IRestrictedErrorInfo** ppRestrictedErrorInfo
    );

STDAPI
RoReportFailedDelegate(
    _In_ IUnknown* punkDelegate,
    _In_ IRestrictedErrorInfo* pRestrictedErrorInfo
    );

STDAPI_(BOOL)
IsErrorPropagationEnabled(
    void
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // #if (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* _ROERROR_H */
