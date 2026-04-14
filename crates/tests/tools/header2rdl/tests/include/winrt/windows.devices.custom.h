
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
#ifndef __windows2Edevices2Ecustom_h__
#define __windows2Edevices2Ecustom_h__
#ifndef __windows2Edevices2Ecustom_p_h__
#define __windows2Edevices2Ecustom_p_h__


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
#if !defined(WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION)
#define WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION)

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
#include "Windows.Storage.Streams.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                interface ICustomDevice;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice ABI::Windows::Devices::Custom::ICustomDevice

#endif // ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                interface ICustomDeviceStatics;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics ABI::Windows::Devices::Custom::ICustomDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                interface IIOControlCode;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode ABI::Windows::Devices::Custom::IIOControlCode

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                interface IIOControlCodeFactory;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory ABI::Windows::Devices::Custom::IIOControlCodeFactory

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                interface IKnownDeviceTypesStatics;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics ABI::Windows::Devices::Custom::IKnownDeviceTypesStatics

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */



#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                class CustomDevice;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2a6344aa-0568-548e-a1a2-b6bb451d228c"))
IAsyncOperation<ABI::Windows::Devices::Custom::CustomDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Custom::CustomDevice*, ABI::Windows::Devices::Custom::ICustomDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Custom.CustomDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Custom::CustomDevice*> __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_USE */

#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1fdd39b0-e0e5-5c59-b27d-a549b1075ce9"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Custom::CustomDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Custom::CustomDevice*, ABI::Windows::Devices::Custom::ICustomDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Custom.CustomDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Custom::CustomDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_USE */

