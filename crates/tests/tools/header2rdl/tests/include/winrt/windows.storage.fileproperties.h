
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
#ifndef __windows2Estorage2Efileproperties_h__
#define __windows2Estorage2Efileproperties_h__
#ifndef __windows2Estorage2Efileproperties_p_h__
#define __windows2Estorage2Efileproperties_p_h__


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
#include "Windows.Devices.Geolocation.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IBasicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties ABI::Windows::Storage::FileProperties::IBasicProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IDocumentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties ABI::Windows::Storage::FileProperties::IDocumentProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IGeotagHelperStatics;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics ABI::Windows::Storage::FileProperties::IGeotagHelperStatics

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IImageProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties ABI::Windows::Storage::FileProperties::IImageProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IMusicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties ABI::Windows::Storage::FileProperties::IMusicProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IStorageItemContentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties ABI::Windows::Storage::FileProperties::IStorageItemContentProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IStorageItemExtraProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties ABI::Windows::Storage::FileProperties::IStorageItemExtraProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IThumbnailProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties ABI::Windows::Storage::FileProperties::IThumbnailProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IVideoProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties ABI::Windows::Storage::FileProperties::IVideoProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geopoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeopoint;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint ABI::Windows::Devices::Geolocation::IGeopoint

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3723e070-c2ae-538f-846e-0f9d280310c0"))
IAsyncOperation<ABI::Windows::Devices::Geolocation::Geopoint*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geopoint*, ABI::Windows::Devices::Geolocation::IGeopoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Geolocation.Geopoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Geolocation::Geopoint*> __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_t;
#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b5f2f60-19b1-5566-9df6-92a42235cbf9"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geopoint*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Geolocation::Geopoint*, ABI::Windows::Devices::Geolocation::IGeopoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Geolocation.Geopoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Geolocation::Geopoint*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("127e39c7-07c1-58e5-b48e-3a4729839fec"))
IAsyncOperation<__FIMap_2_HSTRING_IInspectable*> : IAsyncOperation_impl<__FIMap_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMap`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMap_2_HSTRING_IInspectable*> __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_t;
#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7344f356-8399-5756-a2f8-abd50c4146ff"))
IAsyncOperationCompletedHandler<__FIMap_2_HSTRING_IInspectable*> : IAsyncOperationCompletedHandler_impl<__FIMap_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMap`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMap_2_HSTRING_IInspectable*> __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_t;
#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class DocumentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c86e97c-5699-5700-8d35-d350ad3e4df2"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::DocumentProperties*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::DocumentProperties*, ABI::Windows::Storage::FileProperties::IDocumentProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.DocumentProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::DocumentProperties*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4452ed4c-642b-501b-9617-7d68b4ac3c66"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::DocumentProperties*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::DocumentProperties*, ABI::Windows::Storage::FileProperties::IDocumentProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.DocumentProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::DocumentProperties*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class ImageProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fcd07511-e7f8-5bda-8c04-795a639dae8f"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::ImageProperties*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::ImageProperties*, ABI::Windows::Storage::FileProperties::IImageProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.ImageProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::ImageProperties*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c63729bc-e4c3-564c-b137-2cb4f5966a83"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::ImageProperties*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::ImageProperties*, ABI::Windows::Storage::FileProperties::IImageProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.ImageProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::ImageProperties*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class MusicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d023b76-20a7-56f3-84ab-ce31e6544b71"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::MusicProperties*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::MusicProperties*, ABI::Windows::Storage::FileProperties::IMusicProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.MusicProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::MusicProperties*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d84e1312-d661-5b7f-9566-7421bdedc1ea"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::MusicProperties*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::MusicProperties*, ABI::Windows::Storage::FileProperties::IMusicProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.MusicProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::MusicProperties*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class VideoProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("447d4590-d3f9-58bf-ac58-6f9a50839efe"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::VideoProperties*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::VideoProperties*, ABI::Windows::Storage::FileProperties::IVideoProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.VideoProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::VideoProperties*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43401d34-61ab-5cf2-921f-55b616631d1d"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::VideoProperties*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::VideoProperties*, ABI::Windows::Storage::FileProperties::IVideoProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.VideoProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::VideoProperties*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_USE */

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



