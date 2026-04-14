
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
#ifndef __windows2Estorage2Epickers2Eprovider_h__
#define __windows2Estorage2Epickers2Eprovider_h__
#ifndef __windows2Estorage2Epickers2Eprovider_p_h__
#define __windows2Estorage2Epickers2Eprovider_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IFileOpenPickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI ABI::Windows::Storage::Pickers::Provider::IFileOpenPickerUI

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IFileRemovedEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs ABI::Windows::Storage::Pickers::Provider::IFileRemovedEventArgs

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IFileSavePickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI ABI::Windows::Storage::Pickers::Provider::IFileSavePickerUI

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IPickerClosingDeferral;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral ABI::Windows::Storage::Pickers::Provider::IPickerClosingDeferral

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IPickerClosingEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs ABI::Windows::Storage::Pickers::Provider::IPickerClosingEventArgs

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IPickerClosingOperation;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation ABI::Windows::Storage::Pickers::Provider::IPickerClosingOperation

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface ITargetFileRequest;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest ABI::Windows::Storage::Pickers::Provider::ITargetFileRequest

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface ITargetFileRequestDeferral;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral ABI::Windows::Storage::Pickers::Provider::ITargetFileRequestDeferral

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface ITargetFileRequestedEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs ABI::Windows::Storage::Pickers::Provider::ITargetFileRequestedEventArgs

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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


namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class FileOpenPickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class FileRemovedEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f1fb2939-695b-5f56-841a-a52a7d148572"))
ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::FileRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::IFileOpenPickerUI*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::FileRemovedEventArgs*, ABI::Windows::Storage::Pickers::Provider::IFileRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.Pickers.Provider.FileOpenPickerUI, Windows.Storage.Pickers.Provider.FileRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::FileRemovedEventArgs*> __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class PickerClosingEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2b06bdac-983b-5552-b5c9-b0990a2db3a1"))
ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::PickerClosingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::IFileOpenPickerUI*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::PickerClosingEventArgs*, ABI::Windows::Storage::Pickers::Provider::IPickerClosingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.Pickers.Provider.FileOpenPickerUI, Windows.Storage.Pickers.Provider.PickerClosingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileOpenPickerUI*, ABI::Windows::Storage::Pickers::Provider::PickerClosingEventArgs*> __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class FileSavePickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f1e6a632-f97f-54d9-9e1b-a714edc3ff06"))
ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, ABI::Windows::Storage::Pickers::Provider::IFileSavePickerUI*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.Pickers.Provider.FileSavePickerUI, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, IInspectable*> __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class TargetFileRequestedEventArgs;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3e1f8c7-45c5-5249-b336-a111bfaa985b"))
ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, ABI::Windows::Storage::Pickers::Provider::TargetFileRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, ABI::Windows::Storage::Pickers::Provider::IFileSavePickerUI*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Pickers::Provider::TargetFileRequestedEventArgs*, ABI::Windows::Storage::Pickers::Provider::ITargetFileRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.Pickers.Provider.FileSavePickerUI, Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::Pickers::Provider::FileSavePickerUI*, ABI::Windows::Storage::Pickers::Provider::TargetFileRequestedEventArgs*> __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
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

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    typedef enum AddFileResult : int AddFileResult;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    typedef enum FileSelectionMode : int FileSelectionMode;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    typedef enum SetFileNameResult : int SetFileNameResult;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class PickerClosingDeferral;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class PickerClosingOperation;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class TargetFileRequest;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class TargetFileRequestDeferral;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.Pickers.Provider.AddFileResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    enum AddFileResult : int
                    {
                        AddFileResult_Added = 0,
                        AddFileResult_AlreadyAdded = 1,
                        AddFileResult_NotAllowed = 2,
                        AddFileResult_Unavailable = 3,
                    };
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.Provider.FileSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    enum FileSelectionMode : int
                    {
                        FileSelectionMode_Single = 0,
                        FileSelectionMode_Multiple = 1,
                    };
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.Provider.SetFileNameResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    enum SetFileNameResult : int
                    {
                        SetFileNameResult_Succeeded = 0,
                        SetFileNameResult_NotAllowed = 1,
                        SetFileNameResult_Unavailable = 2,
                    };
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileOpenPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileOpenPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileOpenPickerUI[] = L"Windows.Storage.Pickers.Provider.IFileOpenPickerUI";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("dda45a10-f9d4-40c4-8af5-c5b6b5a61d1d")
                    IFileOpenPickerUI : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddFile(
                            HSTRING id,
                            ABI::Windows::Storage::IStorageFile* file,
                            ABI::Windows::Storage::Pickers::Provider::AddFileResult* addResult
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveFile(
                            HSTRING id
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ContainsFile(
                            HSTRING id,
                            boolean* isContained
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CanAddFile(
                            ABI::Windows::Storage::IStorageFile* file,
                            boolean* canAdd
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllowedFileTypes(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectionMode(
                            ABI::Windows::Storage::Pickers::Provider::FileSelectionMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SettingsIdentifier(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Title(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Title(
                            HSTRING value
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_FileRemoved(
                            __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_FileRemoved(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Closing(
                            __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Closing(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFileOpenPickerUI = __uuidof(IFileOpenPickerUI);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileRemovedEventArgs[] = L"Windows.Storage.Pickers.Provider.IFileRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("13043da7-7fca-4c2b-9eca-6890f9f00185")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    IFileRemovedEventArgs : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFileRemovedEventArgs = __uuidof(IFileRemovedEventArgs);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileSavePickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileSavePickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileSavePickerUI[] = L"Windows.Storage.Pickers.Provider.IFileSavePickerUI";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("9656c1e7-3e56-43cc-8a39-33c73d9d542b")
                    IFileSavePickerUI : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Title(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Title(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllowedFileTypes(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SettingsIdentifier(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FileName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TrySetFileName(
                            HSTRING value,
                            ABI::Windows::Storage::Pickers::Provider::SetFileNameResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_FileNameChanged(
                            __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FileNameChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_TargetFileRequested(
                            __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_TargetFileRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IFileSavePickerUI = __uuidof(IFileSavePickerUI);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingDeferral[] = L"Windows.Storage.Pickers.Provider.IPickerClosingDeferral";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("7af7f71e-1a67-4a31-ae80-e907708a619b")
                    IPickerClosingDeferral : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPickerClosingDeferral = __uuidof(IPickerClosingDeferral);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingEventArgs[] = L"Windows.Storage.Pickers.Provider.IPickerClosingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("7e59f224-b332-4f12-8b9f-a8c2f06b32cd")
                    IPickerClosingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ClosingOperation(
                            ABI::Windows::Storage::Pickers::Provider::IPickerClosingOperation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPickerClosingEventArgs = __uuidof(IPickerClosingEventArgs);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingOperation[] = L"Windows.Storage.Pickers.Provider.IPickerClosingOperation";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("4ce9fb84-beee-4e39-a773-fc5f0eae328d")
                    IPickerClosingOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Storage::Pickers::Provider::IPickerClosingDeferral** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPickerClosingOperation = __uuidof(IPickerClosingOperation);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequest[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequest";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("42bd3355-7f88-478b-8e81-690b20340678")
                    ITargetFileRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TargetFile(
                            ABI::Windows::Storage::IStorageFile** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TargetFile(
                            ABI::Windows::Storage::IStorageFile* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Storage::Pickers::Provider::ITargetFileRequestDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITargetFileRequest = __uuidof(ITargetFileRequest);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequestDeferral[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("4aee9d91-bf15-4da9-95f6-f6b7d558225b")
                    ITargetFileRequestDeferral : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITargetFileRequestDeferral = __uuidof(ITargetFileRequestDeferral);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequestedEventArgs[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    MIDL_INTERFACE("b163dbc1-1b51-4c89-a591-0fd40b3c57c9")
                    ITargetFileRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Storage::Pickers::Provider::ITargetFileRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITargetFileRequestedEventArgs = __uuidof(ITargetFileRequestedEventArgs);
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileOpenPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileOpenPickerUI ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileOpenPickerUI_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileOpenPickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileOpenPickerUI[] = L"Windows.Storage.Pickers.Provider.FileOpenPickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileRemovedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileRemovedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileRemovedEventArgs[] = L"Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileSavePickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileSavePickerUI ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileSavePickerUI_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileSavePickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileSavePickerUI[] = L"Windows.Storage.Pickers.Provider.FileSavePickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingDeferral ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingDeferral[] = L"Windows.Storage.Pickers.Provider.PickerClosingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingEventArgs[] = L"Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingOperation ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingOperation[] = L"Windows.Storage.Pickers.Provider.PickerClosingOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequest ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequest[] = L"Windows.Storage.Pickers.Provider.TargetFileRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral[] = L"Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs[] = L"Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* sender,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* sender,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* sender,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CAddFileResult __x_ABI_CWindows_CStorage_CPickers_CProvider_CAddFileResult;

typedef enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CFileSelectionMode __x_ABI_CWindows_CStorage_CPickers_CProvider_CFileSelectionMode;

typedef enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CSetFileNameResult __x_ABI_CWindows_CStorage_CPickers_CProvider_CSetFileNameResult;

/*
 *
 * Struct Windows.Storage.Pickers.Provider.AddFileResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CAddFileResult
{
    AddFileResult_Added = 0,
    AddFileResult_AlreadyAdded = 1,
    AddFileResult_NotAllowed = 2,
    AddFileResult_Unavailable = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.Provider.FileSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CFileSelectionMode
{
    FileSelectionMode_Single = 0,
    FileSelectionMode_Multiple = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.Pickers.Provider.SetFileNameResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CSetFileNameResult
{
    SetFileNameResult_Succeeded = 0,
    SetFileNameResult_NotAllowed = 1,
    SetFileNameResult_Unavailable = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileOpenPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileOpenPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileOpenPickerUI[] = L"Windows.Storage.Pickers.Provider.IFileOpenPickerUI";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUIVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING id,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CAddFileResult* addResult);
    HRESULT (STDMETHODCALLTYPE* RemoveFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING id);
    HRESULT (STDMETHODCALLTYPE* ContainsFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING id,
        boolean* isContained);
    HRESULT (STDMETHODCALLTYPE* CanAddFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        boolean* canAdd);
    HRESULT (STDMETHODCALLTYPE* get_AllowedFileTypes)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionMode)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CFileSelectionMode* value);
    HRESULT (STDMETHODCALLTYPE* get_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_FileRemoved)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CFileRemovedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_FileRemoved)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Closing)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileOpenPickerUI_Windows__CStorage__CPickers__CProvider__CPickerClosingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closing)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUIVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUIVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_AddFile(This, id, file, addResult) \
    ((This)->lpVtbl->AddFile(This, id, file, addResult))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_RemoveFile(This, id) \
    ((This)->lpVtbl->RemoveFile(This, id))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_ContainsFile(This, id, isContained) \
    ((This)->lpVtbl->ContainsFile(This, id, isContained))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_CanAddFile(This, file, canAdd) \
    ((This)->lpVtbl->CanAddFile(This, file, canAdd))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_get_AllowedFileTypes(This, value) \
    ((This)->lpVtbl->get_AllowedFileTypes(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_get_SelectionMode(This, value) \
    ((This)->lpVtbl->get_SelectionMode(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_get_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->get_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_add_FileRemoved(This, handler, token) \
    ((This)->lpVtbl->add_FileRemoved(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_remove_FileRemoved(This, token) \
    ((This)->lpVtbl->remove_FileRemoved(This, token))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_add_Closing(This, handler, token) \
    ((This)->lpVtbl->add_Closing(This, handler, token))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_remove_Closing(This, token) \
    ((This)->lpVtbl->remove_Closing(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileRemovedEventArgs[] = L"Windows.Storage.Pickers.Provider.IFileRemovedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IFileSavePickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.FileSavePickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IFileSavePickerUI[] = L"Windows.Storage.Pickers.Provider.IFileSavePickerUI";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUIVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AllowedFileTypes)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SettingsIdentifier)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FileName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* TrySetFileName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        HSTRING value,
        enum __x_ABI_CWindows_CStorage_CPickers_CProvider_CSetFileNameResult* result);
    HRESULT (STDMETHODCALLTYPE* add_FileNameChanged)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FileNameChanged)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TargetFileRequested)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        __FITypedEventHandler_2_Windows__CStorage__CPickers__CProvider__CFileSavePickerUI_Windows__CStorage__CPickers__CProvider__CTargetFileRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TargetFileRequested)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUIVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUIVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_get_AllowedFileTypes(This, value) \
    ((This)->lpVtbl->get_AllowedFileTypes(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_get_SettingsIdentifier(This, value) \
    ((This)->lpVtbl->get_SettingsIdentifier(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_get_FileName(This, value) \
    ((This)->lpVtbl->get_FileName(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_TrySetFileName(This, value, result) \
    ((This)->lpVtbl->TrySetFileName(This, value, result))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_add_FileNameChanged(This, handler, token) \
    ((This)->lpVtbl->add_FileNameChanged(This, handler, token))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_remove_FileNameChanged(This, token) \
    ((This)->lpVtbl->remove_FileNameChanged(This, token))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_add_TargetFileRequested(This, handler, token) \
    ((This)->lpVtbl->add_TargetFileRequested(This, handler, token))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_remove_TargetFileRequested(This, token) \
    ((This)->lpVtbl->remove_TargetFileRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingDeferral[] = L"Windows.Storage.Pickers.Provider.IPickerClosingDeferral";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferralVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingEventArgs[] = L"Windows.Storage.Pickers.Provider.IPickerClosingEventArgs";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClosingOperation)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation** value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_get_ClosingOperation(This, value) \
    ((This)->lpVtbl->get_ClosingOperation(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.IPickerClosingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.PickerClosingOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_IPickerClosingOperation[] = L"Windows.Storage.Pickers.Provider.IPickerClosingOperation";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingDeferral** value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperationVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CIPickerClosingOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequest[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequest";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TargetFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* put_TargetFile)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_get_TargetFile(This, value) \
    ((This)->lpVtbl->get_TargetFile(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_put_TargetFile(This, value) \
    ((This)->lpVtbl->put_TargetFile(This, value))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequestDeferral[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferralVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_Pickers_Provider_ITargetFileRequestedEventArgs[] = L"Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs";
typedef struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CStorage_CPickers_CProvider_CITargetFileRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileOpenPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileOpenPickerUI ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileOpenPickerUI_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileOpenPickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileOpenPickerUI[] = L"Windows.Storage.Pickers.Provider.FileOpenPickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileRemovedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileRemovedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("Since Windows 10, only apps can remove files, not end users so the FileRemoved event will not be raised.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileRemovedEventArgs[] = L"Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.FileSavePickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IFileSavePickerUI ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileSavePickerUI_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_FileSavePickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_FileSavePickerUI[] = L"Windows.Storage.Pickers.Provider.FileSavePickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingDeferral ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingDeferral[] = L"Windows.Storage.Pickers.Provider.PickerClosingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingEventArgs[] = L"Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.PickerClosingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.IPickerClosingOperation ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingOperation_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_PickerClosingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_PickerClosingOperation[] = L"Windows.Storage.Pickers.Provider.PickerClosingOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequest ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequest[] = L"Windows.Storage.Pickers.Provider.TargetFileRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequestDeferral ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequestDeferral[] = L"Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Pickers.Provider.ITargetFileRequestedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_Pickers_Provider_TargetFileRequestedEventArgs[] = L"Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
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
#endif // __windows2Estorage2Epickers2Eprovider_p_h__

#endif // __windows2Estorage2Epickers2Eprovider_h__
