
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
#ifndef __windows2Emedia2Edialprotocol_h__
#define __windows2Emedia2Edialprotocol_h__
#ifndef __windows2Emedia2Edialprotocol_p_h__
#define __windows2Emedia2Edialprotocol_p_h__


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
#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialApp;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp ABI::Windows::Media::DialProtocol::IDialApp

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialAppStateDetails;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails ABI::Windows::Media::DialProtocol::IDialAppStateDetails

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDevice;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice ABI::Windows::Media::DialProtocol::IDialDevice

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDevice2;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2 ABI::Windows::Media::DialProtocol::IDialDevice2

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDevicePicker;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker ABI::Windows::Media::DialProtocol::IDialDevicePicker

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDevicePickerFilter;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter ABI::Windows::Media::DialProtocol::IDialDevicePickerFilter

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDeviceSelectedEventArgs;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs ABI::Windows::Media::DialProtocol::IDialDeviceSelectedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDeviceStatics;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics ABI::Windows::Media::DialProtocol::IDialDeviceStatics

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialDisconnectButtonClickedEventArgs;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs ABI::Windows::Media::DialProtocol::IDialDisconnectButtonClickedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialReceiverApp;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp ABI::Windows::Media::DialProtocol::IDialReceiverApp

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialReceiverApp2;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2 ABI::Windows::Media::DialProtocol::IDialReceiverApp2

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                interface IDialReceiverAppStatics;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics ABI::Windows::Media::DialProtocol::IDialReceiverAppStatics

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__

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



