
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
#ifndef __windows2Estorage2Ebulkaccess_h__
#define __windows2Estorage2Ebulkaccess_h__
#ifndef __windows2Estorage2Ebulkaccess_p_h__
#define __windows2Estorage2Ebulkaccess_p_h__


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
#include "Windows.Storage.FileProperties.h"
#include "Windows.Storage.Search.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                interface IFileInformationFactory;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory ABI::Windows::Storage::BulkAccess::IFileInformationFactory

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                interface IFileInformationFactoryFactory;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory ABI::Windows::Storage::BulkAccess::IFileInformationFactoryFactory

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                interface IStorageItemInformation;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation ABI::Windows::Storage::BulkAccess::IStorageItemInformation

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                class FileInformation;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#define DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bf2f6543-230e-50bf-9c57-9e4ba8635903"))
IIterator<ABI::Windows::Storage::BulkAccess::FileInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FileInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.BulkAccess.FileInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::BulkAccess::FileInformation*> __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_t;
#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#define DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dd96d7e9-892b-5932-b677-5bc32588008f"))
IIterable<ABI::Windows::Storage::BulkAccess::FileInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FileInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.BulkAccess.FileInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::BulkAccess::FileInformation*> __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_t;
#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#define DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("50bcd975-67ba-53b7-a5a7-1fb59f04bbb3"))
IVectorView<ABI::Windows::Storage::BulkAccess::FileInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FileInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FileInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::BulkAccess::FileInformation*> __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t;
#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("222f6bb6-e71e-55cb-885d-e051e35995dc"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FileInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ebdb2c85-d27a-5c93-a1b3-6ca3651ada5d"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FileInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                class FolderInformation;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#define DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ff68b5b6-caad-553a-9808-95eea700c9de"))
IIterator<ABI::Windows::Storage::BulkAccess::FolderInformation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FolderInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.BulkAccess.FolderInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::BulkAccess::FolderInformation*> __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_t;
#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#define DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c720bf5-7636-51fd-9ef7-d5f57f071a9b"))
IIterable<ABI::Windows::Storage::BulkAccess::FolderInformation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FolderInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.BulkAccess.FolderInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::BulkAccess::FolderInformation*> __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_t;
#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#define DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("506f1329-dbdc-5a37-91d5-b047cb24276d"))
IVectorView<ABI::Windows::Storage::BulkAccess::FolderInformation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::BulkAccess::FolderInformation*, ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FolderInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::BulkAccess::FolderInformation*> __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t;
#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7f10e569-2bf7-5752-8f75-602809a7d304"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FolderInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("020713ec-604a-5e45-b03f-1b9e65253804"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.FolderInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#define DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3da6401d-1279-55a1-962c-25cd23b99b27"))
IIterator<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> : IIterator_impl<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.BulkAccess.IStorageItemInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t;
#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#define DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43bc252e-a3d6-5f00-a12c-b088d3b912d4"))
IIterable<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> : IIterable_impl<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.BulkAccess.IStorageItemInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t;
#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#define DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("da3a4ef8-d315-529b-a73b-524490573f7e"))
IVectorView<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> : IVectorView_impl<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.IStorageItemInformation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*> __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t;
#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("413d160c-3e1a-5603-acba-1e176d6a6082"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.IStorageItemInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ff163034-ece9-55be-a6f8-08c72aae56b4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.BulkAccess.IStorageItemInformation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5b98b352-e0cf-58de-b2ec-4fd786bbb5a7"))
ITypedEventHandler<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.BulkAccess.IStorageItemInformation, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::BulkAccess::IStorageItemInformation*, IInspectable*> __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class BasicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class DocumentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class ImageProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class MusicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class StorageItemThumbnail;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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
                typedef enum ThumbnailMode : int ThumbnailMode;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                typedef enum ThumbnailOptions : unsigned int ThumbnailOptions;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class VideoProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile2 ABI::Windows::Storage::IStorageFile2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFilePropertiesWithAvailability;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability ABI::Windows::Storage::IStorageFilePropertiesWithAvailability

