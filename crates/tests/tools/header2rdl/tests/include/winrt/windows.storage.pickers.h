
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
#ifndef __windows2Estorage2Epickers_h__
#define __windows2Estorage2Epickers_h__
#ifndef __windows2Estorage2Epickers_p_h__
#define __windows2Estorage2Epickers_p_h__


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
#include "Windows.Storage.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker ABI::Windows::Storage::Pickers::IFileOpenPicker

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPicker2;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2 ABI::Windows::Storage::Pickers::IFileOpenPicker2

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPicker3;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3 ABI::Windows::Storage::Pickers::IFileOpenPicker3

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPickerStatics;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics ABI::Windows::Storage::Pickers::IFileOpenPickerStatics

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPickerStatics2;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2 ABI::Windows::Storage::Pickers::IFileOpenPickerStatics2

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileOpenPickerWithOperationId;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId ABI::Windows::Storage::Pickers::IFileOpenPickerWithOperationId

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileSavePicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker ABI::Windows::Storage::Pickers::IFileSavePicker

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileSavePicker2;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2 ABI::Windows::Storage::Pickers::IFileSavePicker2

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileSavePicker3;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3 ABI::Windows::Storage::Pickers::IFileSavePicker3

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileSavePicker4;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4 ABI::Windows::Storage::Pickers::IFileSavePicker4

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFileSavePickerStatics;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics ABI::Windows::Storage::Pickers::IFileSavePickerStatics

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFolderPicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker ABI::Windows::Storage::Pickers::IFolderPicker

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFolderPicker2;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2 ABI::Windows::Storage::Pickers::IFolderPicker2

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFolderPicker3;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3 ABI::Windows::Storage::Pickers::IFolderPicker3

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                interface IFolderPickerStatics;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics ABI::Windows::Storage::Pickers::IFolderPickerStatics

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43e29f53-0298-55aa-a6c8-4edd323d9598"))
IIterator<ABI::Windows::Storage::StorageFile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageFile*> __FIIterator_1_Windows__CStorage__CStorageFile_t;
#define __FIIterator_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9ac00304-83ea-5688-87b6-ae38aab65d0b"))
IIterable<ABI::Windows::Storage::StorageFile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageFile*> __FIIterable_1_Windows__CStorage__CStorageFile_t;
#define __FIIterable_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("80646519-5e2a-595d-a8cd-2a24b4067f1b"))
IVectorView<ABI::Windows::Storage::StorageFile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageFile*> __FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03362e33-e413-5f29-97d0-48a4780935f9"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFile*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFile*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb4206c5-0988-5104-afa9-253c298f86fd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFile*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFile*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e52f8ce-aced-5a42-95b4-f674dd84885e"))
IAsyncOperation<ABI::Windows::Storage::StorageFile*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFile*> __FIAsyncOperation_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e521c894-2c26-5946-9e61-2b5e188d01ed"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6be9e7d7-e83a-5cbc-802c-1768960b52c3"))
IAsyncOperation<ABI::Windows::Storage::StorageFolder*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c211026e-9e63-5452-ba54-3a07d6a96874"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE */

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



