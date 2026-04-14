
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
#ifndef __windows2Edevices2Einput2Epreview_h__
#define __windows2Edevices2Einput2Epreview_h__
#ifndef __windows2Edevices2Einput2Epreview_p_h__
#define __windows2Edevices2Einput2Epreview_p_h__


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
#include "Windows.Devices.HumanInterfaceDevice.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeDevicePreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview ABI::Windows::Devices::Input::Preview::IGazeDevicePreview

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeDeviceWatcherAddedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherAddedPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeDeviceWatcherPreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeDeviceWatcherRemovedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherRemovedPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeDeviceWatcherUpdatedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherUpdatedPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeEnteredPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeEnteredPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeExitedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeExitedPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeInputSourcePreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreview

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeInputSourcePreviewStatics;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreviewStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazeMovedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs ABI::Windows::Devices::Input::Preview::IGazeMovedPreviewEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    interface IGazePointPreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview ABI::Windows::Devices::Input::Preview::IGazePointPreview

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidBooleanControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidBooleanControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("203203b0-b7f4-542d-b0d0-9caa1fb55d7f"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0ff0fed-a156-58bf-9411-5777df9d57bf"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidNumericControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidNumericControlDescription;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("52b9c36e-7d95-5d1c-acab-23c19ea76f01"))
IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("868f060d-e0d4-571b-b2f7-431d6984a513"))
IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazePointPreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#define DEF___FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad015c81-2d80-501e-bc9a-a63f05f93bac"))
IIterator<ABI::Windows::Devices::Input::Preview::GazePointPreview*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazePointPreview*, ABI::Windows::Devices::Input::Preview::IGazePointPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Input.Preview.GazePointPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Input::Preview::GazePointPreview*> __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t;
#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#define DEF___FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1cf68266-3eb7-5336-840a-3c1d9fdf5349"))
IIterable<ABI::Windows::Devices::Input::Preview::GazePointPreview*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazePointPreview*, ABI::Windows::Devices::Input::Preview::IGazePointPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Input.Preview.GazePointPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Input::Preview::GazePointPreview*> __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t;
#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aab72786-ec34-536f-a7c5-27394753df2c"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidBooleanControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidBooleanControlDescription*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e02ca66c-610a-51b4-aef9-3707b697b985"))
IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*, ABI::Windows::Devices::HumanInterfaceDevice::IHidNumericControlDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::HumanInterfaceDevice::HidNumericControlDescription*> __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t;
#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#define DEF___FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3d3d6148-ad02-56eb-acaf-0ea9e47c0298"))
IVectorView<ABI::Windows::Devices::Input::Preview::GazePointPreview*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazePointPreview*, ABI::Windows::Devices::Input::Preview::IGazePointPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Input.Preview.GazePointPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Input::Preview::GazePointPreview*> __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t;
#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#define DEF___FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("446a643d-387c-5ef6-a8ac-cca9d8a793b4"))
IVector<ABI::Windows::Devices::Input::Preview::GazePointPreview*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazePointPreview*, ABI::Windows::Devices::Input::Preview::IGazePointPreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Input.Preview.GazePointPreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Input::Preview::GazePointPreview*> __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t;
#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CPoint_USE
#define DEF___FIReference_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("84f14c22-a00a-5272-8d3d-82112e66df00"))
IReference<struct ABI::Windows::Foundation::Point> : IReference_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Point> __FIReference_1_Windows__CFoundation__CPoint_t;
#define __FIReference_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CPoint_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeDeviceWatcherPreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("73a19afb-6081-551b-bf73-d5d23155da8e"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeDeviceWatcherPreview, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeDeviceWatcherAddedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5bf95725-6889-544f-ba3b-dda986add8ae"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherAddedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherAddedPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherAddedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeDeviceWatcherPreview, Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherAddedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeDeviceWatcherRemovedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("06c67a46-34b3-53fe-86df-ceb52e2d12e7"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherRemovedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherRemovedPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherRemovedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeDeviceWatcherPreview, Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherRemovedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeDeviceWatcherUpdatedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fe8090ac-7d5d-50a7-a3d3-f311648a7b89"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherUpdatedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherUpdatedPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherUpdatedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeDeviceWatcherPreview, Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherPreview*, ABI::Windows::Devices::Input::Preview::GazeDeviceWatcherUpdatedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeInputSourcePreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeEnteredPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3876b9c5-36a0-5221-be04-4aeefb9870b2"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeEnteredPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeEnteredPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeEnteredPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeInputSourcePreview, Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeEnteredPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeExitedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0f19b16c-73d9-5775-92a3-0f6f942e4eb0"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeExitedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeExitedPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeExitedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeInputSourcePreview, Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeExitedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeMovedPreviewEventArgs;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e7c08e8f-1aba-5952-af5c-d3a2707f4fe4"))
ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeMovedPreviewEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::Preview::GazeMovedPreviewEventArgs*, ABI::Windows::Devices::Input::Preview::IGazeMovedPreviewEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.Preview.GazeInputSourcePreview, Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::Preview::GazeInputSourcePreview*, ABI::Windows::Devices::Input::Preview::GazeMovedPreviewEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                class HidInputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace HumanInterfaceDevice {
                interface IHidInputReport;
            } /* HumanInterfaceDevice */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__

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
            namespace Input {
                namespace Preview {
                    typedef enum GazeDeviceConfigurationStatePreview : int GazeDeviceConfigurationStatePreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    class GazeDevicePreview;
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    enum GazeDeviceConfigurationStatePreview : int
                    {
                        GazeDeviceConfigurationStatePreview_Unknown = 0,
                        GazeDeviceConfigurationStatePreview_Ready = 1,
                        GazeDeviceConfigurationStatePreview_Configuring = 2,
                        GazeDeviceConfigurationStatePreview_ScreenSetupNeeded = 3,
                        GazeDeviceConfigurationStatePreview_UserCalibrationNeeded = 4,
                    };
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDevicePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDevicePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDevicePreview[] = L"Windows.Devices.Input.Preview.IGazeDevicePreview";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7ee9-b389-11e7-b201-c8d3ffb75721")
                    IGazeDevicePreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanTrackEyes(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanTrackHead(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ConfigurationState(
                            ABI::Windows::Devices::Input::Preview::GazeDeviceConfigurationStatePreview* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestCalibrationAsync(
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNumericControlDescriptions(
                            UINT16 usagePage,
                            UINT16 usageId,
                            __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetBooleanControlDescriptions(
                            UINT16 usagePage,
                            UINT16 usageId,
                            __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeDevicePreview = __uuidof(IGazeDevicePreview);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherAddedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7eed-b389-11e7-b201-c8d3ffb75721")
                    IGazeDeviceWatcherAddedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Device(
                            ABI::Windows::Devices::Input::Preview::IGazeDevicePreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeDeviceWatcherAddedPreviewEventArgs = __uuidof(IGazeDeviceWatcherAddedPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherPreview[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7ee7-b389-11e7-b201-c8d3ffb75721")
                    IGazeDeviceWatcherPreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_Added(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Added(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Removed(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Updated(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeDeviceWatcherPreview = __uuidof(IGazeDeviceWatcherPreview);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherRemovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("f2631f08-0e3f-431f-a606-50b35af94a1c")
                    IGazeDeviceWatcherRemovedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Device(
                            ABI::Windows::Devices::Input::Preview::IGazeDevicePreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeDeviceWatcherRemovedPreviewEventArgs = __uuidof(IGazeDeviceWatcherRemovedPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherUpdatedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("7fe830ef-7f08-4737-88e1-4a83ae4e4885")
                    IGazeDeviceWatcherUpdatedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Device(
                            ABI::Windows::Devices::Input::Preview::IGazeDevicePreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeDeviceWatcherUpdatedPreviewEventArgs = __uuidof(IGazeDeviceWatcherUpdatedPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeEnteredPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("2567bf43-1225-489f-9dd1-daa7c50fbf4b")
                    IGazeEnteredPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentPoint(
                            ABI::Windows::Devices::Input::Preview::IGazePointPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeEnteredPreviewEventArgs = __uuidof(IGazeEnteredPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeExitedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("5d0af07e-7d83-40ef-9f0a-fbc1bbdcc5ac")
                    IGazeExitedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentPoint(
                            ABI::Windows::Devices::Input::Preview::IGazePointPreview** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeExitedPreviewEventArgs = __uuidof(IGazeExitedPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeInputSourcePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeInputSourcePreview[] = L"Windows.Devices.Input.Preview.IGazeInputSourcePreview";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7ee8-b389-11e7-b201-c8d3ffb75721")
                    IGazeInputSourcePreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_GazeMoved(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GazeMoved(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GazeEntered(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GazeEntered(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GazeExited(
                            __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GazeExited(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeInputSourcePreview = __uuidof(IGazeInputSourcePreview);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeInputSourcePreviewStatics[] = L"Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7ee6-b389-11e7-b201-c8d3ffb75721")
                    IGazeInputSourcePreviewStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::Devices::Input::Preview::IGazeInputSourcePreview** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                            ABI::Windows::Devices::Input::Preview::IGazeDeviceWatcherPreview** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeInputSourcePreviewStatics = __uuidof(IGazeInputSourcePreviewStatics);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeMovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7eeb-b389-11e7-b201-c8d3ffb75721")
                    IGazeMovedPreviewEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentPoint(
                            ABI::Windows::Devices::Input::Preview::IGazePointPreview** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetIntermediatePoints(
                            __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazeMovedPreviewEventArgs = __uuidof(IGazeMovedPreviewEventArgs);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazePointPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazePointPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazePointPreview[] = L"Windows.Devices.Input.Preview.IGazePointPreview";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("e79e7eea-b389-11e7-b201-c8d3ffb75721")
                    IGazePointPreview : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceDevice(
                            ABI::Windows::Devices::Input::Preview::IGazeDevicePreview** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EyeGazePosition(
                            __FIReference_1_Windows__CFoundation__CPoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HeadGazePosition(
                            __FIReference_1_Windows__CFoundation__CPoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HidInputReport(
                            ABI::Windows::Devices::HumanInterfaceDevice::IHidInputReport** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGazePointPreview = __uuidof(IGazePointPreview);
                } /* Preview */
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDevicePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDevicePreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDevicePreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDevicePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDevicePreview[] = L"Windows.Devices.Input.Preview.GazeDevicePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeInputSourcePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeInputSourcePreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeInputSourcePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeInputSourcePreview[] = L"Windows.Devices.Input.Preview.GazeInputSourcePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazePointPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazePointPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazePointPreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazePointPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazePointPreview[] = L"Windows.Devices.Input.Preview.GazePointPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview;

#endif // ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __FIIterator_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

typedef struct __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl;

interface __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

typedef struct __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __FIIterator_1_Windows__CDevices__CInput__CPreview__CGazePointPreview** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl;

interface __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidBooleanControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription;

typedef struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidNumericControlDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl;

interface __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

typedef struct __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl;

interface __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview;

typedef struct __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __FIVectorView_1_Windows__CDevices__CInput__CPreview__CGazePointPreview** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl;

interface __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CPoint __FIReference_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CPoint;

typedef struct __FIReference_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CPointVtbl;

interface __FIReference_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CPoint_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* sender,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport;

#endif // ____x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CInput_CPreview_CGazeDeviceConfigurationStatePreview __x_ABI_CWindows_CDevices_CInput_CPreview_CGazeDeviceConfigurationStatePreview;

/*
 *
 * Struct Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CDevices_CInput_CPreview_CGazeDeviceConfigurationStatePreview
{
    GazeDeviceConfigurationStatePreview_Unknown = 0,
    GazeDeviceConfigurationStatePreview_Ready = 1,
    GazeDeviceConfigurationStatePreview_Configuring = 2,
    GazeDeviceConfigurationStatePreview_ScreenSetupNeeded = 3,
    GazeDeviceConfigurationStatePreview_UserCalibrationNeeded = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDevicePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDevicePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDevicePreview[] = L"Windows.Devices.Input.Preview.IGazeDevicePreview";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CanTrackEyes)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanTrackHead)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ConfigurationState)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPreview_CGazeDeviceConfigurationStatePreview* value);
    HRESULT (STDMETHODCALLTYPE* RequestCalibrationAsync)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* GetNumericControlDescriptions)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        UINT16 usagePage,
        UINT16 usageId,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidNumericControlDescription** result);
    HRESULT (STDMETHODCALLTYPE* GetBooleanControlDescriptions)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview* This,
        UINT16 usagePage,
        UINT16 usageId,
        __FIVectorView_1_Windows__CDevices__CHumanInterfaceDevice__CHidBooleanControlDescription** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreviewVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_get_CanTrackEyes(This, value) \
    ((This)->lpVtbl->get_CanTrackEyes(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_get_CanTrackHead(This, value) \
    ((This)->lpVtbl->get_CanTrackHead(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_get_ConfigurationState(This, value) \
    ((This)->lpVtbl->get_ConfigurationState(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_RequestCalibrationAsync(This, operation) \
    ((This)->lpVtbl->RequestCalibrationAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_GetNumericControlDescriptions(This, usagePage, usageId, result) \
    ((This)->lpVtbl->GetNumericControlDescriptions(This, usagePage, usageId, result))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_GetBooleanControlDescriptions(This, usagePage, usageId, result) \
    ((This)->lpVtbl->GetBooleanControlDescriptions(This, usagePage, usageId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherAddedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherAddedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherPreview[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherAddedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherRemovedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherUpdatedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeDeviceWatcherPreview_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreviewVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherRemovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherRemovedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeDeviceWatcherUpdatedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherUpdatedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeEnteredPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPoint)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_get_CurrentPoint(This, value) \
    ((This)->lpVtbl->get_CurrentPoint(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeEnteredPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeExitedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPoint)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_get_CurrentPoint(This, value) \
    ((This)->lpVtbl->get_CurrentPoint(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeExitedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeInputSourcePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeInputSourcePreview[] = L"Windows.Devices.Input.Preview.IGazeInputSourcePreview";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_GazeMoved)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeMovedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GazeMoved)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GazeEntered)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeEnteredPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GazeEntered)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GazeExited)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPreview__CGazeInputSourcePreview_Windows__CDevices__CInput__CPreview__CGazeExitedPreviewEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GazeExited)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_add_GazeMoved(This, handler, token) \
    ((This)->lpVtbl->add_GazeMoved(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_remove_GazeMoved(This, token) \
    ((This)->lpVtbl->remove_GazeMoved(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_add_GazeEntered(This, handler, token) \
    ((This)->lpVtbl->add_GazeEntered(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_remove_GazeEntered(This, token) \
    ((This)->lpVtbl->remove_GazeEntered(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_add_GazeExited(This, handler, token) \
    ((This)->lpVtbl->add_GazeExited(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_remove_GazeExited(This, token) \
    ((This)->lpVtbl->remove_GazeExited(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeInputSourcePreviewStatics[] = L"Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreview** result);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDeviceWatcherPreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_CreateWatcher(This, result) \
    ((This)->lpVtbl->CreateWatcher(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeInputSourcePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazeMovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPoint)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview** value);
    HRESULT (STDMETHODCALLTYPE* GetIntermediatePoints)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs* This,
        __FIVector_1_Windows__CDevices__CInput__CPreview__CGazePointPreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_get_CurrentPoint(This, value) \
    ((This)->lpVtbl->get_CurrentPoint(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_GetIntermediatePoints(This, result) \
    ((This)->lpVtbl->GetIntermediatePoints(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeMovedPreviewEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.Input.Preview.IGazePointPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.Preview.GazePointPreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_Preview_IGazePointPreview[] = L"Windows.Devices.Input.Preview.IGazePointPreview";
typedef struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceDevice)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazeDevicePreview** value);
    HRESULT (STDMETHODCALLTYPE* get_EyeGazePosition)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        __FIReference_1_Windows__CFoundation__CPoint** value);
    HRESULT (STDMETHODCALLTYPE* get_HeadGazePosition)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        __FIReference_1_Windows__CFoundation__CPoint** value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_HidInputReport)(__x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview* This,
        __x_ABI_CWindows_CDevices_CHumanInterfaceDevice_CIHidInputReport** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreviewVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_get_SourceDevice(This, value) \
    ((This)->lpVtbl->get_SourceDevice(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_get_EyeGazePosition(This, value) \
    ((This)->lpVtbl->get_EyeGazePosition(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_get_HeadGazePosition(This, value) \
    ((This)->lpVtbl->get_HeadGazePosition(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_get_HidInputReport(This, value) \
    ((This)->lpVtbl->get_HidInputReport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CPreview_CIGazePointPreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDevicePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDevicePreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDevicePreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDevicePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDevicePreview[] = L"Windows.Devices.Input.Preview.GazeDevicePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherAddedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherAddedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherPreview[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherRemovedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherRemovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeDeviceWatcherUpdatedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeDeviceWatcherUpdatedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeEnteredPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeEnteredPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeExitedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeExitedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeInputSourcePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.Preview.IGazeInputSourcePreviewStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeInputSourcePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeInputSourcePreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeInputSourcePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeInputSourcePreview[] = L"Windows.Devices.Input.Preview.GazeInputSourcePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazeMovedPreviewEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazeMovedPreviewEventArgs[] = L"Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.Input.Preview.GazePointPreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.Preview.IGazePointPreview ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_Input_Preview_GazePointPreview_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_Preview_GazePointPreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_Preview_GazePointPreview[] = L"Windows.Devices.Input.Preview.GazePointPreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Einput2Epreview_p_h__

#endif // __windows2Edevices2Einput2Epreview_h__
