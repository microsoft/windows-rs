
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
#ifndef __windows2Edevices2Espi_h__
#define __windows2Edevices2Espi_h__
#ifndef __windows2Edevices2Espi_p_h__
#define __windows2Edevices2Espi_p_h__


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
#include "Windows.Devices.Spi.Provider.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiBusInfo;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo ABI::Windows::Devices::Spi::ISpiBusInfo

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiConnectionSettings;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings ABI::Windows::Devices::Spi::ISpiConnectionSettings

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiConnectionSettingsFactory;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory ABI::Windows::Devices::Spi::ISpiConnectionSettingsFactory

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiController;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiController ABI::Windows::Devices::Spi::ISpiController

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiControllerStatics;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics ABI::Windows::Devices::Spi::ISpiControllerStatics

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiDevice;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice ABI::Windows::Devices::Spi::ISpiDevice

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                interface ISpiDeviceStatics;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics ABI::Windows::Devices::Spi::ISpiDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                class SpiController;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b6b0df6f-c097-5844-93bd-7821998fdb8e"))
IAsyncOperation<ABI::Windows::Devices::Spi::SpiController*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiController*, ABI::Windows::Devices::Spi::ISpiController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Spi.SpiController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Spi::SpiController*> __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e94d949-a844-5b25-a3cc-afabeb18c1d2"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Spi::SpiController*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiController*, ABI::Windows::Devices::Spi::ISpiController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Spi.SpiController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Spi::SpiController*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                class SpiDevice;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("feb8466a-878f-577b-bbca-89575cfc56e4"))
IAsyncOperation<ABI::Windows::Devices::Spi::SpiDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiDevice*, ABI::Windows::Devices::Spi::ISpiDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Spi.SpiDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Spi::SpiDevice*> __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a88a28ba-6966-55e7-8c81-7c65f74e39c0"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Spi::SpiDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiDevice*, ABI::Windows::Devices::Spi::ISpiDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Spi.SpiDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Spi::SpiDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIIterator_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fd7d5997-544c-5be9-b0fa-1d0efbfc4a03"))
IIterator<ABI::Windows::Devices::Spi::SpiController*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiController*, ABI::Windows::Devices::Spi::ISpiController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Spi.SpiController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Spi::SpiController*> __FIIterator_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSpi__CSpiController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIIterable_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7b076938-dc1b-5368-9003-059291d37f35"))
IIterable<ABI::Windows::Devices::Spi::SpiController*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiController*, ABI::Windows::Devices::Spi::ISpiController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Spi.SpiController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Spi::SpiController*> __FIIterable_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSpi__CSpiController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("35fec489-44a2-5543-8a0c-b52e2f9cf87c"))
IVectorView<ABI::Windows::Devices::Spi::SpiController*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Spi::SpiController*, ABI::Windows::Devices::Spi::ISpiController*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.SpiController>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Spi::SpiController*> __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89624331-f802-56f7-9b33-17c616ecbcfa"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.SpiController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c8afc9cb-6807-57ec-84c9-9f3dbc003450"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Spi.SpiController>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CSpi__CSpiController*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_USE */

#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000


#ifndef DEF___FIIterator_1_int_USE
#define DEF___FIIterator_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfea7f78-50c2-5f1d-a6ea-9e978d2699ff"))
IIterator<int> : IIterator_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<int> __FIIterator_1_int_t;
#define __FIIterator_1_int ABI::Windows::Foundation::Collections::__FIIterator_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_int_USE */



#ifndef DEF___FIIterable_1_int_USE
#define DEF___FIIterable_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("81a643fb-f51c-5565-83c4-f96425777b66"))
IIterable<int> : IIterable_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<int> __FIIterable_1_int_t;
#define __FIIterable_1_int ABI::Windows::Foundation::Collections::__FIIterable_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_int_USE */



#ifndef DEF___FIVectorView_1_int_USE
#define DEF___FIVectorView_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d720cdf-3934-5d3f-9a55-40e8063b086a"))
IVectorView<int> : IVectorView_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<int> __FIVectorView_1_int_t;
#define __FIVectorView_1_int ABI::Windows::Foundation::Collections::__FIVectorView_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_int_USE */