#ifndef DEF___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("174f26c7-79ea-5f7c-bd70-ac4457f2cac8"))
IKeyValuePair<HSTRING, __FIVector_1_HSTRING*> : IKeyValuePair_impl<HSTRING, __FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, __FIVector_1_HSTRING*> __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a11824c9-e458-502a-afd8-ce3ce0abd6fe"))
IIterator<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVector`1<String>>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4fed2669-d0d3-59f6-91d9-95902d728d6a"))
IIterable<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVector`1<String>>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING___FIVector_1_HSTRING_USE
#define DEF___FIMapView_2_HSTRING___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("153f9c9c-d22a-5c9e-9c74-8b85c908b000"))
IMapView<HSTRING, __FIVector_1_HSTRING*> : IMapView_impl<HSTRING, __FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, __FIVector_1_HSTRING*> __FIMapView_2_HSTRING___FIVector_1_HSTRING_t;
#define __FIMapView_2_HSTRING___FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING___FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING___FIVector_1_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING___FIVector_1_HSTRING_USE
#define DEF___FIMap_2_HSTRING___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e475ca9d-6afb-5992-993e-53e6ef7a9ecd"))
IMap<HSTRING, __FIVector_1_HSTRING*> : IMap_impl<HSTRING, __FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, __FIVector_1_HSTRING*> __FIMap_2_HSTRING___FIVector_1_HSTRING_t;
#define __FIMap_2_HSTRING___FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING___FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                typedef enum PickerLocationId : int PickerLocationId;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                typedef enum PickerViewMode : int PickerViewMode;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                class FileOpenPicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                class FileSavePicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                class FolderPicker;
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.Pickers.PickerLocationId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                enum PickerLocationId : int
                {
                    PickerLocationId_DocumentsLibrary = 0,
                    PickerLocationId_ComputerFolder = 1,
                    PickerLocationId_Desktop = 2,
                    PickerLocationId_Downloads = 3,
                    PickerLocationId_HomeGroup = 4,
                    PickerLocationId_MusicLibrary = 5,
                    PickerLocationId_PicturesLibrary = 6,
                    PickerLocationId_VideosLibrary = 7,
                    PickerLocationId_Objects3D = 8,
                    PickerLocationId_Unspecified = 9,
                };
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.PickerViewMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                enum PickerViewMode : int
                {
                    PickerViewMode_List = 0,
                    PickerViewMode_Thumbnail = 1,
                };
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker[] = L"Windows.Storage.Pickers.IFileOpenPicker";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("2ca8278a-12c5-4c5f-8977-94547793c241")
                IFileOpenPicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewMode(
                        ABI::Windows::Storage::Pickers::PickerViewMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewMode(
                        ABI::Windows::Storage::Pickers::PickerViewMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SettingsIdentifier(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SettingsIdentifier(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CommitButtonText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CommitButtonText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileTypeFilter(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleFileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickMultipleFilesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPicker = __uuidof(IFileOpenPicker);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker2[] = L"Windows.Storage.Pickers.IFileOpenPicker2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("8ceb6cd2-b446-46f7-b265-90f8e55ad650")
                IFileOpenPicker2 : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickSingleFileAsync/PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ContinuationData(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE PickSingleFileAndContinue(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE PickMultipleFilesAndContinue(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPicker2 = __uuidof(IFileOpenPicker2);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker3[] = L"Windows.Storage.Pickers.IFileOpenPicker3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("d9a5c5b3-c5dc-5b98-bd80-a8d0ca0584d8")
                IFileOpenPicker3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPicker3 = __uuidof(IFileOpenPicker3);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerStatics[] = L"Windows.Storage.Pickers.IFileOpenPickerStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("6821573b-2f02-4833-96d4-abbfad72b67b")
                IFileOpenPickerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ResumePickSingleFileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerStatics = __uuidof(IFileOpenPickerStatics);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerStatics2[] = L"Windows.Storage.Pickers.IFileOpenPickerStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("e8917415-eddd-5c98-b6f3-366fdfcad392")
                IFileOpenPickerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Storage::Pickers::IFileOpenPicker** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerStatics2 = __uuidof(IFileOpenPickerStatics2);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerWithOperationId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerWithOperationId[] = L"Windows.Storage.Pickers.IFileOpenPickerWithOperationId";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("3f57b569-2522-4ca5-aa73-a15509f1fcbf")
                IFileOpenPickerWithOperationId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE PickSingleFileAsync(
                        HSTRING pickerOperationId,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerWithOperationId = __uuidof(IFileOpenPickerWithOperationId);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker[] = L"Windows.Storage.Pickers.IFileSavePicker";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("3286ffcb-617f-4cc5-af6a-b3fdf29ad145")
                IFileSavePicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SettingsIdentifier(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SettingsIdentifier(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CommitButtonText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CommitButtonText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileTypeChoices(
                        __FIMap_2_HSTRING___FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultFileExtension(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultFileExtension(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuggestedSaveFile(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuggestedSaveFile(
                        ABI::Windows::Storage::IStorageFile* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuggestedFileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuggestedFileName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSaveFileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePicker = __uuidof(IFileSavePicker);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker2[] = L"Windows.Storage.Pickers.IFileSavePicker2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("0ec313a2-d24b-449a-8197-e89104fd42cc")
                IFileSavePicker2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContinuationData(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickSaveFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE PickSaveFileAndContinue(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePicker2 = __uuidof(IFileSavePicker2);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker3[] = L"Windows.Storage.Pickers.IFileSavePicker3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("698aec69-ba3c-4e51-bd90-4abcbbf4cfaf")
                IFileSavePicker3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EnterpriseId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EnterpriseId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePicker3 = __uuidof(IFileSavePicker3);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker4[] = L"Windows.Storage.Pickers.IFileSavePicker4";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("e7d83a5a-ddfa-5de0-8b70-c842c21988ec")
                IFileSavePicker4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePicker4 = __uuidof(IFileSavePicker4);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePickerStatics[] = L"Windows.Storage.Pickers.IFileSavePickerStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("28e3cf9e-961c-5e2c-aed7-e64737f4ce37")
                IFileSavePickerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Storage::Pickers::IFileSavePicker** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePickerStatics = __uuidof(IFileSavePickerStatics);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker[] = L"Windows.Storage.Pickers.IFolderPicker";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("084f7799-f3fb-400a-99b1-7b4a772fd60d")
                IFolderPicker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewMode(
                        ABI::Windows::Storage::Pickers::PickerViewMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewMode(
                        ABI::Windows::Storage::Pickers::PickerViewMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SettingsIdentifier(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SettingsIdentifier(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuggestedStartLocation(
                        ABI::Windows::Storage::Pickers::PickerLocationId value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CommitButtonText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CommitButtonText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileTypeFilter(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PickSingleFolderAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFolderPicker = __uuidof(IFolderPicker);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker2[] = L"Windows.Storage.Pickers.IFolderPicker2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("8eb3ba97-dc85-4616-be94-9660881f2f5d")
                IFolderPicker2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContinuationData(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Instead, use PickSingleFolderAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE PickFolderAndContinue(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IFolderPicker2 = __uuidof(IFolderPicker2);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker3[] = L"Windows.Storage.Pickers.IFolderPicker3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("673b1e29-d326-53c0-bd24-a25c714cee36")
                IFolderPicker3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFolderPicker3 = __uuidof(IFolderPicker3);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPickerStatics[] = L"Windows.Storage.Pickers.IFolderPickerStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                MIDL_INTERFACE("9be34740-7ca1-5942-a3c8-46f2551ecff3")
                IFolderPickerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Storage::Pickers::IFolderPicker** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFolderPickerStatics = __uuidof(IFolderPickerStatics);
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Storage.Pickers.FileExtensionVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<String>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileExtensionVector_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileExtensionVector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileExtensionVector[] = L"Windows.Storage.Pickers.FileExtensionVector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FileOpenPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFileOpenPickerStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.Pickers.IFileOpenPickerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFileOpenPicker2
 *    Windows.Storage.Pickers.IFileOpenPickerWithOperationId
 *    Windows.Storage.Pickers.IFileOpenPicker ** Default Interface **
 *    Windows.Storage.Pickers.IFileOpenPicker3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileOpenPicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileOpenPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileOpenPicker[] = L"Windows.Storage.Pickers.FileOpenPicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FilePickerFileTypesOrderedMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, Windows.Foundation.Collections.IVector`1<String>> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVector`1<String>>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap[] = L"Windows.Storage.Pickers.FilePickerFileTypesOrderedMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FilePickerSelectedFilesArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFile>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FilePickerSelectedFilesArray_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FilePickerSelectedFilesArray_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FilePickerSelectedFilesArray[] = L"Windows.Storage.Pickers.FilePickerSelectedFilesArray";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FileSavePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFileSavePickerStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFileSavePicker2
 *    Windows.Storage.Pickers.IFileSavePicker3
 *    Windows.Storage.Pickers.IFileSavePicker ** Default Interface **
 *    Windows.Storage.Pickers.IFileSavePicker4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileSavePicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileSavePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileSavePicker[] = L"Windows.Storage.Pickers.FileSavePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FolderPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFolderPickerStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFolderPicker2
 *    Windows.Storage.Pickers.IFolderPicker ** Default Interface **
 *    Windows.Storage.Pickers.IFolderPicker3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FolderPicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FolderPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FolderPicker[] = L"Windows.Storage.Pickers.FolderPicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2 __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3 __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2 __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2 __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3 __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4 __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2 __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3 __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageFile __FIIterator_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageFile __FIIterable_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        __FIIterator_1_Windows__CStorage__CStorageFile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageFile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageFile __FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIVectorView_1_Windows__CStorage__CStorageFile** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile __FIAsyncOperation_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder __FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
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

#if !defined(____FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        __FIVector_1_HSTRING** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING___FIVector_1_HSTRING __FIMapView_2_HSTRING___FIVector_1_HSTRING;

#if !defined(____FIMapView_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING___FIVector_1_HSTRING __FIMapView_2_HSTRING___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING___FIVector_1_HSTRING;

typedef struct __FIMapView_2_HSTRING___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key,
        __FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING___FIVector_1_HSTRING* This,
        __FIMapView_2_HSTRING___FIVector_1_HSTRING** first,
        __FIMapView_2_HSTRING___FIVector_1_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING___FIVector_1_HSTRINGVtbl;

interface __FIMapView_2_HSTRING___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING___FIVector_1_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING___FIVector_1_HSTRING __FIMap_2_HSTRING___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING___FIVector_1_HSTRING;

typedef struct __FIMap_2_HSTRING___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key,
        __FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        __FIMapView_2_HSTRING___FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key,
        __FIVector_1_HSTRING* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING___FIVector_1_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING___FIVector_1_HSTRINGVtbl;

interface __FIMap_2_HSTRING___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING___FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING___FIVector_1_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId;

typedef enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode;

/*
 *
 * Struct Windows.Storage.Pickers.PickerLocationId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId
{
    PickerLocationId_DocumentsLibrary = 0,
    PickerLocationId_ComputerFolder = 1,
    PickerLocationId_Desktop = 2,
    PickerLocationId_Downloads = 3,
    PickerLocationId_HomeGroup = 4,
    PickerLocationId_MusicLibrary = 5,
    PickerLocationId_PicturesLibrary = 6,
    PickerLocationId_VideosLibrary = 7,
    PickerLocationId_Objects3D = 8,
    PickerLocationId_Unspecified = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.PickerViewMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode
{
    PickerViewMode_List = 0,
    PickerViewMode_Thumbnail = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker[] = L"Windows.Storage.Pickers.IFileOpenPicker";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewMode)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewMode)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode value);
    HRESULT (STDMETHODCALLTYPE* get_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId* value);
    HRESULT (STDMETHODCALLTYPE* put_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId value);
    HRESULT (STDMETHODCALLTYPE* get_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FileTypeFilter)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* PickSingleFileAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* PickMultipleFilesAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_get_ViewMode(This, value) \
    ((This)->lpVtbl->get_ViewMode(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_put_ViewMode(This, value) \
    ((This)->lpVtbl->put_ViewMode(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_get_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->get_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_put_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->put_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_get_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->get_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_put_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->put_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_get_CommitButtonText(This, value) \
    ((This)->lpVtbl->get_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_put_CommitButtonText(This, value) \
    ((This)->lpVtbl->put_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_get_FileTypeFilter(This, value) \
    ((This)->lpVtbl->get_FileTypeFilter(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_PickSingleFileAsync(This, operation) \
    ((This)->lpVtbl->PickSingleFileAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_PickMultipleFilesAsync(This, operation) \
    ((This)->lpVtbl->PickMultipleFilesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker2[] = L"Windows.Storage.Pickers.IFileOpenPicker2";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync/PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ContinuationData)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* PickSingleFileAndContinue)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* PickMultipleFilesAndContinue)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync/PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_get_ContinuationData(This, value) \
    ((This)->lpVtbl->get_ContinuationData(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_PickSingleFileAndContinue(This) \
    ((This)->lpVtbl->PickSingleFileAndContinue(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickMultipleFilesAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_PickMultipleFilesAndContinue(This) \
    ((This)->lpVtbl->PickMultipleFilesAndContinue(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPicker3[] = L"Windows.Storage.Pickers.IFileOpenPicker3";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerStatics[] = L"Windows.Storage.Pickers.IFileOpenPickerStatics";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ResumePickSingleFileAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_ResumePickSingleFileAsync(This, operation) \
    ((This)->lpVtbl->ResumePickSingleFileAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerStatics2[] = L"Windows.Storage.Pickers.IFileOpenPickerStatics2";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForUser)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPicker** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_CreateForUser(This, user, result) \
    ((This)->lpVtbl->CreateForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileOpenPickerWithOperationId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileOpenPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileOpenPickerWithOperationId[] = L"Windows.Storage.Pickers.IFileOpenPickerWithOperationId";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PickSingleFileAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId* This,
        HSTRING pickerOperationId,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationIdVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_PickSingleFileAsync(This, pickerOperationId, operation) \
    ((This)->lpVtbl->PickSingleFileAsync(This, pickerOperationId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileOpenPickerWithOperationId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker[] = L"Windows.Storage.Pickers.IFileSavePicker";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId* value);
    HRESULT (STDMETHODCALLTYPE* put_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId value);
    HRESULT (STDMETHODCALLTYPE* get_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FileTypeChoices)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        __FIMap_2_HSTRING___FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultFileExtension)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultFileExtension)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedSaveFile)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* put_SuggestedSaveFile)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedFileName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SuggestedFileName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* PickSaveFileAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->get_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->put_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->get_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->put_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_CommitButtonText(This, value) \
    ((This)->lpVtbl->get_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_CommitButtonText(This, value) \
    ((This)->lpVtbl->put_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_FileTypeChoices(This, value) \
    ((This)->lpVtbl->get_FileTypeChoices(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_DefaultFileExtension(This, value) \
    ((This)->lpVtbl->get_DefaultFileExtension(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_DefaultFileExtension(This, value) \
    ((This)->lpVtbl->put_DefaultFileExtension(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_SuggestedSaveFile(This, value) \
    ((This)->lpVtbl->get_SuggestedSaveFile(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_SuggestedSaveFile(This, value) \
    ((This)->lpVtbl->put_SuggestedSaveFile(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_get_SuggestedFileName(This, value) \
    ((This)->lpVtbl->get_SuggestedFileName(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_put_SuggestedFileName(This, value) \
    ((This)->lpVtbl->put_SuggestedFileName(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_PickSaveFileAsync(This, operation) \
    ((This)->lpVtbl->PickSaveFileAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker2[] = L"Windows.Storage.Pickers.IFileSavePicker2";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContinuationData)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSaveFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* PickSaveFileAndContinue)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_get_ContinuationData(This, value) \
    ((This)->lpVtbl->get_ContinuationData(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSaveFileAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_PickSaveFileAndContinue(This) \
    ((This)->lpVtbl->PickSaveFileAndContinue(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker3[] = L"Windows.Storage.Pickers.IFileSavePicker3";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnterpriseId)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_EnterpriseId)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_get_EnterpriseId(This, value) \
    ((This)->lpVtbl->get_EnterpriseId(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_put_EnterpriseId(This, value) \
    ((This)->lpVtbl->put_EnterpriseId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePicker4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePicker4[] = L"Windows.Storage.Pickers.IFileSavePicker4";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFileSavePickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FileSavePicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFileSavePickerStatics[] = L"Windows.Storage.Pickers.IFileSavePickerStatics";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForUser)(__x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CStorage_CPickers_CIFileSavePicker** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_CreateForUser(This, user, result) \
    ((This)->lpVtbl->CreateForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFileSavePickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker[] = L"Windows.Storage.Pickers.IFolderPicker";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewMode)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewMode)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerViewMode value);
    HRESULT (STDMETHODCALLTYPE* get_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId* value);
    HRESULT (STDMETHODCALLTYPE* put_SuggestedStartLocation)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CPickerLocationId value);
    HRESULT (STDMETHODCALLTYPE* get_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CommitButtonText)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FileTypeFilter)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* PickSingleFolderAsync)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_get_ViewMode(This, value) \
    ((This)->lpVtbl->get_ViewMode(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_put_ViewMode(This, value) \
    ((This)->lpVtbl->put_ViewMode(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_get_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->get_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_put_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->put_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_get_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->get_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_put_SuggestedStartLocation(This, value) \
    ((This)->lpVtbl->put_SuggestedStartLocation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_get_CommitButtonText(This, value) \
    ((This)->lpVtbl->get_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_put_CommitButtonText(This, value) \
    ((This)->lpVtbl->put_CommitButtonText(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_get_FileTypeFilter(This, value) \
    ((This)->lpVtbl->get_FileTypeFilter(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_PickSingleFolderAsync(This, operation) \
    ((This)->lpVtbl->PickSingleFolderAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker2[] = L"Windows.Storage.Pickers.IFolderPicker2";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContinuationData)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFolderAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* PickFolderAndContinue)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_get_ContinuationData(This, value) \
    ((This)->lpVtbl->get_ContinuationData(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Instead, use PickSingleFolderAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_PickFolderAndContinue(This) \
    ((This)->lpVtbl->PickFolderAndContinue(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPicker3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPicker3[] = L"Windows.Storage.Pickers.IFolderPicker3";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3Vtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPicker3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Storage.Pickers.IFolderPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.FolderPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_IFolderPickerStatics[] = L"Windows.Storage.Pickers.IFolderPickerStatics";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForUser)(__x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CStorage_CPickers_CIFolderPicker** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_CreateForUser(This, user, result) \
    ((This)->lpVtbl->CreateForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CIFolderPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Storage.Pickers.FileExtensionVector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<String> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<String>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileExtensionVector_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileExtensionVector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileExtensionVector[] = L"Windows.Storage.Pickers.FileExtensionVector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FileOpenPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFileOpenPickerStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.Pickers.IFileOpenPickerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFileOpenPicker2
 *    Windows.Storage.Pickers.IFileOpenPickerWithOperationId
 *    Windows.Storage.Pickers.IFileOpenPicker ** Default Interface **
 *    Windows.Storage.Pickers.IFileOpenPicker3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileOpenPicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileOpenPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileOpenPicker[] = L"Windows.Storage.Pickers.FileOpenPicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FilePickerFileTypesOrderedMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<String, Windows.Foundation.Collections.IVector`1<String>> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Foundation.Collections.IVector`1<String>>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FilePickerFileTypesOrderedMap[] = L"Windows.Storage.Pickers.FilePickerFileTypesOrderedMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FilePickerSelectedFilesArray
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFile>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FilePickerSelectedFilesArray_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FilePickerSelectedFilesArray_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FilePickerSelectedFilesArray[] = L"Windows.Storage.Pickers.FilePickerSelectedFilesArray";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FileSavePicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFileSavePickerStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFileSavePicker2
 *    Windows.Storage.Pickers.IFileSavePicker3
 *    Windows.Storage.Pickers.IFileSavePicker ** Default Interface **
 *    Windows.Storage.Pickers.IFileSavePicker4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FileSavePicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FileSavePicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FileSavePicker[] = L"Windows.Storage.Pickers.FileSavePicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.FolderPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.Pickers.IFolderPickerStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.IFolderPicker2
 *    Windows.Storage.Pickers.IFolderPicker ** Default Interface **
 *    Windows.Storage.Pickers.IFolderPicker3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_FolderPicker_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_FolderPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_FolderPicker[] = L"Windows.Storage.Pickers.FolderPicker";
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
#endif // __windows2Estorage2Epickers_p_h__

#endif // __windows2Estorage2Epickers_h__
