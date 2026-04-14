// Copyright (c) Microsoft Corporation. All rights reserved.

#ifndef __ROAPI_H_
#define __ROAPI_H_

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
#include <inspectable.h>
#include <combaseapi.h>
#include <activation.h>
#include <hstring.h>

#ifdef _ROAPI_
#define ROAPI
#else
#define ROAPI DECLSPEC_IMPORT
#endif // _ROAPI_

#ifdef __cplusplus
extern "C" {
#endif

// Ro initialization flags; passed to Windows::Runtime::Initialize
typedef enum RO_INIT_TYPE
{
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
    RO_INIT_SINGLETHREADED     = 0,      // Single-threaded application
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
    RO_INIT_MULTITHREADED      = 1,      // COM calls objects on any thread.
} RO_INIT_TYPE;

// RegisterActivationFactory/RevokeActivationFactory registration cookie
#ifdef __cplusplus
typedef struct _RO_REGISTRATION_COOKIE {} *RO_REGISTRATION_COOKIE;
#else
typedef struct _RO_REGISTRATION_COOKIE *RO_REGISTRATION_COOKIE; /* make this header includable in C files */
#endif
// RegisterActivationFactory/DllGetActivationFactory callback
typedef HRESULT (STDAPICALLTYPE * PFNGETACTIVATIONFACTORY)(HSTRING, IActivationFactory **);

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// forward declarations of the dllexport'd versions
ROAPI
_Check_return_
HRESULT
WINAPI
RoInitialize(
    _In_ RO_INIT_TYPE initType
    );

ROAPI
void
WINAPI
RoUninitialize(
    );

ROAPI
_Check_return_
HRESULT
WINAPI
RoActivateInstance(
    _In_ HSTRING activatableClassId,
    _COM_Outptr_ IInspectable** instance
    );

ROAPI
_Check_return_
HRESULT
WINAPI
RoRegisterActivationFactories(
    _In_reads_(count) HSTRING* activatableClassIds,
    _In_reads_(count) PFNGETACTIVATIONFACTORY* activationFactoryCallbacks,
    _In_ UINT32 count,
    _Out_ RO_REGISTRATION_COOKIE* cookie
    );

ROAPI
void
WINAPI
RoRevokeActivationFactories(
    _In_ RO_REGISTRATION_COOKIE cookie
    );

ROAPI
_Check_return_
HRESULT
WINAPI
RoGetActivationFactory(
    _In_ HSTRING activatableClassId,
    _In_ REFIID iid,
    _COM_Outptr_ void** factory
    );

typedef interface IApartmentShutdown IApartmentShutdown;
DECLARE_HANDLE(APARTMENT_SHUTDOWN_REGISTRATION_COOKIE);

ROAPI
_Check_return_
HRESULT
WINAPI
RoRegisterForApartmentShutdown(
    _In_ IApartmentShutdown* callbackObject,
    _Out_ UINT64* apartmentIdentifier,
    _Out_ APARTMENT_SHUTDOWN_REGISTRATION_COOKIE* regCookie
    );

ROAPI
_Check_return_
HRESULT
WINAPI
RoUnregisterForApartmentShutdown(
    _In_ APARTMENT_SHUTDOWN_REGISTRATION_COOKIE regCookie
    );

ROAPI
_Check_return_
HRESULT
WINAPI
RoGetApartmentIdentifier(
    _Out_ UINT64* apartmentIdentifier
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus

namespace Windows
{
    namespace Foundation
    {
        // initialize / uninitialize
        _Check_return_
        __inline HRESULT Initialize(_In_ RO_INIT_TYPE initType
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
             = RO_INIT_SINGLETHREADED
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
            )
        {
            return RoInitialize(initType);
        }

        __inline void Uninitialize()
        {
            RoUninitialize();
        }

        // creation
        template<class T>
        _Check_return_
        __inline  HRESULT ActivateInstance(_In_ HSTRING activatableClassId,_COM_Outptr_ T** instance)
        {
            *instance = nullptr;

            IInspectable* pInspectable;
            HRESULT hr = RoActivateInstance(activatableClassId, &pInspectable);
            if (SUCCEEDED(hr))
            {
                if (__uuidof(T) == __uuidof(IInspectable))
                {
                    *instance = static_cast<T*>(pInspectable);
                }
                else
                {
                    hr = pInspectable->QueryInterface(IID_PPV_ARGS(instance));
                    pInspectable->Release();
                }
            }
            return hr;
        }

        // registration
        _Check_return_
        __inline HRESULT RegisterActivationFactories(
                _In_reads_(count) HSTRING* activatableClassIds,
                _In_reads_(count) PFNGETACTIVATIONFACTORY* activationFactoryCallbacks,
                _In_ UINT32 count,
                _Out_ RO_REGISTRATION_COOKIE* cookie)
        {
            return RoRegisterActivationFactories(activatableClassIds, activationFactoryCallbacks, count, cookie);
        }

        __inline void RevokeActivationFactories(_In_ RO_REGISTRATION_COOKIE cookie)
        {
            RoRevokeActivationFactories(cookie);
        }

        // get activation factory
        template<class T>
        _Check_return_
        __inline HRESULT GetActivationFactory(
            _In_        HSTRING activatableClassId,
            _COM_Outptr_ T**     factory)
        {
            return RoGetActivationFactory(activatableClassId, IID_PPV_ARGS(factory));
        }
    }
}

namespace ABI
{
    namespace Windows
    {
        namespace Foundation
        {
            // initialize / uninitialize
            _Check_return_
            __inline HRESULT Initialize(_In_ RO_INIT_TYPE initType
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
                 = RO_INIT_SINGLETHREADED
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
                )
            {
                return RoInitialize(initType);
            }

            __inline void Uninitialize()
            {
                RoUninitialize();
            }

            // creation
            template<class T>
            _Check_return_
            __inline  HRESULT ActivateInstance(_In_ HSTRING activatableClassId,_COM_Outptr_ T** instance)
            {
                return ::Windows::Foundation::ActivateInstance(activatableClassId, instance);
            }

            // registration
            _Check_return_
            __inline HRESULT RegisterActivationFactories(
                    _In_reads_(count) HSTRING* activatableClassIds,
                    _In_reads_(count) PFNGETACTIVATIONFACTORY* activationFactoryCallbacks,
                    _In_ UINT32 count,
                    _Out_ RO_REGISTRATION_COOKIE* cookie)
            {
                return RoRegisterActivationFactories(activatableClassIds, activationFactoryCallbacks, count, cookie);
            }

            __inline void RevokeActivationFactories(_In_ RO_REGISTRATION_COOKIE cookie)
            {
                RoRevokeActivationFactories(cookie);
            }

            // get activation factory
            template<class T>
            _Check_return_
            __inline HRESULT GetActivationFactory(
                _In_        HSTRING activatableClassId,
                _COM_Outptr_ T**     factory)
            {
                return RoGetActivationFactory(activatableClassId, IID_PPV_ARGS(factory));
            }
        }
    }
}

#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
