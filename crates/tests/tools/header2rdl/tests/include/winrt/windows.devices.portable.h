
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
#ifndef __windows2Edevices2Eportable_h__
#define __windows2Edevices2Eportable_h__
#ifndef __windows2Edevices2Eportable_p_h__
#define __windows2Edevices2Eportable_p_h__


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
#if !defined(WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION)
#define WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                interface IServiceDeviceStatics;
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics ABI::Windows::Devices::Portable::IServiceDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                interface IStorageDeviceStatics;
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics ABI::Windows::Devices::Portable::IStorageDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                typedef enum ServiceDeviceType : int ServiceDeviceType;
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Portable.ServiceDeviceType
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                enum ServiceDeviceType : int
                {
                    ServiceDeviceType_CalendarService = 0,
                    ServiceDeviceType_ContactsService = 1,
                    ServiceDeviceType_DeviceStatusService = 2,
                    ServiceDeviceType_NotesService = 3,
                    ServiceDeviceType_RingtonesService = 4,
                    ServiceDeviceType_SmsService = 5,
                    ServiceDeviceType_TasksService = 6,
                };
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Portable.IServiceDeviceStatics
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Portable.ServiceDevice
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Portable_IServiceDeviceStatics[] = L"Windows.Devices.Portable.IServiceDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                MIDL_INTERFACE("a88214e1-59c7-4a20-aba6-9f6707937230")
                IServiceDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        ABI::Windows::Devices::Portable::ServiceDeviceType serviceType,
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromServiceId(
                        GUID serviceId,
                        HSTRING* selector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IServiceDeviceStatics = __uuidof(IServiceDeviceStatics);
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Portable.IStorageDeviceStatics
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Portable.StorageDevice
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Portable_IStorageDeviceStatics[] = L"Windows.Devices.Portable.IStorageDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Portable {
                MIDL_INTERFACE("5ece44ee-1b23-4dd2-8652-bc164f003128")
                IStorageDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING deviceId,
                        ABI::Windows::Storage::IStorageFolder** deviceRoot
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* selector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStorageDeviceStatics = __uuidof(IStorageDeviceStatics);
            } /* Portable */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Portable.ServiceDevice
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Portable.IServiceDeviceStatics interface starting with version 1.0 of the Windows.Devices.Portable.PortableDeviceContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Portable_ServiceDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Portable_ServiceDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Portable_ServiceDevice[] = L"Windows.Devices.Portable.ServiceDevice";
#endif
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Portable.StorageDevice
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Portable.IStorageDeviceStatics interface starting with version 1.0 of the Windows.Devices.Portable.PortableDeviceContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Portable_StorageDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Portable_StorageDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Portable_StorageDevice[] = L"Windows.Devices.Portable.StorageDevice";
#endif
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CPortable_CServiceDeviceType __x_ABI_CWindows_CDevices_CPortable_CServiceDeviceType;

/*
 *
 * Struct Windows.Devices.Portable.ServiceDeviceType
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CPortable_CServiceDeviceType
{
    ServiceDeviceType_CalendarService = 0,
    ServiceDeviceType_ContactsService = 1,
    ServiceDeviceType_DeviceStatusService = 2,
    ServiceDeviceType_NotesService = 3,
    ServiceDeviceType_RingtonesService = 4,
    ServiceDeviceType_SmsService = 5,
    ServiceDeviceType_TasksService = 6,
};
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Portable.IServiceDeviceStatics
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Portable.ServiceDevice
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Portable_IServiceDeviceStatics[] = L"Windows.Devices.Portable.IServiceDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        enum __x_ABI_CWindows_CDevices_CPortable_CServiceDeviceType serviceType,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromServiceId)(__x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics* This,
        GUID serviceId,
        HSTRING* selector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_GetDeviceSelector(This, serviceType, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, serviceType, selector))

#define __x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_GetDeviceSelectorFromServiceId(This, serviceId, selector) \
    ((This)->lpVtbl->GetDeviceSelectorFromServiceId(This, serviceId, selector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPortable_CIServiceDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Portable.IStorageDeviceStatics
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Portable.StorageDevice
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Portable_IStorageDeviceStatics[] = L"Windows.Devices.Portable.IStorageDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CStorage_CIStorageFolder** deviceRoot);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics* This,
        HSTRING* selector);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_FromId(This, deviceId, deviceRoot) \
    ((This)->lpVtbl->FromId(This, deviceId, deviceRoot))

#define __x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_GetDeviceSelector(This, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, selector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPortable_CIStorageDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Portable.ServiceDevice
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Portable.IServiceDeviceStatics interface starting with version 1.0 of the Windows.Devices.Portable.PortableDeviceContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Portable_ServiceDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Portable_ServiceDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Portable_ServiceDevice[] = L"Windows.Devices.Portable.ServiceDevice";
#endif
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Portable.StorageDevice
 *
 * Introduced to Windows.Devices.Portable.PortableDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Portable.IStorageDeviceStatics interface starting with version 1.0 of the Windows.Devices.Portable.PortableDeviceContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Portable_StorageDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Portable_StorageDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Portable_StorageDevice[] = L"Windows.Devices.Portable.StorageDevice";
#endif
#endif // WINDOWS_DEVICES_PORTABLE_PORTABLEDEVICECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Eportable_p_h__

#endif // __windows2Edevices2Eportable_h__