#endif // ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder2 ABI::Windows::Storage::IStorageFolder2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItem;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItem ABI::Windows::Storage::IStorageItem

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItem2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItem2 ABI::Windows::Storage::IStorageItem2

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItemProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItemProperties ABI::Windows::Storage::IStorageItemProperties

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItemPropertiesWithProvider;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider ABI::Windows::Storage::IStorageItemPropertiesWithProvider

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Search {
                interface IStorageFolderQueryOperations;
            } /* Search */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations ABI::Windows::Storage::Search::IStorageFolderQueryOperations

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Search {
                interface IStorageQueryResultBase;
            } /* Search */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase ABI::Windows::Storage::Search::IStorageQueryResultBase

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference ABI::Windows::Storage::Streams::IInputStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

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
        namespace Storage {
            namespace BulkAccess {
                class FileInformationFactory;
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Storage.BulkAccess.IFileInformationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.BulkAccess.FileInformationFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IFileInformationFactory[] = L"Windows.Storage.BulkAccess.IFileInformationFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                MIDL_INTERFACE("401d88be-960f-4d6d-a7d0-1a3861e76c83")
                IFileInformationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetItemsAsync(
                        UINT32 startIndex,
                        UINT32 maxItemsToRetrieve,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetItemsAsyncDefaultStartAndCount(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFilesAsync(
                        UINT32 startIndex,
                        UINT32 maxItemsToRetrieve,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFilesAsyncDefaultStartAndCount(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFoldersAsync(
                        UINT32 startIndex,
                        UINT32 maxItemsToRetrieve,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFoldersAsyncDefaultStartAndCount(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVirtualizedItemsVector(
                        IInspectable** vector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVirtualizedFilesVector(
                        IInspectable** vector
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVirtualizedFoldersVector(
                        IInspectable** vector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileInformationFactory = __uuidof(IFileInformationFactory);
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.BulkAccess.IFileInformationFactoryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.BulkAccess.FileInformationFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IFileInformationFactoryFactory[] = L"Windows.Storage.BulkAccess.IFileInformationFactoryFactory";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                MIDL_INTERFACE("84ea0e7d-e4a2-4f00-8afa-af5e0f826bd5")
                IFileInformationFactoryFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithMode(
                        ABI::Windows::Storage::Search::IStorageQueryResultBase* queryResult,
                        ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                        ABI::Windows::Storage::BulkAccess::IFileInformationFactory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithModeAndSize(
                        ABI::Windows::Storage::Search::IStorageQueryResultBase* queryResult,
                        ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                        UINT32 requestedThumbnailSize,
                        ABI::Windows::Storage::BulkAccess::IFileInformationFactory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithModeAndSizeAndOptions(
                        ABI::Windows::Storage::Search::IStorageQueryResultBase* queryResult,
                        ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                        UINT32 requestedThumbnailSize,
                        ABI::Windows::Storage::FileProperties::ThumbnailOptions thumbnailOptions,
                        ABI::Windows::Storage::BulkAccess::IFileInformationFactory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithModeAndSizeAndOptionsAndFlags(
                        ABI::Windows::Storage::Search::IStorageQueryResultBase* queryResult,
                        ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                        UINT32 requestedThumbnailSize,
                        ABI::Windows::Storage::FileProperties::ThumbnailOptions thumbnailOptions,
                        boolean delayLoad,
                        ABI::Windows::Storage::BulkAccess::IFileInformationFactory** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileInformationFactoryFactory = __uuidof(IFileInformationFactoryFactory);
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.BulkAccess.IStorageItemInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IStorageItemInformation[] = L"Windows.Storage.BulkAccess.IStorageItemInformation";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace BulkAccess {
                MIDL_INTERFACE("87a5cb8b-8972-4f40-8de0-d86fb179d8fa")
                IStorageItemInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MusicProperties(
                        ABI::Windows::Storage::FileProperties::IMusicProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoProperties(
                        ABI::Windows::Storage::FileProperties::IVideoProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageProperties(
                        ABI::Windows::Storage::FileProperties::IImageProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentProperties(
                        ABI::Windows::Storage::FileProperties::IDocumentProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BasicProperties(
                        ABI::Windows::Storage::FileProperties::IBasicProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ThumbnailUpdated(
                        __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* changedHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ThumbnailUpdated(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PropertiesUpdated(
                        __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* changedHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PropertiesUpdated(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStorageItemInformation = __uuidof(IStorageItemInformation);
            } /* BulkAccess */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FileInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IStorageItemInformation ** Default Interface **
 *    Windows.Storage.IStorageFile
 *    Windows.Storage.Streams.IInputStreamReference
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFilePropertiesWithAvailability
 *    Windows.Storage.IStorageFile2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformation_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FileInformation[] = L"Windows.Storage.BulkAccess.FileInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FileInformationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.BulkAccess.IFileInformationFactoryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IFileInformationFactory ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformationFactory_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformationFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FileInformationFactory[] = L"Windows.Storage.BulkAccess.FileInformationFactory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FolderInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IStorageItemInformation ** Default Interface **
 *    Windows.Storage.IStorageFolder
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.Search.IStorageFolderQueryOperations
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageFolder2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FolderInformation_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FolderInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FolderInformation[] = L"Windows.Storage.BulkAccess.FolderInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory;

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory;

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation;

#endif // ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation;

typedef struct __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl;

interface __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation;

typedef struct __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __FIIterator_1_Windows__CStorage__CBulkAccess__CFileInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl;

interface __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

typedef struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl;

interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation;

typedef struct __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl;

interface __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation;

typedef struct __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __FIIterator_1_Windows__CStorage__CBulkAccess__CFolderInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl;

interface __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

typedef struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl;

interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

typedef struct __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl;

interface __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

typedef struct __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __FIIterator_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl;

interface __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

typedef struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl;

interface __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* This,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode;

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions;

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile2 __x_ABI_CWindows_CStorage_CIStorageFile2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder2 __x_ABI_CWindows_CStorage_CIStorageFolder2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItem __x_ABI_CWindows_CStorage_CIStorageItem;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItem2 __x_ABI_CWindows_CStorage_CIStorageItem2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItemProperties __x_ABI_CWindows_CStorage_CIStorageItemProperties;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations;

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase;

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

/*
 *
 * Interface Windows.Storage.BulkAccess.IFileInformationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.BulkAccess.FileInformationFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IFileInformationFactory[] = L"Windows.Storage.BulkAccess.IFileInformationFactory";
typedef struct __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetItemsAsync)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        UINT32 startIndex,
        UINT32 maxItemsToRetrieve,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetItemsAsyncDefaultStartAndCount)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CIStorageItemInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetFilesAsync)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        UINT32 startIndex,
        UINT32 maxItemsToRetrieve,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetFilesAsyncDefaultStartAndCount)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFileInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetFoldersAsync)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        UINT32 startIndex,
        UINT32 maxItemsToRetrieve,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetFoldersAsyncDefaultStartAndCount)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CBulkAccess__CFolderInformation** operation);
    HRESULT (STDMETHODCALLTYPE* GetVirtualizedItemsVector)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        IInspectable** vector);
    HRESULT (STDMETHODCALLTYPE* GetVirtualizedFilesVector)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        IInspectable** vector);
    HRESULT (STDMETHODCALLTYPE* GetVirtualizedFoldersVector)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory* This,
        IInspectable** vector);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetItemsAsync(This, startIndex, maxItemsToRetrieve, operation) \
    ((This)->lpVtbl->GetItemsAsync(This, startIndex, maxItemsToRetrieve, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetItemsAsyncDefaultStartAndCount(This, operation) \
    ((This)->lpVtbl->GetItemsAsyncDefaultStartAndCount(This, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetFilesAsync(This, startIndex, maxItemsToRetrieve, operation) \
    ((This)->lpVtbl->GetFilesAsync(This, startIndex, maxItemsToRetrieve, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetFilesAsyncDefaultStartAndCount(This, operation) \
    ((This)->lpVtbl->GetFilesAsyncDefaultStartAndCount(This, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetFoldersAsync(This, startIndex, maxItemsToRetrieve, operation) \
    ((This)->lpVtbl->GetFoldersAsync(This, startIndex, maxItemsToRetrieve, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetFoldersAsyncDefaultStartAndCount(This, operation) \
    ((This)->lpVtbl->GetFoldersAsyncDefaultStartAndCount(This, operation))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetVirtualizedItemsVector(This, vector) \
    ((This)->lpVtbl->GetVirtualizedItemsVector(This, vector))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetVirtualizedFilesVector(This, vector) \
    ((This)->lpVtbl->GetVirtualizedFilesVector(This, vector))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_GetVirtualizedFoldersVector(This, vector) \
    ((This)->lpVtbl->GetVirtualizedFoldersVector(This, vector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.BulkAccess.IFileInformationFactoryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.BulkAccess.FileInformationFactory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IFileInformationFactoryFactory[] = L"Windows.Storage.BulkAccess.IFileInformationFactoryFactory";
typedef struct __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithMode)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase* queryResult,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithModeAndSize)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase* queryResult,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedThumbnailSize,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithModeAndSizeAndOptions)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase* queryResult,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedThumbnailSize,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions thumbnailOptions,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithModeAndSizeAndOptionsAndFlags)(__x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory* This,
        __x_ABI_CWindows_CStorage_CSearch_CIStorageQueryResultBase* queryResult,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedThumbnailSize,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions thumbnailOptions,
        boolean delayLoad,
        __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactory** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactoryVtbl;

interface __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_CreateWithMode(This, queryResult, mode, value) \
    ((This)->lpVtbl->CreateWithMode(This, queryResult, mode, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_CreateWithModeAndSize(This, queryResult, mode, requestedThumbnailSize, value) \
    ((This)->lpVtbl->CreateWithModeAndSize(This, queryResult, mode, requestedThumbnailSize, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_CreateWithModeAndSizeAndOptions(This, queryResult, mode, requestedThumbnailSize, thumbnailOptions, value) \
    ((This)->lpVtbl->CreateWithModeAndSizeAndOptions(This, queryResult, mode, requestedThumbnailSize, thumbnailOptions, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_CreateWithModeAndSizeAndOptionsAndFlags(This, queryResult, mode, requestedThumbnailSize, thumbnailOptions, delayLoad, value) \
    ((This)->lpVtbl->CreateWithModeAndSizeAndOptionsAndFlags(This, queryResult, mode, requestedThumbnailSize, thumbnailOptions, delayLoad, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIFileInformationFactoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.BulkAccess.IStorageItemInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_BulkAccess_IStorageItemInformation[] = L"Windows.Storage.BulkAccess.IStorageItemInformation";
typedef struct __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MusicProperties)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIMusicProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoProperties)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIVideoProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageProperties)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIImageProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentProperties)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIDocumentProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_BasicProperties)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
    HRESULT (STDMETHODCALLTYPE* add_ThumbnailUpdated)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* changedHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ThumbnailUpdated)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_PropertiesUpdated)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        __FITypedEventHandler_2_Windows__CStorage__CBulkAccess__CIStorageItemInformation_IInspectable* changedHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_PropertiesUpdated)(__x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformationVtbl;

interface __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_MusicProperties(This, value) \
    ((This)->lpVtbl->get_MusicProperties(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_VideoProperties(This, value) \
    ((This)->lpVtbl->get_VideoProperties(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_ImageProperties(This, value) \
    ((This)->lpVtbl->get_ImageProperties(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_DocumentProperties(This, value) \
    ((This)->lpVtbl->get_DocumentProperties(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_BasicProperties(This, value) \
    ((This)->lpVtbl->get_BasicProperties(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_add_ThumbnailUpdated(This, changedHandler, eventCookie) \
    ((This)->lpVtbl->add_ThumbnailUpdated(This, changedHandler, eventCookie))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_remove_ThumbnailUpdated(This, eventCookie) \
    ((This)->lpVtbl->remove_ThumbnailUpdated(This, eventCookie))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_add_PropertiesUpdated(This, changedHandler, eventCookie) \
    ((This)->lpVtbl->add_PropertiesUpdated(This, changedHandler, eventCookie))

#define __x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_remove_PropertiesUpdated(This, eventCookie) \
    ((This)->lpVtbl->remove_PropertiesUpdated(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation;
#endif /* !defined(____x_ABI_CWindows_CStorage_CBulkAccess_CIStorageItemInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FileInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IStorageItemInformation ** Default Interface **
 *    Windows.Storage.IStorageFile
 *    Windows.Storage.Streams.IInputStreamReference
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFilePropertiesWithAvailability
 *    Windows.Storage.IStorageFile2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformation_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FileInformation[] = L"Windows.Storage.BulkAccess.FileInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FileInformationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Storage.BulkAccess.IFileInformationFactoryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IFileInformationFactory ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformationFactory_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FileInformationFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FileInformationFactory[] = L"Windows.Storage.BulkAccess.FileInformationFactory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.BulkAccess.FolderInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.BulkAccess.IStorageItemInformation ** Default Interface **
 *    Windows.Storage.IStorageFolder
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.Search.IStorageFolderQueryOperations
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageFolder2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_BulkAccess_FolderInformation_DEFINED
#define RUNTIMECLASS_Windows_Storage_BulkAccess_FolderInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_BulkAccess_FolderInformation[] = L"Windows.Storage.BulkAccess.FolderInformation";
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
#endif // __windows2Estorage2Ebulkaccess_p_h__

#endif // __windows2Estorage2Ebulkaccess_h__
