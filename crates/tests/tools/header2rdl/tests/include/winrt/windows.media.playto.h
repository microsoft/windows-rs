
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
#ifndef __windows2Emedia2Eplayto_h__
#define __windows2Emedia2Eplayto_h__
#ifndef __windows2Emedia2Eplayto_p_h__
#define __windows2Emedia2Eplayto_p_h__


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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface ICurrentTimeChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs ABI::Windows::Media::PlayTo::ICurrentTimeChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IMuteChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs ABI::Windows::Media::PlayTo::IMuteChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToConnection;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection ABI::Windows::Media::PlayTo::IPlayToConnection

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToConnectionErrorEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs ABI::Windows::Media::PlayTo::IPlayToConnectionErrorEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToConnectionStateChangedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs ABI::Windows::Media::PlayTo::IPlayToConnectionStateChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToConnectionTransferredEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs ABI::Windows::Media::PlayTo::IPlayToConnectionTransferredEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToManager;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager ABI::Windows::Media::PlayTo::IPlayToManager

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToManagerStatics;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics ABI::Windows::Media::PlayTo::IPlayToManagerStatics

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToReceiver;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver ABI::Windows::Media::PlayTo::IPlayToReceiver

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSource;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource ABI::Windows::Media::PlayTo::IPlayToSource

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSourceDeferral;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral ABI::Windows::Media::PlayTo::IPlayToSourceDeferral

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSourceRequest;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest ABI::Windows::Media::PlayTo::IPlayToSourceRequest

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSourceRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs ABI::Windows::Media::PlayTo::IPlayToSourceRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSourceSelectedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs ABI::Windows::Media::PlayTo::IPlayToSourceSelectedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlayToSourceWithPreferredSourceUri;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri ABI::Windows::Media::PlayTo::IPlayToSourceWithPreferredSourceUri

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IPlaybackRateChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs ABI::Windows::Media::PlayTo::IPlaybackRateChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface ISourceChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs ABI::Windows::Media::PlayTo::ISourceChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                interface IVolumeChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs ABI::Windows::Media::PlayTo::IVolumeChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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