#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("84e30b9c-351d-5fcb-8b0a-bc145407f915"))
IAsyncOperation<__FIMap_2_HSTRING_HSTRING*> : IAsyncOperation_impl<__FIMap_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMap`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMap_2_HSTRING_HSTRING*> __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_t;
#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("39bb624e-b5c6-5785-ba46-3f45aaf3ef35"))
IAsyncOperationCompletedHandler<__FIMap_2_HSTRING_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIMap_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMap`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMap_2_HSTRING_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                typedef enum DialAppLaunchResult : int DialAppLaunchResult;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4512c15e-1fc4-5648-bd49-51533a3fe6b4"))
IAsyncOperation<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult> : IAsyncOperation_impl<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.DialProtocol.DialAppLaunchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult> __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("164c0aa8-3d2b-579b-94a3-cc4925c695ec"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.DialProtocol.DialAppLaunchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::DialProtocol::DialAppLaunchResult> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialAppStateDetails;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e50a07a2-2cef-5fc7-b14c-d6dd8517c58e"))
IAsyncOperation<ABI::Windows::Media::DialProtocol::DialAppStateDetails*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialAppStateDetails*, ABI::Windows::Media::DialProtocol::IDialAppStateDetails*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.DialProtocol.DialAppStateDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::DialProtocol::DialAppStateDetails*> __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_t;
#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("38c62dd5-1f16-55c0-8ec7-ca0fc841d614"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::DialProtocol::DialAppStateDetails*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialAppStateDetails*, ABI::Windows::Media::DialProtocol::IDialAppStateDetails*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.DialProtocol.DialAppStateDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::DialProtocol::DialAppStateDetails*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                typedef enum DialAppStopResult : int DialAppStopResult;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8598f24e-0d62-517a-961c-31fca73acedd"))
IAsyncOperation<enum ABI::Windows::Media::DialProtocol::DialAppStopResult> : IAsyncOperation_impl<enum ABI::Windows::Media::DialProtocol::DialAppStopResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.DialProtocol.DialAppStopResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::DialProtocol::DialAppStopResult> __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c755f54a-ef7b-563a-9b14-462e72d9665a"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::DialProtocol::DialAppStopResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::DialProtocol::DialAppStopResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.DialProtocol.DialAppStopResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::DialProtocol::DialAppStopResult> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialDevice;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("471cada5-1ee4-51c0-b6b5-bf72f5f50422"))
IAsyncOperation<ABI::Windows::Media::DialProtocol::DialDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDevice*, ABI::Windows::Media::DialProtocol::IDialDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.DialProtocol.DialDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::DialProtocol::DialDevice*> __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_t;
#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("81bc7d1b-7d06-555f-811b-42ec0fa71b55"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::DialProtocol::DialDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDevice*, ABI::Windows::Media::DialProtocol::IDialDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.DialProtocol.DialDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::DialProtocol::DialDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */



#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialDevicePicker;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dac94028-1b44-5f45-b9e3-abcf4ab044bf"))
ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::IDialDevicePicker*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.DialProtocol.DialDevicePicker, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialDeviceSelectedEventArgs;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8375c0d7-e7c3-56bc-9880-46b69ca10d45"))
ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::DialDeviceSelectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::IDialDevicePicker*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDeviceSelectedEventArgs*, ABI::Windows::Media::DialProtocol::IDialDeviceSelectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.DialProtocol.DialDevicePicker, Windows.Media.DialProtocol.DialDeviceSelectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::DialDeviceSelectedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialDisconnectButtonClickedEventArgs;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("13492af0-1c7f-57e8-b57e-a5ae8f2c462e"))
ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::DialDisconnectButtonClickedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::IDialDevicePicker*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::DialProtocol::DialDisconnectButtonClickedEventArgs*, ABI::Windows::Media::DialProtocol::IDialDisconnectButtonClickedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.DialProtocol.DialDevicePicker, Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::DialProtocol::DialDevicePicker*, ABI::Windows::Media::DialProtocol::DialDisconnectButtonClickedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_USE */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace DialProtocol {
                typedef enum DialAppState : int DialAppState;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                typedef enum DialDeviceDisplayStatus : int DialDeviceDisplayStatus;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialApp;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialDevicePickerFilter;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                class DialReceiverApp;
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppLaunchResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                enum DialAppLaunchResult : int
                {
                    DialAppLaunchResult_Launched = 0,
                    DialAppLaunchResult_FailedToLaunch = 1,
                    DialAppLaunchResult_NotFound = 2,
                    DialAppLaunchResult_NetworkFailure = 3,
                };
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                enum DialAppState : int
                {
                    DialAppState_Unknown = 0,
                    DialAppState_Stopped = 1,
                    DialAppState_Running = 2,
                    DialAppState_NetworkFailure = 3,
                };
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppStopResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                enum DialAppStopResult : int
                {
                    DialAppStopResult_Stopped = 0,
                    DialAppStopResult_StopFailed = 1,
                    DialAppStopResult_OperationNotSupported = 2,
                    DialAppStopResult_NetworkFailure = 3,
                };
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialDeviceDisplayStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                enum DialDeviceDisplayStatus : int
                {
                    DialDeviceDisplayStatus_None = 0,
                    DialDeviceDisplayStatus_Connecting = 1,
                    DialDeviceDisplayStatus_Connected = 2,
                    DialDeviceDisplayStatus_Disconnecting = 3,
                    DialDeviceDisplayStatus_Disconnected = 4,
                    DialDeviceDisplayStatus_Error = 5,
                };
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialApp[] = L"Windows.Media.DialProtocol.IDialApp";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("555ffbd3-45b7-49f3-bbd7-302db6084646")
                IDialApp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestLaunchAsync(
                        HSTRING appArgument,
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppStateAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialApp = __uuidof(IDialApp);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialAppStateDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialAppStateDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialAppStateDetails[] = L"Windows.Media.DialProtocol.IDialAppStateDetails";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("ddc4a4a1-f5de-400d-bea4-8c8466bb2961")
                IDialAppStateDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::DialProtocol::DialAppState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FullXml(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialAppStateDetails = __uuidof(IDialAppStateDetails);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevice[] = L"Windows.Media.DialProtocol.IDialDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("fff0edaf-759f-41d2-a20a-7f29ce0b3784")
                IDialDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDialApp(
                        HSTRING appName,
                        ABI::Windows::Media::DialProtocol::IDialApp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDevice = __uuidof(IDialDevice);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevice2[] = L"Windows.Media.DialProtocol.IDialDevice2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("bab7f3d5-5bfb-4eba-8b32-b57c5c5ee5c9")
                IDialDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDevice2 = __uuidof(IDialDevice2);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevicePicker[] = L"Windows.Media.DialProtocol.IDialDevicePicker";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("ba7e520a-ff59-4f4b-bdac-d89f495ad6e1")
                IDialDevicePicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Filter(
                        ABI::Windows::Media::DialProtocol::IDialDevicePickerFilter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Appearance(
                        ABI::Windows::Devices::Enumeration::IDevicePickerAppearance** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DialDeviceSelected(
                        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DialDeviceSelected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DisconnectButtonClicked(
                        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DisconnectButtonClicked(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DialDevicePickerDismissed(
                        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DialDevicePickerDismissed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Show(
                        ABI::Windows::Foundation::Rect selection
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowWithPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleDialDeviceAsync(
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleDialDeviceAsyncWithPlacement(
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Hide(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDisplayStatus(
                        ABI::Windows::Media::DialProtocol::IDialDevice* device,
                        ABI::Windows::Media::DialProtocol::DialDeviceDisplayStatus status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDevicePicker = __uuidof(IDialDevicePicker);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevicePickerFilter[] = L"Windows.Media.DialProtocol.IDialDevicePickerFilter";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("c17c93ba-86c0-485d-b8d6-0f9a8f641590")
                IDialDevicePickerFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedAppNames(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDevicePickerFilter = __uuidof(IDialDevicePickerFilter);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDeviceSelectedEventArgs[] = L"Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("480b92ad-ac76-47eb-9c06-a19304da0247")
                IDialDeviceSelectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedDialDevice(
                        ABI::Windows::Media::DialProtocol::IDialDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDeviceSelectedEventArgs = __uuidof(IDialDeviceSelectedEventArgs);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDeviceStatics[] = L"Windows.Media.DialProtocol.IDialDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("aa69cc95-01f8-4758-8461-2bbd1cdc3cf3")
                IDialDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING appName,
                        HSTRING* selector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING value,
                        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeviceInfoSupportsDialAsync(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDeviceStatics = __uuidof(IDialDeviceStatics);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDisconnectButtonClickedEventArgs[] = L"Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("52765152-9c81-4e55-adc2-0ebe99cde3b6")
                IDialDisconnectButtonClickedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::Media::DialProtocol::IDialDevice** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialDisconnectButtonClickedEventArgs = __uuidof(IDialDisconnectButtonClickedEventArgs);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverApp[] = L"Windows.Media.DialProtocol.IDialReceiverApp";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("fd3e7c57-5045-470e-b304-4dd9b13e7d11")
                IDialReceiverApp : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAdditionalDataAsync(
                        __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetAdditionalDataAsync(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* additionalData,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialReceiverApp = __uuidof(IDialReceiverApp);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverApp2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverApp2[] = L"Windows.Media.DialProtocol.IDialReceiverApp2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("530c5805-9130-42ac-a504-1977dcb2ea8a")
                IDialReceiverApp2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetUniqueDeviceNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialReceiverApp2 = __uuidof(IDialReceiverApp2);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverAppStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverAppStatics[] = L"Windows.Media.DialProtocol.IDialReceiverAppStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace DialProtocol {
                MIDL_INTERFACE("53183a3c-4c36-4d02-b28a-f2a9da38ec52")
                IDialReceiverAppStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Current(
                        ABI::Windows::Media::DialProtocol::IDialReceiverApp** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialReceiverAppStatics = __uuidof(IDialReceiverAppStatics);
            } /* DialProtocol */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Media.DialProtocol.DialApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialApp ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialApp_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialApp[] = L"Windows.Media.DialProtocol.DialApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialAppStateDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialAppStateDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialAppStateDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialAppStateDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialAppStateDetails[] = L"Windows.Media.DialProtocol.DialAppStateDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.DialProtocol.IDialDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevice ** Default Interface **
 *    Windows.Media.DialProtocol.IDialDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevice[] = L"Windows.Media.DialProtocol.DialDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevicePicker[] = L"Windows.Media.DialProtocol.DialDevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevicePickerFilter[] = L"Windows.Media.DialProtocol.DialDevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs[] = L"Windows.Media.DialProtocol.DialDeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs[] = L"Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialReceiverApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.DialProtocol.IDialReceiverAppStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialReceiverApp ** Default Interface **
 *    Windows.Media.DialProtocol.IDialReceiverApp2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialReceiverApp_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialReceiverApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialReceiverApp[] = L"Windows.Media.DialProtocol.DialReceiverApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2 __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2 __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics;

#endif // ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_FWD_DEFINED__

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

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING;

#if !defined(____FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING;

typedef struct __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* This,
        __FIMap_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING* This,
        __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppLaunchResult __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppLaunchResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppLaunchResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppStopResult __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppStopResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppStopResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialAppStopResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CDialProtocol__CDialDevice_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* sender,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* sender,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
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

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppState __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppState;

typedef enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialDeviceDisplayStatus __x_ABI_CWindows_CMedia_CDialProtocol_CDialDeviceDisplayStatus;

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppLaunchResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppLaunchResult
{
    DialAppLaunchResult_Launched = 0,
    DialAppLaunchResult_FailedToLaunch = 1,
    DialAppLaunchResult_NotFound = 2,
    DialAppLaunchResult_NetworkFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppState
{
    DialAppState_Unknown = 0,
    DialAppState_Stopped = 1,
    DialAppState_Running = 2,
    DialAppState_NetworkFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialAppStopResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppStopResult
{
    DialAppStopResult_Stopped = 0,
    DialAppStopResult_StopFailed = 1,
    DialAppStopResult_OperationNotSupported = 2,
    DialAppStopResult_NetworkFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.DialProtocol.DialDeviceDisplayStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialDeviceDisplayStatus
{
    DialDeviceDisplayStatus_None = 0,
    DialDeviceDisplayStatus_Connecting = 1,
    DialDeviceDisplayStatus_Connected = 2,
    DialDeviceDisplayStatus_Disconnecting = 3,
    DialDeviceDisplayStatus_Disconnected = 4,
    DialDeviceDisplayStatus_Error = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialApp[] = L"Windows.Media.DialProtocol.IDialApp";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* RequestLaunchAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        HSTRING appArgument,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppLaunchResult** value);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStopResult** value);
    HRESULT (STDMETHODCALLTYPE* GetAppStateAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp* This,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialAppStateDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_get_AppName(This, value) \
    ((This)->lpVtbl->get_AppName(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_RequestLaunchAsync(This, appArgument, value) \
    ((This)->lpVtbl->RequestLaunchAsync(This, appArgument, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_StopAsync(This, value) \
    ((This)->lpVtbl->StopAsync(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_GetAppStateAsync(This, value) \
    ((This)->lpVtbl->GetAppStateAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialAppStateDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialAppStateDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialAppStateDetails[] = L"Windows.Media.DialProtocol.IDialAppStateDetails";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialAppState* value);
    HRESULT (STDMETHODCALLTYPE* get_FullXml)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetailsVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_get_FullXml(This, value) \
    ((This)->lpVtbl->get_FullXml(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialAppStateDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevice[] = L"Windows.Media.DialProtocol.IDialDevice";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetDialApp)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* This,
        HSTRING appName,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialApp** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_GetDialApp(This, appName, value) \
    ((This)->lpVtbl->GetDialApp(This, appName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevice2[] = L"Windows.Media.DialProtocol.IDialDevice2";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2Vtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevicePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevicePicker[] = L"Windows.Media.DialProtocol.IDialDevicePicker";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Filter)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter** value);
    HRESULT (STDMETHODCALLTYPE* get_Appearance)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDevicePickerAppearance** value);
    HRESULT (STDMETHODCALLTYPE* add_DialDeviceSelected)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDeviceSelectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DialDeviceSelected)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DisconnectButtonClicked)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_Windows__CMedia__CDialProtocol__CDialDisconnectButtonClickedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisconnectButtonClicked)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DialDevicePickerDismissed)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __FITypedEventHandler_2_Windows__CMedia__CDialProtocol__CDialDevicePicker_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DialDevicePickerDismissed)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection);
    HRESULT (STDMETHODCALLTYPE* ShowWithPlacement)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement);
    HRESULT (STDMETHODCALLTYPE* PickSingleDialDeviceAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation);
    HRESULT (STDMETHODCALLTYPE* PickSingleDialDeviceAsyncWithPlacement)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation);
    HRESULT (STDMETHODCALLTYPE* Hide)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This);
    HRESULT (STDMETHODCALLTYPE* SetDisplayStatus)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice* device,
        enum __x_ABI_CWindows_CMedia_CDialProtocol_CDialDeviceDisplayStatus status);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_get_Filter(This, value) \
    ((This)->lpVtbl->get_Filter(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_get_Appearance(This, value) \
    ((This)->lpVtbl->get_Appearance(This, value))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_add_DialDeviceSelected(This, handler, token) \
    ((This)->lpVtbl->add_DialDeviceSelected(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_remove_DialDeviceSelected(This, token) \
    ((This)->lpVtbl->remove_DialDeviceSelected(This, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_add_DisconnectButtonClicked(This, handler, token) \
    ((This)->lpVtbl->add_DisconnectButtonClicked(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_remove_DisconnectButtonClicked(This, token) \
    ((This)->lpVtbl->remove_DisconnectButtonClicked(This, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_add_DialDevicePickerDismissed(This, handler, token) \
    ((This)->lpVtbl->add_DialDevicePickerDismissed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_remove_DialDevicePickerDismissed(This, token) \
    ((This)->lpVtbl->remove_DialDevicePickerDismissed(This, token))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_Show(This, selection) \
    ((This)->lpVtbl->Show(This, selection))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_ShowWithPlacement(This, selection, preferredPlacement) \
    ((This)->lpVtbl->ShowWithPlacement(This, selection, preferredPlacement))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_PickSingleDialDeviceAsync(This, selection, operation) \
    ((This)->lpVtbl->PickSingleDialDeviceAsync(This, selection, operation))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_PickSingleDialDeviceAsyncWithPlacement(This, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->PickSingleDialDeviceAsyncWithPlacement(This, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_Hide(This) \
    ((This)->lpVtbl->Hide(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_SetDisplayStatus(This, device, status) \
    ((This)->lpVtbl->SetDisplayStatus(This, device, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevicePickerFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDevicePickerFilter[] = L"Windows.Media.DialProtocol.IDialDevicePickerFilter";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedAppNames)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilterVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_get_SupportedAppNames(This, value) \
    ((This)->lpVtbl->get_SupportedAppNames(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevicePickerFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDeviceSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDeviceSelectedEventArgs[] = L"Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SelectedDialDevice)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_get_SelectedDialDevice(This, value) \
    ((This)->lpVtbl->get_SelectedDialDevice(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDeviceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDevice
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDeviceStatics[] = L"Windows.Media.DialProtocol.IDialDeviceStatics";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        HSTRING appName,
        HSTRING* selector);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        HSTRING value,
        __FIAsyncOperation_1_Windows__CMedia__CDialProtocol__CDialDevice** operation);
    HRESULT (STDMETHODCALLTYPE* DeviceInfoSupportsDialAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_GetDeviceSelector(This, appName, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, appName, selector))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_FromIdAsync(This, value, operation) \
    ((This)->lpVtbl->FromIdAsync(This, value, operation))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_DeviceInfoSupportsDialAsync(This, device, operation) \
    ((This)->lpVtbl->DeviceInfoSupportsDialAsync(This, device, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialDisconnectButtonClickedEventArgs[] = L"Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialDisconnectButtonClickedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverApp[] = L"Windows.Media.DialProtocol.IDialReceiverApp";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAdditionalDataAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        __FIAsyncOperation_1___FIMap_2_HSTRING_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* SetAdditionalDataAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* additionalData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_GetAdditionalDataAsync(This, operation) \
    ((This)->lpVtbl->GetAdditionalDataAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_SetAdditionalDataAsync(This, additionalData, operation) \
    ((This)->lpVtbl->SetAdditionalDataAsync(This, additionalData, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverApp2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverApp2[] = L"Windows.Media.DialProtocol.IDialReceiverApp2";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetUniqueDeviceNameAsync)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2* This,
        __FIAsyncOperation_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2Vtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_GetUniqueDeviceNameAsync(This, operation) \
    ((This)->lpVtbl->GetUniqueDeviceNameAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.DialProtocol.IDialReceiverAppStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.DialProtocol.DialReceiverApp
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_DialProtocol_IDialReceiverAppStatics[] = L"Windows.Media.DialProtocol.IDialReceiverAppStatics";
typedef struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics* This,
        __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverApp** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CDialProtocol_CIDialReceiverAppStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Media.DialProtocol.DialApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialApp ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialApp_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialApp[] = L"Windows.Media.DialProtocol.DialApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialAppStateDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialAppStateDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialAppStateDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialAppStateDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialAppStateDetails[] = L"Windows.Media.DialProtocol.DialAppStateDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevice
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.DialProtocol.IDialDeviceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevice ** Default Interface **
 *    Windows.Media.DialProtocol.IDialDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevice[] = L"Windows.Media.DialProtocol.DialDevice";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevicePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevicePicker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePicker_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevicePicker[] = L"Windows.Media.DialProtocol.DialDevicePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDevicePickerFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDevicePickerFilter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePickerFilter_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDevicePickerFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDevicePickerFilter[] = L"Windows.Media.DialProtocol.DialDevicePickerFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDeviceSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDeviceSelectedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDeviceSelectedEventArgs[] = L"Windows.Media.DialProtocol.DialDeviceSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialDisconnectButtonClickedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialDisconnectButtonClickedEventArgs[] = L"Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.DialProtocol.DialReceiverApp
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.DialProtocol.IDialReceiverAppStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.DialProtocol.IDialReceiverApp ** Default Interface **
 *    Windows.Media.DialProtocol.IDialReceiverApp2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Media_DialProtocol_DialReceiverApp_DEFINED
#define RUNTIMECLASS_Windows_Media_DialProtocol_DialReceiverApp_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_DialProtocol_DialReceiverApp[] = L"Windows.Media.DialProtocol.DialReceiverApp";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Edialprotocol_p_h__

#endif // __windows2Emedia2Edialprotocol_h__
