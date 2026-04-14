
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
#ifndef __windows2Edevices2Egpio_h__
#define __windows2Edevices2Egpio_h__
#ifndef __windows2Edevices2Egpio_p_h__
#define __windows2Edevices2Egpio_p_h__


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
#if !defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)
#define WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION)

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
#include "Windows.Devices.h"
#include "Windows.Devices.Gpio.Provider.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioChangeCounter;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter ABI::Windows::Devices::Gpio::IGpioChangeCounter

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioChangeCounterFactory;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory ABI::Windows::Devices::Gpio::IGpioChangeCounterFactory

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioChangeReader;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader ABI::Windows::Devices::Gpio::IGpioChangeReader

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioChangeReaderFactory;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory ABI::Windows::Devices::Gpio::IGpioChangeReaderFactory

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioController;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController ABI::Windows::Devices::Gpio::IGpioController

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioControllerStatics;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics ABI::Windows::Devices::Gpio::IGpioControllerStatics

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioControllerStatics2;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2 ABI::Windows::Devices::Gpio::IGpioControllerStatics2

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioPin;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin ABI::Windows::Devices::Gpio::IGpioPin

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                interface IGpioPinValueChangedEventArgs;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs ABI::Windows::Devices::Gpio::IGpioPinValueChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                class GpioController;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ed045917-96c7-5735-b4be-d79619d4835e"))
IAsyncOperation<ABI::Windows::Devices::Gpio::GpioController*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioController*, ABI::Windows::Devices::Gpio::IGpioController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Gpio.GpioController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Gpio::GpioController*> __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("370167c0-0f7b-5e77-9bae-d35089a3db75"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Gpio::GpioController*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioController*, ABI::Windows::Devices::Gpio::IGpioController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Gpio.GpioController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Gpio::GpioController*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("67944db0-6c56-5a2f-9e7b-63ca1aa8c411"))
IIterator<ABI::Windows::Devices::Gpio::GpioController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioController*, ABI::Windows::Devices::Gpio::IGpioController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Gpio.GpioController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Gpio::GpioController*> __FIIterator_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGpio__CGpioController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("415c3794-b2b6-5f5c-9a05-ae9268514726"))
IIterable<ABI::Windows::Devices::Gpio::GpioController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioController*, ABI::Windows::Devices::Gpio::IGpioController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Gpio.GpioController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Gpio::GpioController*> __FIIterable_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGpio__CGpioController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fc72a82-2c57-5c01-a652-a8bdac685d30"))
IVectorView<ABI::Windows::Devices::Gpio::GpioController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioController*, ABI::Windows::Devices::Gpio::IGpioController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Gpio.GpioController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Gpio::GpioController*> __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5da3faf4-60a7-5a14-9319-3941dfb13fed"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Gpio.GpioController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee427f2e-7d37-558f-9718-9cbcbff40c94"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Gpio.GpioController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CGpio__CGpioController*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef struct GpioChangeRecord GpioChangeRecord;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#define DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a4c620b9-cb89-5a25-bf16-5f412c1a3388"))
IIterator<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> : IIterator_impl<struct ABI::Windows::Devices::Gpio::GpioChangeRecord>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Gpio.GpioChangeRecord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_t;
#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#define DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b4afbf4f-620e-5725-878a-78c6ed10374e"))
IIterable<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> : IIterable_impl<struct ABI::Windows::Devices::Gpio::GpioChangeRecord>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Gpio.GpioChangeRecord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_t;
#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#define DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d30ab625-1264-539e-acef-306dd214dc3b"))
IVectorView<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> : IVectorView_impl<struct ABI::Windows::Devices::Gpio::GpioChangeRecord>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Gpio.GpioChangeRecord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_t;
#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#define DEF___FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c8c443c2-f7d4-5386-ad15-31838882bd9e"))
IVector<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> : IVector_impl<struct ABI::Windows::Devices::Gpio::GpioChangeRecord>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Gpio.GpioChangeRecord>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<struct ABI::Windows::Devices::Gpio::GpioChangeRecord> __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_t;
#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                class GpioPin;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                class GpioPinValueChangedEventArgs;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44ba689b-7d42-5374-add9-ab41e877a34b"))
ITypedEventHandler<ABI::Windows::Devices::Gpio::GpioPin*, ABI::Windows::Devices::Gpio::GpioPinValueChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioPin*, ABI::Windows::Devices::Gpio::IGpioPin*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Gpio::GpioPinValueChangedEventArgs*, ABI::Windows::Devices::Gpio::IGpioPinValueChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Gpio.GpioPin, Windows.Devices.Gpio.GpioPinValueChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Gpio::GpioPin*, ABI::Windows::Devices::Gpio::GpioPinValueChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                namespace Provider {
                    interface IGpioProvider;
                } /* Provider */
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider ABI::Windows::Devices::Gpio::Provider::IGpioProvider

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioChangePolarity : int GpioChangePolarity;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioOpenStatus : int GpioOpenStatus;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioPinDriveMode : int GpioPinDriveMode;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioPinEdge : int GpioPinEdge;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioPinValue : int GpioPinValue;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef enum GpioSharingMode : int GpioSharingMode;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                typedef struct GpioChangeCount GpioChangeCount;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                class GpioChangeCounter;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                class GpioChangeReader;
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangePolarity
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioChangePolarity : int
                {
                    GpioChangePolarity_Falling = 0,
                    GpioChangePolarity_Rising = 1,
                    GpioChangePolarity_Both = 2,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Devices.Gpio.GpioOpenStatus
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioOpenStatus : int
                {
                    GpioOpenStatus_PinOpened = 0,
                    GpioOpenStatus_PinUnavailable = 1,
                    GpioOpenStatus_SharingViolation = 2,
                    GpioOpenStatus_MuxingConflict = 3,
                    GpioOpenStatus_UnknownError = 4,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinDriveMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioPinDriveMode : int
                {
                    GpioPinDriveMode_Input = 0,
                    GpioPinDriveMode_Output = 1,
                    GpioPinDriveMode_InputPullUp = 2,
                    GpioPinDriveMode_InputPullDown = 3,
                    GpioPinDriveMode_OutputOpenDrain = 4,
                    GpioPinDriveMode_OutputOpenDrainPullUp = 5,
                    GpioPinDriveMode_OutputOpenSource = 6,
                    GpioPinDriveMode_OutputOpenSourcePullDown = 7,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinEdge
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioPinEdge : int
                {
                    GpioPinEdge_FallingEdge = 0,
                    GpioPinEdge_RisingEdge = 1,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinValue
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioPinValue : int
                {
                    GpioPinValue_Low = 0,
                    GpioPinValue_High = 1,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                enum GpioSharingMode : int
                {
                    GpioSharingMode_Exclusive = 0,
                    GpioSharingMode_SharedReadOnly = 1,
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangeCount
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                struct GpioChangeCount
                {
                    UINT64 Count;
                    ABI::Windows::Foundation::TimeSpan RelativeTime;
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangeRecord
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                struct GpioChangeRecord
                {
                    ABI::Windows::Foundation::TimeSpan RelativeTime;
                    ABI::Windows::Devices::Gpio::GpioPinEdge Edge;
                };
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeCounter
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeCounter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeCounter[] = L"Windows.Devices.Gpio.IGpioChangeCounter";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("cb5ec0de-6801-43ff-803d-4576628a8b26")
                IGpioChangeCounter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Polarity(
                        ABI::Windows::Devices::Gpio::GpioChangePolarity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Polarity(
                        ABI::Windows::Devices::Gpio::GpioChangePolarity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStarted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Read(
                        ABI::Windows::Devices::Gpio::GpioChangeCount* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Reset(
                        ABI::Windows::Devices::Gpio::GpioChangeCount* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioChangeCounter = __uuidof(IGpioChangeCounter);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeCounterFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeCounter
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeCounterFactory[] = L"Windows.Devices.Gpio.IGpioChangeCounterFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("147d94b6-0a9e-410c-b4fa-f89f4052084d")
                IGpioChangeCounterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Gpio::IGpioPin* pin,
                        ABI::Windows::Devices::Gpio::IGpioChangeCounter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioChangeCounterFactory = __uuidof(IGpioChangeCounterFactory);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeReader
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeReader[] = L"Windows.Devices.Gpio.IGpioChangeReader";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("0abc885f-e031-48e8-8590-70de78363c6d")
                IGpioChangeReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Capacity(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Length(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEmpty(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOverflowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Polarity(
                        ABI::Windows::Devices::Gpio::GpioChangePolarity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Polarity(
                        ABI::Windows::Devices::Gpio::GpioChangePolarity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsStarted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNextItem(
                        ABI::Windows::Devices::Gpio::GpioChangeRecord* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PeekNextItem(
                        ABI::Windows::Devices::Gpio::GpioChangeRecord* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAllItems(
                        __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WaitForItemsAsync(
                        INT32 count,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioChangeReader = __uuidof(IGpioChangeReader);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeReaderFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeReader
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeReaderFactory[] = L"Windows.Devices.Gpio.IGpioChangeReaderFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("a9598ef3-390e-441a-9d1c-e8de0b2df0df")
                IGpioChangeReaderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Devices::Gpio::IGpioPin* pin,
                        ABI::Windows::Devices::Gpio::IGpioChangeReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithCapacity(
                        ABI::Windows::Devices::Gpio::IGpioPin* pin,
                        INT32 minCapacity,
                        ABI::Windows::Devices::Gpio::IGpioChangeReader** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioChangeReaderFactory = __uuidof(IGpioChangeReaderFactory);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioController[] = L"Windows.Devices.Gpio.IGpioController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("284012e3-7461-469c-a8bc-61d69d08a53c")
                IGpioController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PinCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenPin(
                        INT32 pinNumber,
                        ABI::Windows::Devices::Gpio::IGpioPin** pin
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenPinWithSharingMode(
                        INT32 pinNumber,
                        ABI::Windows::Devices::Gpio::GpioSharingMode sharingMode,
                        ABI::Windows::Devices::Gpio::IGpioPin** pin
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryOpenPin(
                        INT32 pinNumber,
                        ABI::Windows::Devices::Gpio::GpioSharingMode sharingMode,
                        ABI::Windows::Devices::Gpio::IGpioPin** pin,
                        ABI::Windows::Devices::Gpio::GpioOpenStatus* openStatus,
                        boolean* succeeded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioController = __uuidof(IGpioController);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioControllerStatics[] = L"Windows.Devices.Gpio.IGpioControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("2ed6f42e-7af7-4116-9533-c43d99a1fb64")
                IGpioControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Devices::Gpio::IGpioController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioControllerStatics = __uuidof(IGpioControllerStatics);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioControllerStatics2[] = L"Windows.Devices.Gpio.IGpioControllerStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("912b7d20-6ca4-4106-a373-fffd346b0e5b")
                IGpioControllerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                        ABI::Windows::Devices::Gpio::Provider::IGpioProvider* provider,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioControllerStatics2 = __uuidof(IGpioControllerStatics2);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioPin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioPin[] = L"Windows.Devices.Gpio.IGpioPin";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("11d9b087-afae-4790-9ee9-e0eac942d201")
                IGpioPin : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ValueChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ValueChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DebounceTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DebounceTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PinNumber(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SharingMode(
                        ABI::Windows::Devices::Gpio::GpioSharingMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsDriveModeSupported(
                        ABI::Windows::Devices::Gpio::GpioPinDriveMode driveMode,
                        boolean* supported
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDriveMode(
                        ABI::Windows::Devices::Gpio::GpioPinDriveMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDriveMode(
                        ABI::Windows::Devices::Gpio::GpioPinDriveMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Write(
                        ABI::Windows::Devices::Gpio::GpioPinValue value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Read(
                        ABI::Windows::Devices::Gpio::GpioPinValue* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioPin = __uuidof(IGpioPin);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioPin;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioPinValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioPinValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioPinValueChangedEventArgs[] = L"Windows.Devices.Gpio.IGpioPinValueChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Gpio {
                MIDL_INTERFACE("3137aae1-703d-4059-bd24-b5b25dffb84e")
                IGpioPinValueChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Edge(
                        ABI::Windows::Devices::Gpio::GpioPinEdge* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGpioPinValueChangedEventArgs = __uuidof(IGpioPinValueChangedEventArgs);
            } /* Gpio */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioChangeCounter
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.IGpioChangeCounterFactory interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioChangeCounter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeCounter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeCounter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioChangeCounter[] = L"Windows.Devices.Gpio.GpioChangeCounter";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Gpio.GpioChangeReader
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.IGpioChangeReaderFactory interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioChangeReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeReader_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioChangeReader[] = L"Windows.Devices.Gpio.GpioChangeReader";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Gpio.GpioController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Gpio.IGpioControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Gpio.IGpioControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioController[] = L"Windows.Devices.Gpio.GpioController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioPin ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioPin_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioPin_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioPin[] = L"Windows.Devices.Gpio.GpioPin";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioPinValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioPinValueChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioPinValueChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioPinValueChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioPinValueChangedEventArgs[] = L"Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioController __x_ABI_CWindows_CDevices_CGpio_CIGpioController;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2 __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioPin __x_ABI_CWindows_CDevices_CGpio_CIGpioPin;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGpio__CGpioController __FIIterator_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIIterator_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGpio__CGpioController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIIterator_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGpio__CGpioController __FIIterable_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIIterable_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIIterator_1_Windows__CDevices__CGpio__CGpioController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIIterable_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGpio__CGpioController __FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIVectorView_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIVectorView_1_Windows__CDevices__CGpio__CGpioController** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord;

typedef struct __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl;

interface __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord;

typedef struct __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        __FIIterator_1_Windows__CDevices__CGpio__CGpioChangeRecord** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl;

interface __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord;

typedef struct __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl;

interface __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord;

typedef struct __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        __FIVectorView_1_Windows__CDevices__CGpio__CGpioChangeRecord** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl;

interface __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecordVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin* sender,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider;

#endif // ____x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioOpenStatus __x_ABI_CWindows_CDevices_CGpio_CGpioOpenStatus;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinEdge __x_ABI_CWindows_CDevices_CGpio_CGpioPinEdge;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinValue __x_ABI_CWindows_CDevices_CGpio_CGpioPinValue;

typedef enum __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode;

typedef struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeCount __x_ABI_CWindows_CDevices_CGpio_CGpioChangeCount;

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangePolarity
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity
{
    GpioChangePolarity_Falling = 0,
    GpioChangePolarity_Rising = 1,
    GpioChangePolarity_Both = 2,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Devices.Gpio.GpioOpenStatus
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioOpenStatus
{
    GpioOpenStatus_PinOpened = 0,
    GpioOpenStatus_PinUnavailable = 1,
    GpioOpenStatus_SharingViolation = 2,
    GpioOpenStatus_MuxingConflict = 3,
    GpioOpenStatus_UnknownError = 4,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinDriveMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode
{
    GpioPinDriveMode_Input = 0,
    GpioPinDriveMode_Output = 1,
    GpioPinDriveMode_InputPullUp = 2,
    GpioPinDriveMode_InputPullDown = 3,
    GpioPinDriveMode_OutputOpenDrain = 4,
    GpioPinDriveMode_OutputOpenDrainPullUp = 5,
    GpioPinDriveMode_OutputOpenSource = 6,
    GpioPinDriveMode_OutputOpenSourcePullDown = 7,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinEdge
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinEdge
{
    GpioPinEdge_FallingEdge = 0,
    GpioPinEdge_RisingEdge = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioPinValue
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinValue
{
    GpioPinValue_Low = 0,
    GpioPinValue_High = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode
{
    GpioSharingMode_Exclusive = 0,
    GpioSharingMode_SharedReadOnly = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangeCount
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeCount
{
    UINT64 Count;
    struct __x_ABI_CWindows_CFoundation_CTimeSpan RelativeTime;
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Devices.Gpio.GpioChangeRecord
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord
{
    struct __x_ABI_CWindows_CFoundation_CTimeSpan RelativeTime;
    enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinEdge Edge;
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeCounter
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeCounter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeCounter[] = L"Windows.Devices.Gpio.IGpioChangeCounter";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Polarity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity value);
    HRESULT (STDMETHODCALLTYPE* get_Polarity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStarted)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeCount* value);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeCount* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_put_Polarity(This, value) \
    ((This)->lpVtbl->put_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_get_Polarity(This, value) \
    ((This)->lpVtbl->get_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_get_IsStarted(This, value) \
    ((This)->lpVtbl->get_IsStarted(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_Read(This, value) \
    ((This)->lpVtbl->Read(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_Reset(This, value) \
    ((This)->lpVtbl->Reset(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeCounterFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeCounter
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeCounterFactory[] = L"Windows.Devices.Gpio.IGpioChangeCounterFactory";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin* pin,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounter** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_Create(This, pin, value) \
    ((This)->lpVtbl->Create(This, pin, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeCounterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeReader
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeReader[] = L"Windows.Devices.Gpio.IGpioChangeReader";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capacity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEmpty)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOverflowed)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Polarity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity value);
    HRESULT (STDMETHODCALLTYPE* get_Polarity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioChangePolarity* value);
    HRESULT (STDMETHODCALLTYPE* get_IsStarted)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* GetNextItem)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* value);
    HRESULT (STDMETHODCALLTYPE* PeekNextItem)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        struct __x_ABI_CWindows_CDevices_CGpio_CGpioChangeRecord* value);
    HRESULT (STDMETHODCALLTYPE* GetAllItems)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        __FIVector_1_Windows__CDevices__CGpio__CGpioChangeRecord** value);
    HRESULT (STDMETHODCALLTYPE* WaitForItemsAsync)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader* This,
        INT32 count,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_Capacity(This, value) \
    ((This)->lpVtbl->get_Capacity(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_IsEmpty(This, value) \
    ((This)->lpVtbl->get_IsEmpty(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_IsOverflowed(This, value) \
    ((This)->lpVtbl->get_IsOverflowed(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_put_Polarity(This, value) \
    ((This)->lpVtbl->put_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_Polarity(This, value) \
    ((This)->lpVtbl->get_Polarity(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_get_IsStarted(This, value) \
    ((This)->lpVtbl->get_IsStarted(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_GetNextItem(This, value) \
    ((This)->lpVtbl->GetNextItem(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_PeekNextItem(This, value) \
    ((This)->lpVtbl->PeekNextItem(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_GetAllItems(This, value) \
    ((This)->lpVtbl->GetAllItems(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_WaitForItemsAsync(This, count, operation) \
    ((This)->lpVtbl->WaitForItemsAsync(This, count, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioChangeReaderFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioChangeReader
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioChangeReaderFactory[] = L"Windows.Devices.Gpio.IGpioChangeReaderFactory";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin* pin,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithCapacity)(__x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin* pin,
        INT32 minCapacity,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReader** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_Create(This, pin, value) \
    ((This)->lpVtbl->Create(This, pin, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_CreateWithCapacity(This, pin, minCapacity, value) \
    ((This)->lpVtbl->CreateWithCapacity(This, pin, minCapacity, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioChangeReaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioController[] = L"Windows.Devices.Gpio.IGpioController";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinCount)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* OpenPin)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        INT32 pinNumber,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin** pin);
    HRESULT (STDMETHODCALLTYPE* OpenPinWithSharingMode)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        INT32 pinNumber,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode sharingMode,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin** pin);
    HRESULT (STDMETHODCALLTYPE* TryOpenPin)(__x_ABI_CWindows_CDevices_CGpio_CIGpioController* This,
        INT32 pinNumber,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode sharingMode,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioPin** pin,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioOpenStatus* openStatus,
        boolean* succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_get_PinCount(This, value) \
    ((This)->lpVtbl->get_PinCount(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_OpenPin(This, pinNumber, pin) \
    ((This)->lpVtbl->OpenPin(This, pinNumber, pin))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_OpenPinWithSharingMode(This, pinNumber, sharingMode, pin) \
    ((This)->lpVtbl->OpenPinWithSharingMode(This, pinNumber, sharingMode, pin))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioController_TryOpenPin(This, pinNumber, sharingMode, pin, openStatus, succeeded) \
    ((This)->lpVtbl->TryOpenPin(This, pinNumber, sharingMode, pin, openStatus, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioControllerStatics[] = L"Windows.Devices.Gpio.IGpioControllerStatics";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics* This,
        __x_ABI_CWindows_CDevices_CGpio_CIGpioController** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioControllerStatics2
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioControllerStatics2[] = L"Windows.Devices.Gpio.IGpioControllerStatics2";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        __x_ABI_CWindows_CDevices_CGpio_CProvider_CIGpioProvider* provider,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CGpio__CGpioController** operation);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2* This,
        __FIAsyncOperation_1_Windows__CDevices__CGpio__CGpioController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_GetControllersAsync(This, provider, operation) \
    ((This)->lpVtbl->GetControllersAsync(This, provider, operation))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioControllerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioPin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioPin[] = L"Windows.Devices.Gpio.IGpioPin";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioPinVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ValueChanged)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        __FITypedEventHandler_2_Windows__CDevices__CGpio__CGpioPin_Windows__CDevices__CGpio__CGpioPinValueChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ValueChanged)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_DebounceTimeout)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DebounceTimeout)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_PinNumber)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SharingMode)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioSharingMode* value);
    HRESULT (STDMETHODCALLTYPE* IsDriveModeSupported)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode driveMode,
        boolean* supported);
    HRESULT (STDMETHODCALLTYPE* GetDriveMode)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode* value);
    HRESULT (STDMETHODCALLTYPE* SetDriveMode)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinDriveMode value);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinValue value);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPin* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinValue* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioPinVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioPin
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioPinVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_add_ValueChanged(This, handler, token) \
    ((This)->lpVtbl->add_ValueChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_remove_ValueChanged(This, token) \
    ((This)->lpVtbl->remove_ValueChanged(This, token))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_get_DebounceTimeout(This, value) \
    ((This)->lpVtbl->get_DebounceTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_put_DebounceTimeout(This, value) \
    ((This)->lpVtbl->put_DebounceTimeout(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_get_PinNumber(This, value) \
    ((This)->lpVtbl->get_PinNumber(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_get_SharingMode(This, value) \
    ((This)->lpVtbl->get_SharingMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_IsDriveModeSupported(This, driveMode, supported) \
    ((This)->lpVtbl->IsDriveModeSupported(This, driveMode, supported))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_GetDriveMode(This, value) \
    ((This)->lpVtbl->GetDriveMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_SetDriveMode(This, value) \
    ((This)->lpVtbl->SetDriveMode(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_Write(This, value) \
    ((This)->lpVtbl->Write(This, value))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPin_Read(This, value) \
    ((This)->lpVtbl->Read(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioPin;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPin_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Gpio.IGpioPinValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Gpio.GpioPinValueChangedEventArgs
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Gpio_IGpioPinValueChangedEventArgs[] = L"Windows.Devices.Gpio.IGpioPinValueChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Edge)(__x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs* This,
        enum __x_ABI_CWindows_CDevices_CGpio_CGpioPinEdge* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_get_Edge(This, value) \
    ((This)->lpVtbl->get_Edge(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CGpio_CIGpioPinValueChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioChangeCounter
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.IGpioChangeCounterFactory interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioChangeCounter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeCounter_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeCounter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioChangeCounter[] = L"Windows.Devices.Gpio.GpioChangeCounter";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Gpio.GpioChangeReader
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Gpio.IGpioChangeReaderFactory interface starting with version 3.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioChangeReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeReader_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioChangeReader[] = L"Windows.Devices.Gpio.GpioChangeReader";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Devices.Gpio.GpioController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Gpio.IGpioControllerStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *   Static Methods exist on the Windows.Devices.Gpio.IGpioControllerStatics2 interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioController[] = L"Windows.Devices.Gpio.GpioController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioPin
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioPin ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioPin_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioPin_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioPin[] = L"Windows.Devices.Gpio.GpioPin";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Gpio.GpioPinValueChangedEventArgs
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Gpio.IGpioPinValueChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Gpio_GpioPinValueChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Gpio_GpioPinValueChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Gpio_GpioPinValueChangedEventArgs[] = L"Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Egpio_p_h__

#endif // __windows2Edevices2Egpio_h__
