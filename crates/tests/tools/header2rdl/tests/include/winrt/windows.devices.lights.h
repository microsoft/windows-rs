
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
#ifndef __windows2Edevices2Elights_h__
#define __windows2Edevices2Elights_h__
#ifndef __windows2Edevices2Elights_p_h__
#define __windows2Edevices2Elights_p_h__


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

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILamp;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILamp ABI::Windows::Devices::Lights::ILamp

#endif // ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampArray;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampArray ABI::Windows::Devices::Lights::ILampArray

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampArray2;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampArray2 ABI::Windows::Devices::Lights::ILampArray2

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampArrayStatics;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics ABI::Windows::Devices::Lights::ILampArrayStatics

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampAvailabilityChangedEventArgs;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs ABI::Windows::Devices::Lights::ILampAvailabilityChangedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampInfo;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampInfo ABI::Windows::Devices::Lights::ILampInfo

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampStatics;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampStatics ABI::Windows::Devices::Lights::ILampStatics

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                class Lamp;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("52a69dfd-f0d6-5931-b8e1-f38066d71bf2"))
IAsyncOperation<ABI::Windows::Devices::Lights::Lamp*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Lamp*, ABI::Windows::Devices::Lights::ILamp*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Lights.Lamp>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Lights::Lamp*> __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_t;
#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("191a8c6e-60dd-5a21-a53c-bf3f940a1dde"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Lights::Lamp*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Lamp*, ABI::Windows::Devices::Lights::ILamp*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Lights.Lamp>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Lights::Lamp*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                class LampArray;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e9a9900-6eb1-5db1-b778-9a64a16542f8"))
IAsyncOperation<ABI::Windows::Devices::Lights::LampArray*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::LampArray*, ABI::Windows::Devices::Lights::ILampArray*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Lights.LampArray>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Lights::LampArray*> __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_t;
#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eba415db-42af-54b8-a2f3-5b34494c8972"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Lights::LampArray*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::LampArray*, ABI::Windows::Devices::Lights::ILampArray*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Lights.LampArray>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Lights::LampArray*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bee8834-b9a7-5a80-a746-5ef097227878"))
IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c3d2fd-b8a1-5620-b746-7ee6d533aca3"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CUI__CColor_USE
#define DEF___FIReference_1_Windows__CUI__CColor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab8e5d11-b0c1-5a21-95ae-f16bf3a37624"))
IReference<struct ABI::Windows::UI::Color> : IReference_impl<struct ABI::Windows::UI::Color>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.UI.Color>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::UI::Color> __FIReference_1_Windows__CUI__CColor_t;
#define __FIReference_1_Windows__CUI__CColor ABI::Windows::Foundation::__FIReference_1_Windows__CUI__CColor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CUI__CColor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                class LampAvailabilityChangedEventArgs;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("556a02d9-7685-576f-89ca-b62dc481d29d"))
ITypedEventHandler<ABI::Windows::Devices::Lights::Lamp*, ABI::Windows::Devices::Lights::LampAvailabilityChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Lamp*, ABI::Windows::Devices::Lights::ILamp*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::LampAvailabilityChangedEventArgs*, ABI::Windows::Devices::Lights::ILampAvailabilityChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Lights.Lamp, Windows.Devices.Lights.LampAvailabilityChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Lights::Lamp*, ABI::Windows::Devices::Lights::LampAvailabilityChangedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1229c9f2-783e-5bc8-9a1e-772e45e66401"))
ITypedEventHandler<ABI::Windows::Devices::Lights::LampArray*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::LampArray*, ABI::Windows::Devices::Lights::ILampArray*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Lights.LampArray, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Lights::LampArray*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            typedef enum VirtualKey : int VirtualKey;
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                typedef enum LampArrayKind : int LampArrayKind;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                typedef enum LampPurposes : unsigned int LampPurposes;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                class LampInfo;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Lights.LampArrayKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                enum LampArrayKind : int
                {
                    LampArrayKind_Undefined = 0,
                    LampArrayKind_Keyboard = 1,
                    LampArrayKind_Mouse = 2,
                    LampArrayKind_GameController = 3,
                    LampArrayKind_Peripheral = 4,
                    LampArrayKind_Scene = 5,
                    LampArrayKind_Notification = 6,
                    LampArrayKind_Chassis = 7,
                    LampArrayKind_Wearable = 8,
                    LampArrayKind_Furniture = 9,
                    LampArrayKind_Art = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
                    LampArrayKind_Headset = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    LampArrayKind_Microphone = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    LampArrayKind_Speaker = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                };
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.LampPurposes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                enum LampPurposes : unsigned int
                {
                    LampPurposes_Undefined = 0,
                    LampPurposes_Control = 0x1,
                    LampPurposes_Accent = 0x2,
                    LampPurposes_Branding = 0x4,
                    LampPurposes_Status = 0x8,
                    LampPurposes_Illumination = 0x10,
                    LampPurposes_Presentation = 0x20,
                };

                DEFINE_ENUM_FLAG_OPERATORS(LampPurposes)
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Lamp
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILamp[] = L"Windows.Devices.Lights.ILamp";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("047d5b9a-ea45-4b2b-b1a2-14dff00bde7b")
                ILamp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BrightnessLevel(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BrightnessLevel(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsColorSettable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Color(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Color(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AvailabilityChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AvailabilityChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILamp = __uuidof(ILamp);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILamp;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Lights.ILampArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArray[] = L"Windows.Devices.Lights.ILampArray";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("7ace9787-c8a0-4e95-a1e0-d58676538649")
                ILampArray : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareVendorId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareProductId(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HardwareVersion(
                        UINT16* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LampArrayKind(
                        ABI::Windows::Devices::Lights::LampArrayKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LampCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinUpdateInterval(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BrightnessLevel(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BrightnessLevel(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsConnected(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsVirtualKeys(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetLampInfo(
                        INT32 lampIndex,
                        ABI::Windows::Devices::Lights::ILampInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetIndicesForKey(
                        ABI::Windows::System::VirtualKey key,
                        UINT32* resultLength,
                        INT32** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetIndicesForPurposes(
                        ABI::Windows::Devices::Lights::LampPurposes purposes,
                        UINT32* resultLength,
                        INT32** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColor(
                        ABI::Windows::UI::Color desiredColor
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColorForIndex(
                        INT32 lampIndex,
                        ABI::Windows::UI::Color desiredColor
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSingleColorForIndices(
                        ABI::Windows::UI::Color desiredColor,
                        UINT32 lampIndexesLength,
                        INT32* lampIndexes
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColorsForIndices(
                        UINT32 desiredColorsLength,
                        ABI::Windows::UI::Color* desiredColors,
                        UINT32 lampIndexesLength,
                        INT32* lampIndexes
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColorsForKey(
                        ABI::Windows::UI::Color desiredColor,
                        ABI::Windows::System::VirtualKey key
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColorsForKeys(
                        UINT32 desiredColorsLength,
                        ABI::Windows::UI::Color* desiredColors,
                        UINT32 keysLength,
                        ABI::Windows::System::VirtualKey* keys
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetColorsForPurposes(
                        ABI::Windows::UI::Color desiredColor,
                        ABI::Windows::Devices::Lights::LampPurposes purposes
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendMessageAsync(
                        INT32 messageId,
                        ABI::Windows::Storage::Streams::IBuffer* message,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMessageAsync(
                        INT32 messageId,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampArray = __uuidof(ILampArray);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArray;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampArray2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArray2[] = L"Windows.Devices.Lights.ILampArray2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("050c181f-60a8-4711-a1af-1b1b4c658ea2")
                ILampArray2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AvailabilityChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AvailabilityChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampArray2 = __uuidof(ILampArray2);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArray2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.Lights.ILampArrayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArrayStatics[] = L"Windows.Devices.Lights.ILampArrayStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("7bb8c98d-5fc1-452d-bb1f-4ad410d398ff")
                ILampArrayStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampArrayStatics = __uuidof(ILampArrayStatics);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArrayStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampAvailabilityChangedEventArgs[] = L"Windows.Devices.Lights.ILampAvailabilityChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("4f6e3ded-07a2-499d-9260-67e304532ba4")
                ILampAvailabilityChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAvailable(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampAvailabilityChangedEventArgs = __uuidof(ILampAvailabilityChangedEventArgs);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Lights.ILampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampInfo[] = L"Windows.Devices.Lights.ILampInfo";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("30bb521c-0acf-49da-8c10-150b9cf62713")
                ILampInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Index(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Purposes(
                        ABI::Windows::Devices::Lights::LampPurposes* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RedLevelCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GreenLevelCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BlueLevelCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GainLevelCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FixedColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNearestSupportedColor(
                        ABI::Windows::UI::Color desiredColor,
                        ABI::Windows::UI::Color* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateLatency(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampInfo = __uuidof(ILampInfo);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Lamp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampStatics[] = L"Windows.Devices.Lights.ILampStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                MIDL_INTERFACE("a822416c-8885-401e-b821-8e8b38a8e8ec")
                ILampStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILampStatics = __uuidof(ILampStatics);
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.Lamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.ILampStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILamp ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Lamp_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Lamp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Lamp[] = L"Windows.Devices.Lights.Lamp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.LampArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.ILampArrayStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampArray ** Default Interface **
 *    Windows.Devices.Lights.ILampArray2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampArray_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampArray_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampArray[] = L"Windows.Devices.Lights.LampArray";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.LampAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampAvailabilityChangedEventArgs[] = L"Windows.Devices.Lights.LampAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.LampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampInfo[] = L"Windows.Devices.Lights.LampInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILamp __x_ABI_CWindows_CDevices_CLights_CILamp;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILamp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampArray __x_ABI_CWindows_CDevices_CLights_CILampArray;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampArray2 __x_ABI_CWindows_CDevices_CLights_CILampArray2;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampInfo __x_ABI_CWindows_CDevices_CLights_CILampInfo;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampStatics __x_ABI_CWindows_CDevices_CLights_CILampStatics;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CLights__CLamp;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CLights__CLampVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* This,
        __x_ABI_CWindows_CDevices_CLights_CILamp** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CLights__CLampVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CLights__CLampVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp* This,
        __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLamp_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArrayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArrayVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArrayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArrayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray* This,
        __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArrayVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArrayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CLights__CLampArray_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CUI__CColor __FIReference_1_Windows__CUI__CColor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CUI__CColor;

typedef struct __FIReference_1_Windows__CUI__CColorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CUI__CColor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CUI__CColor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CUI__CColor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CUI__CColor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CUI__CColor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CUI__CColor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CUI__CColor* This,
        struct __x_ABI_CWindows_CUI_CColor* result);

    END_INTERFACE
} __FIReference_1_Windows__CUI__CColorVtbl;

interface __FIReference_1_Windows__CUI__CColor
{
    CONST_VTBL struct __FIReference_1_Windows__CUI__CColorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CUI__CColor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CUI__CColor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CUI__CColor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CUI__CColor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CUI__CColor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CUI__CColor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CUI__CColor_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* This,
        __x_ABI_CWindows_CDevices_CLights_CILamp* sender,
        __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef enum __x_ABI_CWindows_CSystem_CVirtualKey __x_ABI_CWindows_CSystem_CVirtualKey;

typedef enum __x_ABI_CWindows_CDevices_CLights_CLampArrayKind __x_ABI_CWindows_CDevices_CLights_CLampArrayKind;

typedef enum __x_ABI_CWindows_CDevices_CLights_CLampPurposes __x_ABI_CWindows_CDevices_CLights_CLampPurposes;

/*
 *
 * Struct Windows.Devices.Lights.LampArrayKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CLights_CLampArrayKind
{
    LampArrayKind_Undefined = 0,
    LampArrayKind_Keyboard = 1,
    LampArrayKind_Mouse = 2,
    LampArrayKind_GameController = 3,
    LampArrayKind_Peripheral = 4,
    LampArrayKind_Scene = 5,
    LampArrayKind_Notification = 6,
    LampArrayKind_Chassis = 7,
    LampArrayKind_Wearable = 8,
    LampArrayKind_Furniture = 9,
    LampArrayKind_Art = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
    LampArrayKind_Headset = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    LampArrayKind_Microphone = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    LampArrayKind_Speaker = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.LampPurposes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CLights_CLampPurposes
{
    LampPurposes_Undefined = 0,
    LampPurposes_Control = 0x1,
    LampPurposes_Accent = 0x2,
    LampPurposes_Branding = 0x4,
    LampPurposes_Status = 0x8,
    LampPurposes_Illumination = 0x10,
    LampPurposes_Presentation = 0x20,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Lamp
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILamp[] = L"Windows.Devices.Lights.ILamp";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILamp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILamp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BrightnessLevel)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_BrightnessLevel)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_IsColorSettable)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* add_AvailabilityChanged)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        __FITypedEventHandler_2_Windows__CDevices__CLights__CLamp_Windows__CDevices__CLights__CLampAvailabilityChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AvailabilityChanged)(__x_ABI_CWindows_CDevices_CLights_CILamp* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILamp
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILamp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_get_BrightnessLevel(This, value) \
    ((This)->lpVtbl->get_BrightnessLevel(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_put_BrightnessLevel(This, value) \
    ((This)->lpVtbl->put_BrightnessLevel(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_get_IsColorSettable(This, value) \
    ((This)->lpVtbl->get_IsColorSettable(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_add_AvailabilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_AvailabilityChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CLights_CILamp_remove_AvailabilityChanged(This, token) \
    ((This)->lpVtbl->remove_AvailabilityChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILamp;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILamp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Lights.ILampArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArray[] = L"Windows.Devices.Lights.ILampArray";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampArrayVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareVendorId)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareProductId)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareVersion)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_LampArrayKind)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        enum __x_ABI_CWindows_CDevices_CLights_CLampArrayKind* value);
    HRESULT (STDMETHODCALLTYPE* get_LampCount)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinUpdateInterval)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BrightnessLevel)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_BrightnessLevel)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnected)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsVirtualKeys)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetLampInfo)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        INT32 lampIndex,
        __x_ABI_CWindows_CDevices_CLights_CILampInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetIndicesForKey)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey key,
        UINT32* resultLength,
        INT32** result);
    HRESULT (STDMETHODCALLTYPE* GetIndicesForPurposes)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        enum __x_ABI_CWindows_CDevices_CLights_CLampPurposes purposes,
        UINT32* resultLength,
        INT32** result);
    HRESULT (STDMETHODCALLTYPE* SetColor)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor);
    HRESULT (STDMETHODCALLTYPE* SetColorForIndex)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        INT32 lampIndex,
        struct __x_ABI_CWindows_CUI_CColor desiredColor);
    HRESULT (STDMETHODCALLTYPE* SetSingleColorForIndices)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor,
        UINT32 lampIndexesLength,
        INT32* lampIndexes);
    HRESULT (STDMETHODCALLTYPE* SetColorsForIndices)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        UINT32 desiredColorsLength,
        struct __x_ABI_CWindows_CUI_CColor* desiredColors,
        UINT32 lampIndexesLength,
        INT32* lampIndexes);
    HRESULT (STDMETHODCALLTYPE* SetColorsForKey)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor,
        enum __x_ABI_CWindows_CSystem_CVirtualKey key);
    HRESULT (STDMETHODCALLTYPE* SetColorsForKeys)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        UINT32 desiredColorsLength,
        struct __x_ABI_CWindows_CUI_CColor* desiredColors,
        UINT32 keysLength,
        enum __x_ABI_CWindows_CSystem_CVirtualKey* keys);
    HRESULT (STDMETHODCALLTYPE* SetColorsForPurposes)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor,
        enum __x_ABI_CWindows_CDevices_CLights_CLampPurposes purposes);
    HRESULT (STDMETHODCALLTYPE* SendMessageAsync)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        INT32 messageId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* message,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RequestMessageAsync)(__x_ABI_CWindows_CDevices_CLights_CILampArray* This,
        INT32 messageId,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampArrayVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampArray
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampArrayVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_HardwareVendorId(This, value) \
    ((This)->lpVtbl->get_HardwareVendorId(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_HardwareProductId(This, value) \
    ((This)->lpVtbl->get_HardwareProductId(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_HardwareVersion(This, value) \
    ((This)->lpVtbl->get_HardwareVersion(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_LampArrayKind(This, value) \
    ((This)->lpVtbl->get_LampArrayKind(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_LampCount(This, value) \
    ((This)->lpVtbl->get_LampCount(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_MinUpdateInterval(This, value) \
    ((This)->lpVtbl->get_MinUpdateInterval(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_BrightnessLevel(This, value) \
    ((This)->lpVtbl->get_BrightnessLevel(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_put_BrightnessLevel(This, value) \
    ((This)->lpVtbl->put_BrightnessLevel(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_IsConnected(This, value) \
    ((This)->lpVtbl->get_IsConnected(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_get_SupportsVirtualKeys(This, value) \
    ((This)->lpVtbl->get_SupportsVirtualKeys(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetLampInfo(This, lampIndex, result) \
    ((This)->lpVtbl->GetLampInfo(This, lampIndex, result))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetIndicesForKey(This, key, resultLength, result) \
    ((This)->lpVtbl->GetIndicesForKey(This, key, resultLength, result))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_GetIndicesForPurposes(This, purposes, resultLength, result) \
    ((This)->lpVtbl->GetIndicesForPurposes(This, purposes, resultLength, result))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColor(This, desiredColor) \
    ((This)->lpVtbl->SetColor(This, desiredColor))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColorForIndex(This, lampIndex, desiredColor) \
    ((This)->lpVtbl->SetColorForIndex(This, lampIndex, desiredColor))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetSingleColorForIndices(This, desiredColor, lampIndexesLength, lampIndexes) \
    ((This)->lpVtbl->SetSingleColorForIndices(This, desiredColor, lampIndexesLength, lampIndexes))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColorsForIndices(This, desiredColorsLength, desiredColors, lampIndexesLength, lampIndexes) \
    ((This)->lpVtbl->SetColorsForIndices(This, desiredColorsLength, desiredColors, lampIndexesLength, lampIndexes))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColorsForKey(This, desiredColor, key) \
    ((This)->lpVtbl->SetColorsForKey(This, desiredColor, key))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColorsForKeys(This, desiredColorsLength, desiredColors, keysLength, keys) \
    ((This)->lpVtbl->SetColorsForKeys(This, desiredColorsLength, desiredColors, keysLength, keys))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SetColorsForPurposes(This, desiredColor, purposes) \
    ((This)->lpVtbl->SetColorsForPurposes(This, desiredColor, purposes))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_SendMessageAsync(This, messageId, message, operation) \
    ((This)->lpVtbl->SendMessageAsync(This, messageId, message, operation))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray_RequestMessageAsync(This, messageId, operation) \
    ((This)->lpVtbl->RequestMessageAsync(This, messageId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArray;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampArray2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArray2[] = L"Windows.Devices.Lights.ILampArray2";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampArray2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailable)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_AvailabilityChanged)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        __FITypedEventHandler_2_Windows__CDevices__CLights__CLampArray_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AvailabilityChanged)(__x_ABI_CWindows_CDevices_CLights_CILampArray2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampArray2Vtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampArray2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampArray2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_get_IsAvailable(This, value) \
    ((This)->lpVtbl->get_IsAvailable(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_add_AvailabilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_AvailabilityChanged(This, handler, token))

#define __x_ABI_CWindows_CDevices_CLights_CILampArray2_remove_AvailabilityChanged(This, token) \
    ((This)->lpVtbl->remove_AvailabilityChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArray2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArray2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Devices.Lights.ILampArrayStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampArray
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampArrayStatics[] = L"Windows.Devices.Lights.ILampArrayStatics";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampArrayStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CLights_CILampArrayStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CLights__CLampArray** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampArrayStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampArrayStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampArrayStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampArrayStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampAvailabilityChangedEventArgs[] = L"Windows.Devices.Lights.ILampAvailabilityChangedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailable)(__x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_get_IsAvailable(This, value) \
    ((This)->lpVtbl->get_IsAvailable(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Lights.ILampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.LampInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampInfo[] = L"Windows.Devices.Lights.ILampInfo";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Index)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Purposes)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        enum __x_ABI_CWindows_CDevices_CLights_CLampPurposes* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* get_RedLevelCount)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_GreenLevelCount)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_BlueLevelCount)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_GainLevelCount)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FixedColor)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* GetNearestSupportedColor)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor,
        struct __x_ABI_CWindows_CUI_CColor* result);
    HRESULT (STDMETHODCALLTYPE* get_UpdateLatency)(__x_ABI_CWindows_CDevices_CLights_CILampInfo* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampInfoVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_Index(This, value) \
    ((This)->lpVtbl->get_Index(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_Purposes(This, value) \
    ((This)->lpVtbl->get_Purposes(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_RedLevelCount(This, value) \
    ((This)->lpVtbl->get_RedLevelCount(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_GreenLevelCount(This, value) \
    ((This)->lpVtbl->get_GreenLevelCount(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_BlueLevelCount(This, value) \
    ((This)->lpVtbl->get_BlueLevelCount(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_GainLevelCount(This, value) \
    ((This)->lpVtbl->get_GainLevelCount(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_FixedColor(This, value) \
    ((This)->lpVtbl->get_FixedColor(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_GetNearestSupportedColor(This, desiredColor, result) \
    ((This)->lpVtbl->GetNearestSupportedColor(This, desiredColor, result))

#define __x_ABI_CWindows_CDevices_CLights_CILampInfo_get_UpdateLatency(This, value) \
    ((This)->lpVtbl->get_UpdateLatency(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampInfo;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.ILampStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Lamp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_ILampStatics[] = L"Windows.Devices.Lights.ILampStatics";
typedef struct __x_ABI_CWindows_CDevices_CLights_CILampStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp** operation);
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CLights_CILampStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CLights__CLamp** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CILampStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CILampStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CILampStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_GetDeviceSelector(This, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_FromIdAsync(This, deviceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, operation))

#define __x_ABI_CWindows_CDevices_CLights_CILampStatics_GetDefaultAsync(This, operation) \
    ((This)->lpVtbl->GetDefaultAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CILampStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CILampStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.Lamp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.ILampStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILamp ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Lamp_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Lamp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Lamp[] = L"Windows.Devices.Lights.Lamp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.LampArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.ILampArrayStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampArray ** Default Interface **
 *    Windows.Devices.Lights.ILampArray2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampArray_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampArray_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampArray[] = L"Windows.Devices.Lights.LampArray";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.LampAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampAvailabilityChangedEventArgs[] = L"Windows.Devices.Lights.LampAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Lights.LampInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.ILampInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_LampInfo_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_LampInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_LampInfo[] = L"Windows.Devices.Lights.LampInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Elights_p_h__

#endif // __windows2Edevices2Elights_h__
