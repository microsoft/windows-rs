
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
#ifndef __windows2Edevices2Einput_h__
#define __windows2Edevices2Einput_h__
#ifndef __windows2Edevices2Einput_p_h__
#define __windows2Edevices2Einput_p_h__


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
#include "Windows.Devices.Haptics.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IKeyboardCapabilities;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities ABI::Windows::Devices::Input::IKeyboardCapabilities

#endif // ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IMouseCapabilities;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities ABI::Windows::Devices::Input::IMouseCapabilities

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IMouseDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice ABI::Windows::Devices::Input::IMouseDevice

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IMouseDeviceStatics;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics ABI::Windows::Devices::Input::IMouseDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IMouseEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs ABI::Windows::Devices::Input::IMouseEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenButtonListener;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener ABI::Windows::Devices::Input::IPenButtonListener

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenButtonListenerStatics;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics ABI::Windows::Devices::Input::IPenButtonListenerStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice ABI::Windows::Devices::Input::IPenDevice

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDevice2;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2 ABI::Windows::Devices::Input::IPenDevice2

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDeviceStatics;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics ABI::Windows::Devices::Input::IPenDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDockListener;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener ABI::Windows::Devices::Input::IPenDockListener

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDockListenerStatics;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics ABI::Windows::Devices::Input::IPenDockListenerStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenDockedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs ABI::Windows::Devices::Input::IPenDockedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenTailButtonClickedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs ABI::Windows::Devices::Input::IPenTailButtonClickedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenTailButtonDoubleClickedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs ABI::Windows::Devices::Input::IPenTailButtonDoubleClickedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenTailButtonLongPressedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs ABI::Windows::Devices::Input::IPenTailButtonLongPressedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPenUndockedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs ABI::Windows::Devices::Input::IPenUndockedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPointerDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice ABI::Windows::Devices::Input::IPointerDevice

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPointerDevice2;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2 ABI::Windows::Devices::Input::IPointerDevice2

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface IPointerDeviceStatics;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics ABI::Windows::Devices::Input::IPointerDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                interface ITouchCapabilities;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities ABI::Windows::Devices::Input::ITouchCapabilities

