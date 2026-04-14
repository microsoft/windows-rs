
#pragma warning( disable: 4049 )  /* more than 64k source lines */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include <rpc.h>
#include <rpcndr.h>

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include <windows.h>
#include <ole2.h>
#endif /*COM_NO_WINDOWS_H*/
#ifndef __windows2Edevices2Egeolocation2Eprovider_h__
#define __windows2Edevices2Egeolocation2Eprovider_h__
#ifndef __windows2Edevices2Egeolocation2Eprovider_p_h__
#define __windows2Edevices2Egeolocation2Eprovider_p_h__


#pragma once

//
// Deprecated attribute support
//

#pragma push_macro("DEPRECATED")
#undef DEPRECATED

#if !defined(DISABLE_WINRT_DEPRECATION)
#if defined(__cplusplus)
#if __cplusplus >= 201402
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#elif defined(_MSC_VER)
#if _MSC_VER >= 1900
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#else
#define DEPRECATED(x) __declspec(deprecated(x))
#define DEPRECATEDENUMERATOR(x)
#endif // _MSC_VER >= 1900
#else // Not Standard C++ or MSVC, ignore the construct.
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  // C++ deprecation
#else // C - disable deprecation
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif
#else // Deprecation is disabled
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  /* DEPRECATED */

// Disable Deprecation for this header, MIDL verifies that cross-type access is acceptable
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#else
#pragma warning(push)
#pragma warning(disable: 4996)
#endif

// Ensure that the setting of the /ns_prefix command line switch is consistent for all headers.
// If you get an error from the compiler indicating "warning C4005: 'CHECK_NS_PREFIX_STATE': macro redefinition", this
// indicates that you have included two different headers with different settings for the /ns_prefix MIDL command line switch
#if !defined(DISABLE_NS_PREFIX_CHECKS)
#define CHECK_NS_PREFIX_STATE "always"
#endif // !defined(DISABLE_NS_PREFIX_CHECKS)


#pragma push_macro("MIDL_CONST_ID")
#undef MIDL_CONST_ID
#define MIDL_CONST_ID const __declspec(selectany)


//  API Contract Inclusion Definitions
#if !defined(SPECIFIC_API_CONTRACT_DEFINITIONS)
#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Geolocation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Provider {
                    interface IGeolocationProvider;
                } /* Provider */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider ABI::Windows::Devices::Geolocation::Provider::IGeolocationProvider

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef struct BasicGeoposition BasicGeoposition;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef enum PositionSource : int PositionSource;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Provider {
                    typedef enum LocationOverrideStatus : int LocationOverrideStatus;
                } /* Provider */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Geolocation.Provider.LocationOverrideStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Provider {
                    enum LocationOverrideStatus : int
                    {
                        LocationOverrideStatus_Success = 0,
                        LocationOverrideStatus_AccessDenied = 1,
                        LocationOverrideStatus_AlreadyStarted = 2,
                        LocationOverrideStatus_Other = 3,
                    };
                } /* Provider */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.Geolocation.Provider.IGeolocationProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Provider.GeolocationProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Provider_IGeolocationProvider[] = L"Windows.Devices.Geolocation.Provider.IGeolocationProvider";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                namespace Provider {
                    MIDL_INTERFACE("e4cf071d-3f64-509f-8dc2-0b74a059829d")
                    IGeolocationProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsOverridden(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetOverridePosition(
                            ABI::Windows::Devices::Geolocation::BasicGeoposition newPosition,
                            ABI::Windows::Devices::Geolocation::PositionSource positionSource,
                            DOUBLE accuracyInMeters,
                            ABI::Windows::Devices::Geolocation::Provider::LocationOverrideStatus* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearOverridePosition(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_IsOverriddenChanged(
                            __FIEventHandler_1_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_IsOverriddenChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGeolocationProvider = __uuidof(IGeolocationProvider);
                } /* Provider */
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.Geolocation.Provider.GeolocationProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Provider.IGeolocationProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Provider_GeolocationProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Provider_GeolocationProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Provider_GeolocationProvider[] = L"Windows.Devices.Geolocation.Provider.GeolocationProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource;

typedef enum __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CLocationOverrideStatus __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CLocationOverrideStatus;

/*
 *
 * Struct Windows.Devices.Geolocation.Provider.LocationOverrideStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CLocationOverrideStatus
{
    LocationOverrideStatus_Success = 0,
    LocationOverrideStatus_AccessDenied = 1,
    LocationOverrideStatus_AlreadyStarted = 2,
    LocationOverrideStatus_Other = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.Geolocation.Provider.IGeolocationProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Geolocation.Provider.GeolocationProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Geolocation_Provider_IGeolocationProvider[] = L"Windows.Devices.Geolocation.Provider.IGeolocationProvider";
typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsOverridden)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetOverridePosition)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition newPosition,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CPositionSource positionSource,
        DOUBLE accuracyInMeters,
        enum __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CLocationOverrideStatus* result);
    HRESULT (STDMETHODCALLTYPE* ClearOverridePosition)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This);
    HRESULT (STDMETHODCALLTYPE* add_IsOverriddenChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsOverriddenChanged)(__x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProviderVtbl;

interface __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_get_IsOverridden(This, value) \
    ((This)->lpVtbl->get_IsOverridden(This, value))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_SetOverridePosition(This, newPosition, positionSource, accuracyInMeters, result) \
    ((This)->lpVtbl->SetOverridePosition(This, newPosition, positionSource, accuracyInMeters, result))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_ClearOverridePosition(This) \
    ((This)->lpVtbl->ClearOverridePosition(This))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_add_IsOverriddenChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsOverriddenChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_remove_IsOverriddenChanged(This, token) \
    ((This)->lpVtbl->remove_IsOverriddenChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGeolocation_CProvider_CIGeolocationProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Devices.Geolocation.Provider.GeolocationProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Geolocation.Provider.IGeolocationProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Devices_Geolocation_Provider_GeolocationProvider_DEFINED
#define RUNTIMECLASS_Windows_Devices_Geolocation_Provider_GeolocationProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Geolocation_Provider_GeolocationProvider[] = L"Windows.Devices.Geolocation.Provider.GeolocationProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Egeolocation2Eprovider_p_h__

#endif // __windows2Edevices2Egeolocation2Eprovider_h__
