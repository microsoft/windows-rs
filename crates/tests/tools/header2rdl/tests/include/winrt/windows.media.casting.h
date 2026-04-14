
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
#ifndef __windows2Emedia2Ecasting_h__
#define __windows2Emedia2Ecasting_h__
#ifndef __windows2Emedia2Ecasting_p_h__
#define __windows2Emedia2Ecasting_p_h__


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
#include "Windows.Devices.Enumeration.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingConnection;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection ABI::Windows::Media::Casting::ICastingConnection

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingConnectionErrorOccurredEventArgs;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs ABI::Windows::Media::Casting::ICastingConnectionErrorOccurredEventArgs

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingDevice;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice ABI::Windows::Media::Casting::ICastingDevice

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingDevicePicker;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker ABI::Windows::Media::Casting::ICastingDevicePicker

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingDevicePickerFilter;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter ABI::Windows::Media::Casting::ICastingDevicePickerFilter

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingDeviceSelectedEventArgs;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs ABI::Windows::Media::Casting::ICastingDeviceSelectedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingDeviceStatics;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics ABI::Windows::Media::Casting::ICastingDeviceStatics

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                interface ICastingSource;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource ABI::Windows::Media::Casting::ICastingSource

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__

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



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                typedef enum CastingConnectionErrorStatus : int CastingConnectionErrorStatus;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("25c821bc-deb8-5850-8af2-d3ac35426bd2"))
IAsyncOperation<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus> : IAsyncOperation_impl<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Casting.CastingConnectionErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus> __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7216a94a-a10a-5763-8e96-bf33c582ed92"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Casting.CastingConnectionErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Casting::CastingConnectionErrorStatus> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingDevice;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b4ee058-4c69-5e70-8834-c1d171cc1b22"))
IAsyncOperation<ABI::Windows::Media::Casting::CastingDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingDevice*, ABI::Windows::Media::Casting::ICastingDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Casting.CastingDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Casting::CastingDevice*> __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f0c69b9e-14cb-510a-8ef0-7e86d771baaf"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Casting::CastingDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingDevice*, ABI::Windows::Media::Casting::ICastingDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Casting.CastingDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Casting::CastingDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                typedef enum CastingPlaybackTypes : unsigned int CastingPlaybackTypes;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dff10e53-4c5e-5dba-9269-cd61881bb8b3"))
IAsyncOperation<enum ABI::Windows::Media::Casting::CastingPlaybackTypes> : IAsyncOperation_impl<enum ABI::Windows::Media::Casting::CastingPlaybackTypes>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Casting.CastingPlaybackTypes>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Casting::CastingPlaybackTypes> __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b55e550c-dfa8-50be-be8f-5d42c9dac120"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Casting::CastingPlaybackTypes> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Casting::CastingPlaybackTypes>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Casting.CastingPlaybackTypes>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Casting::CastingPlaybackTypes> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingSource;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCasting__CCastingSource_USE
#define DEF___FIIterator_1_Windows__CMedia__CCasting__CCastingSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f6c93e9-cc77-5ef4-b2b7-25cfcfc09720"))
IIterator<ABI::Windows::Media::Casting::CastingSource*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingSource*, ABI::Windows::Media::Casting::ICastingSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Casting.CastingSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Casting::CastingSource*> __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_t;
#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCasting__CCastingSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCasting__CCastingSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCasting__CCastingSource_USE
#define DEF___FIIterable_1_Windows__CMedia__CCasting__CCastingSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1abb2cc9-46a2-58b1-91aa-28699d66d1ab"))
IIterable<ABI::Windows::Media::Casting::CastingSource*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingSource*, ABI::Windows::Media::Casting::ICastingSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Casting.CastingSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Casting::CastingSource*> __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_t;
#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCasting__CCastingSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCasting__CCastingSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7828da61-dea4-5485-b27a-8f78e0472402"))
IVectorView<ABI::Windows::Media::Casting::CastingSource*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingSource*, ABI::Windows::Media::Casting::ICastingSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Casting.CastingSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Casting::CastingSource*> __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_t;
#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CCasting__CCastingSource_USE
#define DEF___FIVector_1_Windows__CMedia__CCasting__CCastingSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c2e3dea8-92e0-50af-9c93-83b3e86b73b4"))
IVector<ABI::Windows::Media::Casting::CastingSource*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingSource*, ABI::Windows::Media::Casting::ICastingSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Casting.CastingSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Casting::CastingSource*> __FIVector_1_Windows__CMedia__CCasting__CCastingSource_t;
#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CCasting__CCastingSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CCasting__CCastingSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingConnection;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f1576170-9b08-5a9b-87d2-c1ac47bc6681"))
ITypedEventHandler<ABI::Windows::Media::Casting::CastingConnection*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingConnection*, ABI::Windows::Media::Casting::ICastingConnection*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Casting.CastingConnection, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Casting::CastingConnection*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingConnectionErrorOccurredEventArgs;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("78afbbd0-9811-5f0e-9566-47c3e8cdd929"))
ITypedEventHandler<ABI::Windows::Media::Casting::CastingConnection*, ABI::Windows::Media::Casting::CastingConnectionErrorOccurredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingConnection*, ABI::Windows::Media::Casting::ICastingConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingConnectionErrorOccurredEventArgs*, ABI::Windows::Media::Casting::ICastingConnectionErrorOccurredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Casting.CastingConnection, Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Casting::CastingConnection*, ABI::Windows::Media::Casting::CastingConnectionErrorOccurredEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingDevicePicker;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a64b972b-aa63-5c61-9cde-cd6ffea8a247"))
ITypedEventHandler<ABI::Windows::Media::Casting::CastingDevicePicker*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingDevicePicker*, ABI::Windows::Media::Casting::ICastingDevicePicker*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Casting.CastingDevicePicker, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Casting::CastingDevicePicker*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingDeviceSelectedEventArgs;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b3655b33-c4ad-5f4c-a187-b2e4c770a16b"))
ITypedEventHandler<ABI::Windows::Media::Casting::CastingDevicePicker*, ABI::Windows::Media::Casting::CastingDeviceSelectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingDevicePicker*, ABI::Windows::Media::Casting::ICastingDevicePicker*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Casting::CastingDeviceSelectedEventArgs*, ABI::Windows::Media::Casting::ICastingDeviceSelectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Casting.CastingDevicePicker, Windows.Media.Casting.CastingDeviceSelectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Casting::CastingDevicePicker*, ABI::Windows::Media::Casting::CastingDeviceSelectedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation ABI::Windows::Devices::Enumeration::IDeviceInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DevicePickerAppearance;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDevicePickerAppearance;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance ABI::Windows::Devices::Enumeration::IDevicePickerAppearance

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__

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
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

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
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                typedef enum CastingConnectionState : int CastingConnectionState;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                class CastingDevicePickerFilter;
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Casting.CastingConnectionErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                enum CastingConnectionErrorStatus : int
                {
                    CastingConnectionErrorStatus_Succeeded = 0,
                    CastingConnectionErrorStatus_DeviceDidNotRespond = 1,
                    CastingConnectionErrorStatus_DeviceError = 2,
                    CastingConnectionErrorStatus_DeviceLocked = 3,
                    CastingConnectionErrorStatus_ProtectedPlaybackFailed = 4,
                    CastingConnectionErrorStatus_InvalidCastingSource = 5,
                    CastingConnectionErrorStatus_Unknown = 6,
                };
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Casting.CastingConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                enum CastingConnectionState : int
                {
                    CastingConnectionState_Disconnected = 0,
                    CastingConnectionState_Connected = 1,
                    CastingConnectionState_Rendering = 2,
                    CastingConnectionState_Disconnecting = 3,
                    CastingConnectionState_Connecting = 4,
                };
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Casting.CastingPlaybackTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                enum CastingPlaybackTypes : unsigned int
                {
                    CastingPlaybackTypes_None = 0,
                    CastingPlaybackTypes_Audio = 0x1,
                    CastingPlaybackTypes_Video = 0x2,
                    CastingPlaybackTypes_Picture = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(CastingPlaybackTypes)
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingConnection
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingConnection[] = L"Windows.Media.Casting.ICastingConnection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("cd951653-c2f1-4498-8b78-5fb4cd3640dd")
                ICastingConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::Casting::CastingConnectionState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::Media::Casting::ICastingDevice** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Media::Casting::ICastingSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Source(
                        ABI::Windows::Media::Casting::ICastingSource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ErrorOccurred(
                        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ErrorOccurred(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStartCastingAsync(
                        ABI::Windows::Media::Casting::ICastingSource* value,
                        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisconnectAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingConnection = __uuidof(ICastingConnection);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingConnectionErrorOccurredEventArgs[] = L"Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("a7fb3c69-8719-4f00-81fb-961863c79a32")
                ICastingConnectionErrorOccurredEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ErrorStatus(
                        ABI::Windows::Media::Casting::CastingConnectionErrorStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingConnectionErrorOccurredEventArgs = __uuidof(ICastingConnectionErrorOccurredEventArgs);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevice[] = L"Windows.Media.Casting.ICastingDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("de721c83-4a43-4ad1-a6d2-2492a796c3f2")
                ICastingDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedCastingPlaybackTypesAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCastingConnection(
                        ABI::Windows::Media::Casting::ICastingConnection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingDevice = __uuidof(ICastingDevice);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevicePicker[] = L"Windows.Media.Casting.ICastingDevicePicker";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("dcd39924-0591-49be-aacb-4b82ee756a95")
                ICastingDevicePicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Filter(
                        ABI::Windows::Media::Casting::ICastingDevicePickerFilter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Appearance(
                        ABI::Windows::Devices::Enumeration::IDevicePickerAppearance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CastingDeviceSelected(
                        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CastingDeviceSelected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CastingDevicePickerDismissed(
                        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CastingDevicePickerDismissed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Show(
                        ABI::Windows::Foundation::Rect selection
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowWithPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Hide(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingDevicePicker = __uuidof(ICastingDevicePicker);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevicePickerFilter[] = L"Windows.Media.Casting.ICastingDevicePickerFilter";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("be8c619c-b563-4354-ae33-9fdaad8c6291")
                ICastingDevicePickerFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsAudio(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsAudio(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsVideo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsVideo(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsPictures(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsPictures(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedCastingSources(
                        __FIVector_1_Windows__CMedia__CCasting__CCastingSource** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingDevicePickerFilter = __uuidof(ICastingDevicePickerFilter);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDeviceSelectedEventArgs[] = L"Windows.Media.Casting.ICastingDeviceSelectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("dc439e86-dd57-4d0d-9400-af45e4fb3663")
                ICastingDeviceSelectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedCastingDevice(
                        ABI::Windows::Media::Casting::ICastingDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingDeviceSelectedEventArgs = __uuidof(ICastingDeviceSelectedEventArgs);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDeviceStatics[] = L"Windows.Media.Casting.ICastingDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("e7d958d7-4d13-4237-a365-4c4f6a4cfd2f")
                ICastingDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        ABI::Windows::Media::Casting::CastingPlaybackTypes type,
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorFromCastingSourceAsync(
                        ABI::Windows::Media::Casting::ICastingSource* castingSource,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING value,
                        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeviceInfoSupportsCastingAsync(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingDeviceStatics = __uuidof(ICastingDeviceStatics);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingSource[] = L"Windows.Media.Casting.ICastingSource";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Casting {
                MIDL_INTERFACE("f429ea72-3467-47e6-a027-522923e9d727")
                ICastingSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredSourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PreferredSourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICastingSource = __uuidof(ICastingSource);
            } /* Casting */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingConnection[] = L"Windows.Media.Casting.CastingConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs[] = L"Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Casting.ICastingDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevice[] = L"Windows.Media.Casting.CastingDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevicePicker[] = L"Windows.Media.Casting.CastingDevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevicePickerFilter[] = L"Windows.Media.Casting.CastingDevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDeviceSelectedEventArgs[] = L"Windows.Media.Casting.CastingDeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingSource[] = L"Windows.Media.Casting.CastingSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingConnection __x_ABI_CWindows_CMedia_CCasting_CICastingConnection;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevice __x_ABI_CWindows_CMedia_CCasting_CICastingDevice;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCasting_CICastingSource __x_ABI_CWindows_CMedia_CCasting_CICastingSource;

#endif // ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionErrorStatus __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionErrorStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionErrorStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* This,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice* This,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CCasting_CCastingPlaybackTypes __x_ABI_CWindows_CMedia_CCasting_CCastingPlaybackTypes;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        enum __x_ABI_CWindows_CMedia_CCasting_CCastingPlaybackTypes* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* This,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCasting__CCastingPlaybackTypes_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCasting__CCastingSource __FIIterator_1_Windows__CMedia__CCasting__CCastingSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCasting__CCastingSource;

typedef struct __FIIterator_1_Windows__CMedia__CCasting__CCastingSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCasting__CCastingSourceVtbl;

interface __FIIterator_1_Windows__CMedia__CCasting__CCastingSource
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCasting__CCastingSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCasting__CCastingSource_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCasting__CCastingSource __FIIterable_1_Windows__CMedia__CCasting__CCastingSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCasting__CCastingSource;

typedef struct __FIIterable_1_Windows__CMedia__CCasting__CCastingSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCasting__CCastingSource* This,
        __FIIterator_1_Windows__CMedia__CCasting__CCastingSource** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCasting__CCastingSourceVtbl;

interface __FIIterable_1_Windows__CMedia__CCasting__CCastingSource
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCasting__CCastingSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCasting__CCastingSource_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCasting__CCastingSource;

typedef struct __FIVectorView_1_Windows__CMedia__CCasting__CCastingSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCasting__CCastingSourceVtbl;

interface __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCasting__CCastingSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CCasting__CCastingSource __FIVector_1_Windows__CMedia__CCasting__CCastingSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CCasting__CCastingSource;

typedef struct __FIVector_1_Windows__CMedia__CCasting__CCastingSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        __FIVectorView_1_Windows__CMedia__CCasting__CCastingSource** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CCasting__CCastingSource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CCasting__CCastingSourceVtbl;

interface __FIVector_1_Windows__CMedia__CCasting__CCastingSource
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CCasting__CCastingSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CCasting__CCastingSource_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CCasting__CCastingSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingConnection* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingConnection* sender,
        __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* sender,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionState __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionState;

/*
 *
 * Struct Windows.Media.Casting.CastingConnectionErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionErrorStatus
{
    CastingConnectionErrorStatus_Succeeded = 0,
    CastingConnectionErrorStatus_DeviceDidNotRespond = 1,
    CastingConnectionErrorStatus_DeviceError = 2,
    CastingConnectionErrorStatus_DeviceLocked = 3,
    CastingConnectionErrorStatus_ProtectedPlaybackFailed = 4,
    CastingConnectionErrorStatus_InvalidCastingSource = 5,
    CastingConnectionErrorStatus_Unknown = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Casting.CastingConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionState
{
    CastingConnectionState_Disconnected = 0,
    CastingConnectionState_Connected = 1,
    CastingConnectionState_Rendering = 2,
    CastingConnectionState_Disconnecting = 3,
    CastingConnectionState_Connecting = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Casting.CastingPlaybackTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CCasting_CCastingPlaybackTypes
{
    CastingPlaybackTypes_None = 0,
    CastingPlaybackTypes_Audio = 0x1,
    CastingPlaybackTypes_Video = 0x2,
    CastingPlaybackTypes_Picture = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingConnection
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingConnection[] = L"Windows.Media.Casting.ICastingConnection";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionState* value);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevice** value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource** value);
    HRESULT (STDMETHODCALLTYPE* put_Source)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ErrorOccurred)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingConnection_Windows__CMedia__CCasting__CCastingConnectionErrorOccurredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ErrorOccurred)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* RequestStartCastingAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* value,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* DisconnectAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnection* This,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingConnectionErrorStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_put_Source(This, value) \
    ((This)->lpVtbl->put_Source(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_add_ErrorOccurred(This, handler, token) \
    ((This)->lpVtbl->add_ErrorOccurred(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_remove_ErrorOccurred(This, token) \
    ((This)->lpVtbl->remove_ErrorOccurred(This, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_RequestStartCastingAsync(This, value, operation) \
    ((This)->lpVtbl->RequestStartCastingAsync(This, value, operation))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnection_DisconnectAsync(This, operation) \
    ((This)->lpVtbl->DisconnectAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingConnectionErrorOccurredEventArgs[] = L"Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorStatus)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CCasting_CCastingConnectionErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_get_ErrorStatus(This, value) \
    ((This)->lpVtbl->get_ErrorStatus(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingConnectionErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevice[] = L"Windows.Media.Casting.ICastingDevice";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
    HRESULT (STDMETHODCALLTYPE* GetSupportedCastingPlaybackTypesAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingPlaybackTypes** operation);
    HRESULT (STDMETHODCALLTYPE* CreateCastingConnection)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevice* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_GetSupportedCastingPlaybackTypesAsync(This, operation) \
    ((This)->lpVtbl->GetSupportedCastingPlaybackTypesAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevice_CreateCastingConnection(This, value) \
    ((This)->lpVtbl->CreateCastingConnection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevicePicker[] = L"Windows.Media.Casting.ICastingDevicePicker";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Filter)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter** value);
    HRESULT (STDMETHODCALLTYPE* get_Appearance)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance** value);
    HRESULT (STDMETHODCALLTYPE* add_CastingDeviceSelected)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_Windows__CMedia__CCasting__CCastingDeviceSelectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CastingDeviceSelected)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CastingDevicePickerDismissed)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        __FITypedEventHandler_2_Windows__CMedia__CCasting__CCastingDevicePicker_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CastingDevicePickerDismissed)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection);
    HRESULT (STDMETHODCALLTYPE* ShowWithPlacement)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement);
    HRESULT (STDMETHODCALLTYPE* Hide)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_get_Filter(This, value) \
    ((This)->lpVtbl->get_Filter(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_get_Appearance(This, value) \
    ((This)->lpVtbl->get_Appearance(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_add_CastingDeviceSelected(This, handler, token) \
    ((This)->lpVtbl->add_CastingDeviceSelected(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_remove_CastingDeviceSelected(This, token) \
    ((This)->lpVtbl->remove_CastingDeviceSelected(This, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_add_CastingDevicePickerDismissed(This, handler, token) \
    ((This)->lpVtbl->add_CastingDevicePickerDismissed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_remove_CastingDevicePickerDismissed(This, token) \
    ((This)->lpVtbl->remove_CastingDevicePickerDismissed(This, token))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_Show(This, selection) \
    ((This)->lpVtbl->Show(This, selection))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_ShowWithPlacement(This, selection, preferredPlacement) \
    ((This)->lpVtbl->ShowWithPlacement(This, selection, preferredPlacement))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_Hide(This) \
    ((This)->lpVtbl->Hide(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDevicePickerFilter[] = L"Windows.Media.Casting.ICastingDevicePickerFilter";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportsAudio)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsAudio)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsVideo)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsVideo)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsPictures)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsPictures)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedCastingSources)(__x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter* This,
        __FIVector_1_Windows__CMedia__CCasting__CCastingSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilterVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_get_SupportsAudio(This, value) \
    ((This)->lpVtbl->get_SupportsAudio(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_put_SupportsAudio(This, value) \
    ((This)->lpVtbl->put_SupportsAudio(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_get_SupportsVideo(This, value) \
    ((This)->lpVtbl->get_SupportsVideo(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_put_SupportsVideo(This, value) \
    ((This)->lpVtbl->put_SupportsVideo(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_get_SupportsPictures(This, value) \
    ((This)->lpVtbl->get_SupportsPictures(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_put_SupportsPictures(This, value) \
    ((This)->lpVtbl->put_SupportsPictures(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_get_SupportedCastingSources(This, value) \
    ((This)->lpVtbl->get_SupportedCastingSources(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDeviceSelectedEventArgs[] = L"Windows.Media.Casting.ICastingDeviceSelectedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedCastingDevice)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_get_SelectedCastingDevice(This, value) \
    ((This)->lpVtbl->get_SelectedCastingDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingDeviceStatics[] = L"Windows.Media.Casting.ICastingDeviceStatics";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        enum __x_ABI_CWindows_CMedia_CCasting_CCastingPlaybackTypes type,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorFromCastingSourceAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        __x_ABI_CWindows_CMedia_CCasting_CICastingSource* castingSource,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        HSTRING value,
        __FIAsyncOperation_1_Windows__CMedia__CCasting__CCastingDevice** operation);
    HRESULT (STDMETHODCALLTYPE* DeviceInfoSupportsCastingAsync)(__x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_GetDeviceSelector(This, type, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, type, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_GetDeviceSelectorFromCastingSourceAsync(This, castingSource, operation) \
    ((This)->lpVtbl->GetDeviceSelectorFromCastingSourceAsync(This, castingSource, operation))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_FromIdAsync(This, value, operation) \
    ((This)->lpVtbl->FromIdAsync(This, value, operation))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_DeviceInfoSupportsCastingAsync(This, device, operation) \
    ((This)->lpVtbl->DeviceInfoSupportsCastingAsync(This, device, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Casting.ICastingSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Casting.CastingSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Casting_ICastingSource[] = L"Windows.Media.Casting.ICastingSource";
typedef struct __x_ABI_CWindows_CMedia_CCasting_CICastingSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PreferredSourceUri)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_PreferredSourceUri)(__x_ABI_CWindows_CMedia_CCasting_CICastingSource* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCasting_CICastingSourceVtbl;

interface __x_ABI_CWindows_CMedia_CCasting_CICastingSource
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCasting_CICastingSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_get_PreferredSourceUri(This, value) \
    ((This)->lpVtbl->get_PreferredSourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CCasting_CICastingSource_put_PreferredSourceUri(This, value) \
    ((This)->lpVtbl->put_PreferredSourceUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCasting_CICastingSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCasting_CICastingSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingConnection[] = L"Windows.Media.Casting.CastingConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingConnectionErrorOccurredEventArgs[] = L"Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Casting.ICastingDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevice[] = L"Windows.Media.Casting.CastingDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevicePicker[] = L"Windows.Media.Casting.CastingDevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDevicePickerFilter[] = L"Windows.Media.Casting.CastingDevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingDeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingDeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingDeviceSelectedEventArgs[] = L"Windows.Media.Casting.CastingDeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Casting.CastingSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Casting.ICastingSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Casting_CastingSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Casting_CastingSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Casting_CastingSource[] = L"Windows.Media.Casting.CastingSource";
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
#endif // __windows2Emedia2Ecasting_p_h__

#endif // __windows2Emedia2Ecasting_h__