#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToConnection;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToConnectionErrorEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8f599f3d-3655-5993-a119-741ec4ee6e42"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionErrorEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::IPlayToConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnectionErrorEventArgs*, ABI::Windows::Media::PlayTo::IPlayToConnectionErrorEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToConnection, Windows.Media.PlayTo.PlayToConnectionErrorEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionErrorEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToConnectionStateChangedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("def77c1c-9363-5b47-b9f8-a236c50d372e"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::IPlayToConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnectionStateChangedEventArgs*, ABI::Windows::Media::PlayTo::IPlayToConnectionStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToConnection, Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToConnectionTransferredEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7278e7f2-0360-5571-8c3e-ea032767c473"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionTransferredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::IPlayToConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToConnectionTransferredEventArgs*, ABI::Windows::Media::PlayTo::IPlayToConnectionTransferredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToConnection, Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToConnection*, ABI::Windows::Media::PlayTo::PlayToConnectionTransferredEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToManager;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToSourceRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ac9f3a4-39b6-51d8-8e08-f137908e922f"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::PlayToSourceRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::IPlayToManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToSourceRequestedEventArgs*, ABI::Windows::Media::PlayTo::IPlayToSourceRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToManager, Windows.Media.PlayTo.PlayToSourceRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::PlayToSourceRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToSourceSelectedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bf87431a-7451-52a5-a5e2-bfaf408e0c88"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::PlayToSourceSelectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::IPlayToManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToSourceSelectedEventArgs*, ABI::Windows::Media::PlayTo::IPlayToSourceSelectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToManager, Windows.Media.PlayTo.PlayToSourceSelectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToManager*, ABI::Windows::Media::PlayTo::PlayToSourceSelectedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToReceiver;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7a89b9c2-78d4-5866-88f1-4cdc0a0e75b8"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class CurrentTimeChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("42abdf9f-14a6-5c1f-839b-86029505b1d0"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::CurrentTimeChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::CurrentTimeChangeRequestedEventArgs*, ABI::Windows::Media::PlayTo::ICurrentTimeChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::CurrentTimeChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class MuteChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0674cf85-b6fd-5926-9b72-35fe517a3f44"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::MuteChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::MuteChangeRequestedEventArgs*, ABI::Windows::Media::PlayTo::IMuteChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Windows.Media.PlayTo.MuteChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::MuteChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlaybackRateChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e7594396-58c3-5316-bc4f-1d13eadbeffd"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::PlaybackRateChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlaybackRateChangeRequestedEventArgs*, ABI::Windows::Media::PlayTo::IPlaybackRateChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::PlaybackRateChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class SourceChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1466d074-b7b4-5814-b2d7-847c5ae7d87d"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::SourceChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::SourceChangeRequestedEventArgs*, ABI::Windows::Media::PlayTo::ISourceChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Windows.Media.PlayTo.SourceChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::SourceChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class VolumeChangeRequestedEventArgs;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9c3f7c84-9b4d-5cdc-8144-1c2f617b6ede"))
ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::VolumeChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::IPlayToReceiver*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlayTo::VolumeChangeRequestedEventArgs*, ABI::Windows::Media::PlayTo::IVolumeChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.PlayTo.PlayToReceiver, Windows.Media.PlayTo.VolumeChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::PlayTo::PlayToReceiver*, ABI::Windows::Media::PlayTo::VolumeChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

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
            typedef struct TimeSpan TimeSpan;
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

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
            namespace PlayTo {
                typedef enum PlayToConnectionError : int PlayToConnectionError;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                typedef enum PlayToConnectionState : int PlayToConnectionState;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToSource;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToSourceDeferral;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                class PlayToSourceRequest;
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.PlayTo.PlayToConnectionError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnectionError may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorStatus.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                PlayToConnectionError : int
                {
                    PlayToConnectionError_None = 0,
                    PlayToConnectionError_DeviceNotResponding = 1,
                    PlayToConnectionError_DeviceError = 2,
                    PlayToConnectionError_DeviceLocked = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    PlayToConnectionError_ProtectedPlaybackFailed = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.PlayTo.PlayToConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnectionState may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionState.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                PlayToConnectionState : int
                {
                    PlayToConnectionState_Disconnected = 0,
                    PlayToConnectionState_Connected = 1,
                    PlayToConnectionState_Rendering = 2,
                };
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_ICurrentTimeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("99711324-edc7-4bf5-91f6-3c8627db59e5")
                ICurrentTimeChangeRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Time(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICurrentTimeChangeRequestedEventArgs = __uuidof(ICurrentTimeChangeRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IMuteChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.MuteChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IMuteChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IMuteChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("e4b4f5f6-af1f-4f1e-b437-7da32400e1d4")
                IMuteChangeRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Mute(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMuteChangeRequestedEventArgs = __uuidof(IMuteChangeRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnection[] = L"Windows.Media.PlayTo.IPlayToConnection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("112fbfc8-f235-4fde-8d41-9bf27c9e9a40")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToConnection : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::PlayTo::PlayToConnectionState* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_Transferred(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_Transferred(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_Error(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_Error(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToConnection = __uuidof(IPlayToConnection);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionErrorEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionErrorEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("bf5eada6-88e6-445f-9d40-d9b9f8939896")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToConnectionErrorEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Code(
                        ABI::Windows::Media::PlayTo::PlayToConnectionError* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToConnectionErrorEventArgs = __uuidof(IPlayToConnectionErrorEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionStateChangedEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("68c4b50f-0c20-4980-8602-58c62238d423")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToConnectionStateChangedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PreviousState(
                        ABI::Windows::Media::PlayTo::PlayToConnectionState* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentState(
                        ABI::Windows::Media::PlayTo::PlayToConnectionState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToConnectionStateChangedEventArgs = __uuidof(IPlayToConnectionStateChangedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionTransferredEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("fae3193a-0683-47d9-8df0-18cbb48984d8")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToConnectionTransferredEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PreviousSource(
                        ABI::Windows::Media::PlayTo::IPlayToSource** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentSource(
                        ABI::Windows::Media::PlayTo::IPlayToSource** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToConnectionTransferredEventArgs = __uuidof(IPlayToConnectionTransferredEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToManager[] = L"Windows.Media.PlayTo.IPlayToManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("f56a206e-1b77-42ef-8f0d-b949f8d9b260")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToManager : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_SourceRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_SourceRequested(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_SourceSelected(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_SourceSelected(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultSourceSelection(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultSourceSelection(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToManager = __uuidof(IPlayToManager);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToManagerStatics[] = L"Windows.Media.PlayTo.IPlayToManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("64e6a887-3982-4f3b-ba20-6155e435325b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Media::PlayTo::IPlayToManager** playToManager
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ShowPlayToUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToManagerStatics = __uuidof(IPlayToManagerStatics);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToReceiver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToReceiver[] = L"Windows.Media.PlayTo.IPlayToReceiver";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("ac15cf47-a162-4aa6-af1b-3aa35f3b9069")
                IPlayToReceiver : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_PlayRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PlayRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PauseRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PauseRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SourceChangeRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SourceChangeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PlaybackRateChangeRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PlaybackRateChangeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CurrentTimeChangeRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CurrentTimeChangeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MuteChangeRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MuteChangeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_VolumeChangeRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VolumeChangeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TimeUpdateRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TimeUpdateRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StopRequested(
                        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StopRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyVolumeChange(
                        DOUBLE volume,
                        boolean mute
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyRateChange(
                        DOUBLE rate
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyLoadedMetadata(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyTimeUpdate(
                        ABI::Windows::Foundation::TimeSpan currentTime
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyDurationChange(
                        ABI::Windows::Foundation::TimeSpan duration
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifySeeking(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifySeeked(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyPaused(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyPlaying(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyEnded(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyError(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyStopped(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsImage(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsImage(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsAudio(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsAudio(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SupportsVideo(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsVideo(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopAsync(
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToReceiver = __uuidof(IPlayToReceiver);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSource[] = L"Windows.Media.PlayTo.IPlayToSource";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("7f138a08-fbb7-4b09-8356-aa5f4e335c31")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSource : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Connection(
                        ABI::Windows::Media::PlayTo::IPlayToConnection** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Next(
                        ABI::Windows::Media::PlayTo::IPlayToSource** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Next(
                        ABI::Windows::Media::PlayTo::IPlayToSource* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE PlayNext(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSource = __uuidof(IPlayToSource);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceDeferral[] = L"Windows.Media.PlayTo.IPlayToSourceDeferral";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("4100891d-278e-4f29-859b-a9e501053e7d")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSourceDeferral : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSourceDeferral = __uuidof(IPlayToSourceDeferral);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceRequest[] = L"Windows.Media.PlayTo.IPlayToSourceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("f8584665-64f4-44a0-ac0d-468d2b8fda83")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSourceRequest : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE DisplayErrorString(
                        HSTRING errorString
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Media::PlayTo::IPlayToSourceDeferral** deferral
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetSource(
                        ABI::Windows::Media::PlayTo::IPlayToSource* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSourceRequest = __uuidof(IPlayToSourceRequest);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceRequestedEventArgs[] = L"Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("c5cdc330-29df-4ec6-9da9-9fbdfcfc1b3e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSourceRequestedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SourceRequest(
                        ABI::Windows::Media::PlayTo::IPlayToSourceRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSourceRequestedEventArgs = __uuidof(IPlayToSourceRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceSelectedEventArgs[] = L"Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("0c9d8511-5202-4dcb-8c67-abda12bb3c12")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSourceSelectedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsImage(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsAudio(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsVideo(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSourceSelectedEventArgs = __uuidof(IPlayToSourceSelectedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceWithPreferredSourceUri[] = L"Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("aab253eb-3301-4dc4-afba-b2f2ed9635a0")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IPlayToSourceWithPreferredSourceUri : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredSourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_PreferredSourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlayToSourceWithPreferredSourceUri = __uuidof(IPlayToSourceWithPreferredSourceUri);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("0f5661ae-2c88-4cca-8540-d586095d13a5")
                IPlaybackRateChangeRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Rate(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlaybackRateChangeRequestedEventArgs = __uuidof(IPlaybackRateChangeRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.ISourceChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.SourceChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_ISourceChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.ISourceChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("fb3f3a96-7aa6-4a8b-86e7-54f6c6d34f64")
                ISourceChangeRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Stream(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Author(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Album(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Genre(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Date(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rating(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISourceChangeRequestedEventArgs = __uuidof(ISourceChangeRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.VolumeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IVolumeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace PlayTo {
                MIDL_INTERFACE("6f026d5c-cf75-4c2b-913e-6d7c6c329179")
                IVolumeChangeRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Volume(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVolumeChangeRequestedEventArgs = __uuidof(IVolumeChangeRequestedEventArgs);
            } /* PlayTo */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.MuteChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IMuteChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_MuteChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_MuteChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_MuteChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnection_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnection[] = L"Windows.Media.PlayTo.PlayToConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionErrorEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.PlayTo.IPlayToManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToManager_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToManager[] = L"Windows.Media.PlayTo.PlayToManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToReceiver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToReceiver_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToReceiver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToReceiver[] = L"Windows.Media.PlayTo.PlayToReceiver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSource ** Default Interface **
 *    Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSource_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSource_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSource[] = L"Windows.Media.PlayTo.PlayToSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceDeferral_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceDeferral_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceDeferral[] = L"Windows.Media.PlayTo.PlayToSourceDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequest_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceRequest[] = L"Windows.Media.PlayTo.PlayToSourceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs[] = L"Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs[] = L"Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.SourceChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.ISourceChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_SourceChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_SourceChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_SourceChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.VolumeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* sender,
        __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionError __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionError;

typedef enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState;

/*
 *
 * Struct Windows.Media.PlayTo.PlayToConnectionError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionError may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorStatus.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionError
{
    PlayToConnectionError_None = 0,
    PlayToConnectionError_DeviceNotResponding = 1,
    PlayToConnectionError_DeviceError = 2,
    PlayToConnectionError_DeviceLocked = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    PlayToConnectionError_ProtectedPlaybackFailed = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.PlayTo.PlayToConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionState may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionState.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState
{
    PlayToConnectionState_Disconnected = 0,
    PlayToConnectionState_Connected = 1,
    PlayToConnectionState_Rendering = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_ICurrentTimeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Time)(__x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_get_Time(This, value) \
    ((This)->lpVtbl->get_Time(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CICurrentTimeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IMuteChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.MuteChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IMuteChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IMuteChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mute)(__x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_get_Mute(This, value) \
    ((This)->lpVtbl->get_Mute(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIMuteChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnection[] = L"Windows.Media.PlayTo.IPlayToConnection";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionStateChangedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_Transferred)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionTransferredEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_Transferred)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_Error)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToConnection_Windows__CMedia__CPlayTo__CPlayToConnectionErrorEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_Error)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_add_Transferred(This, handler, token) \
    ((This)->lpVtbl->add_Transferred(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_remove_Transferred(This, token) \
    ((This)->lpVtbl->remove_Transferred(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_add_Error(This, handler, token) \
    ((This)->lpVtbl->add_Error(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_remove_Error(This, token) \
    ((This)->lpVtbl->remove_Error(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionErrorEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionErrorEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Code)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionError* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_get_Code(This, value) \
    ((This)->lpVtbl->get_Code(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionErrorEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionStateChangedEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PreviousState)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_CurrentState)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CPlayTo_CPlayToConnectionState* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_get_PreviousState(This, value) \
    ((This)->lpVtbl->get_PreviousState(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_get_CurrentState(This, value) \
    ((This)->lpVtbl->get_CurrentState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToConnectionTransferredEventArgs[] = L"Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PreviousSource)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_CurrentSource)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_get_PreviousSource(This, value) \
    ((This)->lpVtbl->get_PreviousSource(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_get_CurrentSource(This, value) \
    ((This)->lpVtbl->get_CurrentSource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnectionTransferredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToManager[] = L"Windows.Media.PlayTo.IPlayToManager";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SourceRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceRequestedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SourceRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SourceSelected)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToManager_Windows__CMedia__CPlayTo__CPlayToSourceSelectedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SourceSelected)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_DefaultSourceSelection)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_DefaultSourceSelection)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_add_SourceRequested(This, handler, token) \
    ((This)->lpVtbl->add_SourceRequested(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_remove_SourceRequested(This, token) \
    ((This)->lpVtbl->remove_SourceRequested(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_add_SourceSelected(This, handler, token) \
    ((This)->lpVtbl->add_SourceSelected(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_remove_SourceSelected(This, token) \
    ((This)->lpVtbl->remove_SourceSelected(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_put_DefaultSourceSelection(This, value) \
    ((This)->lpVtbl->put_DefaultSourceSelection(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_get_DefaultSourceSelection(This, value) \
    ((This)->lpVtbl->get_DefaultSourceSelection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToManagerStatics[] = L"Windows.Media.PlayTo.IPlayToManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManager** playToManager);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ShowPlayToUI)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_GetForCurrentView(This, playToManager) \
    ((This)->lpVtbl->GetForCurrentView(This, playToManager))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_ShowPlayToUI(This) \
    ((This)->lpVtbl->ShowPlayToUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToReceiver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToReceiver[] = L"Windows.Media.PlayTo.IPlayToReceiver";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PlayRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PlayRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PauseRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PauseRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SourceChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CSourceChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SourceChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PlaybackRateChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CPlaybackRateChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PlaybackRateChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CurrentTimeChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CCurrentTimeChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CurrentTimeChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_MuteChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CMuteChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MuteChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_VolumeChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_Windows__CMedia__CPlayTo__CVolumeChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VolumeChangeRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TimeUpdateRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TimeUpdateRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StopRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __FITypedEventHandler_2_Windows__CMedia__CPlayTo__CPlayToReceiver_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StopRequested)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* NotifyVolumeChange)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        DOUBLE volume,
        boolean mute);
    HRESULT (STDMETHODCALLTYPE* NotifyRateChange)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        DOUBLE rate);
    HRESULT (STDMETHODCALLTYPE* NotifyLoadedMetadata)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyTimeUpdate)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan currentTime);
    HRESULT (STDMETHODCALLTYPE* NotifyDurationChange)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration);
    HRESULT (STDMETHODCALLTYPE* NotifySeeking)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifySeeked)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyPaused)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyPlaying)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyEnded)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyError)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* NotifyStopped)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_FriendlyName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsImage)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsImage)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsAudio)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsAudio)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SupportsVideo)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsVideo)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiverVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_PlayRequested(This, handler, token) \
    ((This)->lpVtbl->add_PlayRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_PlayRequested(This, token) \
    ((This)->lpVtbl->remove_PlayRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_PauseRequested(This, handler, token) \
    ((This)->lpVtbl->add_PauseRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_PauseRequested(This, token) \
    ((This)->lpVtbl->remove_PauseRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_SourceChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_SourceChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_SourceChangeRequested(This, token) \
    ((This)->lpVtbl->remove_SourceChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_PlaybackRateChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_PlaybackRateChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_PlaybackRateChangeRequested(This, token) \
    ((This)->lpVtbl->remove_PlaybackRateChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_CurrentTimeChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_CurrentTimeChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_CurrentTimeChangeRequested(This, token) \
    ((This)->lpVtbl->remove_CurrentTimeChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_MuteChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_MuteChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_MuteChangeRequested(This, token) \
    ((This)->lpVtbl->remove_MuteChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_VolumeChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_VolumeChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_VolumeChangeRequested(This, token) \
    ((This)->lpVtbl->remove_VolumeChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_TimeUpdateRequested(This, handler, token) \
    ((This)->lpVtbl->add_TimeUpdateRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_TimeUpdateRequested(This, token) \
    ((This)->lpVtbl->remove_TimeUpdateRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_add_StopRequested(This, handler, token) \
    ((This)->lpVtbl->add_StopRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_remove_StopRequested(This, token) \
    ((This)->lpVtbl->remove_StopRequested(This, token))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyVolumeChange(This, volume, mute) \
    ((This)->lpVtbl->NotifyVolumeChange(This, volume, mute))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyRateChange(This, rate) \
    ((This)->lpVtbl->NotifyRateChange(This, rate))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyLoadedMetadata(This) \
    ((This)->lpVtbl->NotifyLoadedMetadata(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyTimeUpdate(This, currentTime) \
    ((This)->lpVtbl->NotifyTimeUpdate(This, currentTime))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyDurationChange(This, duration) \
    ((This)->lpVtbl->NotifyDurationChange(This, duration))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifySeeking(This) \
    ((This)->lpVtbl->NotifySeeking(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifySeeked(This) \
    ((This)->lpVtbl->NotifySeeked(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyPaused(This) \
    ((This)->lpVtbl->NotifyPaused(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyPlaying(This) \
    ((This)->lpVtbl->NotifyPlaying(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyEnded(This) \
    ((This)->lpVtbl->NotifyEnded(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyError(This) \
    ((This)->lpVtbl->NotifyError(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_NotifyStopped(This) \
    ((This)->lpVtbl->NotifyStopped(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_put_FriendlyName(This, value) \
    ((This)->lpVtbl->put_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_put_SupportsImage(This, value) \
    ((This)->lpVtbl->put_SupportsImage(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_get_SupportsImage(This, value) \
    ((This)->lpVtbl->get_SupportsImage(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_put_SupportsAudio(This, value) \
    ((This)->lpVtbl->put_SupportsAudio(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_get_SupportsAudio(This, value) \
    ((This)->lpVtbl->get_SupportsAudio(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_put_SupportsVideo(This, value) \
    ((This)->lpVtbl->put_SupportsVideo(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_get_SupportsVideo(This, value) \
    ((This)->lpVtbl->get_SupportsVideo(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_StartAsync(This, action) \
    ((This)->lpVtbl->StartAsync(This, action))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_StopAsync(This, action) \
    ((This)->lpVtbl->StopAsync(This, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToReceiver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSource[] = L"Windows.Media.PlayTo.IPlayToSource";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToConnection** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Next)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Next)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* PlayNext)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_get_Next(This, value) \
    ((This)->lpVtbl->get_Next(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_put_Next(This, value) \
    ((This)->lpVtbl->put_Next(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_PlayNext(This) \
    ((This)->lpVtbl->PlayNext(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceDeferral[] = L"Windows.Media.PlayTo.IPlayToSourceDeferral";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferralVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceRequest[] = L"Windows.Media.PlayTo.IPlayToSourceRequest";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* DisplayErrorString)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        HSTRING errorString);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceDeferral** deferral);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetSource)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSource* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_DisplayErrorString(This, errorString) \
    ((This)->lpVtbl->DisplayErrorString(This, errorString))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_SetSource(This, value) \
    ((This)->lpVtbl->SetSource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceRequestedEventArgs[] = L"Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SourceRequest)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_get_SourceRequest(This, value) \
    ((This)->lpVtbl->get_SourceRequest(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSourceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceSelectedEventArgs[] = L"Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SupportsImage)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SupportsAudio)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SupportsVideo)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_get_SupportsImage(This, value) \
    ((This)->lpVtbl->get_SupportsImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_get_SupportsAudio(This, value) \
    ((This)->lpVtbl->get_SupportsAudio(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_get_SupportsVideo(This, value) \
    ((This)->lpVtbl->get_SupportsVideo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlayToSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlayToSourceWithPreferredSourceUri[] = L"Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PreferredSourceUri)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_PreferredSourceUri)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUriVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_get_PreferredSourceUri(This, value) \
    ((This)->lpVtbl->get_PreferredSourceUri(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("PlayToSourceWithPreferredSourceUri may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_put_PreferredSourceUri(This, value) \
    ((This)->lpVtbl->put_PreferredSourceUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlayToSourceWithPreferredSourceUri_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IPlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Rate)(__x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_get_Rate(This, value) \
    ((This)->lpVtbl->get_Rate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.ISourceChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.SourceChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_ISourceChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.ISourceChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Stream)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Album)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Genre)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Date)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_Rating)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Stream(This, value) \
    ((This)->lpVtbl->get_Stream(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Album(This, value) \
    ((This)->lpVtbl->get_Album(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Genre(This, value) \
    ((This)->lpVtbl->get_Genre(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Date(This, value) \
    ((This)->lpVtbl->get_Date(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Rating(This, value) \
    ((This)->lpVtbl->get_Rating(This, value))

#define __x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CISourceChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlayTo.VolumeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_PlayTo_IVolumeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Volume)(__x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_get_Volume(This, value) \
    ((This)->lpVtbl->get_Volume(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CPlayTo_CIVolumeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_CurrentTimeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.MuteChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IMuteChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_MuteChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_MuteChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_MuteChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnection_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnection may be altered or unavailable for releases after Windows 10. Instead, use CastingConnection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnection[] = L"Windows.Media.PlayTo.PlayToConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionErrorEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionErrorEventArgs may be altered or unavailable for releases after Windows 10. Instead, use CastingConnectionErrorOccurredEventArgs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionErrorEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionStateChangedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionStateChangedEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToConnectionTransferredEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToConnectionTransferredEventArgs[] = L"Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.PlayTo.IPlayToManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToManager_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToManager may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToManager[] = L"Windows.Media.PlayTo.PlayToManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToReceiver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToReceiver ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToReceiver_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToReceiver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToReceiver[] = L"Windows.Media.PlayTo.PlayToReceiver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSource ** Default Interface **
 *    Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSource_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSource_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSource may be altered or unavailable for releases after Windows 10. Instead, use CastingSource.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSource[] = L"Windows.Media.PlayTo.PlayToSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceDeferral_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceDeferral_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceDeferral may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceDeferral[] = L"Windows.Media.PlayTo.PlayToSourceDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequest_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequest may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceRequest[] = L"Windows.Media.PlayTo.PlayToSourceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceRequestedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceRequestedEventArgs[] = L"Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlayToSourceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("PlayToSourceSelectedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlayToSourceSelectedEventArgs[] = L"Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_PlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.SourceChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.ISourceChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_SourceChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_SourceChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_SourceChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlayTo.VolumeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlayTo_VolumeChangeRequestedEventArgs[] = L"Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
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
#endif // __windows2Emedia2Eplayto_p_h__

#endif // __windows2Emedia2Eplayto_h__
