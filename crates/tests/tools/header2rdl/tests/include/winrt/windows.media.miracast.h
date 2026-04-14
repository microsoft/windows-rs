
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
#ifndef __windows2Emedia2Emiracast_h__
#define __windows2Emedia2Emiracast_h__
#ifndef __windows2Emedia2Emiracast_p_h__
#define __windows2Emedia2Emiracast_p_h__


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
#include "Windows.ApplicationModel.Core.h"
#include "Windows.Graphics.h"
#include "Windows.Media.Core.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiver;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver ABI::Windows::Media::Miracast::IMiracastReceiver

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverApplySettingsResult;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult ABI::Windows::Media::Miracast::IMiracastReceiverApplySettingsResult

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverConnection;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection ABI::Windows::Media::Miracast::IMiracastReceiverConnection

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverConnectionCreatedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs ABI::Windows::Media::Miracast::IMiracastReceiverConnectionCreatedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverCursorImageChannel;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel ABI::Windows::Media::Miracast::IMiracastReceiverCursorImageChannel

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverCursorImageChannelSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings ABI::Windows::Media::Miracast::IMiracastReceiverCursorImageChannelSettings

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverDisconnectedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs ABI::Windows::Media::Miracast::IMiracastReceiverDisconnectedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverGameControllerDevice;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice ABI::Windows::Media::Miracast::IMiracastReceiverGameControllerDevice

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverInputDevices;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices ABI::Windows::Media::Miracast::IMiracastReceiverInputDevices

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverKeyboardDevice;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice ABI::Windows::Media::Miracast::IMiracastReceiverKeyboardDevice

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverMediaSourceCreatedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs ABI::Windows::Media::Miracast::IMiracastReceiverMediaSourceCreatedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverSession;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession ABI::Windows::Media::Miracast::IMiracastReceiverSession

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverSessionStartResult;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult ABI::Windows::Media::Miracast::IMiracastReceiverSessionStartResult

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings ABI::Windows::Media::Miracast::IMiracastReceiverSettings

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus ABI::Windows::Media::Miracast::IMiracastReceiverStatus

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverStreamControl;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl ABI::Windows::Media::Miracast::IMiracastReceiverStreamControl

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastReceiverVideoStreamSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                interface IMiracastTransmitter;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter ABI::Windows::Media::Miracast::IMiracastTransmitter

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverApplySettingsResult;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6fec734b-823d-5b06-ad81-0455f97f556f"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*, ABI::Windows::Media::Miracast::IMiracastReceiverApplySettingsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverApplySettingsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("afcb4574-7ac3-56ea-9a6a-cf535f0cf01e"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*, ABI::Windows::Media::Miracast::IMiracastReceiverApplySettingsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverApplySettingsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverSession;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5ab880e1-2c0d-5d2f-bf95-037515624a8c"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSession*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::IMiracastReceiverSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSession*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9d4308cb-4bcf-5b1b-b8b3-0484de9f3537"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::IMiracastReceiverSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverSessionStartResult;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4520f20-1984-52e5-9b70-15a9ce94aef8"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*, ABI::Windows::Media::Miracast::IMiracastReceiverSessionStartResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverSessionStartResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eb315ed3-f94b-5fcc-9512-98ac9d8a423f"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*, ABI::Windows::Media::Miracast::IMiracastReceiverSessionStartResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverSessionStartResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSessionStartResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("922c5527-4300-5995-8ddc-923dd4ba7010"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSettings*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSettings*, ABI::Windows::Media::Miracast::IMiracastReceiverSettings*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverSettings>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverSettings*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e89ca08-40e1-52f6-8649-04841e01820d"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSettings*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSettings*, ABI::Windows::Media::Miracast::IMiracastReceiverSettings*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverSettings>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverSettings*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aa26649e-265d-5e79-8eef-a7fe894dc9f2"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverStatus*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverStatus*, ABI::Windows::Media::Miracast::IMiracastReceiverStatus*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverStatus*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8243f2be-82a3-5335-b3c9-ae653b3b695c"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverStatus*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverStatus*, ABI::Windows::Media::Miracast::IMiracastReceiverStatus*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverStatus*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverVideoStreamSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67ca293b-c811-57b2-b4fc-007b7efb64a0"))
IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*, ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Miracast.MiracastReceiverVideoStreamSettings>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*> __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b88cbc92-b616-57d1-9f9b-6bba5d5acfa9"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*, ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Miracast.MiracastReceiverVideoStreamSettings>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Miracast::MiracastReceiverVideoStreamSettings*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverConnection;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#define DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("618a96b9-8b3b-5dbb-acf6-b015ff651785"))
IIterator<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverConnection*, ABI::Windows::Media::Miracast::IMiracastReceiverConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Miracast.MiracastReceiverConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t;
#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#define DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e989eb90-1f6f-5084-9bfb-1a5decca4f23"))
IIterable<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverConnection*, ABI::Windows::Media::Miracast::IMiracastReceiverConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Miracast.MiracastReceiverConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t;
#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastTransmitter;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#define DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f903966b-1c85-5726-af5b-afc28a3b3cf4"))
IIterator<ABI::Windows::Media::Miracast::MiracastTransmitter*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastTransmitter*, ABI::Windows::Media::Miracast::IMiracastTransmitter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Miracast.MiracastTransmitter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Miracast::MiracastTransmitter*> __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t;
#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#define DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5a3f4149-9b6b-5ff0-85a0-fea37b0990eb"))
IIterable<ABI::Windows::Media::Miracast::MiracastTransmitter*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastTransmitter*, ABI::Windows::Media::Miracast::IMiracastTransmitter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Miracast.MiracastTransmitter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Miracast::MiracastTransmitter*> __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t;
#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#define DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98ac8f8c-2322-54cf-b2c6-7a56a9d2220b"))
IVectorView<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverConnection*, ABI::Windows::Media::Miracast::IMiracastReceiverConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Miracast.MiracastReceiverConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Miracast::MiracastReceiverConnection*> __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t;
#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#define DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e1bf46c-49f6-5892-bcf7-80564ea2b606"))
IVectorView<ABI::Windows::Media::Miracast::MiracastTransmitter*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastTransmitter*, ABI::Windows::Media::Miracast::IMiracastTransmitter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Miracast.MiracastTransmitter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Miracast::MiracastTransmitter*> __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t;
#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiver;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44eb06ea-0014-5aed-83a1-95d225d06688"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiver*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiver*, ABI::Windows::Media::Miracast::IMiracastReceiver*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiver, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiver*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverCursorImageChannel;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("401d6f64-cb30-59c3-a663-f84ab6edf1fa"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverCursorImageChannel*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverCursorImageChannel*, ABI::Windows::Media::Miracast::IMiracastReceiverCursorImageChannel*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverCursorImageChannel, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverCursorImageChannel*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverGameControllerDevice;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10a9d2a7-5049-5e19-9d22-72da7d6bb643"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverGameControllerDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverGameControllerDevice*, ABI::Windows::Media::Miracast::IMiracastReceiverGameControllerDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverGameControllerDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverGameControllerDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverKeyboardDevice;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("97d896c7-a5ea-5209-92c0-a0278087afd1"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverKeyboardDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverKeyboardDevice*, ABI::Windows::Media::Miracast::IMiracastReceiverKeyboardDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverKeyboardDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverKeyboardDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverConnectionCreatedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("69d03828-7a8a-549a-8253-7850e4db605a"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverConnectionCreatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::IMiracastReceiverSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverConnectionCreatedEventArgs*, ABI::Windows::Media::Miracast::IMiracastReceiverConnectionCreatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverSession, Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverConnectionCreatedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverDisconnectedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4964b5f-147c-57e3-82d0-cc7de5ff2def"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverDisconnectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::IMiracastReceiverSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverDisconnectedEventArgs*, ABI::Windows::Media::Miracast::IMiracastReceiverDisconnectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverSession, Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverDisconnectedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverMediaSourceCreatedEventArgs;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("929ec57a-92cc-50f8-ae4a-bb6a67152e82"))
ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverMediaSourceCreatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::IMiracastReceiverSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Miracast::MiracastReceiverMediaSourceCreatedEventArgs*, ABI::Windows::Media::Miracast::IMiracastReceiverMediaSourceCreatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Miracast.MiracastReceiverSession, Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Miracast::MiracastReceiverSession*, ABI::Windows::Media::Miracast::MiracastReceiverMediaSourceCreatedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class CoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView ABI::Windows::ApplicationModel::Core::ICoreApplicationView

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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
        namespace Graphics {
            typedef struct PointInt32 PointInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct SizeInt32 SizeInt32;
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class MediaSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaSource2;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaSource2 ABI::Windows::Media::Core::IMediaSource2

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverApplySettingsStatus : int MiracastReceiverApplySettingsStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverAuthorizationMethod : int MiracastReceiverAuthorizationMethod;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverDisconnectReason : int MiracastReceiverDisconnectReason;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverGameControllerDeviceUsageMode : int MiracastReceiverGameControllerDeviceUsageMode;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverListeningStatus : int MiracastReceiverListeningStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverSessionStartStatus : int MiracastReceiverSessionStartStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastReceiverWiFiStatus : int MiracastReceiverWiFiStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                typedef enum MiracastTransmitterAuthorizationStatus : int MiracastTransmitterAuthorizationStatus;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverCursorImageChannelSettings;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverInputDevices;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                class MiracastReceiverStreamControl;
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverApplySettingsStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverApplySettingsStatus : int
                {
                    MiracastReceiverApplySettingsStatus_Success = 0,
                    MiracastReceiverApplySettingsStatus_UnknownFailure = 1,
                    MiracastReceiverApplySettingsStatus_MiracastNotSupported = 2,
                    MiracastReceiverApplySettingsStatus_AccessDenied = 3,
                    MiracastReceiverApplySettingsStatus_FriendlyNameTooLong = 4,
                    MiracastReceiverApplySettingsStatus_ModelNameTooLong = 5,
                    MiracastReceiverApplySettingsStatus_ModelNumberTooLong = 6,
                    MiracastReceiverApplySettingsStatus_InvalidSettings = 7,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverAuthorizationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverAuthorizationMethod : int
                {
                    MiracastReceiverAuthorizationMethod_None = 0,
                    MiracastReceiverAuthorizationMethod_ConfirmConnection = 1,
                    MiracastReceiverAuthorizationMethod_PinDisplayIfRequested = 2,
                    MiracastReceiverAuthorizationMethod_PinDisplayRequired = 3,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverDisconnectReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverDisconnectReason : int
                {
                    MiracastReceiverDisconnectReason_Finished = 0,
                    MiracastReceiverDisconnectReason_AppSpecificError = 1,
                    MiracastReceiverDisconnectReason_ConnectionNotAccepted = 2,
                    MiracastReceiverDisconnectReason_DisconnectedByUser = 3,
                    MiracastReceiverDisconnectReason_FailedToStartStreaming = 4,
                    MiracastReceiverDisconnectReason_MediaDecodingError = 5,
                    MiracastReceiverDisconnectReason_MediaStreamingError = 6,
                    MiracastReceiverDisconnectReason_MediaDecryptionError = 7,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverGameControllerDeviceUsageMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverGameControllerDeviceUsageMode : int
                {
                    MiracastReceiverGameControllerDeviceUsageMode_AsGameController = 0,
                    MiracastReceiverGameControllerDeviceUsageMode_AsMouseAndKeyboard = 1,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverListeningStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverListeningStatus : int
                {
                    MiracastReceiverListeningStatus_NotListening = 0,
                    MiracastReceiverListeningStatus_Listening = 1,
                    MiracastReceiverListeningStatus_ConnectionPending = 2,
                    MiracastReceiverListeningStatus_Connected = 3,
                    MiracastReceiverListeningStatus_DisabledByPolicy = 4,
                    MiracastReceiverListeningStatus_TemporarilyDisabled = 5,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverSessionStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverSessionStartStatus : int
                {
                    MiracastReceiverSessionStartStatus_Success = 0,
                    MiracastReceiverSessionStartStatus_UnknownFailure = 1,
                    MiracastReceiverSessionStartStatus_MiracastNotSupported = 2,
                    MiracastReceiverSessionStartStatus_AccessDenied = 3,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverWiFiStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastReceiverWiFiStatus : int
                {
                    MiracastReceiverWiFiStatus_MiracastSupportUndetermined = 0,
                    MiracastReceiverWiFiStatus_MiracastNotSupported = 1,
                    MiracastReceiverWiFiStatus_MiracastSupportNotOptimized = 2,
                    MiracastReceiverWiFiStatus_MiracastSupported = 3,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastTransmitterAuthorizationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                enum MiracastTransmitterAuthorizationStatus : int
                {
                    MiracastTransmitterAuthorizationStatus_Undecided = 0,
                    MiracastTransmitterAuthorizationStatus_Allowed = 1,
                    MiracastTransmitterAuthorizationStatus_AlwaysPrompt = 2,
                    MiracastTransmitterAuthorizationStatus_Blocked = 3,
                };
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiver[] = L"Windows.Media.Miracast.IMiracastReceiver";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("7a315258-e444-51b4-aff7-b88daa1229e0")
                IMiracastReceiver : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultSettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentSettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentSettingsAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisconnectAllAndApplySettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverSettings* settings,
                        ABI::Windows::Media::Miracast::IMiracastReceiverApplySettingsResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisconnectAllAndApplySettingsAsync(
                        ABI::Windows::Media::Miracast::IMiracastReceiverSettings* settings,
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        ABI::Windows::Media::Miracast::IMiracastReceiverStatus** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStatusAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSession(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView* view,
                        ABI::Windows::Media::Miracast::IMiracastReceiverSession** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSessionAsync(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView* view,
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearKnownTransmitters(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveKnownTransmitter(
                        ABI::Windows::Media::Miracast::IMiracastTransmitter* transmitter
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiver = __uuidof(IMiracastReceiver);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverApplySettingsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverApplySettingsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverApplySettingsResult[] = L"Windows.Media.Miracast.IMiracastReceiverApplySettingsResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("d0aa6272-09cd-58e1-a4f2-5d5143d312f9")
                IMiracastReceiverApplySettingsResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Miracast::MiracastReceiverApplySettingsStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverApplySettingsResult = __uuidof(IMiracastReceiverApplySettingsResult);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverConnection[] = L"Windows.Media.Miracast.IMiracastReceiverConnection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("704b2f36-d2e5-551f-a854-f822b7917d28")
                IMiracastReceiverConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Disconnect(
                        ABI::Windows::Media::Miracast::MiracastReceiverDisconnectReason reason
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisconnectWithMessage(
                        ABI::Windows::Media::Miracast::MiracastReceiverDisconnectReason reason,
                        HSTRING message
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PauseAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResumeAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Transmitter(
                        ABI::Windows::Media::Miracast::IMiracastTransmitter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InputDevices(
                        ABI::Windows::Media::Miracast::IMiracastReceiverInputDevices** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CursorImageChannel(
                        ABI::Windows::Media::Miracast::IMiracastReceiverCursorImageChannel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StreamControl(
                        ABI::Windows::Media::Miracast::IMiracastReceiverStreamControl** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverConnection = __uuidof(IMiracastReceiverConnection);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverConnectionCreatedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("7d8dfa39-307a-5c0f-94bd-d0c69d169982")
                IMiracastReceiverConnectionCreatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Connection(
                        ABI::Windows::Media::Miracast::IMiracastReceiverConnection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pin(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverConnectionCreatedEventArgs = __uuidof(IMiracastReceiverConnectionCreatedEventArgs);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverCursorImageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverCursorImageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverCursorImageChannel[] = L"Windows.Media.Miracast.IMiracastReceiverCursorImageChannel";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("d9ac332d-723a-5a9d-b90a-81153efa2a0f")
                IMiracastReceiverCursorImageChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxImageSize(
                        ABI::Windows::Graphics::SizeInt32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Graphics::PointInt32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageStream(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ImageStreamChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ImageStreamChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PositionChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PositionChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverCursorImageChannel = __uuidof(IMiracastReceiverCursorImageChannel);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverCursorImageChannelSettings[] = L"Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("ccdbedff-bd00-5b9c-8e4c-00cacf86b634")
                IMiracastReceiverCursorImageChannelSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxImageSize(
                        ABI::Windows::Graphics::SizeInt32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxImageSize(
                        ABI::Windows::Graphics::SizeInt32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverCursorImageChannelSettings = __uuidof(IMiracastReceiverCursorImageChannelSettings);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverDisconnectedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("d9a15e5e-5fee-57e6-b4b0-04727db93229")
                IMiracastReceiverDisconnectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Connection(
                        ABI::Windows::Media::Miracast::IMiracastReceiverConnection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverDisconnectedEventArgs = __uuidof(IMiracastReceiverDisconnectedEventArgs);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverGameControllerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverGameControllerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverGameControllerDevice[] = L"Windows.Media.Miracast.IMiracastReceiverGameControllerDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("2d7171e8-bed4-5118-a058-e2477eb5888d")
                IMiracastReceiverGameControllerDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransmitInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransmitInput(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRequestedByTransmitter(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTransmittingInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mode(
                        ABI::Windows::Media::Miracast::MiracastReceiverGameControllerDeviceUsageMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mode(
                        ABI::Windows::Media::Miracast::MiracastReceiverGameControllerDeviceUsageMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverGameControllerDevice = __uuidof(IMiracastReceiverGameControllerDevice);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverInputDevices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverInputDevices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverInputDevices[] = L"Windows.Media.Miracast.IMiracastReceiverInputDevices";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("da35bb02-28aa-5ee8-96f5-a42901c66f00")
                IMiracastReceiverInputDevices : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Keyboard(
                        ABI::Windows::Media::Miracast::IMiracastReceiverKeyboardDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GameController(
                        ABI::Windows::Media::Miracast::IMiracastReceiverGameControllerDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverInputDevices = __uuidof(IMiracastReceiverInputDevices);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverKeyboardDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverKeyboardDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverKeyboardDevice[] = L"Windows.Media.Miracast.IMiracastReceiverKeyboardDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("beb67272-06c0-54ff-ac96-217464ff2501")
                IMiracastReceiverKeyboardDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransmitInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransmitInput(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRequestedByTransmitter(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTransmittingInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverKeyboardDevice = __uuidof(IMiracastReceiverKeyboardDevice);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverMediaSourceCreatedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("17cf519e-1246-531d-945a-6b158e39c3aa")
                IMiracastReceiverMediaSourceCreatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Connection(
                        ABI::Windows::Media::Miracast::IMiracastReceiverConnection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaSource(
                        ABI::Windows::Media::Core::IMediaSource2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CursorImageChannelSettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverCursorImageChannelSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverMediaSourceCreatedEventArgs = __uuidof(IMiracastReceiverMediaSourceCreatedEventArgs);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSession[] = L"Windows.Media.Miracast.IMiracastReceiverSession";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("1d2bcdb4-ef8b-5209-bfc9-c32116504803")
                IMiracastReceiverSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ConnectionCreated(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConnectionCreated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MediaSourceCreated(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MediaSourceCreated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Disconnected(
                        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Disconnected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowConnectionTakeover(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowConnectionTakeover(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSimultaneousConnections(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxSimultaneousConnections(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(
                        ABI::Windows::Media::Miracast::IMiracastReceiverSessionStartResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverSession = __uuidof(IMiracastReceiverSession);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSessionStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSessionStartResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSessionStartResult[] = L"Windows.Media.Miracast.IMiracastReceiverSessionStartResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("b7c573ee-40ca-51ff-95f2-c9de34f2e90e")
                IMiracastReceiverSessionStartResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Miracast::MiracastReceiverSessionStartStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverSessionStartResult = __uuidof(IMiracastReceiverSessionStartResult);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSettings[] = L"Windows.Media.Miracast.IMiracastReceiverSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("57cd2f24-c55a-5fbe-9464-eb05307705dd")
                IMiracastReceiverSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ModelName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ModelNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AuthorizationMethod(
                        ABI::Windows::Media::Miracast::MiracastReceiverAuthorizationMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AuthorizationMethod(
                        ABI::Windows::Media::Miracast::MiracastReceiverAuthorizationMethod value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequireAuthorizationFromKnownTransmitters(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequireAuthorizationFromKnownTransmitters(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverSettings = __uuidof(IMiracastReceiverSettings);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverStatus[] = L"Windows.Media.Miracast.IMiracastReceiverStatus";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("c28a5591-23ab-519e-ad09-90bff6dcc87e")
                IMiracastReceiverStatus : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ListeningStatus(
                        ABI::Windows::Media::Miracast::MiracastReceiverListeningStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WiFiStatus(
                        ABI::Windows::Media::Miracast::MiracastReceiverWiFiStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsConnectionTakeoverSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSimultaneousConnections(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KnownTransmitters(
                        __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverStatus = __uuidof(IMiracastReceiverStatus);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverStreamControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverStreamControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverStreamControl[] = L"Windows.Media.Miracast.IMiracastReceiverStreamControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("38ea2d8b-2769-5ad7-8a8a-254b9df7ba82")
                IMiracastReceiverStreamControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetVideoStreamSettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVideoStreamSettingsAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SuggestVideoStreamSettings(
                        ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings* settings
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SuggestVideoStreamSettingsAsync(
                        ABI::Windows::Media::Miracast::IMiracastReceiverVideoStreamSettings* settings,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MuteAudio(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MuteAudio(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverStreamControl = __uuidof(IMiracastReceiverStreamControl);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverVideoStreamSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverVideoStreamSettings[] = L"Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("169b5e1b-149d-52d0-b126-6f89744e4f50")
                IMiracastReceiverVideoStreamSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Graphics::SizeInt32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Size(
                        ABI::Windows::Graphics::SizeInt32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Bitrate(
                        INT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastReceiverVideoStreamSettings = __uuidof(IMiracastReceiverVideoStreamSettings);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastTransmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastTransmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastTransmitter[] = L"Windows.Media.Miracast.IMiracastTransmitter";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Miracast {
                MIDL_INTERFACE("342d79fd-2e64-5508-8a30-833d1eac70d0")
                IMiracastTransmitter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AuthorizationStatus(
                        ABI::Windows::Media::Miracast::MiracastTransmitterAuthorizationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AuthorizationStatus(
                        ABI::Windows::Media::Miracast::MiracastTransmitterAuthorizationStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConnections(
                        __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MacAddress(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastConnectionTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMiracastTransmitter = __uuidof(IMiracastTransmitter);
            } /* Miracast */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiver_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiver[] = L"Windows.Media.Miracast.MiracastReceiver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverApplySettingsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverApplySettingsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverApplySettingsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverApplySettingsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverApplySettingsResult[] = L"Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverConnection[] = L"Windows.Media.Miracast.MiracastReceiverConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverCursorImageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverCursorImageChannel ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannel_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverCursorImageChannel[] = L"Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings[] = L"Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverGameControllerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverGameControllerDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverGameControllerDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverGameControllerDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverGameControllerDevice[] = L"Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverInputDevices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverInputDevices ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverInputDevices_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverInputDevices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverInputDevices[] = L"Windows.Media.Miracast.MiracastReceiverInputDevices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverKeyboardDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverKeyboardDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverKeyboardDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverKeyboardDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverKeyboardDevice[] = L"Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSession[] = L"Windows.Media.Miracast.MiracastReceiverSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSessionStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSessionStartResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSessionStartResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSessionStartResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSessionStartResult[] = L"Windows.Media.Miracast.MiracastReceiverSessionStartResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSettings[] = L"Windows.Media.Miracast.MiracastReceiverSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverStatus ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverStatus[] = L"Windows.Media.Miracast.MiracastReceiverStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverStreamControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverStreamControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStreamControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStreamControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverStreamControl[] = L"Windows.Media.Miracast.MiracastReceiverStreamControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverVideoStreamSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings[] = L"Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastTransmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastTransmitter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastTransmitter_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastTransmitter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastTransmitter[] = L"Windows.Media.Miracast.MiracastTransmitter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter;

#endif // ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

typedef struct __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl;

interface __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

typedef struct __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        __FIIterator_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl;

interface __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

typedef struct __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl;

interface __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

typedef struct __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        __FIIterator_1_Windows__CMedia__CMiracast__CMiracastTransmitter** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl;

interface __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection;

typedef struct __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl;

interface __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter;

typedef struct __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl;

interface __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* sender,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* sender,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* sender,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CGraphics_CPointInt32 __x_ABI_CWindows_CGraphics_CPointInt32;

typedef struct __x_ABI_CWindows_CGraphics_CSizeInt32 __x_ABI_CWindows_CGraphics_CSizeInt32;

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaSource2 __x_ABI_CWindows_CMedia_CCore_CIMediaSource2;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverApplySettingsStatus __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverApplySettingsStatus;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverAuthorizationMethod __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverAuthorizationMethod;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverDisconnectReason __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverDisconnectReason;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverGameControllerDeviceUsageMode __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverGameControllerDeviceUsageMode;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverListeningStatus __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverListeningStatus;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverSessionStartStatus __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverSessionStartStatus;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverWiFiStatus __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverWiFiStatus;

typedef enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastTransmitterAuthorizationStatus __x_ABI_CWindows_CMedia_CMiracast_CMiracastTransmitterAuthorizationStatus;

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverApplySettingsStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverApplySettingsStatus
{
    MiracastReceiverApplySettingsStatus_Success = 0,
    MiracastReceiverApplySettingsStatus_UnknownFailure = 1,
    MiracastReceiverApplySettingsStatus_MiracastNotSupported = 2,
    MiracastReceiverApplySettingsStatus_AccessDenied = 3,
    MiracastReceiverApplySettingsStatus_FriendlyNameTooLong = 4,
    MiracastReceiverApplySettingsStatus_ModelNameTooLong = 5,
    MiracastReceiverApplySettingsStatus_ModelNumberTooLong = 6,
    MiracastReceiverApplySettingsStatus_InvalidSettings = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverAuthorizationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverAuthorizationMethod
{
    MiracastReceiverAuthorizationMethod_None = 0,
    MiracastReceiverAuthorizationMethod_ConfirmConnection = 1,
    MiracastReceiverAuthorizationMethod_PinDisplayIfRequested = 2,
    MiracastReceiverAuthorizationMethod_PinDisplayRequired = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverDisconnectReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverDisconnectReason
{
    MiracastReceiverDisconnectReason_Finished = 0,
    MiracastReceiverDisconnectReason_AppSpecificError = 1,
    MiracastReceiverDisconnectReason_ConnectionNotAccepted = 2,
    MiracastReceiverDisconnectReason_DisconnectedByUser = 3,
    MiracastReceiverDisconnectReason_FailedToStartStreaming = 4,
    MiracastReceiverDisconnectReason_MediaDecodingError = 5,
    MiracastReceiverDisconnectReason_MediaStreamingError = 6,
    MiracastReceiverDisconnectReason_MediaDecryptionError = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverGameControllerDeviceUsageMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverGameControllerDeviceUsageMode
{
    MiracastReceiverGameControllerDeviceUsageMode_AsGameController = 0,
    MiracastReceiverGameControllerDeviceUsageMode_AsMouseAndKeyboard = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverListeningStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverListeningStatus
{
    MiracastReceiverListeningStatus_NotListening = 0,
    MiracastReceiverListeningStatus_Listening = 1,
    MiracastReceiverListeningStatus_ConnectionPending = 2,
    MiracastReceiverListeningStatus_Connected = 3,
    MiracastReceiverListeningStatus_DisabledByPolicy = 4,
    MiracastReceiverListeningStatus_TemporarilyDisabled = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverSessionStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverSessionStartStatus
{
    MiracastReceiverSessionStartStatus_Success = 0,
    MiracastReceiverSessionStartStatus_UnknownFailure = 1,
    MiracastReceiverSessionStartStatus_MiracastNotSupported = 2,
    MiracastReceiverSessionStartStatus_AccessDenied = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastReceiverWiFiStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverWiFiStatus
{
    MiracastReceiverWiFiStatus_MiracastSupportUndetermined = 0,
    MiracastReceiverWiFiStatus_MiracastNotSupported = 1,
    MiracastReceiverWiFiStatus_MiracastSupportNotOptimized = 2,
    MiracastReceiverWiFiStatus_MiracastSupported = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.Media.Miracast.MiracastTransmitterAuthorizationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastTransmitterAuthorizationStatus
{
    MiracastTransmitterAuthorizationStatus_Undecided = 0,
    MiracastTransmitterAuthorizationStatus_Allowed = 1,
    MiracastTransmitterAuthorizationStatus_AlwaysPrompt = 2,
    MiracastTransmitterAuthorizationStatus_Blocked = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiver[] = L"Windows.Media.Miracast.IMiracastReceiver";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultSettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetCurrentSettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetCurrentSettingsAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSettings** operation);
    HRESULT (STDMETHODCALLTYPE* DisconnectAllAndApplySettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* settings,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult** result);
    HRESULT (STDMETHODCALLTYPE* DisconnectAllAndApplySettingsAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* settings,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverApplySettingsResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetStatusAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverStatus** operation);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateSession)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* view,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession** result);
    HRESULT (STDMETHODCALLTYPE* CreateSessionAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* view,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSession** operation);
    HRESULT (STDMETHODCALLTYPE* ClearKnownTransmitters)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This);
    HRESULT (STDMETHODCALLTYPE* RemoveKnownTransmitter)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* transmitter);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetDefaultSettings(This, result) \
    ((This)->lpVtbl->GetDefaultSettings(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetCurrentSettings(This, result) \
    ((This)->lpVtbl->GetCurrentSettings(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetCurrentSettingsAsync(This, operation) \
    ((This)->lpVtbl->GetCurrentSettingsAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_DisconnectAllAndApplySettings(This, settings, result) \
    ((This)->lpVtbl->DisconnectAllAndApplySettings(This, settings, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_DisconnectAllAndApplySettingsAsync(This, settings, operation) \
    ((This)->lpVtbl->DisconnectAllAndApplySettingsAsync(This, settings, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetStatus(This, result) \
    ((This)->lpVtbl->GetStatus(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_GetStatusAsync(This, operation) \
    ((This)->lpVtbl->GetStatusAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_CreateSession(This, view, result) \
    ((This)->lpVtbl->CreateSession(This, view, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_CreateSessionAsync(This, view, operation) \
    ((This)->lpVtbl->CreateSessionAsync(This, view, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_ClearKnownTransmitters(This) \
    ((This)->lpVtbl->ClearKnownTransmitters(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_RemoveKnownTransmitter(This, transmitter) \
    ((This)->lpVtbl->RemoveKnownTransmitter(This, transmitter))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverApplySettingsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverApplySettingsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverApplySettingsResult[] = L"Windows.Media.Miracast.IMiracastReceiverApplySettingsResult";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverApplySettingsStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResultVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverApplySettingsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverConnection[] = L"Windows.Media.Miracast.IMiracastReceiverConnection";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Disconnect)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverDisconnectReason reason);
    HRESULT (STDMETHODCALLTYPE* DisconnectWithMessage)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverDisconnectReason reason,
        HSTRING message);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* PauseAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This);
    HRESULT (STDMETHODCALLTYPE* ResumeAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_Transmitter)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter** value);
    HRESULT (STDMETHODCALLTYPE* get_InputDevices)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices** value);
    HRESULT (STDMETHODCALLTYPE* get_CursorImageChannel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel** value);
    HRESULT (STDMETHODCALLTYPE* get_StreamControl)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_Disconnect(This, reason) \
    ((This)->lpVtbl->Disconnect(This, reason))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_DisconnectWithMessage(This, reason, message) \
    ((This)->lpVtbl->DisconnectWithMessage(This, reason, message))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_PauseAsync(This, operation) \
    ((This)->lpVtbl->PauseAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_ResumeAsync(This, operation) \
    ((This)->lpVtbl->ResumeAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_get_Transmitter(This, value) \
    ((This)->lpVtbl->get_Transmitter(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_get_InputDevices(This, value) \
    ((This)->lpVtbl->get_InputDevices(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_get_CursorImageChannel(This, value) \
    ((This)->lpVtbl->get_CursorImageChannel(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_get_StreamControl(This, value) \
    ((This)->lpVtbl->get_StreamControl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverConnectionCreatedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** value);
    HRESULT (STDMETHODCALLTYPE* get_Pin)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_get_Pin(This, value) \
    ((This)->lpVtbl->get_Pin(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnectionCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverCursorImageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverCursorImageChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverCursorImageChannel[] = L"Windows.Media.Miracast.IMiracastReceiverCursorImageChannel";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxImageSize)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        struct __x_ABI_CWindows_CGraphics_CPointInt32* value);
    HRESULT (STDMETHODCALLTYPE* get_ImageStream)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
    HRESULT (STDMETHODCALLTYPE* add_ImageStreamChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ImageStreamChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PositionChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverCursorImageChannel_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PositionChanged)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_get_MaxImageSize(This, value) \
    ((This)->lpVtbl->get_MaxImageSize(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_get_ImageStream(This, value) \
    ((This)->lpVtbl->get_ImageStream(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_add_ImageStreamChanged(This, handler, token) \
    ((This)->lpVtbl->add_ImageStreamChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_remove_ImageStreamChanged(This, token) \
    ((This)->lpVtbl->remove_ImageStreamChanged(This, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_add_PositionChanged(This, handler, token) \
    ((This)->lpVtbl->add_PositionChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_remove_PositionChanged(This, token) \
    ((This)->lpVtbl->remove_PositionChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverCursorImageChannelSettings[] = L"Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MaxImageSize)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxImageSize)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_get_MaxImageSize(This, value) \
    ((This)->lpVtbl->get_MaxImageSize(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_put_MaxImageSize(This, value) \
    ((This)->lpVtbl->put_MaxImageSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverDisconnectedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverDisconnectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverGameControllerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverGameControllerDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverGameControllerDevice[] = L"Windows.Media.Miracast.IMiracastReceiverGameControllerDevice";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransmitInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_TransmitInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequestedByTransmitter)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTransmittingInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverGameControllerDeviceUsageMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverGameControllerDeviceUsageMode value);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverGameControllerDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_get_TransmitInput(This, value) \
    ((This)->lpVtbl->get_TransmitInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_put_TransmitInput(This, value) \
    ((This)->lpVtbl->put_TransmitInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_get_IsRequestedByTransmitter(This, value) \
    ((This)->lpVtbl->get_IsRequestedByTransmitter(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_get_IsTransmittingInput(This, value) \
    ((This)->lpVtbl->get_IsTransmittingInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverInputDevices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverInputDevices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverInputDevices[] = L"Windows.Media.Miracast.IMiracastReceiverInputDevices";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevicesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Keyboard)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_GameController)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverGameControllerDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevicesVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevicesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_get_Keyboard(This, value) \
    ((This)->lpVtbl->get_Keyboard(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_get_GameController(This, value) \
    ((This)->lpVtbl->get_GameController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverInputDevices_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverKeyboardDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverKeyboardDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverKeyboardDevice[] = L"Windows.Media.Miracast.IMiracastReceiverKeyboardDevice";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransmitInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_TransmitInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequestedByTransmitter)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTransmittingInput)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverKeyboardDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_get_TransmitInput(This, value) \
    ((This)->lpVtbl->get_TransmitInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_put_TransmitInput(This, value) \
    ((This)->lpVtbl->put_TransmitInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_get_IsRequestedByTransmitter(This, value) \
    ((This)->lpVtbl->get_IsRequestedByTransmitter(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_get_IsTransmittingInput(This, value) \
    ((This)->lpVtbl->get_IsTransmittingInput(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverKeyboardDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverMediaSourceCreatedEventArgs[] = L"Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverConnection** value);
    HRESULT (STDMETHODCALLTYPE* get_MediaSource)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaSource2** value);
    HRESULT (STDMETHODCALLTYPE* get_CursorImageChannelSettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverCursorImageChannelSettings** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_get_MediaSource(This, value) \
    ((This)->lpVtbl->get_MediaSource(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_get_CursorImageChannelSettings(This, value) \
    ((This)->lpVtbl->get_CursorImageChannelSettings(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverMediaSourceCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSession[] = L"Windows.Media.Miracast.IMiracastReceiverSession";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ConnectionCreated)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverConnectionCreatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConnectionCreated)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_MediaSourceCreated)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverMediaSourceCreatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MediaSourceCreated)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Disconnected)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        __FITypedEventHandler_2_Windows__CMedia__CMiracast__CMiracastReceiverSession_Windows__CMedia__CMiracast__CMiracastReceiverDisconnectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Disconnected)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_AllowConnectionTakeover)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowConnectionTakeover)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSimultaneousConnections)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxSimultaneousConnections)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult** result);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverSessionStartResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_add_ConnectionCreated(This, handler, token) \
    ((This)->lpVtbl->add_ConnectionCreated(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_remove_ConnectionCreated(This, token) \
    ((This)->lpVtbl->remove_ConnectionCreated(This, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_add_MediaSourceCreated(This, handler, token) \
    ((This)->lpVtbl->add_MediaSourceCreated(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_remove_MediaSourceCreated(This, token) \
    ((This)->lpVtbl->remove_MediaSourceCreated(This, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_add_Disconnected(This, handler, token) \
    ((This)->lpVtbl->add_Disconnected(This, handler, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_remove_Disconnected(This, token) \
    ((This)->lpVtbl->remove_Disconnected(This, token))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_get_AllowConnectionTakeover(This, value) \
    ((This)->lpVtbl->get_AllowConnectionTakeover(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_put_AllowConnectionTakeover(This, value) \
    ((This)->lpVtbl->put_AllowConnectionTakeover(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_get_MaxSimultaneousConnections(This, value) \
    ((This)->lpVtbl->get_MaxSimultaneousConnections(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_put_MaxSimultaneousConnections(This, value) \
    ((This)->lpVtbl->put_MaxSimultaneousConnections(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_Start(This, result) \
    ((This)->lpVtbl->Start(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSessionStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSessionStartResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSessionStartResult[] = L"Windows.Media.Miracast.IMiracastReceiverSessionStartResult";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverSessionStartStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResultVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSessionStartResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverSettings[] = L"Windows.Media.Miracast.IMiracastReceiverSettings";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ModelName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ModelName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ModelNumber)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ModelNumber)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AuthorizationMethod)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverAuthorizationMethod* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthorizationMethod)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverAuthorizationMethod value);
    HRESULT (STDMETHODCALLTYPE* get_RequireAuthorizationFromKnownTransmitters)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RequireAuthorizationFromKnownTransmitters)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_get_ModelName(This, value) \
    ((This)->lpVtbl->get_ModelName(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_put_ModelName(This, value) \
    ((This)->lpVtbl->put_ModelName(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_get_ModelNumber(This, value) \
    ((This)->lpVtbl->get_ModelNumber(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_put_ModelNumber(This, value) \
    ((This)->lpVtbl->put_ModelNumber(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_get_AuthorizationMethod(This, value) \
    ((This)->lpVtbl->get_AuthorizationMethod(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_put_AuthorizationMethod(This, value) \
    ((This)->lpVtbl->put_AuthorizationMethod(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_get_RequireAuthorizationFromKnownTransmitters(This, value) \
    ((This)->lpVtbl->get_RequireAuthorizationFromKnownTransmitters(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_put_RequireAuthorizationFromKnownTransmitters(This, value) \
    ((This)->lpVtbl->put_RequireAuthorizationFromKnownTransmitters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverStatus[] = L"Windows.Media.Miracast.IMiracastReceiverStatus";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ListeningStatus)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverListeningStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_WiFiStatus)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastReceiverWiFiStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnectionTakeoverSupported)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSimultaneousConnections)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_KnownTransmitters)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus* This,
        __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastTransmitter** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatusVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_get_ListeningStatus(This, value) \
    ((This)->lpVtbl->get_ListeningStatus(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_get_WiFiStatus(This, value) \
    ((This)->lpVtbl->get_WiFiStatus(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_get_IsConnectionTakeoverSupported(This, value) \
    ((This)->lpVtbl->get_IsConnectionTakeoverSupported(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_get_MaxSimultaneousConnections(This, value) \
    ((This)->lpVtbl->get_MaxSimultaneousConnections(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_get_KnownTransmitters(This, value) \
    ((This)->lpVtbl->get_KnownTransmitters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverStreamControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverStreamControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverStreamControl[] = L"Windows.Media.Miracast.IMiracastReceiverStreamControl";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetVideoStreamSettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetVideoStreamSettingsAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        __FIAsyncOperation_1_Windows__CMedia__CMiracast__CMiracastReceiverVideoStreamSettings** operation);
    HRESULT (STDMETHODCALLTYPE* SuggestVideoStreamSettings)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* settings);
    HRESULT (STDMETHODCALLTYPE* SuggestVideoStreamSettingsAsync)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* settings,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_MuteAudio)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_MuteAudio)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControlVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_GetVideoStreamSettings(This, result) \
    ((This)->lpVtbl->GetVideoStreamSettings(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_GetVideoStreamSettingsAsync(This, operation) \
    ((This)->lpVtbl->GetVideoStreamSettingsAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_SuggestVideoStreamSettings(This, settings) \
    ((This)->lpVtbl->SuggestVideoStreamSettings(This, settings))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_SuggestVideoStreamSettingsAsync(This, settings, operation) \
    ((This)->lpVtbl->SuggestVideoStreamSettingsAsync(This, settings, operation))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_get_MuteAudio(This, value) \
    ((This)->lpVtbl->get_MuteAudio(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_put_MuteAudio(This, value) \
    ((This)->lpVtbl->put_MuteAudio(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverStreamControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastReceiverVideoStreamSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastReceiverVideoStreamSettings[] = L"Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32* value);
    HRESULT (STDMETHODCALLTYPE* put_Size)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        struct __x_ABI_CWindows_CGraphics_CSizeInt32 value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Bitrate)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_put_Size(This, value) \
    ((This)->lpVtbl->put_Size(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_put_Bitrate(This, value) \
    ((This)->lpVtbl->put_Bitrate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastReceiverVideoStreamSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Media.Miracast.IMiracastTransmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Media.Miracast.MiracastTransmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Miracast_IMiracastTransmitter[] = L"Windows.Media.Miracast.IMiracastTransmitter";
typedef struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AuthorizationStatus)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastTransmitterAuthorizationStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthorizationStatus)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        enum __x_ABI_CWindows_CMedia_CMiracast_CMiracastTransmitterAuthorizationStatus value);
    HRESULT (STDMETHODCALLTYPE* GetConnections)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        __FIVectorView_1_Windows__CMedia__CMiracast__CMiracastReceiverConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_MacAddress)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LastConnectionTime)(__x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitterVtbl;

interface __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_get_AuthorizationStatus(This, value) \
    ((This)->lpVtbl->get_AuthorizationStatus(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_put_AuthorizationStatus(This, value) \
    ((This)->lpVtbl->put_AuthorizationStatus(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_GetConnections(This, result) \
    ((This)->lpVtbl->GetConnections(This, result))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_get_MacAddress(This, value) \
    ((This)->lpVtbl->get_MacAddress(This, value))

#define __x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_get_LastConnectionTime(This, value) \
    ((This)->lpVtbl->get_LastConnectionTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMiracast_CIMiracastTransmitter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiver_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiver[] = L"Windows.Media.Miracast.MiracastReceiver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverApplySettingsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverApplySettingsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverApplySettingsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverApplySettingsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverApplySettingsResult[] = L"Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverConnection[] = L"Windows.Media.Miracast.MiracastReceiverConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverConnectionCreatedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverCursorImageChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverCursorImageChannel ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannel_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverCursorImageChannel[] = L"Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverCursorImageChannelSettings[] = L"Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverDisconnectedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverGameControllerDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverGameControllerDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverGameControllerDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverGameControllerDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverGameControllerDevice[] = L"Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverInputDevices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverInputDevices ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverInputDevices_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverInputDevices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverInputDevices[] = L"Windows.Media.Miracast.MiracastReceiverInputDevices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverKeyboardDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverKeyboardDevice ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverKeyboardDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverKeyboardDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverKeyboardDevice[] = L"Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverMediaSourceCreatedEventArgs[] = L"Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSession[] = L"Windows.Media.Miracast.MiracastReceiverSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSessionStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSessionStartResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSessionStartResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSessionStartResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSessionStartResult[] = L"Windows.Media.Miracast.MiracastReceiverSessionStartResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverSettings[] = L"Windows.Media.Miracast.MiracastReceiverSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverStatus ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStatus_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverStatus[] = L"Windows.Media.Miracast.MiracastReceiverStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverStreamControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverStreamControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStreamControl_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverStreamControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverStreamControl[] = L"Windows.Media.Miracast.MiracastReceiverStreamControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastReceiverVideoStreamSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastReceiverVideoStreamSettings[] = L"Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Media.Miracast.MiracastTransmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Miracast.IMiracastTransmitter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Media_Miracast_MiracastTransmitter_DEFINED
#define RUNTIMECLASS_Windows_Media_Miracast_MiracastTransmitter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Miracast_MiracastTransmitter[] = L"Windows.Media.Miracast.MiracastTransmitter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Emiracast_p_h__

#endif // __windows2Emedia2Emiracast_h__