#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                typedef enum DeviceAccessMode : int DeviceAccessMode;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                typedef enum DeviceSharingMode : int DeviceSharingMode;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                typedef enum IOControlAccessMode : int IOControlAccessMode;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                typedef enum IOControlBufferingMethod : int IOControlBufferingMethod;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                class IOControlCode;
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Custom.DeviceAccessMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                enum DeviceAccessMode : int
                {
                    DeviceAccessMode_Read = 0,
                    DeviceAccessMode_Write = 1,
                    DeviceAccessMode_ReadWrite = 2,
                };
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.DeviceSharingMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                enum DeviceSharingMode : int
                {
                    DeviceSharingMode_Shared = 0,
                    DeviceSharingMode_Exclusive = 1,
                };
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.IOControlAccessMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                enum IOControlAccessMode : int
                {
                    IOControlAccessMode_Any = 0,
                    IOControlAccessMode_Read = 1,
                    IOControlAccessMode_Write = 2,
                    IOControlAccessMode_ReadWrite = 3,
                };
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.IOControlBufferingMethod
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                enum IOControlBufferingMethod : int
                {
                    IOControlBufferingMethod_Buffered = 0,
                    IOControlBufferingMethod_DirectInput = 1,
                    IOControlBufferingMethod_DirectOutput = 2,
                    IOControlBufferingMethod_Neither = 3,
                };
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.ICustomDevice
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.CustomDevice
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_ICustomDevice[] = L"Windows.Devices.Custom.ICustomDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                MIDL_INTERFACE("dd30251f-c48b-43bd-bcb1-dec88f15143e")
                ICustomDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputStream(
                        ABI::Windows::Storage::Streams::IOutputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendIOControlAsync(
                        ABI::Windows::Devices::Custom::IIOControlCode* ioControlCode,
                        ABI::Windows::Storage::Streams::IBuffer* inputBuffer,
                        ABI::Windows::Storage::Streams::IBuffer* outputBuffer,
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendIOControlAsync(
                        ABI::Windows::Devices::Custom::IIOControlCode* ioControlCode,
                        ABI::Windows::Storage::Streams::IBuffer* inputBuffer,
                        ABI::Windows::Storage::Streams::IBuffer* outputBuffer,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICustomDevice = __uuidof(ICustomDevice);
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CICustomDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.ICustomDeviceStatics
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.CustomDevice
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_ICustomDeviceStatics[] = L"Windows.Devices.Custom.ICustomDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                MIDL_INTERFACE("c8220312-ef4c-46b1-a58e-eeb308dc8917")
                ICustomDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        GUID classGuid,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        ABI::Windows::Devices::Custom::DeviceAccessMode desiredAccess,
                        ABI::Windows::Devices::Custom::DeviceSharingMode sharingMode,
                        __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICustomDeviceStatics = __uuidof(ICustomDeviceStatics);
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IIOControlCode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IIOControlCode[] = L"Windows.Devices.Custom.IIOControlCode";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                MIDL_INTERFACE("0e9559e7-60c8-4375-a761-7f8808066c60")
                IIOControlCode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AccessMode(
                        ABI::Windows::Devices::Custom::IOControlAccessMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BufferingMethod(
                        ABI::Windows::Devices::Custom::IOControlBufferingMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Function(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceType(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ControlCode(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIOControlCode = __uuidof(IIOControlCode);
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIIOControlCode;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IIOControlCodeFactory
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.IOControlCode
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IIOControlCodeFactory[] = L"Windows.Devices.Custom.IIOControlCodeFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                MIDL_INTERFACE("856a7cf0-4c11-44ae-afc6-b8d4a212788f")
                IIOControlCodeFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateIOControlCode(
                        UINT16 deviceType,
                        UINT16 function,
                        ABI::Windows::Devices::Custom::IOControlAccessMode accessMode,
                        ABI::Windows::Devices::Custom::IOControlBufferingMethod bufferingMethod,
                        ABI::Windows::Devices::Custom::IIOControlCode** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIOControlCodeFactory = __uuidof(IIOControlCodeFactory);
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IKnownDeviceTypesStatics
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.KnownDeviceTypes
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IKnownDeviceTypesStatics[] = L"Windows.Devices.Custom.IKnownDeviceTypesStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Custom {
                MIDL_INTERFACE("ee5479c2-5448-45da-ad1b-24948c239094")
                IKnownDeviceTypesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Unknown(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownDeviceTypesStatics = __uuidof(IKnownDeviceTypesStatics);
            } /* Custom */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.CustomDevice
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Custom.ICustomDeviceStatics interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Custom.ICustomDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_CustomDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_CustomDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_CustomDevice[] = L"Windows.Devices.Custom.CustomDevice";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.IOControlCode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Custom.IIOControlCodeFactory interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Custom.IIOControlCode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_IOControlCode_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_IOControlCode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_IOControlCode[] = L"Windows.Devices.Custom.IOControlCode";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.KnownDeviceTypes
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Custom.IKnownDeviceTypesStatics interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_KnownDeviceTypes_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_KnownDeviceTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_KnownDeviceTypes[] = L"Windows.Devices.Custom.KnownDeviceTypes";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CCustom_CICustomDevice __x_ABI_CWindows_CDevices_CCustom_CICustomDevice;

#endif // ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode;

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory;

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics;

#endif // ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice;

#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* This,
        __x_ABI_CWindows_CDevices_CCustom_CICustomDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CCustom__CCustomDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CCustom_CDeviceAccessMode __x_ABI_CWindows_CDevices_CCustom_CDeviceAccessMode;

typedef enum __x_ABI_CWindows_CDevices_CCustom_CDeviceSharingMode __x_ABI_CWindows_CDevices_CCustom_CDeviceSharingMode;

typedef enum __x_ABI_CWindows_CDevices_CCustom_CIOControlAccessMode __x_ABI_CWindows_CDevices_CCustom_CIOControlAccessMode;

typedef enum __x_ABI_CWindows_CDevices_CCustom_CIOControlBufferingMethod __x_ABI_CWindows_CDevices_CCustom_CIOControlBufferingMethod;

/*
 *
 * Struct Windows.Devices.Custom.DeviceAccessMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CCustom_CDeviceAccessMode
{
    DeviceAccessMode_Read = 0,
    DeviceAccessMode_Write = 1,
    DeviceAccessMode_ReadWrite = 2,
};
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.DeviceSharingMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CCustom_CDeviceSharingMode
{
    DeviceSharingMode_Shared = 0,
    DeviceSharingMode_Exclusive = 1,
};
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.IOControlAccessMode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CCustom_CIOControlAccessMode
{
    IOControlAccessMode_Any = 0,
    IOControlAccessMode_Read = 1,
    IOControlAccessMode_Write = 2,
    IOControlAccessMode_ReadWrite = 3,
};
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Custom.IOControlBufferingMethod
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CCustom_CIOControlBufferingMethod
{
    IOControlBufferingMethod_Buffered = 0,
    IOControlBufferingMethod_DirectInput = 1,
    IOControlBufferingMethod_DirectOutput = 2,
    IOControlBufferingMethod_Neither = 3,
};
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.ICustomDevice
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.CustomDevice
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_ICustomDevice[] = L"Windows.Devices.Custom.ICustomDevice";
typedef struct __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputStream)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream** value);
    HRESULT (STDMETHODCALLTYPE* SendIOControlAsync)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* ioControlCode,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* inputBuffer,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* outputBuffer,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* TrySendIOControlAsync)(__x_ABI_CWindows_CDevices_CCustom_CICustomDevice* This,
        __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* ioControlCode,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* inputBuffer,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* outputBuffer,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CCustom_CICustomDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_get_OutputStream(This, value) \
    ((This)->lpVtbl->get_OutputStream(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_SendIOControlAsync(This, ioControlCode, inputBuffer, outputBuffer, operation) \
    ((This)->lpVtbl->SendIOControlAsync(This, ioControlCode, inputBuffer, outputBuffer, operation))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDevice_TrySendIOControlAsync(This, ioControlCode, inputBuffer, outputBuffer, operation) \
    ((This)->lpVtbl->TrySendIOControlAsync(This, ioControlCode, inputBuffer, outputBuffer, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CICustomDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.ICustomDeviceStatics
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.CustomDevice
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_ICustomDeviceStatics[] = L"Windows.Devices.Custom.ICustomDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        GUID classGuid,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CDevices_CCustom_CDeviceAccessMode desiredAccess,
        enum __x_ABI_CWindows_CDevices_CCustom_CDeviceSharingMode sharingMode,
        __FIAsyncOperation_1_Windows__CDevices__CCustom__CCustomDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_GetDeviceSelector(This, classGuid, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, classGuid, value))

#define __x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_FromIdAsync(This, deviceId, desiredAccess, sharingMode, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, desiredAccess, sharingMode, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CICustomDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IIOControlCode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IIOControlCode[] = L"Windows.Devices.Custom.IIOControlCode";
typedef struct __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AccessMode)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        enum __x_ABI_CWindows_CDevices_CCustom_CIOControlAccessMode* value);
    HRESULT (STDMETHODCALLTYPE* get_BufferingMethod)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        enum __x_ABI_CWindows_CDevices_CCustom_CIOControlBufferingMethod* value);
    HRESULT (STDMETHODCALLTYPE* get_Function)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceType)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ControlCode)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCode* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeVtbl;

interface __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_get_AccessMode(This, value) \
    ((This)->lpVtbl->get_AccessMode(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_get_BufferingMethod(This, value) \
    ((This)->lpVtbl->get_BufferingMethod(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_get_Function(This, value) \
    ((This)->lpVtbl->get_Function(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_get_DeviceType(This, value) \
    ((This)->lpVtbl->get_DeviceType(This, value))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_get_ControlCode(This, value) \
    ((This)->lpVtbl->get_ControlCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIIOControlCode;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCode_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IIOControlCodeFactory
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.IOControlCode
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IIOControlCodeFactory[] = L"Windows.Devices.Custom.IIOControlCodeFactory";
typedef struct __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateIOControlCode)(__x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory* This,
        UINT16 deviceType,
        UINT16 function,
        enum __x_ABI_CWindows_CDevices_CCustom_CIOControlAccessMode accessMode,
        enum __x_ABI_CWindows_CDevices_CCustom_CIOControlBufferingMethod bufferingMethod,
        __x_ABI_CWindows_CDevices_CCustom_CIIOControlCode** instance);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_CreateIOControlCode(This, deviceType, function, accessMode, bufferingMethod, instance) \
    ((This)->lpVtbl->CreateIOControlCode(This, deviceType, function, accessMode, bufferingMethod, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIIOControlCodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Custom.IKnownDeviceTypesStatics
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Custom.KnownDeviceTypes
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Custom_IKnownDeviceTypesStatics[] = L"Windows.Devices.Custom.IKnownDeviceTypesStatics";
typedef struct __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unknown)(__x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_get_Unknown(This, value) \
    ((This)->lpVtbl->get_Unknown(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CCustom_CIKnownDeviceTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.CustomDevice
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Custom.ICustomDeviceStatics interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Custom.ICustomDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_CustomDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_CustomDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_CustomDevice[] = L"Windows.Devices.Custom.CustomDevice";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.IOControlCode
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Custom.IIOControlCodeFactory interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Custom.IIOControlCode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_IOControlCode_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_IOControlCode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_IOControlCode[] = L"Windows.Devices.Custom.IOControlCode";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Custom.KnownDeviceTypes
 *
 * Introduced to Windows.Devices.Custom.CustomDeviceContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Custom.IKnownDeviceTypesStatics interface starting with version 1.0 of the Windows.Devices.Custom.CustomDeviceContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Custom_KnownDeviceTypes_DEFINED
#define RUNTIMECLASS_Windows_Devices_Custom_KnownDeviceTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Custom_KnownDeviceTypes[] = L"Windows.Devices.Custom.KnownDeviceTypes";
#endif
#endif // WINDOWS_DEVICES_CUSTOM_CUSTOMDEVICECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ecustom_p_h__

#endif // __windows2Edevices2Ecustom_h__
