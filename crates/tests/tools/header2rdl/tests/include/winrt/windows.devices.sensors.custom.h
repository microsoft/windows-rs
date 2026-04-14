
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
#ifndef __windows2Edevices2Esensors2Ecustom_h__
#define __windows2Edevices2Esensors2Ecustom_h__
#ifndef __windows2Edevices2Esensors2Ecustom_p_h__
#define __windows2Edevices2Esensors2Ecustom_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensor;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor ABI::Windows::Devices::Sensors::Custom::ICustomSensor

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensor2;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2 ABI::Windows::Devices::Sensors::Custom::ICustomSensor2

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensorReading;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading ABI::Windows::Devices::Sensors::Custom::ICustomSensorReading

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensorReading2;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2 ABI::Windows::Devices::Sensors::Custom::ICustomSensorReading2

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensorReadingChangedEventArgs;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs ABI::Windows::Devices::Sensors::Custom::ICustomSensorReadingChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    interface ICustomSensorStatics;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics ABI::Windows::Devices::Sensors::Custom::ICustomSensorStatics

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    class CustomSensor;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7fbfbe55-9674-54e3-a269-9caa820ed23c"))
IAsyncOperation<ABI::Windows::Devices::Sensors::Custom::CustomSensor*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sensors::Custom::CustomSensor*, ABI::Windows::Devices::Sensors::Custom::ICustomSensor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sensors.Custom.CustomSensor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Sensors::Custom::CustomSensor*> __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("808b62d7-6e02-5680-a59e-118a98a4e70f"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sensors::Custom::CustomSensor*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sensors::Custom::CustomSensor*, ABI::Windows::Devices::Sensors::Custom::ICustomSensor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Sensors.Custom.CustomSensor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sensors::Custom::CustomSensor*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("604d0c4c-91de-5c2a-935f-362f13eaf800"))
IReference<struct ABI::Windows::Foundation::TimeSpan> : IReference_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::TimeSpan> __FIReference_1_Windows__CFoundation__CTimeSpan_t;
#define __FIReference_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CTimeSpan_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    class CustomSensorReadingChangedEventArgs;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aa9460cb-f08c-5963-b232-cc4075e984e7"))
ITypedEventHandler<ABI::Windows::Devices::Sensors::Custom::CustomSensor*, ABI::Windows::Devices::Sensors::Custom::CustomSensorReadingChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sensors::Custom::CustomSensor*, ABI::Windows::Devices::Sensors::Custom::ICustomSensor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sensors::Custom::CustomSensorReadingChangedEventArgs*, ABI::Windows::Devices::Sensors::Custom::ICustomSensorReadingChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Sensors.Custom.CustomSensor, Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Sensors::Custom::CustomSensor*, ABI::Windows::Devices::Sensors::Custom::CustomSensorReadingChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    class CustomSensorReading;
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensor[] = L"Windows.Devices.Sensors.Custom.ICustomSensor";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("a136f9ad-4034-4b4d-99dd-531aac649c09")
                    ICustomSensor : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentReading(
                            ABI::Windows::Devices::Sensors::Custom::ICustomSensorReading** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MinimumReportInterval(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ReportInterval(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ReportInterval(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ReadingChanged(
                            __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ReadingChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensor = __uuidof(ICustomSensor);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensor2[] = L"Windows.Devices.Sensors.Custom.ICustomSensor2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("20db3111-ec58-4d9f-bfbd-e77825088510")
                    ICustomSensor2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_ReportLatency(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ReportLatency(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxBatchSize(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensor2 = __uuidof(ICustomSensor2);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReading[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReading";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("64004f4d-446a-4366-a87a-5f963268ec53")
                    ICustomSensorReading : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_HSTRING_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensorReading = __uuidof(ICustomSensorReading);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReading2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReading2[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReading2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("223c98ea-bf73-4992-9a48-d3c897594ccb")
                    ICustomSensorReading2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PerformanceCount(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensorReading2 = __uuidof(ICustomSensorReading2);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReadingChangedEventArgs[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("6b202023-cffd-4cc1-8ff0-e21823d76fcc")
                    ICustomSensorReadingChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Reading(
                            ABI::Windows::Devices::Sensors::Custom::ICustomSensorReading** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensorReadingChangedEventArgs = __uuidof(ICustomSensorReadingChangedEventArgs);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorStatics[] = L"Windows.Devices.Sensors.Custom.ICustomSensorStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                namespace Custom {
                    MIDL_INTERFACE("992052cf-f422-4c7d-836b-e7dc74a7124b")
                    ICustomSensorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                            GUID interfaceId,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING sensorId,
                            __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomSensorStatics = __uuidof(ICustomSensorStatics);
                } /* Custom */
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sensors.Custom.ICustomSensorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensor ** Default Interface **
 *    Windows.Devices.Sensors.Custom.ICustomSensor2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensor[] = L"Windows.Devices.Sensors.Custom.CustomSensor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensorReading ** Default Interface **
 *    Windows.Devices.Sensors.Custom.ICustomSensorReading2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReading_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReading_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensorReading[] = L"Windows.Devices.Sensors.Custom.CustomSensorReading";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs[] = L"Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2 __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2 __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics;

#endif // ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* This,
        __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSensors__CCustom__CCustomSensor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CTimeSpan __FIReference_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIReference_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* sender,
        __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensor[] = L"Windows.Devices.Sensors.Custom.ICustomSensor";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentReading)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading** value);
    HRESULT (STDMETHODCALLTYPE* get_MinimumReportInterval)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ReportInterval)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ReportInterval)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* add_ReadingChanged)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        __FITypedEventHandler_2_Windows__CDevices__CSensors__CCustom__CCustomSensor_Windows__CDevices__CSensors__CCustom__CCustomSensorReadingChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ReadingChanged)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorVtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_GetCurrentReading(This, value) \
    ((This)->lpVtbl->GetCurrentReading(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_get_MinimumReportInterval(This, value) \
    ((This)->lpVtbl->get_MinimumReportInterval(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_put_ReportInterval(This, value) \
    ((This)->lpVtbl->put_ReportInterval(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_get_ReportInterval(This, value) \
    ((This)->lpVtbl->get_ReportInterval(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_add_ReadingChanged(This, handler, token) \
    ((This)->lpVtbl->add_ReadingChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_remove_ReadingChanged(This, token) \
    ((This)->lpVtbl->remove_ReadingChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensor2[] = L"Windows.Devices.Sensors.Custom.ICustomSensor2";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ReportLatency)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ReportLatency)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxBatchSize)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2Vtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_put_ReportLatency(This, value) \
    ((This)->lpVtbl->put_ReportLatency(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_get_ReportLatency(This, value) \
    ((This)->lpVtbl->get_ReportLatency(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_get_MaxBatchSize(This, value) \
    ((This)->lpVtbl->get_MaxBatchSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReading[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReading";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingVtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReading2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReading2[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReading2";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PerformanceCount)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2Vtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_get_PerformanceCount(This, value) \
    ((This)->lpVtbl->get_PerformanceCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorReadingChangedEventArgs[] = L"Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reading)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReading** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_get_Reading(This, value) \
    ((This)->lpVtbl->get_Reading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorReadingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sensors.Custom.ICustomSensorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sensors.Custom.CustomSensor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sensors_Custom_ICustomSensorStatics[] = L"Windows.Devices.Sensors.Custom.ICustomSensorStatics";
typedef struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        GUID interfaceId,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics* This,
        HSTRING sensorId,
        __FIAsyncOperation_1_Windows__CDevices__CSensors__CCustom__CCustomSensor** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_GetDeviceSelector(This, interfaceId, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, interfaceId, result))

#define __x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_FromIdAsync(This, sensorId, result) \
    ((This)->lpVtbl->FromIdAsync(This, sensorId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSensors_CCustom_CICustomSensorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sensors.Custom.ICustomSensorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensor ** Default Interface **
 *    Windows.Devices.Sensors.Custom.ICustomSensor2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensor_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensor[] = L"Windows.Devices.Sensors.Custom.CustomSensor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensorReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensorReading ** Default Interface **
 *    Windows.Devices.Sensors.Custom.ICustomSensorReading2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReading_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReading_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensorReading[] = L"Windows.Devices.Sensors.Custom.CustomSensorReading";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sensors.Custom.ICustomSensorReadingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sensors_Custom_CustomSensorReadingChangedEventArgs[] = L"Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
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
#endif // __windows2Edevices2Esensors2Ecustom_p_h__

#endif // __windows2Edevices2Esensors2Ecustom_h__