#endif // ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PointerDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDevice_USE
#define DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de94641c-7960-5fcd-abe8-d6ba609ef7d3"))
IIterator<ABI::Windows::Devices::Input::PointerDevice*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PointerDevice*, ABI::Windows::Devices::Input::IPointerDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Input.PointerDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Input::PointerDevice*> __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_t;
#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CInput__CPointerDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDevice_USE
#define DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad26662c-845b-5c6d-aeaa-406f48c21ae9"))
IIterable<ABI::Windows::Devices::Input::PointerDevice*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PointerDevice*, ABI::Windows::Devices::Input::IPointerDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Input.PointerDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Input::PointerDevice*> __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_t;
#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CInput__CPointerDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                typedef struct PointerDeviceUsage PointerDeviceUsage;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#define DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9ab2160d-11ef-5eca-8dd9-3e13aa4e5f99"))
IIterator<struct ABI::Windows::Devices::Input::PointerDeviceUsage> : IIterator_impl<struct ABI::Windows::Devices::Input::PointerDeviceUsage>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Input.PointerDeviceUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Devices::Input::PointerDeviceUsage> __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_t;
#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#define DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("592d6618-eaab-5a79-a47a-c7fc0b749a4e"))
IIterable<struct ABI::Windows::Devices::Input::PointerDeviceUsage> : IIterable_impl<struct ABI::Windows::Devices::Input::PointerDeviceUsage>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Input.PointerDeviceUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Devices::Input::PointerDeviceUsage> __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_t;
#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_USE
#define DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf5674f1-9808-5a2b-80b8-5684ed0ea816"))
IVectorView<ABI::Windows::Devices::Input::PointerDevice*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PointerDevice*, ABI::Windows::Devices::Input::IPointerDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Input.PointerDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Input::PointerDevice*> __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_t;
#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#define DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e5a2c7e-3830-50d5-92ba-3163c89cbbd0"))
IVectorView<struct ABI::Windows::Devices::Input::PointerDeviceUsage> : IVectorView_impl<struct ABI::Windows::Devices::Input::PointerDeviceUsage>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Input.PointerDeviceUsage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Devices::Input::PointerDeviceUsage> __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_t;
#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class MouseDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class MouseEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5d72e594-28e4-5895-a34b-ea910f70fdbb"))
ITypedEventHandler<ABI::Windows::Devices::Input::MouseDevice*, ABI::Windows::Devices::Input::MouseEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::MouseDevice*, ABI::Windows::Devices::Input::IMouseDevice*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::MouseEventArgs*, ABI::Windows::Devices::Input::IMouseEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.MouseDevice, Windows.Devices.Input.MouseEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::MouseDevice*, ABI::Windows::Devices::Input::MouseEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenButtonListener;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0a0ee5ed-041b-5569-b4a4-a1e58f1343a2"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::IPenButtonListener*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenButtonListener, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenTailButtonClickedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2eda7109-f1c9-5f26-a054-3d25062cc0de"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonClickedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::IPenButtonListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenTailButtonClickedEventArgs*, ABI::Windows::Devices::Input::IPenTailButtonClickedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenButtonListener, Windows.Devices.Input.PenTailButtonClickedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonClickedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenTailButtonDoubleClickedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e6e58d94-6f7b-593f-81f0-033150b4432c"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonDoubleClickedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::IPenButtonListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenTailButtonDoubleClickedEventArgs*, ABI::Windows::Devices::Input::IPenTailButtonDoubleClickedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenButtonListener, Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonDoubleClickedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenTailButtonLongPressedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ca6dab1f-20d6-52b0-94ce-6dcc75a1a984"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonLongPressedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::IPenButtonListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenTailButtonLongPressedEventArgs*, ABI::Windows::Devices::Input::IPenTailButtonLongPressedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenButtonListener, Windows.Devices.Input.PenTailButtonLongPressedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenButtonListener*, ABI::Windows::Devices::Input::PenTailButtonLongPressedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenDockListener;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7ceae82f-499a-5b31-a1d8-3b11c53c7871"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::IPenDockListener*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenDockListener, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenDockedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("19786a6c-8e23-5de7-a773-e711a5a068c5"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::PenDockedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::IPenDockListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenDockedEventArgs*, ABI::Windows::Devices::Input::IPenDockedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenDockListener, Windows.Devices.Input.PenDockedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::PenDockedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenUndockedEventArgs;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51dba508-3194-5c91-a2e7-f94637e75a13"))
ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::PenUndockedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::IPenDockListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Input::PenUndockedEventArgs*, ABI::Windows::Devices::Input::IPenUndockedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Input.PenDockListener, Windows.Devices.Input.PenUndockedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Input::PenDockListener*, ABI::Windows::Devices::Input::PenUndockedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class SimpleHapticsController;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface ISimpleHapticsController;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController ABI::Windows::Devices::Haptics::ISimpleHapticsController

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                typedef enum PointerDeviceType : int PointerDeviceType;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                typedef struct MouseDelta MouseDelta;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                class PenDevice;
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Input.PointerDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                enum PointerDeviceType : int
                {
                    PointerDeviceType_Touch = 0,
                    PointerDeviceType_Pen = 1,
                    PointerDeviceType_Mouse = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
                    PointerDeviceType_Touchpad = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
                };
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Input.MouseDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                struct MouseDelta
                {
                    INT32 X;
                    INT32 Y;
                };
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Input.PointerDeviceUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                struct PointerDeviceUsage
                {
                    UINT32 UsagePage;
                    UINT32 Usage;
                    INT32 MinLogical;
                    INT32 MaxLogical;
                    INT32 MinPhysical;
                    INT32 MaxPhysical;
                    UINT32 Unit;
                    FLOAT PhysicalMultiplier;
                };
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IKeyboardCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.KeyboardCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IKeyboardCapabilities[] = L"Windows.Devices.Input.IKeyboardCapabilities";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("3a3f9b56-6798-4bbc-833e-0f34b17c65ff")
                IKeyboardCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_KeyboardPresent(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyboardCapabilities = __uuidof(IKeyboardCapabilities);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseCapabilities[] = L"Windows.Devices.Input.IMouseCapabilities";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("bca5e023-7dd9-4b6b-9a92-55d43cb38f73")
                IMouseCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MousePresent(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VerticalWheelPresent(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HorizontalWheelPresent(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SwapButtons(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NumberOfButtons(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMouseCapabilities = __uuidof(IMouseCapabilities);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseDevice[] = L"Windows.Devices.Input.IMouseDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("88edf458-f2c8-49f4-be1f-c256b388bc11")
                IMouseDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_MouseMoved(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MouseMoved(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMouseDevice = __uuidof(IMouseDevice);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseDeviceStatics[] = L"Windows.Devices.Input.IMouseDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("484a9045-6d70-49db-8e68-46ffbd17d38d")
                IMouseDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Devices::Input::IMouseDevice** mouseDevice
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMouseDeviceStatics = __uuidof(IMouseDeviceStatics);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseEventArgs[] = L"Windows.Devices.Input.IMouseEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("f625aa5d-2354-4cc7-9230-96941c969fde")
                IMouseEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MouseDelta(
                        ABI::Windows::Devices::Input::MouseDelta* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMouseEventArgs = __uuidof(IMouseEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPenButtonListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenButtonListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenButtonListener[] = L"Windows.Devices.Input.IPenButtonListener";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("8245c376-1ee3-53f7-b1f7-8334a16f2815")
                IPenButtonListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsSupportedChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsSupportedChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TailButtonClicked(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TailButtonClicked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TailButtonDoubleClicked(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TailButtonDoubleClicked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TailButtonLongPressed(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TailButtonLongPressed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenButtonListener = __uuidof(IPenButtonListener);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenButtonListener;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenButtonListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenButtonListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenButtonListenerStatics[] = L"Windows.Devices.Input.IPenButtonListenerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("19a8a584-862f-5f69-bfea-05f6584f133f")
                IPenButtonListenerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Devices::Input::IPenButtonListener** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenButtonListenerStatics = __uuidof(IPenButtonListenerStatics);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDevice[] = L"Windows.Devices.Input.IPenDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("31856eba-a738-5a8c-b8f6-f97ef68d18ef")
                IPenDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PenId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenDevice = __uuidof(IPenDevice);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Input.IPenDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDevice2[] = L"Windows.Devices.Input.IPenDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("0207d327-7fb8-5566-8c34-f8342037b7f9")
                IPenDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenDevice2 = __uuidof(IPenDevice2);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Input.IPenDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDeviceStatics[] = L"Windows.Devices.Input.IPenDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("9dfbbe01-0966-5180-bcb4-b85060e39479")
                IPenDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetFromPointerId(
                        UINT32 pointerId,
                        ABI::Windows::Devices::Input::IPenDevice** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenDeviceStatics = __uuidof(IPenDeviceStatics);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Input.IPenDockListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockListener[] = L"Windows.Devices.Input.IPenDockListener";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("759f4d90-1dc0-55cb-ad18-b9101456f592")
                IPenDockListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsSupportedChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsSupportedChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Docked(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Docked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Undocked(
                        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Undocked(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenDockListener = __uuidof(IPenDockListener);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockListener;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDockListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockListenerStatics[] = L"Windows.Devices.Input.IPenDockListenerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("cab75e9a-0016-5c72-969e-a97e11992a93")
                IPenDockListenerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Devices::Input::IPenDockListener** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPenDockListenerStatics = __uuidof(IPenDockListenerStatics);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockedEventArgs[] = L"Windows.Devices.Input.IPenDockedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("fd4277c6-ca63-5d4e-9ed3-a28a54521c8c")
                IPenDockedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPenDockedEventArgs = __uuidof(IPenDockedEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonClickedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonClickedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("5d2fb7b6-6ad3-5d3e-ab29-05ea2410e390")
                IPenTailButtonClickedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPenTailButtonClickedEventArgs = __uuidof(IPenTailButtonClickedEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonDoubleClickedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("846321a2-618a-5478-b04c-b358231da4a7")
                IPenTailButtonDoubleClickedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPenTailButtonDoubleClickedEventArgs = __uuidof(IPenTailButtonDoubleClickedEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonLongPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonLongPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonLongPressedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonLongPressedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("f37c606e-c60a-5f42-b818-a53112406c13")
                IPenTailButtonLongPressedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPenTailButtonLongPressedEventArgs = __uuidof(IPenTailButtonLongPressedEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenUndockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenUndockedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenUndockedEventArgs[] = L"Windows.Devices.Input.IPenUndockedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("ccd09150-261b-59e6-a5d4-c1964cd03feb")
                IPenUndockedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IPenUndockedEventArgs = __uuidof(IPenUndockedEventArgs);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPointerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDevice[] = L"Windows.Devices.Input.IPointerDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("93c9bafc-ebcb-467e-82c6-276feae36b5a")
                IPointerDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerDeviceType(
                        ABI::Windows::Devices::Input::PointerDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIntegrated(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxContacts(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhysicalDeviceRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScreenRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedUsages(
                        __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerDevice = __uuidof(IPointerDevice);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPointerDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDevice2[] = L"Windows.Devices.Input.IPointerDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("f8a6d2a0-c484-489f-ae3e-30d2ee1ffd3e")
                IPointerDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPointersWithZDistance(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerDevice2 = __uuidof(IPointerDevice2);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPointerDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDeviceStatics[] = L"Windows.Devices.Input.IPointerDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("d8b89aa1-d1c6-416e-bd8d-5790914dc563")
                IPointerDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPointerDevice(
                        UINT32 pointerId,
                        ABI::Windows::Devices::Input::IPointerDevice** pointerDevice
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPointerDevices(
                        __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice** pointerDevices
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerDeviceStatics = __uuidof(IPointerDeviceStatics);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.ITouchCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.TouchCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_ITouchCapabilities[] = L"Windows.Devices.Input.ITouchCapabilities";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Input {
                MIDL_INTERFACE("20dd55f9-13f1-46c8-9285-2c05fa3eda6f")
                ITouchCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TouchPresent(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contacts(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITouchCapabilities = __uuidof(ITouchCapabilities);
            } /* Input */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CITouchCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.KeyboardCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IKeyboardCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_KeyboardCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_KeyboardCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_KeyboardCapabilities[] = L"Windows.Devices.Input.KeyboardCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseCapabilities[] = L"Windows.Devices.Input.MouseCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IMouseDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseDevice[] = L"Windows.Devices.Input.MouseDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseEventArgs[] = L"Windows.Devices.Input.MouseEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.PenButtonListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenButtonListenerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenButtonListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenButtonListener_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenButtonListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenButtonListener[] = L"Windows.Devices.Input.PenButtonListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenDeviceStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDevice ** Default Interface **
 *    Windows.Devices.Input.IPenDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDevice[] = L"Windows.Devices.Input.PenDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Devices.Input.PenDockListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenDockListenerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDockListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDockListener_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDockListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDockListener[] = L"Windows.Devices.Input.PenDockListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenDockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDockedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDockedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDockedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDockedEventArgs[] = L"Windows.Devices.Input.PenDockedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonClickedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonClickedEventArgs[] = L"Windows.Devices.Input.PenTailButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs[] = L"Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonLongPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonLongPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonLongPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonLongPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonLongPressedEventArgs[] = L"Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenUndockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenUndockedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenUndockedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenUndockedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenUndockedEventArgs[] = L"Windows.Devices.Input.PenUndockedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PointerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPointerDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPointerDevice ** Default Interface **
 *    Windows.Devices.Input.IPointerDevice2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PointerDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PointerDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PointerDevice[] = L"Windows.Devices.Input.PointerDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.TouchCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.ITouchCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_TouchCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_TouchCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_TouchCapabilities[] = L"Windows.Devices.Input.TouchCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIMouseDevice __x_ABI_CWindows_CDevices_CInput_CIMouseDevice;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDevice __x_ABI_CWindows_CDevices_CInput_CIPenDevice;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDevice2 __x_ABI_CWindows_CDevices_CInput_CIPenDevice2;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDockListener __x_ABI_CWindows_CDevices_CInput_CIPenDockListener;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPointerDevice __x_ABI_CWindows_CDevices_CInput_CIPointerDevice;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2 __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities;

#endif // ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CInput__CPointerDevice __FIIterator_1_Windows__CDevices__CInput__CPointerDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CInput__CPointerDevice;

typedef struct __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CInput__CPointerDevice* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceVtbl;

interface __FIIterator_1_Windows__CDevices__CInput__CPointerDevice
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDevice_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CInput__CPointerDevice __FIIterable_1_Windows__CDevices__CInput__CPointerDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CInput__CPointerDevice;

typedef struct __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CInput__CPointerDevice* This,
        __FIIterator_1_Windows__CDevices__CInput__CPointerDevice** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceVtbl;

interface __FIIterable_1_Windows__CDevices__CInput__CPointerDevice
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDevice_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage;

typedef struct __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl;

interface __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage;

typedef struct __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        __FIIterator_1_Windows__CDevices__CInput__CPointerDeviceUsage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl;

interface __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CInput__CPointerDevice;

typedef struct __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDevice* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceVtbl;

interface __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CInput__CPointerDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage;

typedef struct __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        UINT32 index,
        struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl;

interface __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIMouseDevice* sender,
        __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* sender,
        __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* sender,
        __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* sender,
        __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenDockListener* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenDockListener* sender,
        __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenDockListener* sender,
        __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType;

typedef struct __x_ABI_CWindows_CDevices_CInput_CMouseDelta __x_ABI_CWindows_CDevices_CInput_CMouseDelta;

/*
 *
 * Struct Windows.Devices.Input.PointerDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType
{
    PointerDeviceType_Touch = 0,
    PointerDeviceType_Pen = 1,
    PointerDeviceType_Mouse = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
    PointerDeviceType_Touchpad = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Input.MouseDelta
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CDevices_CInput_CMouseDelta
{
    INT32 X;
    INT32 Y;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Input.PointerDeviceUsage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CDevices_CInput_CPointerDeviceUsage
{
    UINT32 UsagePage;
    UINT32 Usage;
    INT32 MinLogical;
    INT32 MaxLogical;
    INT32 MinPhysical;
    INT32 MaxPhysical;
    UINT32 Unit;
    FLOAT PhysicalMultiplier;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IKeyboardCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.KeyboardCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IKeyboardCapabilities[] = L"Windows.Devices.Input.IKeyboardCapabilities";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyboardPresent)(__x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilitiesVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_get_KeyboardPresent(This, value) \
    ((This)->lpVtbl->get_KeyboardPresent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIKeyboardCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseCapabilities[] = L"Windows.Devices.Input.IMouseCapabilities";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MousePresent)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_VerticalWheelPresent)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalWheelPresent)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SwapButtons)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NumberOfButtons)(__x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilitiesVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_get_MousePresent(This, value) \
    ((This)->lpVtbl->get_MousePresent(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_get_VerticalWheelPresent(This, value) \
    ((This)->lpVtbl->get_VerticalWheelPresent(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_get_HorizontalWheelPresent(This, value) \
    ((This)->lpVtbl->get_HorizontalWheelPresent(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_get_SwapButtons(This, value) \
    ((This)->lpVtbl->get_SwapButtons(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_get_NumberOfButtons(This, value) \
    ((This)->lpVtbl->get_NumberOfButtons(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseDevice[] = L"Windows.Devices.Input.IMouseDevice";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MouseMoved)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CMouseDevice_Windows__CDevices__CInput__CMouseEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_MouseMoved)(__x_ABI_CWindows_CDevices_CInput_CIMouseDevice* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIMouseDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_add_MouseMoved(This, handler, cookie) \
    ((This)->lpVtbl->add_MouseMoved(This, handler, cookie))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDevice_remove_MouseMoved(This, cookie) \
    ((This)->lpVtbl->remove_MouseMoved(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseDeviceStatics[] = L"Windows.Devices.Input.IMouseDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics* This,
        __x_ABI_CWindows_CDevices_CInput_CIMouseDevice** mouseDevice);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_GetForCurrentView(This, mouseDevice) \
    ((This)->lpVtbl->GetForCurrentView(This, mouseDevice))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IMouseEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.MouseEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IMouseEventArgs[] = L"Windows.Devices.Input.IMouseEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MouseDelta)(__x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs* This,
        struct __x_ABI_CWindows_CDevices_CInput_CMouseDelta* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_get_MouseDelta(This, value) \
    ((This)->lpVtbl->get_MouseDelta(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIMouseEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPenButtonListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenButtonListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenButtonListener[] = L"Windows.Devices.Input.IPenButtonListener";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_IsSupportedChanged)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsSupportedChanged)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TailButtonClicked)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonClickedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TailButtonClicked)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TailButtonDoubleClicked)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonDoubleClickedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TailButtonDoubleClicked)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TailButtonLongPressed)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenButtonListener_Windows__CDevices__CInput__CPenTailButtonLongPressedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TailButtonLongPressed)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListener* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_add_IsSupportedChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsSupportedChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_remove_IsSupportedChanged(This, token) \
    ((This)->lpVtbl->remove_IsSupportedChanged(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_add_TailButtonClicked(This, handler, token) \
    ((This)->lpVtbl->add_TailButtonClicked(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_remove_TailButtonClicked(This, token) \
    ((This)->lpVtbl->remove_TailButtonClicked(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_add_TailButtonDoubleClicked(This, handler, token) \
    ((This)->lpVtbl->add_TailButtonDoubleClicked(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_remove_TailButtonDoubleClicked(This, token) \
    ((This)->lpVtbl->remove_TailButtonDoubleClicked(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_add_TailButtonLongPressed(This, handler, token) \
    ((This)->lpVtbl->add_TailButtonLongPressed(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_remove_TailButtonLongPressed(This, token) \
    ((This)->lpVtbl->remove_TailButtonLongPressed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenButtonListener;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenButtonListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenButtonListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenButtonListenerStatics[] = L"Windows.Devices.Input.IPenButtonListenerStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenButtonListener** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenButtonListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDevice[] = L"Windows.Devices.Input.IPenDevice";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PenId)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice_get_PenId(This, value) \
    ((This)->lpVtbl->get_PenId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Input.IPenDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDevice2[] = L"Windows.Devices.Input.IPenDevice2";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CDevices_CInput_CIPenDevice2* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDevice2_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Input.IPenDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDeviceStatics[] = L"Windows.Devices.Input.IPenDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFromPointerId)(__x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics* This,
        UINT32 pointerId,
        __x_ABI_CWindows_CDevices_CInput_CIPenDevice** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_GetFromPointerId(This, pointerId, result) \
    ((This)->lpVtbl->GetFromPointerId(This, pointerId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Devices.Input.IPenDockListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockListener[] = L"Windows.Devices.Input.IPenDockListener";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_IsSupportedChanged)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsSupportedChanged)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Docked)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenDockedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Docked)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Undocked)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        __FITypedEventHandler_2_Windows__CDevices__CInput__CPenDockListener_Windows__CDevices__CInput__CPenUndockedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Undocked)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListener* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDockListener
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_add_IsSupportedChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsSupportedChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_remove_IsSupportedChanged(This, token) \
    ((This)->lpVtbl->remove_IsSupportedChanged(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_add_Docked(This, handler, token) \
    ((This)->lpVtbl->add_Docked(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_remove_Docked(This, token) \
    ((This)->lpVtbl->remove_Docked(This, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_add_Undocked(This, handler, token) \
    ((This)->lpVtbl->add_Undocked(This, handler, token))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListener_remove_Undocked(This, token) \
    ((This)->lpVtbl->remove_Undocked(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockListener;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDockListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockListenerStatics[] = L"Windows.Devices.Input.IPenDockListenerStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics* This,
        __x_ABI_CWindows_CDevices_CInput_CIPenDockListener** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenDockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenDockedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenDockedEventArgs[] = L"Windows.Devices.Input.IPenDockedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenDockedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonClickedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonClickedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonDoubleClickedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonDoubleClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenTailButtonLongPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenTailButtonLongPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenTailButtonLongPressedEventArgs[] = L"Windows.Devices.Input.IPenTailButtonLongPressedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenTailButtonLongPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPenUndockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PenUndockedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPenUndockedEventArgs[] = L"Windows.Devices.Input.IPenUndockedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPenUndockedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Devices.Input.IPointerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDevice[] = L"Windows.Devices.Input.IPointerDevice";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerDeviceType)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        enum __x_ABI_CWindows_CDevices_CInput_CPointerDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_IsIntegrated)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxContacts)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhysicalDeviceRect)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_ScreenRect)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedUsages)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice* This,
        __FIVectorView_1_Windows__CDevices__CInput__CPointerDeviceUsage** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPointerDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_PointerDeviceType(This, value) \
    ((This)->lpVtbl->get_PointerDeviceType(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_IsIntegrated(This, value) \
    ((This)->lpVtbl->get_IsIntegrated(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_MaxContacts(This, value) \
    ((This)->lpVtbl->get_MaxContacts(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_PhysicalDeviceRect(This, value) \
    ((This)->lpVtbl->get_PhysicalDeviceRect(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_ScreenRect(This, value) \
    ((This)->lpVtbl->get_ScreenRect(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice_get_SupportedUsages(This, value) \
    ((This)->lpVtbl->get_SupportedUsages(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPointerDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDevice2[] = L"Windows.Devices.Input.IPointerDevice2";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPointersWithZDistance)(__x_ABI_CWindows_CDevices_CInput_CIPointerDevice2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_get_MaxPointersWithZDistance(This, value) \
    ((This)->lpVtbl->get_MaxPointersWithZDistance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.IPointerDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.PointerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_IPointerDeviceStatics[] = L"Windows.Devices.Input.IPointerDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPointerDevice)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        UINT32 pointerId,
        __x_ABI_CWindows_CDevices_CInput_CIPointerDevice** pointerDevice);
    HRESULT (STDMETHODCALLTYPE* GetPointerDevices)(__x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics* This,
        __FIVectorView_1_Windows__CDevices__CInput__CPointerDevice** pointerDevices);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_GetPointerDevice(This, pointerId, pointerDevice) \
    ((This)->lpVtbl->GetPointerDevice(This, pointerId, pointerDevice))

#define __x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_GetPointerDevices(This, pointerDevices) \
    ((This)->lpVtbl->GetPointerDevices(This, pointerDevices))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CIPointerDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Input.ITouchCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Input.TouchCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Input_ITouchCapabilities[] = L"Windows.Devices.Input.ITouchCapabilities";
typedef struct __x_ABI_CWindows_CDevices_CInput_CITouchCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TouchPresent)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Contacts)(__x_ABI_CWindows_CDevices_CInput_CITouchCapabilities* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CInput_CITouchCapabilitiesVtbl;

interface __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CInput_CITouchCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_get_TouchPresent(This, value) \
    ((This)->lpVtbl->get_TouchPresent(This, value))

#define __x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_get_Contacts(This, value) \
    ((This)->lpVtbl->get_Contacts(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CInput_CITouchCapabilities;
#endif /* !defined(____x_ABI_CWindows_CDevices_CInput_CITouchCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.KeyboardCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IKeyboardCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_KeyboardCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_KeyboardCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_KeyboardCapabilities[] = L"Windows.Devices.Input.KeyboardCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseCapabilities[] = L"Windows.Devices.Input.MouseCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IMouseDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseDevice[] = L"Windows.Devices.Input.MouseDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.MouseEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IMouseEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_MouseEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_MouseEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_MouseEventArgs[] = L"Windows.Devices.Input.MouseEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.PenButtonListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenButtonListenerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenButtonListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenButtonListener_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenButtonListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenButtonListener[] = L"Windows.Devices.Input.PenButtonListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenDeviceStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDevice ** Default Interface **
 *    Windows.Devices.Input.IPenDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDevice[] = L"Windows.Devices.Input.PenDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Devices.Input.PenDockListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPenDockListenerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDockListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDockListener_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDockListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDockListener[] = L"Windows.Devices.Input.PenDockListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenDockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenDockedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenDockedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenDockedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenDockedEventArgs[] = L"Windows.Devices.Input.PenDockedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonClickedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonClickedEventArgs[] = L"Windows.Devices.Input.PenTailButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonDoubleClickedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonDoubleClickedEventArgs[] = L"Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenTailButtonLongPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenTailButtonLongPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenTailButtonLongPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenTailButtonLongPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenTailButtonLongPressedEventArgs[] = L"Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PenUndockedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPenUndockedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PenUndockedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PenUndockedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PenUndockedEventArgs[] = L"Windows.Devices.Input.PenUndockedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Devices.Input.PointerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Input.IPointerDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.IPointerDevice ** Default Interface **
 *    Windows.Devices.Input.IPointerDevice2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_PointerDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_PointerDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_PointerDevice[] = L"Windows.Devices.Input.PointerDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Input.TouchCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Input.ITouchCapabilities ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Input_TouchCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Devices_Input_TouchCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Input_TouchCapabilities[] = L"Windows.Devices.Input.TouchCapabilities";
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
#endif // __windows2Edevices2Einput_p_h__

#endif // __windows2Edevices2Einput_h__
