
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
#ifndef __windows2Edevices2Ehaptics_h__
#define __windows2Edevices2Ehaptics_h__
#ifndef __windows2Edevices2Ehaptics_p_h__
#define __windows2Edevices2Ehaptics_p_h__


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
#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IInputHapticsManager;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager ABI::Windows::Devices::Haptics::IInputHapticsManager

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IInputHapticsManagerStatics;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics ABI::Windows::Devices::Haptics::IInputHapticsManagerStatics

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IKnownSimpleHapticsControllerWaveformsStatics;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics ABI::Windows::Devices::Haptics::IKnownSimpleHapticsControllerWaveformsStatics

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IKnownSimpleHapticsControllerWaveformsStatics2;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2 ABI::Windows::Devices::Haptics::IKnownSimpleHapticsControllerWaveformsStatics2

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface ISimpleHapticsControllerFeedback;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IVibrationDevice;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice ABI::Windows::Devices::Haptics::IVibrationDevice

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                interface IVibrationDeviceStatics;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics ABI::Windows::Devices::Haptics::IVibrationDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                typedef enum VibrationAccessStatus : int VibrationAccessStatus;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("076b2611-5614-55a5-9c58-f9d17a8f0b79"))
IAsyncOperation<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Haptics.VibrationAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a38b59db-4ef1-5bd2-89ef-f1d9f1faca96"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Haptics.VibrationAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Haptics::VibrationAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class VibrationDevice;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44193494-e331-50ca-bb61-6a71bd9b01c4"))
IAsyncOperation<ABI::Windows::Devices::Haptics::VibrationDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::VibrationDevice*, ABI::Windows::Devices::Haptics::IVibrationDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Haptics.VibrationDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Haptics::VibrationDevice*> __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4e22a135-f59a-546d-9fcf-82deb833d968"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Haptics::VibrationDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::VibrationDevice*, ABI::Windows::Devices::Haptics::IVibrationDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Haptics.VibrationDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Haptics::VibrationDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("24e9b323-eef1-533f-ad38-de8fc8ca5692"))
IIterator<ABI::Windows::Devices::Haptics::VibrationDevice*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::VibrationDevice*, ABI::Windows::Devices::Haptics::IVibrationDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Haptics.VibrationDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Haptics::VibrationDevice*> __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1a40c994-8810-5688-9362-c4bb51018552"))
IIterable<ABI::Windows::Devices::Haptics::VibrationDevice*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::VibrationDevice*, ABI::Windows::Devices::Haptics::IVibrationDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Haptics.VibrationDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Haptics::VibrationDevice*> __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("485aa8a6-2d29-5d34-b8d9-b0c961c17f7f"))
IVectorView<ABI::Windows::Devices::Haptics::VibrationDevice*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::VibrationDevice*, ABI::Windows::Devices::Haptics::IVibrationDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Haptics.VibrationDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Haptics::VibrationDevice*> __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bda8b138-7862-59f3-bfd9-5f1cb063df02"))
IAsyncOperation<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Haptics.VibrationDevice>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*> __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("096f6389-6757-56df-af12-cfe1d8f23fc1"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Haptics.VibrationDevice>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class SimpleHapticsControllerFeedback;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#define DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b7d297d6-9666-5c9e-9dcc-5c382eae6750"))
IIterator<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*, ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Haptics.SimpleHapticsControllerFeedback>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t;
#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#define DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8894a0df-33b0-57b0-aa1a-9255eee72dd5"))
IIterable<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*, ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Haptics.SimpleHapticsControllerFeedback>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t;
#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#define DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("51f54b04-bb9d-5c7b-8f5f-67f8caf4b003"))
IVectorView<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*, ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Haptics.SimpleHapticsControllerFeedback>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Haptics::SimpleHapticsControllerFeedback*> __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t;
#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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
            namespace Haptics {
                typedef enum HapticDeviceType : int HapticDeviceType;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                typedef struct HapticsControllerOverrideToken HapticsControllerOverrideToken;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class InputHapticsManager;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                class SimpleHapticsController;
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Haptics.HapticDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                enum HapticDeviceType : int
                {
                    HapticDeviceType_None = 0,
                    HapticDeviceType_Generic = 1,
                    HapticDeviceType_Pen = 2,
                    HapticDeviceType_Touchpad = 3,
                    HapticDeviceType_Mouse = 4,
                };
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Haptics.VibrationAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                enum VibrationAccessStatus : int
                {
                    VibrationAccessStatus_Allowed = 0,
                    VibrationAccessStatus_DeniedByUser = 1,
                    VibrationAccessStatus_DeniedBySystem = 2,
                    VibrationAccessStatus_DeniedByEnergySaver = 3,
                };
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Devices.Haptics.HapticsControllerOverrideToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                struct HapticsControllerOverrideToken
                {
                    INT64 Value;
                };
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IInputHapticsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.InputHapticsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IInputHapticsManager[] = L"Windows.Devices.Haptics.IInputHapticsManager";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("040e91df-bb3a-507c-9e25-a2d2c685b2e5")
                IInputHapticsManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ThreadId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentHapticsControllerDeviceType(
                        ABI::Windows::Devices::Haptics::HapticDeviceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendHapticWaveform(
                        UINT16 waveform,
                        UINT16 waveformFallback,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendHapticWaveformWithIntensity(
                        UINT16 waveform,
                        UINT16 waveformFallback,
                        DOUBLE intensity,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendHapticWaveformForDuration(
                        UINT16 waveform,
                        UINT16 waveformFallback,
                        DOUBLE intensity,
                        ABI::Windows::Foundation::TimeSpan playDuration,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendHapticWaveformForPlayCount(
                        UINT16 waveform,
                        UINT16 waveformFallback,
                        DOUBLE intensity,
                        INT32 playCount,
                        ABI::Windows::Foundation::TimeSpan replayPauseInterval,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryStopFeedback(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetOverrideHapticsController(
                        ABI::Windows::Devices::Haptics::HapticDeviceType deviceType,
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController* controller,
                        ABI::Windows::Devices::Haptics::HapticsControllerOverrideToken* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearOverrideHapticsController(
                        ABI::Windows::Devices::Haptics::HapticsControllerOverrideToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputHapticsManager = __uuidof(IInputHapticsManager);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IInputHapticsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.InputHapticsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IInputHapticsManagerStatics[] = L"Windows.Devices.Haptics.IInputHapticsManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("7bb40f77-e187-5322-844e-aa58223c281a")
                IInputHapticsManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsHapticDevicePresent(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentThread(
                        ABI::Windows::Devices::Haptics::IInputHapticsManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetForThread(
                        UINT32 ThreadId,
                        ABI::Windows::Devices::Haptics::IInputHapticsManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputHapticsManagerStatics = __uuidof(IInputHapticsManagerStatics);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IKnownSimpleHapticsControllerWaveformsStatics[] = L"Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("3d577ef7-4cee-11e6-b535-001bdc06ab3b")
                IKnownSimpleHapticsControllerWaveformsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Click(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BuzzContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RumbleContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Press(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Release(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownSimpleHapticsControllerWaveformsStatics = __uuidof(IKnownSimpleHapticsControllerWaveformsStatics);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IKnownSimpleHapticsControllerWaveformsStatics2[] = L"Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("a7d24c27-b79d-510a-bf79-ff6d49173e1d")
                IKnownSimpleHapticsControllerWaveformsStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BrushContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChiselMarkerContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EraserContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Error(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GalaxyPenContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hover(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InkContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MarkerContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PencilContinuous(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Success(
                        UINT16* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownSimpleHapticsControllerWaveformsStatics2 = __uuidof(IKnownSimpleHapticsControllerWaveformsStatics2);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Haptics.ISimpleHapticsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.SimpleHapticsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_ISimpleHapticsController[] = L"Windows.Devices.Haptics.ISimpleHapticsController";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("3d577ef9-4cee-11e6-b535-001bdc06ab3b")
                ISimpleHapticsController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedFeedback(
                        __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIntensitySupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPlayCountSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPlayDurationSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReplayPauseIntervalSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopFeedback(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendHapticFeedback(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback* feedback
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendHapticFeedbackWithIntensity(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback* feedback,
                        DOUBLE intensity
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendHapticFeedbackForDuration(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback* feedback,
                        DOUBLE intensity,
                        ABI::Windows::Foundation::TimeSpan playDuration
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendHapticFeedbackForPlayCount(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsControllerFeedback* feedback,
                        DOUBLE intensity,
                        INT32 playCount,
                        ABI::Windows::Foundation::TimeSpan replayPauseInterval
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISimpleHapticsController = __uuidof(ISimpleHapticsController);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.ISimpleHapticsControllerFeedback
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.SimpleHapticsControllerFeedback
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_ISimpleHapticsControllerFeedback[] = L"Windows.Devices.Haptics.ISimpleHapticsControllerFeedback";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("3d577ef8-4cee-11e6-b535-001bdc06ab3b")
                ISimpleHapticsControllerFeedback : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Waveform(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISimpleHapticsControllerFeedback = __uuidof(ISimpleHapticsControllerFeedback);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IVibrationDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.VibrationDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IVibrationDevice[] = L"Windows.Devices.Haptics.IVibrationDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("40f21a3e-8844-47ff-b312-06185a3844da")
                IVibrationDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimpleHapticsController(
                        ABI::Windows::Devices::Haptics::ISimpleHapticsController** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVibrationDevice = __uuidof(IVibrationDevice);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IVibrationDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.VibrationDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IVibrationDeviceStatics[] = L"Windows.Devices.Haptics.IVibrationDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Haptics {
                MIDL_INTERFACE("53e2eded-2290-4ac9-8eb3-1a84122eb71c")
                IVibrationDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVibrationDeviceStatics = __uuidof(IVibrationDeviceStatics);
            } /* Haptics */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.InputHapticsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IInputHapticsManagerStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.IInputHapticsManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_InputHapticsManager_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_InputHapticsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_InputHapticsManager[] = L"Windows.Devices.Haptics.InputHapticsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2 interface starting with version 14.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms[] = L"Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.SimpleHapticsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.ISimpleHapticsController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_SimpleHapticsController[] = L"Windows.Devices.Haptics.SimpleHapticsController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.SimpleHapticsControllerFeedback
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.ISimpleHapticsControllerFeedback ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsControllerFeedback_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsControllerFeedback_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_SimpleHapticsControllerFeedback[] = L"Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.VibrationDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IVibrationDeviceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.IVibrationDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_VibrationDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_VibrationDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_VibrationDevice[] = L"Windows.Devices.Haptics.VibrationDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2 __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CDevices_CHaptics_CVibrationAccessStatus __x_ABI_CWindows_CDevices_CHaptics_CVibrationAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CHaptics_CVibrationAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIIterator_1_Windows__CDevices__CHaptics__CVibrationDevice** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

typedef struct __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl;

interface __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

typedef struct __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        __FIIterator_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl;

interface __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback;

typedef struct __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl;

interface __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CDevices_CHaptics_CHapticDeviceType __x_ABI_CWindows_CDevices_CHaptics_CHapticDeviceType;

typedef struct __x_ABI_CWindows_CDevices_CHaptics_CHapticsControllerOverrideToken __x_ABI_CWindows_CDevices_CHaptics_CHapticsControllerOverrideToken;

/*
 *
 * Struct Windows.Devices.Haptics.HapticDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CDevices_CHaptics_CHapticDeviceType
{
    HapticDeviceType_None = 0,
    HapticDeviceType_Generic = 1,
    HapticDeviceType_Pen = 2,
    HapticDeviceType_Touchpad = 3,
    HapticDeviceType_Mouse = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Devices.Haptics.VibrationAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CDevices_CHaptics_CVibrationAccessStatus
{
    VibrationAccessStatus_Allowed = 0,
    VibrationAccessStatus_DeniedByUser = 1,
    VibrationAccessStatus_DeniedBySystem = 2,
    VibrationAccessStatus_DeniedByEnergySaver = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Devices.Haptics.HapticsControllerOverrideToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
struct __x_ABI_CWindows_CDevices_CHaptics_CHapticsControllerOverrideToken
{
    INT64 Value;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IInputHapticsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.InputHapticsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IInputHapticsManager[] = L"Windows.Devices.Haptics.IInputHapticsManager";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ThreadId)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentHapticsControllerDeviceType)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        enum __x_ABI_CWindows_CDevices_CHaptics_CHapticDeviceType* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentHapticsController)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);
    HRESULT (STDMETHODCALLTYPE* TrySendHapticWaveform)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        UINT16 waveform,
        UINT16 waveformFallback,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TrySendHapticWaveformWithIntensity)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        UINT16 waveform,
        UINT16 waveformFallback,
        DOUBLE intensity,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TrySendHapticWaveformForDuration)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        UINT16 waveform,
        UINT16 waveformFallback,
        DOUBLE intensity,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan playDuration,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TrySendHapticWaveformForPlayCount)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        UINT16 waveform,
        UINT16 waveformFallback,
        DOUBLE intensity,
        INT32 playCount,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan replayPauseInterval,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryStopFeedback)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetOverrideHapticsController)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        enum __x_ABI_CWindows_CDevices_CHaptics_CHapticDeviceType deviceType,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* controller,
        struct __x_ABI_CWindows_CDevices_CHaptics_CHapticsControllerOverrideToken* result);
    HRESULT (STDMETHODCALLTYPE* ClearOverrideHapticsController)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager* This,
        struct __x_ABI_CWindows_CDevices_CHaptics_CHapticsControllerOverrideToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_get_ThreadId(This, value) \
    ((This)->lpVtbl->get_ThreadId(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_get_CurrentHapticsControllerDeviceType(This, value) \
    ((This)->lpVtbl->get_CurrentHapticsControllerDeviceType(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_get_CurrentHapticsController(This, value) \
    ((This)->lpVtbl->get_CurrentHapticsController(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_TrySendHapticWaveform(This, waveform, waveformFallback, result) \
    ((This)->lpVtbl->TrySendHapticWaveform(This, waveform, waveformFallback, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_TrySendHapticWaveformWithIntensity(This, waveform, waveformFallback, intensity, result) \
    ((This)->lpVtbl->TrySendHapticWaveformWithIntensity(This, waveform, waveformFallback, intensity, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_TrySendHapticWaveformForDuration(This, waveform, waveformFallback, intensity, playDuration, result) \
    ((This)->lpVtbl->TrySendHapticWaveformForDuration(This, waveform, waveformFallback, intensity, playDuration, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_TrySendHapticWaveformForPlayCount(This, waveform, waveformFallback, intensity, playCount, replayPauseInterval, result) \
    ((This)->lpVtbl->TrySendHapticWaveformForPlayCount(This, waveform, waveformFallback, intensity, playCount, replayPauseInterval, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_TryStopFeedback(This, result) \
    ((This)->lpVtbl->TryStopFeedback(This, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_SetOverrideHapticsController(This, deviceType, controller, result) \
    ((This)->lpVtbl->SetOverrideHapticsController(This, deviceType, controller, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_ClearOverrideHapticsController(This, token) \
    ((This)->lpVtbl->ClearOverrideHapticsController(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IInputHapticsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.InputHapticsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IInputHapticsManagerStatics[] = L"Windows.Devices.Haptics.IInputHapticsManagerStatics";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsHapticDevicePresent)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentThread)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager** result);
    HRESULT (STDMETHODCALLTYPE* TryGetForThread)(__x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics* This,
        UINT32 ThreadId,
        __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_IsHapticDevicePresent(This, result) \
    ((This)->lpVtbl->IsHapticDevicePresent(This, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_GetForCurrentThread(This, result) \
    ((This)->lpVtbl->GetForCurrentThread(This, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_TryGetForThread(This, ThreadId, result) \
    ((This)->lpVtbl->TryGetForThread(This, ThreadId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIInputHapticsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IKnownSimpleHapticsControllerWaveformsStatics[] = L"Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Click)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_BuzzContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_RumbleContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Press)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Release)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_get_Click(This, value) \
    ((This)->lpVtbl->get_Click(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_get_BuzzContinuous(This, value) \
    ((This)->lpVtbl->get_BuzzContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_get_RumbleContinuous(This, value) \
    ((This)->lpVtbl->get_RumbleContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_get_Press(This, value) \
    ((This)->lpVtbl->get_Press(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_get_Release(This, value) \
    ((This)->lpVtbl->get_Release(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IKnownSimpleHapticsControllerWaveformsStatics2[] = L"Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BrushContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_ChiselMarkerContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_EraserContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_GalaxyPenContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Hover)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_InkContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_MarkerContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_PencilContinuous)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Success)(__x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2* This,
        UINT16* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_BrushContinuous(This, value) \
    ((This)->lpVtbl->get_BrushContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_ChiselMarkerContinuous(This, value) \
    ((This)->lpVtbl->get_ChiselMarkerContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_EraserContinuous(This, value) \
    ((This)->lpVtbl->get_EraserContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_GalaxyPenContinuous(This, value) \
    ((This)->lpVtbl->get_GalaxyPenContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_Hover(This, value) \
    ((This)->lpVtbl->get_Hover(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_InkContinuous(This, value) \
    ((This)->lpVtbl->get_InkContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_MarkerContinuous(This, value) \
    ((This)->lpVtbl->get_MarkerContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_PencilContinuous(This, value) \
    ((This)->lpVtbl->get_PencilContinuous(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_get_Success(This, value) \
    ((This)->lpVtbl->get_Success(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIKnownSimpleHapticsControllerWaveformsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.Devices.Haptics.ISimpleHapticsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.SimpleHapticsController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_ISimpleHapticsController[] = L"Windows.Devices.Haptics.ISimpleHapticsController";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFeedback)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        __FIVectorView_1_Windows__CDevices__CHaptics__CSimpleHapticsControllerFeedback** value);
    HRESULT (STDMETHODCALLTYPE* get_IsIntensitySupported)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPlayCountSupported)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPlayDurationSupported)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsReplayPauseIntervalSupported)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* StopFeedback)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This);
    HRESULT (STDMETHODCALLTYPE* SendHapticFeedback)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* feedback);
    HRESULT (STDMETHODCALLTYPE* SendHapticFeedbackWithIntensity)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* feedback,
        DOUBLE intensity);
    HRESULT (STDMETHODCALLTYPE* SendHapticFeedbackForDuration)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* feedback,
        DOUBLE intensity,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan playDuration);
    HRESULT (STDMETHODCALLTYPE* SendHapticFeedbackForPlayCount)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* feedback,
        DOUBLE intensity,
        INT32 playCount,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan replayPauseInterval);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_SupportedFeedback(This, value) \
    ((This)->lpVtbl->get_SupportedFeedback(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_IsIntensitySupported(This, value) \
    ((This)->lpVtbl->get_IsIntensitySupported(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_IsPlayCountSupported(This, value) \
    ((This)->lpVtbl->get_IsPlayCountSupported(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_IsPlayDurationSupported(This, value) \
    ((This)->lpVtbl->get_IsPlayDurationSupported(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_get_IsReplayPauseIntervalSupported(This, value) \
    ((This)->lpVtbl->get_IsReplayPauseIntervalSupported(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_StopFeedback(This) \
    ((This)->lpVtbl->StopFeedback(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_SendHapticFeedback(This, feedback) \
    ((This)->lpVtbl->SendHapticFeedback(This, feedback))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_SendHapticFeedbackWithIntensity(This, feedback, intensity) \
    ((This)->lpVtbl->SendHapticFeedbackWithIntensity(This, feedback, intensity))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_SendHapticFeedbackForDuration(This, feedback, intensity, playDuration) \
    ((This)->lpVtbl->SendHapticFeedbackForDuration(This, feedback, intensity, playDuration))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_SendHapticFeedbackForPlayCount(This, feedback, intensity, playCount, replayPauseInterval) \
    ((This)->lpVtbl->SendHapticFeedbackForPlayCount(This, feedback, intensity, playCount, replayPauseInterval))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.ISimpleHapticsControllerFeedback
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.SimpleHapticsControllerFeedback
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_ISimpleHapticsControllerFeedback[] = L"Windows.Devices.Haptics.ISimpleHapticsControllerFeedback";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Waveform)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedbackVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_get_Waveform(This, value) \
    ((This)->lpVtbl->get_Waveform(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsControllerFeedback_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IVibrationDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.VibrationDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IVibrationDevice[] = L"Windows.Devices.Haptics.IVibrationDevice";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SimpleHapticsController)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice* This,
        __x_ABI_CWindows_CDevices_CHaptics_CISimpleHapticsController** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_get_SimpleHapticsController(This, value) \
    ((This)->lpVtbl->get_SimpleHapticsController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Devices.Haptics.IVibrationDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Haptics.VibrationDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Haptics_IVibrationDeviceStatics[] = L"Windows.Devices.Haptics.IVibrationDeviceStatics";
typedef struct __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice** operation);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CHaptics__CVibrationDevice** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CDevices__CHaptics__CVibrationDevice** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_FindAllAsync(This, operation) \
    ((This)->lpVtbl->FindAllAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CHaptics_CIVibrationDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.InputHapticsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IInputHapticsManagerStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.IInputHapticsManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_InputHapticsManager_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_InputHapticsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_InputHapticsManager[] = L"Windows.Devices.Haptics.InputHapticsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics2 interface starting with version 14.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Devices.Haptics.IKnownSimpleHapticsControllerWaveformsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_KnownSimpleHapticsControllerWaveforms[] = L"Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.SimpleHapticsController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.ISimpleHapticsController ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsController_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_SimpleHapticsController[] = L"Windows.Devices.Haptics.SimpleHapticsController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.SimpleHapticsControllerFeedback
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.ISimpleHapticsControllerFeedback ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsControllerFeedback_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_SimpleHapticsControllerFeedback_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_SimpleHapticsControllerFeedback[] = L"Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Devices.Haptics.VibrationDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Haptics.IVibrationDeviceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Haptics.IVibrationDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Devices_Haptics_VibrationDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Haptics_VibrationDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Haptics_VibrationDevice[] = L"Windows.Devices.Haptics.VibrationDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Ehaptics_p_h__

#endif // __windows2Edevices2Ehaptics_h__
