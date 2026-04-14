
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
#ifndef __windows2Edevices2Ebackground_h__
#define __windows2Edevices2Ebackground_h__
#ifndef __windows2Edevices2Ebackground_p_h__
#define __windows2Edevices2Ebackground_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Background {
                interface IDeviceServicingDetails;
            } /* Background */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails ABI::Windows::Devices::Background::IDeviceServicingDetails

#endif // ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Background {
                interface IDeviceUseDetails;
            } /* Background */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails ABI::Windows::Devices::Background::IDeviceUseDetails

#endif // ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Devices.Background.IDeviceServicingDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Background.DeviceServicingDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Background_IDeviceServicingDetails[] = L"Windows.Devices.Background.IDeviceServicingDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Background {
                MIDL_INTERFACE("4aabee29-2344-4ac4-8527-4a8ef6905645")
                IDeviceServicingDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpectedDuration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceServicingDetails = __uuidof(IDeviceServicingDetails);
            } /* Background */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Background.IDeviceUseDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Background.DeviceUseDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Background_IDeviceUseDetails[] = L"Windows.Devices.Background.IDeviceUseDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Background {
                MIDL_INTERFACE("7d565141-557e-4154-b994-e4f7a11fb323")
                IDeviceUseDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceUseDetails = __uuidof(IDeviceUseDetails);
            } /* Background */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Background.DeviceServicingDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Background.IDeviceServicingDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Background_DeviceServicingDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Background_DeviceServicingDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Background_DeviceServicingDetails[] = L"Windows.Devices.Background.DeviceServicingDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Background.DeviceUseDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Background.IDeviceUseDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Background_DeviceUseDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Background_DeviceUseDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Background_DeviceUseDetails[] = L"Windows.Devices.Background.DeviceUseDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails;

#endif // ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails;

#endif // ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

/*
 *
 * Interface Windows.Devices.Background.IDeviceServicingDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Background.DeviceServicingDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Background_IDeviceServicingDetails[] = L"Windows.Devices.Background.IDeviceServicingDetails";
typedef struct __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpectedDuration)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_get_ExpectedDuration(This, value) \
    ((This)->lpVtbl->get_ExpectedDuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceServicingDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Background.IDeviceUseDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Background.DeviceUseDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Background_IDeviceUseDetails[] = L"Windows.Devices.Background.IDeviceUseDetails";
typedef struct __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CBackground_CIDeviceUseDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Background.DeviceServicingDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Background.IDeviceServicingDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Background_DeviceServicingDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Background_DeviceServicingDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Background_DeviceServicingDetails[] = L"Windows.Devices.Background.DeviceServicingDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Background.DeviceUseDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Background.IDeviceUseDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Background_DeviceUseDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Background_DeviceUseDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Background_DeviceUseDetails[] = L"Windows.Devices.Background.DeviceUseDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ebackground_p_h__

#endif // __windows2Edevices2Ebackground_h__
