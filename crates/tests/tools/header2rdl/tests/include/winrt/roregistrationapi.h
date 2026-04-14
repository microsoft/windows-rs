/*****************************************************************************
 **                                                                         **
 ** RoRegistrationApi.h - Header for Windows Runtime catalog registrations. **
 **                                                                         **
 ** Copyright (c) Microsoft Corporation. All rights reserved.               **
 **                                                                         **
 *****************************************************************************/

#ifndef __ROREGISTRATIONAPI_H_
#define __ROREGISTRATIONAPI_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <wtypesbase.h>
#include <hstring.h>
#include <activationregistration.h>

#include <sdkddkver.h>
#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef _ROAPI_
#define ROAPI
#else
#define ROAPI DECLSPEC_IMPORT
#endif // _ROAPI_

#ifdef ROREGISTRATION_NO_ABI_PREFIX
#ifdef __cplusplus
typedef Windows::Foundation::IActivatableClassRegistration *PActivatableClassRegistration;
#else
typedef __x_Windows_CFoundation_CIActivatableClassRegistration *PActivatableClassRegistration;
#endif
#else // ifdef ROREGISTRATION_NO_ABI_PREFIX
#ifdef __cplusplus
typedef ABI::Windows::Foundation::IActivatableClassRegistration *PActivatableClassRegistration;
#else
typedef __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration *PActivatableClassRegistration;
#endif
#endif // ROREGISTRATION_NO_ABI_PREFIX

#ifdef __cplusplus
extern "C" {
#endif

ROAPI
_Check_return_
HRESULT
WINAPI
RoGetActivatableClassRegistration(
    _In_ HSTRING activatableClassId,
    _COM_Outptr_ PActivatableClassRegistration* activatableClassRegistration
    );

ROAPI
_On_failure_(_At_(*activatableClassIds, _Post_ _Null_))
_Check_return_
HRESULT
WINAPI
RoGetServerActivatableClasses(
    _In_ HSTRING serverName,
    _When_(return >= 0, _Outptr_result_buffer_(*count)) HSTRING** activatableClassIds,
    _Out_ DWORD* count
    );

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#ifndef ROREGISTRATION_NO_ABI_PREFIX
namespace ABI
{
#endif // ROREGISTRATION_NO_ABI_PREFIX
    namespace Windows
    {
        namespace Foundation
        {
            // Query activation catalog
            _Check_return_
            inline HRESULT GetActivatableClassRegistration(
                _In_ HSTRING activatableClassId,
                _COM_Outptr_ PActivatableClassRegistration *activatableClassRegistration
                )
            {
                return RoGetActivatableClassRegistration(activatableClassId, activatableClassRegistration);
            }

            // Query activation catalog
            _On_failure_(_At_(*activatableClassIds, _Post_ _Null_))
            _Check_return_
            inline HRESULT GetServerActivatableClasses(
                       _In_ HSTRING serverName,
                       _When_(return >= 0, _Outptr_result_buffer_(*count)) HSTRING **activatableClassIds,
                       _Out_ DWORD *count)
            {
                return RoGetServerActivatableClasses(serverName, activatableClassIds, count);
            }
        }
    }
#ifndef ROREGISTRATION_NO_ABI_PREFIX
}
#endif // ROREGISTRATION_NO_ABI_PREFIX

#endif // __cplusplus

#endif // #if (NTDDI_VERSION >= NTDDI_WIN8)

#endif