#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                namespace Provider {
                    interface ISpiProvider;
                } /* Provider */
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider ABI::Windows::Devices::Spi::Provider::ISpiProvider

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__

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
        namespace Devices {
            namespace Spi {
                typedef enum SpiMode : int SpiMode;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                typedef enum SpiSharingMode : int SpiSharingMode;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                class SpiBusInfo;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                class SpiConnectionSettings;
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Spi.SpiMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                enum SpiMode : int
                {
                    SpiMode_Mode0 = 0,
                    SpiMode_Mode1 = 1,
                    SpiMode_Mode2 = 2,
                    SpiMode_Mode3 = 3,
                };
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Spi.SpiSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                enum SpiSharingMode : int
                {
                    SpiSharingMode_Exclusive = 0,
                    SpiSharingMode_Shared = 1,
                };
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiBusInfo
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiBusInfo
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiBusInfo[] = L"Windows.Devices.Spi.ISpiBusInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("9929444a-54f2-48c6-b952-9c32fc02c669")
                ISpiBusInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChipSelectLineCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinClockFrequency(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxClockFrequency(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedDataBitLengths(
                        __FIVectorView_1_int** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiBusInfo = __uuidof(ISpiBusInfo);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiConnectionSettings[] = L"Windows.Devices.Spi.ISpiConnectionSettings";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("5283a37f-f935-4b9f-a7a7-3a7890afa5ce")
                ISpiConnectionSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChipSelectLine(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ChipSelectLine(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Devices::Spi::SpiMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Devices::Spi::SpiMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DataBitLength(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DataBitLength(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClockFrequency(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClockFrequency(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SharingMode(
                        ABI::Windows::Devices::Spi::SpiSharingMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SharingMode(
                        ABI::Windows::Devices::Spi::SpiSharingMode value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiConnectionSettings = __uuidof(ISpiConnectionSettings);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiConnectionSettingsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiConnectionSettingsFactory[] = L"Windows.Devices.Spi.ISpiConnectionSettingsFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("ff99081e-10c4-44b7-9fea-a748b5a46f31")
                ISpiConnectionSettingsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        INT32 chipSelectLine,
                        ABI::Windows::Devices::Spi::ISpiConnectionSettings** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiConnectionSettingsFactory = __uuidof(ISpiConnectionSettingsFactory);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiController[] = L"Windows.Devices.Spi.ISpiController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("a8d3c829-9895-4159-a934-8741f1ee6d27")
                ISpiController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDevice(
                        ABI::Windows::Devices::Spi::ISpiConnectionSettings* settings,
                        ABI::Windows::Devices::Spi::ISpiDevice** device
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiController = __uuidof(ISpiController);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.ISpiControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiControllerStatics[] = L"Windows.Devices.Spi.ISpiControllerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("0d5229e2-138b-4e48-b964-4f2f79b9c5a2")
                ISpiControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetControllersAsync(
                        ABI::Windows::Devices::Spi::Provider::ISpiProvider* provider,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiControllerStatics = __uuidof(ISpiControllerStatics);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.ISpiDevice
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiDevice[] = L"Windows.Devices.Spi.ISpiDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("05d5356d-11b6-4d39-84d5-95dfb4c9f2ce")
                ISpiDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionSettings(
                        ABI::Windows::Devices::Spi::ISpiConnectionSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Write(
                        UINT32 bufferLength,
                        BYTE* buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Read(
                        UINT32 bufferLength,
                        BYTE* buffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferSequential(
                        UINT32 writeBufferLength,
                        BYTE* writeBuffer,
                        UINT32 readBufferLength,
                        BYTE* readBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferFullDuplex(
                        UINT32 writeBufferLength,
                        BYTE* writeBuffer,
                        UINT32 readBufferLength,
                        BYTE* readBuffer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiDevice = __uuidof(ISpiDevice);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiDeviceStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiDeviceStatics[] = L"Windows.Devices.Spi.ISpiDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Spi {
                MIDL_INTERFACE("a278e559-5720-4d3f-bd93-56f5ff5a5879")
                ISpiDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromFriendlyName(
                        HSTRING friendlyName,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBusInfo(
                        HSTRING busId,
                        ABI::Windows::Devices::Spi::ISpiBusInfo** busInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING busId,
                        ABI::Windows::Devices::Spi::ISpiConnectionSettings* settings,
                        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpiDeviceStatics = __uuidof(ISpiDeviceStatics);
            } /* Spi */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiBusInfo
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiBusInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiBusInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiBusInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiBusInfo[] = L"Windows.Devices.Spi.SpiBusInfo";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Spi.ISpiConnectionSettingsFactory interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiConnectionSettings[] = L"Windows.Devices.Spi.SpiConnectionSettings";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Spi.ISpiControllerStatics interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiController[] = L"Windows.Devices.Spi.SpiController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Spi.SpiDevice
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Spi.ISpiDeviceStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiDevice[] = L"Windows.Devices.Spi.SpiDevice";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiController __x_ABI_CWindows_CDevices_CSpi_CISpiController;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiDevice __x_ABI_CWindows_CDevices_CSpi_CISpiDevice;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSpi__CSpiDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSpi__CSpiController __FIIterator_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIIterator_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSpi__CSpiController* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIIterator_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSpi__CSpiController_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSpi__CSpiController __FIIterable_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIIterable_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIIterator_1_Windows__CDevices__CSpi__CSpiController** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIIterable_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSpi__CSpiController_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSpi__CSpiController __FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSpi_CISpiController** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIVectorView_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIVectorView_1_Windows__CDevices__CSpi__CSpiController** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

#if !defined(____FIIterator_1_int_INTERFACE_DEFINED__)
#define ____FIIterator_1_int_INTERFACE_DEFINED__

typedef interface __FIIterator_1_int __FIIterator_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_int;

typedef struct __FIIterator_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_int* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_int* This,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_intVtbl;

interface __FIIterator_1_int
{
    CONST_VTBL struct __FIIterator_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_int_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_int_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_int_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_int_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_int_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_int_INTERFACE_DEFINED__)
#define ____FIIterable_1_int_INTERFACE_DEFINED__

typedef interface __FIIterable_1_int __FIIterable_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_int;

typedef struct __FIIterable_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_int* This,
        __FIIterator_1_int** result);

    END_INTERFACE
} __FIIterable_1_intVtbl;

interface __FIIterable_1_int
{
    CONST_VTBL struct __FIIterable_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_int_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_int_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_int_INTERFACE_DEFINED__)
#define ____FIVectorView_1_int_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_int __FIVectorView_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_int;

typedef struct __FIVectorView_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_int* This,
        UINT32 index,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_int* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_int* This,
        INT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_int* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_intVtbl;

interface __FIVectorView_1_int
{
    CONST_VTBL struct __FIVectorView_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_int_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_int_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_int_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_int_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_int_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider;

#endif // ____x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CSpi_CSpiMode __x_ABI_CWindows_CDevices_CSpi_CSpiMode;

typedef enum __x_ABI_CWindows_CDevices_CSpi_CSpiSharingMode __x_ABI_CWindows_CDevices_CSpi_CSpiSharingMode;

/*
 *
 * Struct Windows.Devices.Spi.SpiMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSpi_CSpiMode
{
    SpiMode_Mode0 = 0,
    SpiMode_Mode1 = 1,
    SpiMode_Mode2 = 2,
    SpiMode_Mode3 = 3,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Spi.SpiSharingMode
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSpi_CSpiSharingMode
{
    SpiSharingMode_Exclusive = 0,
    SpiSharingMode_Shared = 1,
};
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiBusInfo
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiBusInfo
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiBusInfo[] = L"Windows.Devices.Spi.ISpiBusInfo";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChipSelectLineCount)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedDataBitLengths)(__x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo* This,
        __FIVectorView_1_int** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfoVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_get_ChipSelectLineCount(This, value) \
    ((This)->lpVtbl->get_ChipSelectLineCount(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_get_MinClockFrequency(This, value) \
    ((This)->lpVtbl->get_MinClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_get_MaxClockFrequency(This, value) \
    ((This)->lpVtbl->get_MaxClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_get_SupportedDataBitLengths(This, value) \
    ((This)->lpVtbl->get_SupportedDataBitLengths(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiConnectionSettings[] = L"Windows.Devices.Spi.ISpiConnectionSettings";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChipSelectLine)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ChipSelectLine)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CSpiMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CSpiMode value);
    HRESULT (STDMETHODCALLTYPE* get_DataBitLength)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DataBitLength)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ClockFrequency)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_SharingMode)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CSpiSharingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_SharingMode)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* This,
        enum __x_ABI_CWindows_CDevices_CSpi_CSpiSharingMode value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_get_ChipSelectLine(This, value) \
    ((This)->lpVtbl->get_ChipSelectLine(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_put_ChipSelectLine(This, value) \
    ((This)->lpVtbl->put_ChipSelectLine(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_get_DataBitLength(This, value) \
    ((This)->lpVtbl->get_DataBitLength(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_put_DataBitLength(This, value) \
    ((This)->lpVtbl->put_DataBitLength(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_get_ClockFrequency(This, value) \
    ((This)->lpVtbl->get_ClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_put_ClockFrequency(This, value) \
    ((This)->lpVtbl->put_ClockFrequency(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_get_SharingMode(This, value) \
    ((This)->lpVtbl->get_SharingMode(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_put_SharingMode(This, value) \
    ((This)->lpVtbl->put_SharingMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiConnectionSettingsFactory
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiConnectionSettings
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiConnectionSettingsFactory[] = L"Windows.Devices.Spi.ISpiConnectionSettingsFactory";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory* This,
        INT32 chipSelectLine,
        __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_Create(This, chipSelectLine, value) \
    ((This)->lpVtbl->Create(This, chipSelectLine, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiController[] = L"Windows.Devices.Spi.ISpiController";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDevice)(__x_ABI_CWindows_CDevices_CSpi_CISpiController* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* settings,
        __x_ABI_CWindows_CDevices_CSpi_CISpiDevice** device);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiControllerVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiController_GetDevice(This, settings, device) \
    ((This)->lpVtbl->GetDevice(This, settings, device))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiController_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.ISpiControllerStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiController
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiControllerStatics[] = L"Windows.Devices.Spi.ISpiControllerStatics";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiController** operation);
    HRESULT (STDMETHODCALLTYPE* GetControllersAsync)(__x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics* This,
        __x_ABI_CWindows_CDevices_CSpi_CProvider_CISpiProvider* provider,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CSpi__CSpiController** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_GetControllersAsync(This, provider, operation) \
    ((This)->lpVtbl->GetControllersAsync(This, provider, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Devices.Spi.ISpiDevice
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Spi.SpiDevice
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiDevice[] = L"Windows.Devices.Spi.ISpiDevice";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionSettings)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings** value);
    HRESULT (STDMETHODCALLTYPE* Write)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* Read)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* TransferSequential)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer);
    HRESULT (STDMETHODCALLTYPE* TransferFullDuplex)(__x_ABI_CWindows_CDevices_CSpi_CISpiDevice* This,
        UINT32 writeBufferLength,
        BYTE* writeBuffer,
        UINT32 readBufferLength,
        BYTE* readBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_get_ConnectionSettings(This, value) \
    ((This)->lpVtbl->get_ConnectionSettings(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_Write(This, bufferLength, buffer) \
    ((This)->lpVtbl->Write(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_Read(This, bufferLength, buffer) \
    ((This)->lpVtbl->Read(This, bufferLength, buffer))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_TransferSequential(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer) \
    ((This)->lpVtbl->TransferSequential(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDevice_TransferFullDuplex(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer) \
    ((This)->lpVtbl->TransferFullDuplex(This, writeBufferLength, writeBuffer, readBufferLength, readBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Spi.ISpiDeviceStatics
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Spi_ISpiDeviceStatics[] = L"Windows.Devices.Spi.ISpiDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromFriendlyName)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        HSTRING friendlyName,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetBusInfo)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        HSTRING busId,
        __x_ABI_CWindows_CDevices_CSpi_CISpiBusInfo** busInfo);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics* This,
        HSTRING busId,
        __x_ABI_CWindows_CDevices_CSpi_CISpiConnectionSettings* settings,
        __FIAsyncOperation_1_Windows__CDevices__CSpi__CSpiDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetDeviceSelector(This, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetDeviceSelectorFromFriendlyName(This, friendlyName, value) \
    ((This)->lpVtbl->GetDeviceSelectorFromFriendlyName(This, friendlyName, value))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_GetBusInfo(This, busId, busInfo) \
    ((This)->lpVtbl->GetBusInfo(This, busId, busInfo))

#define __x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_FromIdAsync(This, busId, settings, operation) \
    ((This)->lpVtbl->FromIdAsync(This, busId, settings, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSpi_CISpiDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiBusInfo
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiBusInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiBusInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiBusInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiBusInfo[] = L"Windows.Devices.Spi.SpiBusInfo";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiConnectionSettings
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Spi.ISpiConnectionSettingsFactory interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiConnectionSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiConnectionSettings_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiConnectionSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiConnectionSettings[] = L"Windows.Devices.Spi.SpiConnectionSettings";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Spi.SpiController
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Spi.ISpiControllerStatics interface starting with version 2.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiController[] = L"Windows.Devices.Spi.SpiController";
#endif
#endif // WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Devices.Spi.SpiDevice
 *
 * Introduced to Windows.Devices.DevicesLowLevelContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Spi.ISpiDeviceStatics interface starting with version 1.0 of the Windows.Devices.DevicesLowLevelContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Spi.ISpiDevice ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_DEVICESLOWLEVELCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Spi_SpiDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Spi_SpiDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Spi_SpiDevice[] = L"Windows.Devices.Spi.SpiDevice";
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
#endif // __windows2Edevices2Espi_p_h__

#endif // __windows2Edevices2Espi_h__