#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                class Geolocator;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                interface IGeolocator;
            } /* Geolocation */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator ABI::Windows::Devices::Geolocation::IGeolocator

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IContentTypeProvider;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider ABI::Windows::Storage::Streams::IContentTypeProvider

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

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
        namespace Storage {
            namespace FileProperties {
                typedef enum PhotoOrientation : int PhotoOrientation;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                typedef enum ThumbnailType : int ThumbnailType;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                typedef enum VideoOrientation : int VideoOrientation;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.FileProperties.PhotoOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum PhotoOrientation : int
                {
                    PhotoOrientation_Unspecified = 0,
                    PhotoOrientation_Normal = 1,
                    PhotoOrientation_FlipHorizontal = 2,
                    PhotoOrientation_Rotate180 = 3,
                    PhotoOrientation_FlipVertical = 4,
                    PhotoOrientation_Transpose = 5,
                    PhotoOrientation_Rotate270 = 6,
                    PhotoOrientation_Transverse = 7,
                    PhotoOrientation_Rotate90 = 8,
                };
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.PropertyPrefetchOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum PropertyPrefetchOptions : unsigned int
                {
                    PropertyPrefetchOptions_None = 0,
                    PropertyPrefetchOptions_MusicProperties = 0x1,
                    PropertyPrefetchOptions_VideoProperties = 0x2,
                    PropertyPrefetchOptions_ImageProperties = 0x4,
                    PropertyPrefetchOptions_DocumentProperties = 0x8,
                    PropertyPrefetchOptions_BasicProperties = 0x10,
                };

                DEFINE_ENUM_FLAG_OPERATORS(PropertyPrefetchOptions)
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum ThumbnailMode : int
                {
                    ThumbnailMode_PicturesView = 0,
                    ThumbnailMode_VideosView = 1,
                    ThumbnailMode_MusicView = 2,
                    ThumbnailMode_DocumentsView = 3,
                    ThumbnailMode_ListView = 4,
                    ThumbnailMode_SingleItem = 5,
                };
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum ThumbnailOptions : unsigned int
                {
                    ThumbnailOptions_None = 0,
                    ThumbnailOptions_ReturnOnlyIfCached = 0x1,
                    ThumbnailOptions_ResizeThumbnail = 0x2,
                    ThumbnailOptions_UseCurrentScale = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(ThumbnailOptions)
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum ThumbnailType : int
                {
                    ThumbnailType_Image = 0,
                    ThumbnailType_Icon = 1,
                };
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.VideoOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                enum VideoOrientation : int
                {
                    VideoOrientation_Normal = 0,
                    VideoOrientation_Rotate90 = 90,
                    VideoOrientation_Rotate180 = 180,
                    VideoOrientation_Rotate270 = 270,
                };
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IBasicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.BasicProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IBasicProperties[] = L"Windows.Storage.FileProperties.IBasicProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("d05d55db-785e-4a66-be02-9beec58aea81")
                IBasicProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DateModified(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ItemDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBasicProperties = __uuidof(IBasicProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IDocumentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.DocumentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IDocumentProperties[] = L"Windows.Storage.FileProperties.IDocumentProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("7eab19bc-1821-4923-b4a9-0aea404d0070")
                IDocumentProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Author(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Comment(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Comment(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDocumentProperties = __uuidof(IDocumentProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IGeotagHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.GeotagHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IGeotagHelperStatics[] = L"Windows.Storage.FileProperties.IGeotagHelperStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("41493244-2524-4655-86a6-ed16f5fc716b")
                IGeotagHelperStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetGeotagAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetGeotagFromGeolocatorAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Devices::Geolocation::IGeolocator* geolocator,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetGeotagAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Devices::Geolocation::IGeopoint* geopoint,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGeotagHelperStatics = __uuidof(IGeotagHelperStatics);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.ImageProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IImageProperties[] = L"Windows.Storage.FileProperties.IImageProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("523c9424-fcff-4275-afee-ecdb9ab47973")
                IImageProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Rating(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rating(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DateTaken(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DateTaken(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Latitude(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Longitude(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CameraManufacturer(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CameraManufacturer(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CameraModel(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CameraModel(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Storage::FileProperties::PhotoOrientation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PeopleNames(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageProperties = __uuidof(IImageProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.MusicProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IMusicProperties[] = L"Windows.Storage.FileProperties.IMusicProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("bc8aab62-66ec-419a-bc5d-ca65a4cb46da")
                IMusicProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Album(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Album(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Artist(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Artist(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Genre(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrackNumber(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TrackNumber(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rating(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rating(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlbumArtist(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AlbumArtist(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Composers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Conductors(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subtitle(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Producers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Publisher(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Publisher(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Writers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Year(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Year(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMusicProperties = __uuidof(IMusicProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IStorageItemContentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.StorageItemContentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IStorageItemContentProperties[] = L"Windows.Storage.FileProperties.IStorageItemContentProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("05294bad-bc38-48bf-85d7-770e0e2ae0ba")
                IStorageItemContentProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetMusicPropertiesAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVideoPropertiesAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetImagePropertiesAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDocumentPropertiesAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStorageItemContentProperties = __uuidof(IStorageItemContentProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IStorageItemExtraProperties[] = L"Windows.Storage.FileProperties.IStorageItemExtraProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("c54361b2-54cd-432b-bdbc-4b19c4b470d7")
                IStorageItemExtraProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RetrievePropertiesAsync(
                        __FIIterable_1_HSTRING* propertiesToRetrieve,
                        __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SavePropertiesAsync(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* propertiesToSave,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SavePropertiesAsyncOverloadDefault(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStorageItemExtraProperties = __uuidof(IStorageItemExtraProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IThumbnailProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.StorageItemThumbnail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IThumbnailProperties[] = L"Windows.Storage.FileProperties.IThumbnailProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("693dd42f-dbe7-49b5-b3b3-2893ac5d3423")
                IThumbnailProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalHeight(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReturnedSmallerCachedSize(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Storage::FileProperties::ThumbnailType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IThumbnailProperties = __uuidof(IThumbnailProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.VideoProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IVideoProperties[] = L"Windows.Storage.FileProperties.IVideoProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                MIDL_INTERFACE("719ae507-68de-4db8-97de-49998c059f2f")
                IVideoProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Rating(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rating(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Latitude(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Longitude(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subtitle(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Producers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Publisher(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Publisher(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Writers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Year(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Year(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Directors(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Storage::FileProperties::VideoOrientation* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoProperties = __uuidof(IVideoProperties);
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.BasicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IBasicProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_BasicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_BasicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_BasicProperties[] = L"Windows.Storage.FileProperties.BasicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.DocumentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IDocumentProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_DocumentProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_DocumentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_DocumentProperties[] = L"Windows.Storage.FileProperties.DocumentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.GeotagHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.FileProperties.IGeotagHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_GeotagHelper_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_GeotagHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_GeotagHelper[] = L"Windows.Storage.FileProperties.GeotagHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.ImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IImageProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_ImageProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_ImageProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_ImageProperties[] = L"Windows.Storage.FileProperties.ImageProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.MusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IMusicProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_MusicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_MusicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_MusicProperties[] = L"Windows.Storage.FileProperties.MusicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.StorageItemContentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IStorageItemContentProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemContentProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemContentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_StorageItemContentProperties[] = L"Windows.Storage.FileProperties.StorageItemContentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.StorageItemThumbnail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Storage.FileProperties.IThumbnailProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemThumbnail_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemThumbnail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_StorageItemThumbnail[] = L"Windows.Storage.FileProperties.StorageItemThumbnail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.VideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IVideoProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_VideoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_VideoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_VideoProperties[] = L"Windows.Storage.FileProperties.VideoProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopointVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint* This,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopointVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CGeolocation__CGeopoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable;

#if !defined(____FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable;

typedef struct __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* This,
        __FIMap_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectableVtbl;

interface __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable* This,
        __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectableVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CDocumentProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImagePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CImageProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CMusicProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CVideoProperties_INTERFACE_DEFINED__
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

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator;

#endif // ____x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

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

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider __x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIContentTypeProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CPhotoOrientation __x_ABI_CWindows_CStorage_CFileProperties_CPhotoOrientation;

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailType __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailType;

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CVideoOrientation __x_ABI_CWindows_CStorage_CFileProperties_CVideoOrientation;

/*
 *
 * Struct Windows.Storage.FileProperties.PhotoOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CPhotoOrientation
{
    PhotoOrientation_Unspecified = 0,
    PhotoOrientation_Normal = 1,
    PhotoOrientation_FlipHorizontal = 2,
    PhotoOrientation_Rotate180 = 3,
    PhotoOrientation_FlipVertical = 4,
    PhotoOrientation_Transpose = 5,
    PhotoOrientation_Rotate270 = 6,
    PhotoOrientation_Transverse = 7,
    PhotoOrientation_Rotate90 = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.PropertyPrefetchOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CPropertyPrefetchOptions
{
    PropertyPrefetchOptions_None = 0,
    PropertyPrefetchOptions_MusicProperties = 0x1,
    PropertyPrefetchOptions_VideoProperties = 0x2,
    PropertyPrefetchOptions_ImageProperties = 0x4,
    PropertyPrefetchOptions_DocumentProperties = 0x8,
    PropertyPrefetchOptions_BasicProperties = 0x10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode
{
    ThumbnailMode_PicturesView = 0,
    ThumbnailMode_VideosView = 1,
    ThumbnailMode_MusicView = 2,
    ThumbnailMode_DocumentsView = 3,
    ThumbnailMode_ListView = 4,
    ThumbnailMode_SingleItem = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions
{
    ThumbnailOptions_None = 0,
    ThumbnailOptions_ReturnOnlyIfCached = 0x1,
    ThumbnailOptions_ResizeThumbnail = 0x2,
    ThumbnailOptions_UseCurrentScale = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.ThumbnailType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailType
{
    ThumbnailType_Image = 0,
    ThumbnailType_Icon = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileProperties.VideoOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileProperties_CVideoOrientation
{
    VideoOrientation_Normal = 0,
    VideoOrientation_Rotate90 = 90,
    VideoOrientation_Rotate180 = 180,
    VideoOrientation_Rotate270 = 270,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IBasicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.BasicProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IBasicProperties[] = L"Windows.Storage.FileProperties.IBasicProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIBasicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_DateModified)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ItemDate)(__x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIBasicPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIBasicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_get_DateModified(This, value) \
    ((This)->lpVtbl->get_DateModified(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_get_ItemDate(This, value) \
    ((This)->lpVtbl->get_ItemDate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IDocumentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.DocumentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IDocumentProperties[] = L"Windows.Storage.FileProperties.IDocumentProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Comment)(__x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_put_Comment(This, value) \
    ((This)->lpVtbl->put_Comment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IGeotagHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.GeotagHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IGeotagHelperStatics[] = L"Windows.Storage.FileProperties.IGeotagHelperStatics";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetGeotagAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CDevices__CGeolocation__CGeopoint** operation);
    HRESULT (STDMETHODCALLTYPE* SetGeotagFromGeolocatorAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeolocator* geolocator,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SetGeotagAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CDevices_CGeolocation_CIGeopoint* geopoint,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_GetGeotagAsync(This, file, operation) \
    ((This)->lpVtbl->GetGeotagAsync(This, file, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_SetGeotagFromGeolocatorAsync(This, file, geolocator, operation) \
    ((This)->lpVtbl->SetGeotagFromGeolocatorAsync(This, file, geolocator, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_SetGeotagAsync(This, file, geopoint, operation) \
    ((This)->lpVtbl->SetGeotagAsync(This, file, geopoint, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIGeotagHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.ImageProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IImageProperties[] = L"Windows.Storage.FileProperties.IImageProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIImagePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DateTaken)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_DateTaken)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Latitude)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Longitude)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_CameraManufacturer)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CameraManufacturer)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CameraModel)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CameraModel)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CPhotoOrientation* value);
    HRESULT (STDMETHODCALLTYPE* get_PeopleNames)(__x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIImagePropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIImagePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Rating(This, value) \
    ((This)->lpVtbl->get_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_put_Rating(This, value) \
    ((This)->lpVtbl->put_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_DateTaken(This, value) \
    ((This)->lpVtbl->get_DateTaken(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_put_DateTaken(This, value) \
    ((This)->lpVtbl->put_DateTaken(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Latitude(This, value) \
    ((This)->lpVtbl->get_Latitude(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Longitude(This, value) \
    ((This)->lpVtbl->get_Longitude(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_CameraManufacturer(This, value) \
    ((This)->lpVtbl->get_CameraManufacturer(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_put_CameraManufacturer(This, value) \
    ((This)->lpVtbl->put_CameraManufacturer(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_CameraModel(This, value) \
    ((This)->lpVtbl->get_CameraModel(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_put_CameraModel(This, value) \
    ((This)->lpVtbl->put_CameraModel(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_get_PeopleNames(This, value) \
    ((This)->lpVtbl->get_PeopleNames(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.MusicProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IMusicProperties[] = L"Windows.Storage.FileProperties.IMusicProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIMusicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Album)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Album)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Artist)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Artist)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Genre)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_TrackNumber)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TrackNumber)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AlbumArtist)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AlbumArtist)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Composers)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Conductors)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subtitle)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Producers)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Publisher)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Publisher)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Writers)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Year)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Year)(__x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIMusicPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIMusicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Album(This, value) \
    ((This)->lpVtbl->get_Album(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Album(This, value) \
    ((This)->lpVtbl->put_Album(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Artist(This, value) \
    ((This)->lpVtbl->get_Artist(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Artist(This, value) \
    ((This)->lpVtbl->put_Artist(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Genre(This, value) \
    ((This)->lpVtbl->get_Genre(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_TrackNumber(This, value) \
    ((This)->lpVtbl->get_TrackNumber(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_TrackNumber(This, value) \
    ((This)->lpVtbl->put_TrackNumber(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Rating(This, value) \
    ((This)->lpVtbl->get_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Rating(This, value) \
    ((This)->lpVtbl->put_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_AlbumArtist(This, value) \
    ((This)->lpVtbl->get_AlbumArtist(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_AlbumArtist(This, value) \
    ((This)->lpVtbl->put_AlbumArtist(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Composers(This, value) \
    ((This)->lpVtbl->get_Composers(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Conductors(This, value) \
    ((This)->lpVtbl->get_Conductors(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Subtitle(This, value) \
    ((This)->lpVtbl->put_Subtitle(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Producers(This, value) \
    ((This)->lpVtbl->get_Producers(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Publisher(This, value) \
    ((This)->lpVtbl->get_Publisher(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Publisher(This, value) \
    ((This)->lpVtbl->put_Publisher(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Writers(This, value) \
    ((This)->lpVtbl->get_Writers(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_get_Year(This, value) \
    ((This)->lpVtbl->get_Year(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_put_Year(This, value) \
    ((This)->lpVtbl->put_Year(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IStorageItemContentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.StorageItemContentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IStorageItemContentProperties[] = L"Windows.Storage.FileProperties.IStorageItemContentProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetMusicPropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CMusicProperties** operation);
    HRESULT (STDMETHODCALLTYPE* GetVideoPropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CVideoProperties** operation);
    HRESULT (STDMETHODCALLTYPE* GetImagePropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CImageProperties** operation);
    HRESULT (STDMETHODCALLTYPE* GetDocumentPropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CDocumentProperties** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetMusicPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetMusicPropertiesAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetVideoPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetVideoPropertiesAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetImagePropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetImagePropertiesAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_GetDocumentPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetDocumentPropertiesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IStorageItemExtraProperties[] = L"Windows.Storage.FileProperties.IStorageItemExtraProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RetrievePropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        __FIIterable_1_HSTRING* propertiesToRetrieve,
        __FIAsyncOperation_1___FIMap_2_HSTRING_IInspectable** operation);
    HRESULT (STDMETHODCALLTYPE* SavePropertiesAsync)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* propertiesToSave,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SavePropertiesAsyncOverloadDefault)(__x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_RetrievePropertiesAsync(This, propertiesToRetrieve, operation) \
    ((This)->lpVtbl->RetrievePropertiesAsync(This, propertiesToRetrieve, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_SavePropertiesAsync(This, propertiesToSave, operation) \
    ((This)->lpVtbl->SavePropertiesAsync(This, propertiesToSave, operation))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_SavePropertiesAsyncOverloadDefault(This, operation) \
    ((This)->lpVtbl->SavePropertiesAsyncOverloadDefault(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemExtraProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IThumbnailProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.StorageItemThumbnail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IThumbnailProperties[] = L"Windows.Storage.FileProperties.IThumbnailProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OriginalWidth)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalHeight)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ReturnedSmallerCachedSize)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailType* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_get_OriginalWidth(This, value) \
    ((This)->lpVtbl->get_OriginalWidth(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_get_OriginalHeight(This, value) \
    ((This)->lpVtbl->get_OriginalHeight(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_get_ReturnedSmallerCachedSize(This, value) \
    ((This)->lpVtbl->get_ReturnedSmallerCachedSize(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIThumbnailProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.FileProperties.IVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileProperties.VideoProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_FileProperties_IVideoProperties[] = L"Windows.Storage.FileProperties.IVideoProperties";
typedef struct __x_ABI_CWindows_CStorage_CFileProperties_CIVideoPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Rating)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Latitude)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Longitude)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subtitle)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Producers)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Publisher)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Publisher)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Writers)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Year)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Year)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Directors)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CVideoOrientation* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CFileProperties_CIVideoPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CFileProperties_CIVideoPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Rating(This, value) \
    ((This)->lpVtbl->get_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_put_Rating(This, value) \
    ((This)->lpVtbl->put_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Latitude(This, value) \
    ((This)->lpVtbl->get_Latitude(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Longitude(This, value) \
    ((This)->lpVtbl->get_Longitude(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_put_Subtitle(This, value) \
    ((This)->lpVtbl->put_Subtitle(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Producers(This, value) \
    ((This)->lpVtbl->get_Producers(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Publisher(This, value) \
    ((This)->lpVtbl->get_Publisher(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_put_Publisher(This, value) \
    ((This)->lpVtbl->put_Publisher(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Writers(This, value) \
    ((This)->lpVtbl->get_Writers(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Year(This, value) \
    ((This)->lpVtbl->get_Year(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_put_Year(This, value) \
    ((This)->lpVtbl->put_Year(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Directors(This, value) \
    ((This)->lpVtbl->get_Directors(This, value))

#define __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.BasicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IBasicProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_BasicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_BasicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_BasicProperties[] = L"Windows.Storage.FileProperties.BasicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.DocumentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IDocumentProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_DocumentProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_DocumentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_DocumentProperties[] = L"Windows.Storage.FileProperties.DocumentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.GeotagHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.FileProperties.IGeotagHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_GeotagHelper_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_GeotagHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_GeotagHelper[] = L"Windows.Storage.FileProperties.GeotagHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.ImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IImageProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_ImageProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_ImageProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_ImageProperties[] = L"Windows.Storage.FileProperties.ImageProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.MusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IMusicProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_MusicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_MusicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_MusicProperties[] = L"Windows.Storage.FileProperties.MusicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.StorageItemContentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IStorageItemContentProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemContentProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemContentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_StorageItemContentProperties[] = L"Windows.Storage.FileProperties.StorageItemContentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.StorageItemThumbnail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamWithContentType ** Default Interface **
 *    Windows.Storage.Streams.IContentTypeProvider
 *    Windows.Storage.Streams.IRandomAccessStream
 *    Windows.Storage.Streams.IOutputStream
 *    Windows.Foundation.IClosable
 *    Windows.Storage.Streams.IInputStream
 *    Windows.Storage.FileProperties.IThumbnailProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemThumbnail_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_StorageItemThumbnail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_StorageItemThumbnail[] = L"Windows.Storage.FileProperties.StorageItemThumbnail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileProperties.VideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.FileProperties.IVideoProperties ** Default Interface **
 *    Windows.Storage.FileProperties.IStorageItemExtraProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileProperties_VideoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileProperties_VideoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileProperties_VideoProperties[] = L"Windows.Storage.FileProperties.VideoProperties";
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
#endif // __windows2Estorage2Efileproperties_p_h__

#endif // __windows2Estorage2Efileproperties_h__
