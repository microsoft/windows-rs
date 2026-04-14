
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
#ifndef __windows2Emedia2Eimport_h__
#define __windows2Emedia2Eimport_h__
#ifndef __windows2Emedia2Eimport_p_h__
#define __windows2Emedia2Eimport_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportDeleteImportedItemsFromSourceResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult ABI::Windows::Media::Import::IPhotoImportDeleteImportedItemsFromSourceResult

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportFindItemsResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult ABI::Windows::Media::Import::IPhotoImportFindItemsResult

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportFindItemsResult2;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2 ABI::Windows::Media::Import::IPhotoImportFindItemsResult2

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportImportItemsResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult ABI::Windows::Media::Import::IPhotoImportImportItemsResult

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportItem;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem ABI::Windows::Media::Import::IPhotoImportItem

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportItem2;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2 ABI::Windows::Media::Import::IPhotoImportItem2

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportItemImportedEventArgs;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs ABI::Windows::Media::Import::IPhotoImportItemImportedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportManagerStatics;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics ABI::Windows::Media::Import::IPhotoImportManagerStatics

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportOperation;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation ABI::Windows::Media::Import::IPhotoImportOperation

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSelectionChangedEventArgs;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs ABI::Windows::Media::Import::IPhotoImportSelectionChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSession;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession ABI::Windows::Media::Import::IPhotoImportSession

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSession2;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2 ABI::Windows::Media::Import::IPhotoImportSession2

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSidecar;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar ABI::Windows::Media::Import::IPhotoImportSidecar

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSource;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource ABI::Windows::Media::Import::IPhotoImportSource

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportSourceStatics;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics ABI::Windows::Media::Import::IPhotoImportSourceStatics

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportStorageMedium;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium ABI::Windows::Media::Import::IPhotoImportStorageMedium

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                interface IPhotoImportVideoSegment;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment ABI::Windows::Media::Import::IPhotoImportVideoSegment

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__

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
        namespace Media {
            namespace Import {
                class PhotoImportSource;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7d70f831-6ee4-5130-a7b8-253a21154e82"))
IIterator<ABI::Windows::Media::Import::PhotoImportSource*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSource*, ABI::Windows::Media::Import::IPhotoImportSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportSource*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("40e01d62-b413-5b43-ab07-ab28b23fc886"))
IIterable<ABI::Windows::Media::Import::PhotoImportSource*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSource*, ABI::Windows::Media::Import::IPhotoImportSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportSource*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8b7e83fc-e035-59dc-8100-fcb935c2d7e4"))
IVectorView<ABI::Windows::Media::Import::PhotoImportSource*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSource*, ABI::Windows::Media::Import::IPhotoImportSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportSource*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3ef45f6e-39b9-5976-8643-6bafea4d1479"))
IAsyncOperation<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportSource>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*> __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("72cde698-9247-5053-8cbd-d9076bfdfda5"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportSource>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c8c5dc1e-eb47-50b8-b5d9-aafe1a82318a"))
IAsyncOperation<ABI::Windows::Media::Import::PhotoImportSource*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSource*, ABI::Windows::Media::Import::IPhotoImportSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Import.PhotoImportSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Import::PhotoImportSource*> __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dc38b22a-872e-53f8-8e97-45ed85df0d23"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Import::PhotoImportSource*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSource*, ABI::Windows::Media::Import::IPhotoImportSource*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Import.PhotoImportSource>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Import::PhotoImportSource*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportDeleteImportedItemsFromSourceResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e24e7c1-f356-59c1-b0e5-b2dfb225eb4e"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, ABI::Windows::Media::Import::IPhotoImportDeleteImportedItemsFromSourceResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e2371a9-281a-5226-ae85-caa55c0d61de"))
IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, ABI::Windows::Media::Import::IPhotoImportDeleteImportedItemsFromSourceResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t;
#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ac6e425d-49e8-50d7-988c-cd5e42038577"))
IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, ABI::Windows::Media::Import::IPhotoImportDeleteImportedItemsFromSourceResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportDeleteImportedItemsFromSourceResult*, double> __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportFindItemsResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dd7a69d4-2456-5250-9653-31bd2d487104"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::IPhotoImportFindItemsResult*>, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Media.Import.PhotoImportFindItemsResult, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e6f9b4e-c6e1-5364-a650-11c35211bead"))
IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::IPhotoImportFindItemsResult*>, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Media.Import.PhotoImportFindItemsResult, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t;
#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("91190f62-7956-5e8f-83f1-84f9fe011b21"))
IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::IPhotoImportFindItemsResult*>, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Media.Import.PhotoImportFindItemsResult, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, UINT32> __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportImportItemsResult;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef struct PhotoImportProgress PhotoImportProgress;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d141ec2-ee90-53a0-9318-10f0ab7f2d17"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, ABI::Windows::Media::Import::IPhotoImportImportItemsResult*>, struct ABI::Windows::Media::Import::PhotoImportProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Media.Import.PhotoImportImportItemsResult, Windows.Media.Import.PhotoImportProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d874ec64-0951-5459-a0dd-0f8bf3917eb1"))
IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, ABI::Windows::Media::Import::IPhotoImportImportItemsResult*>, struct ABI::Windows::Media::Import::PhotoImportProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Media.Import.PhotoImportImportItemsResult, Windows.Media.Import.PhotoImportProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("acd8a978-b2e1-55d0-bbf6-8dc5088d728a"))
IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, ABI::Windows::Media::Import::IPhotoImportImportItemsResult*>, struct ABI::Windows::Media::Import::PhotoImportProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Media.Import.PhotoImportImportItemsResult, Windows.Media.Import.PhotoImportProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Media::Import::PhotoImportImportItemsResult*, struct ABI::Windows::Media::Import::PhotoImportProgress> __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
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


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportItem;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d04d6068-b5a3-508e-bc6b-1dcdfcfb0d08"))
IIterator<ABI::Windows::Media::Import::PhotoImportItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportItem*, ABI::Windows::Media::Import::IPhotoImportItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportItem*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("82347483-3b75-5e95-bba4-abc0b8a320aa"))
IIterable<ABI::Windows::Media::Import::PhotoImportItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportItem*, ABI::Windows::Media::Import::IPhotoImportItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportItem*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportOperation;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7c9dde1a-a8a1-5957-8e0d-c401d19c9237"))
IIterator<ABI::Windows::Media::Import::PhotoImportOperation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportOperation*, ABI::Windows::Media::Import::IPhotoImportOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportOperation*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94f33a8f-115a-50cb-b59d-ab8483a84842"))
IIterable<ABI::Windows::Media::Import::PhotoImportOperation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportOperation*, ABI::Windows::Media::Import::IPhotoImportOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportOperation*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportSidecar;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aef5ebf0-1363-593a-86d5-f92bc230bfd6"))
IIterator<ABI::Windows::Media::Import::PhotoImportSidecar*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSidecar*, ABI::Windows::Media::Import::IPhotoImportSidecar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportSidecar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportSidecar*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2b7f92ad-e596-5669-b622-fbfbc7040e89"))
IIterable<ABI::Windows::Media::Import::PhotoImportSidecar*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSidecar*, ABI::Windows::Media::Import::IPhotoImportSidecar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportSidecar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportSidecar*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportStorageMedium;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("985cb948-9769-55dc-85d9-125a5d03d6bb"))
IIterator<ABI::Windows::Media::Import::PhotoImportStorageMedium*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportStorageMedium*, ABI::Windows::Media::Import::IPhotoImportStorageMedium*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportStorageMedium>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportStorageMedium*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3233cbfe-f9ee-560f-bd0f-e36abe6cda7f"))
IIterable<ABI::Windows::Media::Import::PhotoImportStorageMedium*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportStorageMedium*, ABI::Windows::Media::Import::IPhotoImportStorageMedium*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportStorageMedium>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportStorageMedium*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportVideoSegment;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#define DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c4c16a75-3310-5ab9-9307-78755ab1094d"))
IIterator<ABI::Windows::Media::Import::PhotoImportVideoSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportVideoSegment*, ABI::Windows::Media::Import::IPhotoImportVideoSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Import.PhotoImportVideoSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Import::PhotoImportVideoSegment*> __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t;
#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#define DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("94dd3b44-da03-5d79-bbfb-1beaf2ede482"))
IIterable<ABI::Windows::Media::Import::PhotoImportVideoSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportVideoSegment*, ABI::Windows::Media::Import::IPhotoImportVideoSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Import.PhotoImportVideoSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Import::PhotoImportVideoSegment*> __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t;
#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9a90a84e-924b-5879-88f7-bb2f7b131898"))
IVectorView<ABI::Windows::Media::Import::PhotoImportItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportItem*, ABI::Windows::Media::Import::IPhotoImportItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportItem*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a5b07808-7d18-5300-9f01-1d85149546d2"))
IVectorView<ABI::Windows::Media::Import::PhotoImportOperation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportOperation*, ABI::Windows::Media::Import::IPhotoImportOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportOperation*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a6fa3abe-cdb9-5054-bf3d-525607f9c2d2"))
IVectorView<ABI::Windows::Media::Import::PhotoImportSidecar*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSidecar*, ABI::Windows::Media::Import::IPhotoImportSidecar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportSidecar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportSidecar*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("35499439-e03e-5711-a955-f7c45928bc90"))
IVectorView<ABI::Windows::Media::Import::PhotoImportStorageMedium*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportStorageMedium*, ABI::Windows::Media::Import::IPhotoImportStorageMedium*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportStorageMedium>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportStorageMedium*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#define DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db5493cd-6915-5682-8dd5-1de144ec599d"))
IVectorView<ABI::Windows::Media::Import::PhotoImportVideoSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportVideoSegment*, ABI::Windows::Media::Import::IPhotoImportVideoSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Import.PhotoImportVideoSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Import::PhotoImportVideoSegment*> __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t;
#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_boolean_USE
#define DEF___FIReference_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c00fd60-2950-5939-a21a-2d12c5a01b8a"))
IReference<bool> : IReference_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<bool> __FIReference_1_boolean_t;
#define __FIReference_1_boolean ABI::Windows::Foundation::__FIReference_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_boolean_USE */



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
            namespace Import {
                class PhotoImportItemImportedEventArgs;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a3cce94d-f26e-58d9-8138-599ad63c7069"))
ITypedEventHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::PhotoImportItemImportedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::IPhotoImportFindItemsResult*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportItemImportedEventArgs*, ABI::Windows::Media::Import::IPhotoImportItemImportedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Import.PhotoImportFindItemsResult, Windows.Media.Import.PhotoImportItemImportedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::PhotoImportItemImportedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportSelectionChangedEventArgs;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e67279fe-692f-5602-820b-865098d9b43e"))
ITypedEventHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::PhotoImportSelectionChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::IPhotoImportFindItemsResult*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Import::PhotoImportSelectionChangedEventArgs*, ABI::Windows::Media::Import::IPhotoImportSelectionChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Import.PhotoImportFindItemsResult, Windows.Media.Import.PhotoImportSelectionChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Import::PhotoImportFindItemsResult*, ABI::Windows::Media::Import::PhotoImportSelectionChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

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
        namespace Media {
            namespace Import {
                typedef enum PhotoImportAccessMode : int PhotoImportAccessMode;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportConnectionTransport : int PhotoImportConnectionTransport;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportContentType : int PhotoImportContentType;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportContentTypeFilter : int PhotoImportContentTypeFilter;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportImportMode : int PhotoImportImportMode;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportItemSelectionMode : int PhotoImportItemSelectionMode;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportPowerSource : int PhotoImportPowerSource;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportSourceType : int PhotoImportSourceType;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportStage : int PhotoImportStage;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportStorageMediumType : int PhotoImportStorageMediumType;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportSubfolderCreationMode : int PhotoImportSubfolderCreationMode;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                typedef enum PhotoImportSubfolderDateFormat : int PhotoImportSubfolderDateFormat;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                class PhotoImportSession;
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Import.PhotoImportAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportAccessMode : int
                {
                    PhotoImportAccessMode_ReadWrite = 0,
                    PhotoImportAccessMode_ReadOnly = 1,
                    PhotoImportAccessMode_ReadAndDelete = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportConnectionTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportConnectionTransport : int
                {
                    PhotoImportConnectionTransport_Unknown = 0,
                    PhotoImportConnectionTransport_Usb = 1,
                    PhotoImportConnectionTransport_IP = 2,
                    PhotoImportConnectionTransport_Bluetooth = 3,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportContentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportContentType : int
                {
                    PhotoImportContentType_Unknown = 0,
                    PhotoImportContentType_Image = 1,
                    PhotoImportContentType_Video = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportContentTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportContentTypeFilter : int
                {
                    PhotoImportContentTypeFilter_OnlyImages = 0,
                    PhotoImportContentTypeFilter_OnlyVideos = 1,
                    PhotoImportContentTypeFilter_ImagesAndVideos = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    PhotoImportContentTypeFilter_ImagesAndVideosFromCameraRoll = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportImportMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportImportMode : int
                {
                    PhotoImportImportMode_ImportEverything = 0,
                    PhotoImportImportMode_IgnoreSidecars = 1,
                    PhotoImportImportMode_IgnoreSiblings = 2,
                    PhotoImportImportMode_IgnoreSidecarsAndSiblings = 3,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportItemSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportItemSelectionMode : int
                {
                    PhotoImportItemSelectionMode_SelectAll = 0,
                    PhotoImportItemSelectionMode_SelectNone = 1,
                    PhotoImportItemSelectionMode_SelectNew = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportPowerSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportPowerSource : int
                {
                    PhotoImportPowerSource_Unknown = 0,
                    PhotoImportPowerSource_Battery = 1,
                    PhotoImportPowerSource_External = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSourceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportSourceType : int
                {
                    PhotoImportSourceType_Generic = 0,
                    PhotoImportSourceType_Camera = 1,
                    PhotoImportSourceType_MediaPlayer = 2,
                    PhotoImportSourceType_Phone = 3,
                    PhotoImportSourceType_Video = 4,
                    PhotoImportSourceType_PersonalInfoManager = 5,
                    PhotoImportSourceType_AudioRecorder = 6,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportStage : int
                {
                    PhotoImportStage_NotStarted = 0,
                    PhotoImportStage_FindingItems = 1,
                    PhotoImportStage_ImportingItems = 2,
                    PhotoImportStage_DeletingImportedItemsFromSource = 3,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportStorageMediumType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportStorageMediumType : int
                {
                    PhotoImportStorageMediumType_Undefined = 0,
                    PhotoImportStorageMediumType_Fixed = 1,
                    PhotoImportStorageMediumType_Removable = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSubfolderCreationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportSubfolderCreationMode : int
                {
                    PhotoImportSubfolderCreationMode_DoNotCreateSubfolders = 0,
                    PhotoImportSubfolderCreationMode_CreateSubfoldersFromFileDate = 1,
                    PhotoImportSubfolderCreationMode_CreateSubfoldersFromExifDate = 2,
                    PhotoImportSubfolderCreationMode_KeepOriginalFolderStructure = 3,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSubfolderDateFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                enum PhotoImportSubfolderDateFormat : int
                {
                    PhotoImportSubfolderDateFormat_Year = 0,
                    PhotoImportSubfolderDateFormat_YearMonth = 1,
                    PhotoImportSubfolderDateFormat_YearMonthDay = 2,
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Import.PhotoImportProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                struct PhotoImportProgress
                {
                    UINT32 ItemsImported;
                    UINT32 TotalItemsToImport;
                    UINT64 BytesImported;
                    UINT64 TotalBytesToImport;
                    DOUBLE ImportProgress;
                };
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportDeleteImportedItemsFromSourceResult[] = L"Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("f4e112f8-843d-428a-a1a6-81510292b0ae")
                IPhotoImportDeleteImportedItemsFromSourceResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::Media::Import::IPhotoImportSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasSucceeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeletedItems(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalSizeInBytes(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportDeleteImportedItemsFromSourceResult = __uuidof(IPhotoImportDeleteImportedItemsFromSourceResult);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportFindItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportFindItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportFindItemsResult[] = L"Windows.Media.Import.IPhotoImportFindItemsResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("3915e647-6c78-492b-844e-8fe5e8f6bfb9")
                IPhotoImportFindItemsResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::Media::Import::IPhotoImportSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasSucceeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FoundItems(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectAll(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectNone(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectNewAsync(
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetImportMode(
                        ABI::Windows::Media::Import::PhotoImportImportMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImportMode(
                        ABI::Windows::Media::Import::PhotoImportImportMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedPhotosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedPhotosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedVideosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedVideosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedSidecarsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedSidecarsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedSiblingsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedSiblingsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedTotalCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedTotalSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SelectionChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SelectionChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ImportItemsAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ItemImported(
                        __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* value,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ItemImported(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportFindItemsResult = __uuidof(IPhotoImportFindItemsResult);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportFindItemsResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportFindItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportFindItemsResult2[] = L"Windows.Media.Import.IPhotoImportFindItemsResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("fbdd6a3b-ecf9-406a-815e-5015625b0a88")
                IPhotoImportFindItemsResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddItemsInDateRangeToSelection(
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportFindItemsResult2 = __uuidof(IPhotoImportFindItemsResult2);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportImportItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportImportItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportImportItemsResult[] = L"Windows.Media.Import.IPhotoImportImportItemsResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("e4d4f478-d419-4443-a84e-f06a850c0b00")
                IPhotoImportImportItemsResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::Media::Import::IPhotoImportSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasSucceeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImportedItems(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhotosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideosSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SidecarsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SiblingsSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteImportedItemsFromSourceAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportImportItemsResult = __uuidof(IPhotoImportImportItemsResult);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItem[] = L"Windows.Media.Import.IPhotoImportItem";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("a9d07e76-9bfc-43b8-b356-633b6a988c9e")
                IPhotoImportItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ItemKey(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                        ABI::Windows::Media::Import::PhotoImportContentType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Date(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sibling(
                        ABI::Windows::Media::Import::IPhotoImportSidecar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sidecars(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoSegments(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSelected(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSelected(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImportedFileNames(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeletedFileNames(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportItem = __uuidof(IPhotoImportItem);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItem2[] = L"Windows.Media.Import.IPhotoImportItem2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("f1053505-f53b-46a3-9e30-3610791a9110")
                IPhotoImportItem2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportItem2 = __uuidof(IPhotoImportItem2);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItemImportedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItemImportedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItemImportedEventArgs[] = L"Windows.Media.Import.IPhotoImportItemImportedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("42cb2fdd-7d68-47b5-bc7c-ceb73e0c77dc")
                IPhotoImportItemImportedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ImportedItem(
                        ABI::Windows::Media::Import::IPhotoImportItem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportItemImportedEventArgs = __uuidof(IPhotoImportItemImportedEventArgs);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportManagerStatics[] = L"Windows.Media.Import.IPhotoImportManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("2771903d-a046-4f06-9b9c-bfd662e83287")
                IPhotoImportManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupportedAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllSourcesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPendingOperations(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportManagerStatics = __uuidof(IPhotoImportManagerStatics);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportOperation[] = L"Windows.Media.Import.IPhotoImportOperation";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("d9f797e4-a09a-4ee4-a4b1-20940277a5be")
                IPhotoImportOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Stage(
                        ABI::Windows::Media::Import::PhotoImportStage* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Session(
                        ABI::Windows::Media::Import::IPhotoImportSession** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContinueFindingItemsAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContinueImportingItemsAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContinueDeletingImportedItemsFromSourceAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportOperation = __uuidof(IPhotoImportOperation);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSelectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSelectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSelectionChangedEventArgs[] = L"Windows.Media.Import.IPhotoImportSelectionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("10461782-fa9d-4c30-8bc9-4d64911572d5")
                IPhotoImportSelectionChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSelectionEmpty(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSelectionChangedEventArgs = __uuidof(IPhotoImportSelectionChangedEventArgs);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSession[] = L"Windows.Media.Import.IPhotoImportSession";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("aa63916e-ecdb-4efe-94c6-5f5cafe34cfb")
                IPhotoImportSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Media::Import::IPhotoImportSource** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DestinationFolder(
                        ABI::Windows::Storage::IStorageFolder* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DestinationFolder(
                        ABI::Windows::Storage::IStorageFolder** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AppendSessionDateToDestinationFolder(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppendSessionDateToDestinationFolder(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SubfolderCreationMode(
                        ABI::Windows::Media::Import::PhotoImportSubfolderCreationMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubfolderCreationMode(
                        ABI::Windows::Media::Import::PhotoImportSubfolderCreationMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DestinationFileNamePrefix(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DestinationFileNamePrefix(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindItemsAsync(
                        ABI::Windows::Media::Import::PhotoImportContentTypeFilter contentTypeFilter,
                        ABI::Windows::Media::Import::PhotoImportItemSelectionMode itemSelectionMode,
                        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSession = __uuidof(IPhotoImportSession);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSession2[] = L"Windows.Media.Import.IPhotoImportSession2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("2a526710-3ec6-469d-a375-2b9f4785391e")
                IPhotoImportSession2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_SubfolderDateFormat(
                        ABI::Windows::Media::Import::PhotoImportSubfolderDateFormat value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubfolderDateFormat(
                        ABI::Windows::Media::Import::PhotoImportSubfolderDateFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RememberDeselectedItems(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RememberDeselectedItems(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSession2 = __uuidof(IPhotoImportSession2);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSidecar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSidecar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSidecar[] = L"Windows.Media.Import.IPhotoImportSidecar";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("46d7d757-f802-44c7-9c98-7a71f4bc1486")
                IPhotoImportSidecar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Date(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSidecar = __uuidof(IPhotoImportSidecar);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSource[] = L"Windows.Media.Import.IPhotoImportSource";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("1f8ea35e-145b-4cd6-87f1-54965a982fef")
                IPhotoImportSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Manufacturer(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Model(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SerialNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionProtocol(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionTransport(
                        ABI::Windows::Media::Import::PhotoImportConnectionTransport* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Media::Import::PhotoImportSourceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PowerSource(
                        ABI::Windows::Media::Import::PhotoImportPowerSource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BatteryLevelPercent(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DateTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StorageMedia(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsLocked(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMassStorage(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateImportSession(
                        ABI::Windows::Media::Import::IPhotoImportSession** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSource = __uuidof(IPhotoImportSource);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSourceStatics[] = L"Windows.Media.Import.IPhotoImportSourceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("0528e586-32d8-467c-8cee-23a1b2f43e85")
                IPhotoImportSourceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING sourceId,
                        __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromFolderAsync(
                        ABI::Windows::Storage::IStorageFolder* sourceRootFolder,
                        __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportSourceStatics = __uuidof(IPhotoImportSourceStatics);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportStorageMedium
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportStorageMedium
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportStorageMedium[] = L"Windows.Media.Import.IPhotoImportStorageMedium";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("f2b9b093-fc85-487f-87c2-58d675d05b07")
                IPhotoImportStorageMedium : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SerialNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StorageMediumType(
                        ABI::Windows::Media::Import::PhotoImportStorageMediumType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedAccessMode(
                        ABI::Windows::Media::Import::PhotoImportAccessMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CapacityInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailableSpaceInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Refresh(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportStorageMedium = __uuidof(IPhotoImportStorageMedium);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportVideoSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportVideoSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportVideoSegment[] = L"Windows.Media.Import.IPhotoImportVideoSegment";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Import {
                MIDL_INTERFACE("623c0289-321a-41d8-9166-8c62a333276c")
                IPhotoImportVideoSegment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Date(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sibling(
                        ABI::Windows::Media::Import::IPhotoImportSidecar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sidecars(
                        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoImportVideoSegment = __uuidof(IPhotoImportVideoSegment);
            } /* Import */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult[] = L"Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportFindItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportFindItemsResult ** Default Interface **
 *    Windows.Media.Import.IPhotoImportFindItemsResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportFindItemsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportFindItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportFindItemsResult[] = L"Windows.Media.Import.PhotoImportFindItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportImportItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportImportItemsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportImportItemsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportImportItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportImportItemsResult[] = L"Windows.Media.Import.PhotoImportImportItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportItem ** Default Interface **
 *    Windows.Media.Import.IPhotoImportItem2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportItem_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportItem[] = L"Windows.Media.Import.PhotoImportItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportItemImportedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportItemImportedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportItemImportedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportItemImportedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportItemImportedEventArgs[] = L"Windows.Media.Import.PhotoImportItemImportedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Import.IPhotoImportManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportManager[] = L"Windows.Media.Import.PhotoImportManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportOperation_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportOperation[] = L"Windows.Media.Import.PhotoImportOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSelectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSelectionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSelectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSelectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSelectionChangedEventArgs[] = L"Windows.Media.Import.PhotoImportSelectionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Import.IPhotoImportSession2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSession[] = L"Windows.Media.Import.PhotoImportSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSidecar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSidecar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSidecar_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSidecar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSidecar[] = L"Windows.Media.Import.PhotoImportSidecar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Import.IPhotoImportSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSource[] = L"Windows.Media.Import.PhotoImportSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportStorageMedium
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportStorageMedium ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportStorageMedium_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportStorageMedium_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportStorageMedium[] = L"Windows.Media.Import.PhotoImportStorageMedium";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportVideoSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportVideoSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportVideoSegment_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportVideoSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportVideoSegment[] = L"Windows.Media.Import.PhotoImportVideoSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2 __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2 __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2 __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment;

#endif // ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSource** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource* This,
        __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CImport__CPhotoImportSource_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double* asyncInfo,
        DOUBLE progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32 __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32* asyncInfo,
        UINT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CMedia_CImport_CPhotoImportProgress __x_ABI_CWindows_CMedia_CImport_CPhotoImportProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress* asyncInfo,
        struct __x_ABI_CWindows_CMedia_CImport_CPhotoImportProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItemVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItemVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportOperation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportSidecar** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportStorageMedium** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

typedef struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl;

interface __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

typedef struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        __FIIterator_1_Windows__CMedia__CImport__CPhotoImportVideoSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl;

interface __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItemVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMediumVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment;

typedef struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl;

interface __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_boolean_INTERFACE_DEFINED__)
#define ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef interface __FIReference_1_boolean __FIReference_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_boolean;

typedef struct __FIReference_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIReference_1_booleanVtbl;

interface __FIReference_1_boolean
{
    CONST_VTBL struct __FIReference_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_boolean_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_boolean_INTERFACE_DEFINED__

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
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* sender,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* sender,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportAccessMode __x_ABI_CWindows_CMedia_CImport_CPhotoImportAccessMode;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportConnectionTransport __x_ABI_CWindows_CMedia_CImport_CPhotoImportConnectionTransport;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentType __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentType;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentTypeFilter __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentTypeFilter;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportImportMode __x_ABI_CWindows_CMedia_CImport_CPhotoImportImportMode;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportItemSelectionMode __x_ABI_CWindows_CMedia_CImport_CPhotoImportItemSelectionMode;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportPowerSource __x_ABI_CWindows_CMedia_CImport_CPhotoImportPowerSource;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSourceType __x_ABI_CWindows_CMedia_CImport_CPhotoImportSourceType;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStage __x_ABI_CWindows_CMedia_CImport_CPhotoImportStage;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStorageMediumType __x_ABI_CWindows_CMedia_CImport_CPhotoImportStorageMediumType;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderCreationMode __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderCreationMode;

typedef enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderDateFormat __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderDateFormat;

/*
 *
 * Struct Windows.Media.Import.PhotoImportAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportAccessMode
{
    PhotoImportAccessMode_ReadWrite = 0,
    PhotoImportAccessMode_ReadOnly = 1,
    PhotoImportAccessMode_ReadAndDelete = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportConnectionTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportConnectionTransport
{
    PhotoImportConnectionTransport_Unknown = 0,
    PhotoImportConnectionTransport_Usb = 1,
    PhotoImportConnectionTransport_IP = 2,
    PhotoImportConnectionTransport_Bluetooth = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportContentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentType
{
    PhotoImportContentType_Unknown = 0,
    PhotoImportContentType_Image = 1,
    PhotoImportContentType_Video = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportContentTypeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentTypeFilter
{
    PhotoImportContentTypeFilter_OnlyImages = 0,
    PhotoImportContentTypeFilter_OnlyVideos = 1,
    PhotoImportContentTypeFilter_ImagesAndVideos = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    PhotoImportContentTypeFilter_ImagesAndVideosFromCameraRoll = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportImportMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportImportMode
{
    PhotoImportImportMode_ImportEverything = 0,
    PhotoImportImportMode_IgnoreSidecars = 1,
    PhotoImportImportMode_IgnoreSiblings = 2,
    PhotoImportImportMode_IgnoreSidecarsAndSiblings = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportItemSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportItemSelectionMode
{
    PhotoImportItemSelectionMode_SelectAll = 0,
    PhotoImportItemSelectionMode_SelectNone = 1,
    PhotoImportItemSelectionMode_SelectNew = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportPowerSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportPowerSource
{
    PhotoImportPowerSource_Unknown = 0,
    PhotoImportPowerSource_Battery = 1,
    PhotoImportPowerSource_External = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSourceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSourceType
{
    PhotoImportSourceType_Generic = 0,
    PhotoImportSourceType_Camera = 1,
    PhotoImportSourceType_MediaPlayer = 2,
    PhotoImportSourceType_Phone = 3,
    PhotoImportSourceType_Video = 4,
    PhotoImportSourceType_PersonalInfoManager = 5,
    PhotoImportSourceType_AudioRecorder = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStage
{
    PhotoImportStage_NotStarted = 0,
    PhotoImportStage_FindingItems = 1,
    PhotoImportStage_ImportingItems = 2,
    PhotoImportStage_DeletingImportedItemsFromSource = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportStorageMediumType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStorageMediumType
{
    PhotoImportStorageMediumType_Undefined = 0,
    PhotoImportStorageMediumType_Fixed = 1,
    PhotoImportStorageMediumType_Removable = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSubfolderCreationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderCreationMode
{
    PhotoImportSubfolderCreationMode_DoNotCreateSubfolders = 0,
    PhotoImportSubfolderCreationMode_CreateSubfoldersFromFileDate = 1,
    PhotoImportSubfolderCreationMode_CreateSubfoldersFromExifDate = 2,
    PhotoImportSubfolderCreationMode_KeepOriginalFolderStructure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Import.PhotoImportSubfolderDateFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderDateFormat
{
    PhotoImportSubfolderDateFormat_Year = 0,
    PhotoImportSubfolderDateFormat_YearMonth = 1,
    PhotoImportSubfolderDateFormat_YearMonthDay = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Import.PhotoImportProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CMedia_CImport_CPhotoImportProgress
{
    UINT32 ItemsImported;
    UINT32 TotalItemsToImport;
    UINT64 BytesImported;
    UINT64 TotalBytesToImport;
    DOUBLE ImportProgress;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportDeleteImportedItemsFromSourceResult[] = L"Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession** value);
    HRESULT (STDMETHODCALLTYPE* get_HasSucceeded)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DeletedItems)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResultVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_HasSucceeded(This, value) \
    ((This)->lpVtbl->get_HasSucceeded(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_DeletedItems(This, value) \
    ((This)->lpVtbl->get_DeletedItems(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_PhotosCount(This, value) \
    ((This)->lpVtbl->get_PhotosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_PhotosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PhotosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_VideosCount(This, value) \
    ((This)->lpVtbl->get_VideosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_VideosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_VideosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_SidecarsCount(This, value) \
    ((This)->lpVtbl->get_SidecarsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_SidecarsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SidecarsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_SiblingsCount(This, value) \
    ((This)->lpVtbl->get_SiblingsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_SiblingsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SiblingsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_TotalCount(This, value) \
    ((This)->lpVtbl->get_TotalCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_get_TotalSizeInBytes(This, value) \
    ((This)->lpVtbl->get_TotalSizeInBytes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportDeleteImportedItemsFromSourceResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportFindItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportFindItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportFindItemsResult[] = L"Windows.Media.Import.IPhotoImportFindItemsResult";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession** value);
    HRESULT (STDMETHODCALLTYPE* get_HasSucceeded)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_FoundItems)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* SelectAll)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* SelectNone)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* SelectNewAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* SetImportMode)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportImportMode value);
    HRESULT (STDMETHODCALLTYPE* get_ImportMode)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportImportMode* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedPhotosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedPhotosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedVideosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedVideosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedSidecarsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedSidecarsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedSiblingsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedSiblingsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedTotalCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedTotalSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* add_SelectionChanged)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportSelectionChangedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SelectionChanged)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* ImportItemsAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** operation);
    HRESULT (STDMETHODCALLTYPE* add_ItemImported)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        __FITypedEventHandler_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_Windows__CMedia__CImport__CPhotoImportItemImportedEventArgs* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ItemImported)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResultVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_HasSucceeded(This, value) \
    ((This)->lpVtbl->get_HasSucceeded(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_FoundItems(This, value) \
    ((This)->lpVtbl->get_FoundItems(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_PhotosCount(This, value) \
    ((This)->lpVtbl->get_PhotosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_PhotosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PhotosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_VideosCount(This, value) \
    ((This)->lpVtbl->get_VideosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_VideosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_VideosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SidecarsCount(This, value) \
    ((This)->lpVtbl->get_SidecarsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SidecarsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SidecarsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SiblingsCount(This, value) \
    ((This)->lpVtbl->get_SiblingsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SiblingsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SiblingsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_TotalCount(This, value) \
    ((This)->lpVtbl->get_TotalCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_TotalSizeInBytes(This, value) \
    ((This)->lpVtbl->get_TotalSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_SelectAll(This) \
    ((This)->lpVtbl->SelectAll(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_SelectNone(This) \
    ((This)->lpVtbl->SelectNone(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_SelectNewAsync(This, action) \
    ((This)->lpVtbl->SelectNewAsync(This, action))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_SetImportMode(This, value) \
    ((This)->lpVtbl->SetImportMode(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_ImportMode(This, value) \
    ((This)->lpVtbl->get_ImportMode(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedPhotosCount(This, value) \
    ((This)->lpVtbl->get_SelectedPhotosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedPhotosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SelectedPhotosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedVideosCount(This, value) \
    ((This)->lpVtbl->get_SelectedVideosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedVideosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SelectedVideosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedSidecarsCount(This, value) \
    ((This)->lpVtbl->get_SelectedSidecarsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedSidecarsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SelectedSidecarsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedSiblingsCount(This, value) \
    ((This)->lpVtbl->get_SelectedSiblingsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedSiblingsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SelectedSiblingsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedTotalCount(This, value) \
    ((This)->lpVtbl->get_SelectedTotalCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_get_SelectedTotalSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SelectedTotalSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_add_SelectionChanged(This, value, token) \
    ((This)->lpVtbl->add_SelectionChanged(This, value, token))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_remove_SelectionChanged(This, token) \
    ((This)->lpVtbl->remove_SelectionChanged(This, token))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_ImportItemsAsync(This, operation) \
    ((This)->lpVtbl->ImportItemsAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_add_ItemImported(This, value, token) \
    ((This)->lpVtbl->add_ItemImported(This, value, token))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_remove_ItemImported(This, token) \
    ((This)->lpVtbl->remove_ItemImported(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportFindItemsResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportFindItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportFindItemsResult2[] = L"Windows.Media.Import.IPhotoImportFindItemsResult2";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddItemsInDateRangeToSelection)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_AddItemsInDateRangeToSelection(This, rangeStart, rangeLength) \
    ((This)->lpVtbl->AddItemsInDateRangeToSelection(This, rangeStart, rangeLength))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportFindItemsResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportImportItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportImportItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportImportItemsResult[] = L"Windows.Media.Import.IPhotoImportImportItemsResult";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession** value);
    HRESULT (STDMETHODCALLTYPE* get_HasSucceeded)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ImportedItems)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportItem** value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PhotosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_VideosSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SidecarsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SiblingsSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalCount)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalSizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* DeleteImportedItemsFromSourceAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResultVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_HasSucceeded(This, value) \
    ((This)->lpVtbl->get_HasSucceeded(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_ImportedItems(This, value) \
    ((This)->lpVtbl->get_ImportedItems(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_PhotosCount(This, value) \
    ((This)->lpVtbl->get_PhotosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_PhotosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_PhotosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_VideosCount(This, value) \
    ((This)->lpVtbl->get_VideosCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_VideosSizeInBytes(This, value) \
    ((This)->lpVtbl->get_VideosSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_SidecarsCount(This, value) \
    ((This)->lpVtbl->get_SidecarsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_SidecarsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SidecarsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_SiblingsCount(This, value) \
    ((This)->lpVtbl->get_SiblingsCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_SiblingsSizeInBytes(This, value) \
    ((This)->lpVtbl->get_SiblingsSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_TotalCount(This, value) \
    ((This)->lpVtbl->get_TotalCount(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_get_TotalSizeInBytes(This, value) \
    ((This)->lpVtbl->get_TotalSizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_DeleteImportedItemsFromSourceAsync(This, result) \
    ((This)->lpVtbl->DeleteImportedItemsFromSourceAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportImportItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItem[] = L"Windows.Media.Import.IPhotoImportItem";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ItemKey)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentType* value);
    HRESULT (STDMETHODCALLTYPE* get_SizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Date)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Sibling)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** value);
    HRESULT (STDMETHODCALLTYPE* get_Sidecars)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoSegments)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportVideoSegment** value);
    HRESULT (STDMETHODCALLTYPE* get_IsSelected)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSelected)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_ImportedFileNames)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DeletedFileNames)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_ItemKey(This, value) \
    ((This)->lpVtbl->get_ItemKey(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_SizeInBytes(This, value) \
    ((This)->lpVtbl->get_SizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_Date(This, value) \
    ((This)->lpVtbl->get_Date(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_Sibling(This, value) \
    ((This)->lpVtbl->get_Sibling(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_Sidecars(This, value) \
    ((This)->lpVtbl->get_Sidecars(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_VideoSegments(This, value) \
    ((This)->lpVtbl->get_VideoSegments(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_IsSelected(This, value) \
    ((This)->lpVtbl->get_IsSelected(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_put_IsSelected(This, value) \
    ((This)->lpVtbl->put_IsSelected(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_ImportedFileNames(This, value) \
    ((This)->lpVtbl->get_ImportedFileNames(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_get_DeletedFileNames(This, value) \
    ((This)->lpVtbl->get_DeletedFileNames(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItem2[] = L"Windows.Media.Import.IPhotoImportItem2";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2Vtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportItemImportedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportItemImportedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportItemImportedEventArgs[] = L"Windows.Media.Import.IPhotoImportItemImportedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ImportedItem)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_get_ImportedItem(This, value) \
    ((This)->lpVtbl->get_ImportedItem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportItemImportedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportManagerStatics[] = L"Windows.Media.Import.IPhotoImportManagerStatics";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupportedAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllSourcesAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSource** operation);
    HRESULT (STDMETHODCALLTYPE* GetPendingOperations)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportOperation** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_IsSupportedAsync(This, operation) \
    ((This)->lpVtbl->IsSupportedAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_FindAllSourcesAsync(This, operation) \
    ((This)->lpVtbl->FindAllSourcesAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_GetPendingOperations(This, result) \
    ((This)->lpVtbl->GetPendingOperations(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportOperation[] = L"Windows.Media.Import.IPhotoImportOperation";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Stage)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStage* value);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession** value);
    HRESULT (STDMETHODCALLTYPE* get_ContinueFindingItemsAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* get_ContinueImportingItemsAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportImportItemsResult_Windows__CMedia__CImport__CPhotoImportProgress** operation);
    HRESULT (STDMETHODCALLTYPE* get_ContinueDeletingImportedItemsFromSourceAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportDeleteImportedItemsFromSourceResult_double** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperationVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_get_Stage(This, value) \
    ((This)->lpVtbl->get_Stage(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_get_ContinueFindingItemsAsync(This, operation) \
    ((This)->lpVtbl->get_ContinueFindingItemsAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_get_ContinueImportingItemsAsync(This, operation) \
    ((This)->lpVtbl->get_ContinueImportingItemsAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_get_ContinueDeletingImportedItemsFromSourceAsync(This, operation) \
    ((This)->lpVtbl->get_ContinueDeletingImportedItemsFromSourceAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSelectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSelectionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSelectionChangedEventArgs[] = L"Windows.Media.Import.IPhotoImportSelectionChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSelectionEmpty)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_get_IsSelectionEmpty(This, value) \
    ((This)->lpVtbl->get_IsSelectionEmpty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSelectionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSession[] = L"Windows.Media.Import.IPhotoImportSession";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource** value);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DestinationFolder)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* get_DestinationFolder)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* put_AppendSessionDateToDestinationFolder)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AppendSessionDateToDestinationFolder)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_SubfolderCreationMode)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderCreationMode value);
    HRESULT (STDMETHODCALLTYPE* get_SubfolderCreationMode)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderCreationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_DestinationFileNamePrefix)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DestinationFileNamePrefix)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FindItemsAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportContentTypeFilter contentTypeFilter,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportItemSelectionMode itemSelectionMode,
        __FIAsyncOperationWithProgress_2_Windows__CMedia__CImport__CPhotoImportFindItemsResult_UINT32** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSessionVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_put_DestinationFolder(This, value) \
    ((This)->lpVtbl->put_DestinationFolder(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_DestinationFolder(This, value) \
    ((This)->lpVtbl->get_DestinationFolder(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_put_AppendSessionDateToDestinationFolder(This, value) \
    ((This)->lpVtbl->put_AppendSessionDateToDestinationFolder(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_AppendSessionDateToDestinationFolder(This, value) \
    ((This)->lpVtbl->get_AppendSessionDateToDestinationFolder(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_put_SubfolderCreationMode(This, value) \
    ((This)->lpVtbl->put_SubfolderCreationMode(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_SubfolderCreationMode(This, value) \
    ((This)->lpVtbl->get_SubfolderCreationMode(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_put_DestinationFileNamePrefix(This, value) \
    ((This)->lpVtbl->put_DestinationFileNamePrefix(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_get_DestinationFileNamePrefix(This, value) \
    ((This)->lpVtbl->get_DestinationFileNamePrefix(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_FindItemsAsync(This, contentTypeFilter, itemSelectionMode, operation) \
    ((This)->lpVtbl->FindItemsAsync(This, contentTypeFilter, itemSelectionMode, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSession2[] = L"Windows.Media.Import.IPhotoImportSession2";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_SubfolderDateFormat)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderDateFormat value);
    HRESULT (STDMETHODCALLTYPE* get_SubfolderDateFormat)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSubfolderDateFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_RememberDeselectedItems)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RememberDeselectedItems)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2Vtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_put_SubfolderDateFormat(This, value) \
    ((This)->lpVtbl->put_SubfolderDateFormat(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_get_SubfolderDateFormat(This, value) \
    ((This)->lpVtbl->get_SubfolderDateFormat(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_put_RememberDeselectedItems(This, value) \
    ((This)->lpVtbl->put_RememberDeselectedItems(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_get_RememberDeselectedItems(This, value) \
    ((This)->lpVtbl->get_RememberDeselectedItems(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSidecar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSidecar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSidecar[] = L"Windows.Media.Import.IPhotoImportSidecar";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Date)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecarVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_get_SizeInBytes(This, value) \
    ((This)->lpVtbl->get_SizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_get_Date(This, value) \
    ((This)->lpVtbl->get_Date(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSource[] = L"Windows.Media.Import.IPhotoImportSource";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Manufacturer)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Model)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SerialNumber)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionProtocol)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionTransport)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportConnectionTransport* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportSourceType* value);
    HRESULT (STDMETHODCALLTYPE* get_PowerSource)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportPowerSource* value);
    HRESULT (STDMETHODCALLTYPE* get_BatteryLevelPercent)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_DateTime)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_StorageMedia)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportStorageMedium** value);
    HRESULT (STDMETHODCALLTYPE* get_IsLocked)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* get_IsMassStorage)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* CreateImportSession)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSession** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Manufacturer(This, value) \
    ((This)->lpVtbl->get_Manufacturer(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Model(This, value) \
    ((This)->lpVtbl->get_Model(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_SerialNumber(This, value) \
    ((This)->lpVtbl->get_SerialNumber(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_ConnectionProtocol(This, value) \
    ((This)->lpVtbl->get_ConnectionProtocol(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_ConnectionTransport(This, value) \
    ((This)->lpVtbl->get_ConnectionTransport(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_PowerSource(This, value) \
    ((This)->lpVtbl->get_PowerSource(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_BatteryLevelPercent(This, value) \
    ((This)->lpVtbl->get_BatteryLevelPercent(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_DateTime(This, value) \
    ((This)->lpVtbl->get_DateTime(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_StorageMedia(This, value) \
    ((This)->lpVtbl->get_StorageMedia(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_IsLocked(This, value) \
    ((This)->lpVtbl->get_IsLocked(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_IsMassStorage(This, value) \
    ((This)->lpVtbl->get_IsMassStorage(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_CreateImportSession(This, result) \
    ((This)->lpVtbl->CreateImportSession(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportSourceStatics[] = L"Windows.Media.Import.IPhotoImportSourceStatics";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        HSTRING sourceId,
        __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource** operation);
    HRESULT (STDMETHODCALLTYPE* FromFolderAsync)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* sourceRootFolder,
        __FIAsyncOperation_1_Windows__CMedia__CImport__CPhotoImportSource** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FromIdAsync(This, sourceId, operation) \
    ((This)->lpVtbl->FromIdAsync(This, sourceId, operation))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_FromFolderAsync(This, sourceRootFolder, operation) \
    ((This)->lpVtbl->FromFolderAsync(This, sourceRootFolder, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportStorageMedium
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportStorageMedium
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportStorageMedium[] = L"Windows.Media.Import.IPhotoImportStorageMedium";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMediumVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SerialNumber)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StorageMediumType)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportStorageMediumType* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedAccessMode)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        enum __x_ABI_CWindows_CMedia_CImport_CPhotoImportAccessMode* value);
    HRESULT (STDMETHODCALLTYPE* get_CapacityInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableSpaceInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* Refresh)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMediumVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMediumVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_SerialNumber(This, value) \
    ((This)->lpVtbl->get_SerialNumber(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_StorageMediumType(This, value) \
    ((This)->lpVtbl->get_StorageMediumType(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_SupportedAccessMode(This, value) \
    ((This)->lpVtbl->get_SupportedAccessMode(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_CapacityInBytes(This, value) \
    ((This)->lpVtbl->get_CapacityInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_get_AvailableSpaceInBytes(This, value) \
    ((This)->lpVtbl->get_AvailableSpaceInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_Refresh(This) \
    ((This)->lpVtbl->Refresh(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportStorageMedium_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Import.IPhotoImportVideoSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Import.PhotoImportVideoSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Import_IPhotoImportVideoSegment[] = L"Windows.Media.Import.IPhotoImportVideoSegment";
typedef struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SizeInBytes)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Date)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Sibling)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        __x_ABI_CWindows_CMedia_CImport_CIPhotoImportSidecar** value);
    HRESULT (STDMETHODCALLTYPE* get_Sidecars)(__x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment* This,
        __FIVectorView_1_Windows__CMedia__CImport__CPhotoImportSidecar** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegmentVtbl;

interface __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_get_SizeInBytes(This, value) \
    ((This)->lpVtbl->get_SizeInBytes(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_get_Date(This, value) \
    ((This)->lpVtbl->get_Date(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_get_Sibling(This, value) \
    ((This)->lpVtbl->get_Sibling(This, value))

#define __x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_get_Sidecars(This, value) \
    ((This)->lpVtbl->get_Sidecars(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment;
#endif /* !defined(____x_ABI_CWindows_CMedia_CImport_CIPhotoImportVideoSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportDeleteImportedItemsFromSourceResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportDeleteImportedItemsFromSourceResult[] = L"Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportFindItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportFindItemsResult ** Default Interface **
 *    Windows.Media.Import.IPhotoImportFindItemsResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportFindItemsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportFindItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportFindItemsResult[] = L"Windows.Media.Import.PhotoImportFindItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportImportItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportImportItemsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportImportItemsResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportImportItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportImportItemsResult[] = L"Windows.Media.Import.PhotoImportImportItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportItem ** Default Interface **
 *    Windows.Media.Import.IPhotoImportItem2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportItem_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportItem[] = L"Windows.Media.Import.PhotoImportItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportItemImportedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportItemImportedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportItemImportedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportItemImportedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportItemImportedEventArgs[] = L"Windows.Media.Import.PhotoImportItemImportedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Import.IPhotoImportManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportManager[] = L"Windows.Media.Import.PhotoImportManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportOperation_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportOperation[] = L"Windows.Media.Import.PhotoImportOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSelectionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSelectionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSelectionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSelectionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSelectionChangedEventArgs[] = L"Windows.Media.Import.PhotoImportSelectionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Import.IPhotoImportSession2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSession[] = L"Windows.Media.Import.PhotoImportSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSidecar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSidecar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSidecar_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSidecar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSidecar[] = L"Windows.Media.Import.PhotoImportSidecar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Import.IPhotoImportSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportSource ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportSource[] = L"Windows.Media.Import.PhotoImportSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportStorageMedium
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportStorageMedium ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportStorageMedium_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportStorageMedium_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportStorageMedium[] = L"Windows.Media.Import.PhotoImportStorageMedium";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Import.PhotoImportVideoSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Import.IPhotoImportVideoSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Import_PhotoImportVideoSegment_DEFINED
#define RUNTIMECLASS_Windows_Media_Import_PhotoImportVideoSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Import_PhotoImportVideoSegment[] = L"Windows.Media.Import.PhotoImportVideoSegment";
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
#endif // __windows2Emedia2Eimport_p_h__

#endif // __windows2Emedia2Eimport_h__
